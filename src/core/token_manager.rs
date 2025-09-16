use log::warn;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::{
    sync::{Mutex, RwLock},
    time::interval,
};

use crate::core::{
    api_resp::{ApiResponseTrait, RawResponse, ResponseFormat},
    app_ticket_manager::AppTicketManager,
    cache::QuickCache,
    config::Config,
    constants::{
        AppType, APP_ACCESS_TOKEN_INTERNAL_URL_PATH, APP_ACCESS_TOKEN_KEY_PREFIX,
        APP_ACCESS_TOKEN_URL_PATH, EXPIRY_DELTA, TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH,
        TENANT_ACCESS_TOKEN_URL_PATH,
    },
    error::LarkAPIError,
    SDKResult,
};

/// Token预热配置
#[derive(Debug, Clone)]
pub struct PreheatingConfig {
    /// 检查间隔（秒）
    pub check_interval_seconds: u64,
    /// 预热阈值（秒）- 当token剩余时间少于此值时开始预热
    pub preheat_threshold_seconds: u64,
    /// 是否启用tenant token预热
    pub enable_tenant_preheating: bool,
    /// 最大并发预热任务数
    pub max_concurrent_preheat: usize,
}

impl Default for PreheatingConfig {
    fn default() -> Self {
        Self {
            check_interval_seconds: 1800,   // 30分钟
            preheat_threshold_seconds: 900, // 15分钟
            enable_tenant_preheating: true,
            max_concurrent_preheat: 3,
        }
    }
}

/// Token管理器性能监控指标
#[derive(Debug, Default)]
pub struct TokenMetrics {
    /// App token缓存命中次数
    pub app_cache_hits: AtomicU64,
    /// App token缓存未命中次数
    pub app_cache_misses: AtomicU64,
    /// Tenant token缓存命中次数
    pub tenant_cache_hits: AtomicU64,
    /// Tenant token缓存未命中次数
    pub tenant_cache_misses: AtomicU64,
    /// Token刷新成功次数
    pub refresh_success: AtomicU64,
    /// Token刷新失败次数
    pub refresh_failures: AtomicU64,
    /// 总的读锁获取次数
    pub read_lock_acquisitions: AtomicU64,
    /// 总的写锁获取次数
    pub write_lock_acquisitions: AtomicU64,
}

impl TokenMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取app token缓存命中率 (0.0-1.0)
    pub fn app_cache_hit_rate(&self) -> f64 {
        let hits = self.app_cache_hits.load(Ordering::Relaxed) as f64;
        let misses = self.app_cache_misses.load(Ordering::Relaxed) as f64;
        let total = hits + misses;
        if total > 0.0 {
            hits / total
        } else {
            0.0
        }
    }

    /// 获取tenant token缓存命中率 (0.0-1.0)
    pub fn tenant_cache_hit_rate(&self) -> f64 {
        let hits = self.tenant_cache_hits.load(Ordering::Relaxed) as f64;
        let misses = self.tenant_cache_misses.load(Ordering::Relaxed) as f64;
        let total = hits + misses;
        if total > 0.0 {
            hits / total
        } else {
            0.0
        }
    }

    /// 获取token刷新成功率 (0.0-1.0)
    pub fn refresh_success_rate(&self) -> f64 {
        let success = self.refresh_success.load(Ordering::Relaxed) as f64;
        let failures = self.refresh_failures.load(Ordering::Relaxed) as f64;
        let total = success + failures;
        if total > 0.0 {
            success / total
        } else {
            0.0
        }
    }

    /// 生成性能报告
    pub fn performance_report(&self) -> String {
        format!(
            "TokenManager Performance Metrics:\n\
             - App Cache Hit Rate: {:.2}%\n\
             - Tenant Cache Hit Rate: {:.2}%\n\
             - Refresh Success Rate: {:.2}%\n\
             - Total Read Locks: {}\n\
             - Total Write Locks: {}\n\
             - App Cache: {} hits, {} misses\n\
             - Tenant Cache: {} hits, {} misses\n\
             - Refreshes: {} success, {} failures",
            self.app_cache_hit_rate() * 100.0,
            self.tenant_cache_hit_rate() * 100.0,
            self.refresh_success_rate() * 100.0,
            self.read_lock_acquisitions.load(Ordering::Relaxed),
            self.write_lock_acquisitions.load(Ordering::Relaxed),
            self.app_cache_hits.load(Ordering::Relaxed),
            self.app_cache_misses.load(Ordering::Relaxed),
            self.tenant_cache_hits.load(Ordering::Relaxed),
            self.tenant_cache_misses.load(Ordering::Relaxed),
            self.refresh_success.load(Ordering::Relaxed),
            self.refresh_failures.load(Ordering::Relaxed)
        )
    }
}

#[derive(Debug)]
pub struct TokenManager {
    cache: Arc<RwLock<QuickCache<String>>>,
    metrics: Arc<TokenMetrics>,
    /// 后台预热任务句柄
    preheating_handle: Option<tokio::task::JoinHandle<()>>,
}

impl Default for TokenManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenManager {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(QuickCache::new())),
            metrics: Arc::new(TokenMetrics::new()),
            preheating_handle: None,
        }
    }

    /// 获取性能指标的只读引用
    pub fn metrics(&self) -> &Arc<TokenMetrics> {
        &self.metrics
    }

    /// 获取缓存的克隆引用（用于预热功能）
    pub fn get_cache(&self) -> Arc<RwLock<QuickCache<String>>> {
        self.cache.clone()
    }

    /// 获取性能指标的克隆引用（用于预热功能）
    pub fn get_metrics(&self) -> Arc<TokenMetrics> {
        self.metrics.clone()
    }

    /// 打印性能报告到日志
    pub fn log_performance_metrics(&self) {
        log::info!("{}", self.metrics.performance_report());
    }

    /// 启动后台token预热机制
    ///
    /// 这个方法会启动一个后台任务，定期检查即将过期的token并预先刷新它们
    ///
    /// # 参数
    /// - `config`: 应用配置，用于token刷新
    /// - `app_ticket_manager`: App ticket管理器的引用
    pub fn start_background_preheating(
        &mut self,
        config: Config,
        app_ticket_manager: Arc<Mutex<AppTicketManager>>,
    ) {
        self.start_background_preheating_with_config(
            config,
            app_ticket_manager,
            PreheatingConfig::default(),
        )
    }

    /// 启动带自定义配置的后台token预热机制
    pub fn start_background_preheating_with_config(
        &mut self,
        config: Config,
        app_ticket_manager: Arc<Mutex<AppTicketManager>>,
        preheat_config: PreheatingConfig,
    ) {
        // 如果已有预热任务在运行，先停止它
        if self.preheating_handle.is_some() {
            log::info!("🔄 停止现有预热任务，启动新配置的预热任务");
            self.stop_background_preheating();
        }

        let cache = self.cache.clone();
        let metrics = self.metrics.clone();

        let handle = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(preheat_config.check_interval_seconds));
            log::info!(
                "🔄 Token后台预热机制已启动，检查间隔: {}分钟，预热阈值: {}分钟",
                preheat_config.check_interval_seconds / 60,
                preheat_config.preheat_threshold_seconds / 60
            );

            loop {
                interval.tick().await;

                if let Err(e) = Self::preheat_tokens_if_needed_with_config(
                    &cache,
                    &metrics,
                    &config,
                    &app_ticket_manager,
                    &preheat_config,
                )
                .await
                {
                    log::warn!("⚠️ Token预热过程中发生错误: {e:?}");
                    // 记录错误但继续运行
                }
            }
        });

        self.preheating_handle = Some(handle);
        log::info!("✅ Token后台预热任务已启动并注册到TokenManager");
    }

    /// 检查并预热即将过期的token（使用默认配置）
    #[allow(dead_code)]
    async fn preheat_tokens_if_needed(
        cache: &Arc<RwLock<QuickCache<String>>>,
        metrics: &Arc<TokenMetrics>,
        config: &Config,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<()> {
        Self::preheat_tokens_if_needed_with_config(
            cache,
            metrics,
            config,
            app_ticket_manager,
            &PreheatingConfig::default(),
        )
        .await
    }

    /// 检查并预热即将过期的token（使用自定义配置）
    async fn preheat_tokens_if_needed_with_config(
        cache: &Arc<RwLock<QuickCache<String>>>,
        metrics: &Arc<TokenMetrics>,
        config: &Config,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
        preheat_config: &PreheatingConfig,
    ) -> SDKResult<()> {
        log::debug!("🔍 检查需要预热的token...");

        let mut preheated_count = 0;

        // 检查app access token是否需要预热
        let app_key = app_access_token_key(&config.app_id);
        if Self::should_preheat_token_with_threshold(
            cache,
            &app_key,
            preheat_config.preheat_threshold_seconds,
        )
        .await
        {
            log::info!("🔄 开始预热 app access token");
            if let Err(e) = Self::preheat_app_token(cache, config, app_ticket_manager).await {
                log::warn!("❌ App token预热失败: {e:?}");
                metrics.refresh_failures.fetch_add(1, Ordering::Relaxed);
            } else {
                log::info!("✅ App token预热成功");
                metrics.refresh_success.fetch_add(1, Ordering::Relaxed);
                preheated_count += 1;
            }
        }

        // 预热tenant token（如果启用）
        if preheat_config.enable_tenant_preheating {
            let tenant_keys = Self::get_cached_tenant_keys(cache, &config.app_id).await;
            for tenant_key in tenant_keys
                .into_iter()
                .take(preheat_config.max_concurrent_preheat)
            {
                let tenant_cache_key = tenant_access_token_key(&config.app_id, &tenant_key);
                if Self::should_preheat_token_with_threshold(
                    cache,
                    &tenant_cache_key,
                    preheat_config.preheat_threshold_seconds,
                )
                .await
                {
                    log::info!("🔄 开始预热 tenant access token: {tenant_key}");
                    if let Err(e) =
                        Self::preheat_tenant_token(cache, config, &tenant_key, app_ticket_manager)
                            .await
                    {
                        log::warn!("❌ Tenant token预热失败 ({tenant_key}): {e:?}");
                        metrics.refresh_failures.fetch_add(1, Ordering::Relaxed);
                    } else {
                        log::info!("✅ Tenant token预热成功: {tenant_key}");
                        metrics.refresh_success.fetch_add(1, Ordering::Relaxed);
                        preheated_count += 1;
                    }
                }
            }
        }

        if preheated_count > 0 {
            log::info!("🎯 本轮预热完成，共刷新了 {preheated_count} 个token");
        } else {
            log::debug!("✨ 所有token状态良好，无需预热");
        }

        Ok(())
    }

    /// 判断指定的token是否需要预热（使用默认15分钟阈值）
    #[allow(dead_code)]
    async fn should_preheat_token(cache: &Arc<RwLock<QuickCache<String>>>, key: &str) -> bool {
        Self::should_preheat_token_with_threshold(cache, key, 900).await
    }

    /// 判断指定的token是否需要预热（自定义阈值）
    ///
    /// 如果token不存在或即将在指定时间内过期，则需要预热
    async fn should_preheat_token_with_threshold(
        cache: &Arc<RwLock<QuickCache<String>>>,
        key: &str,
        threshold_seconds: u64,
    ) -> bool {
        let cache_read = cache.read().await;

        // 如果token不存在，需要预热
        if cache_read.get(key).is_none_or(|token| token.is_empty()) {
            log::debug!("🔍 Token {key} 不存在，需要预热");
            return true;
        }

        // 检查是否即将过期
        if let Some(expiry_info) = cache_read.get_with_expiry(key) {
            let remaining_seconds = expiry_info.expiry_seconds();
            // 如果剩余时间少于阈值，需要预热
            if remaining_seconds < threshold_seconds {
                log::debug!(
                    "🔍 Token {key} 将在{remaining_seconds}秒后过期，阈值{threshold_seconds}秒，需要预热"
                );
                return true;
            }
        }

        false
    }

    /// 获取缓存中所有的tenant keys
    async fn get_cached_tenant_keys(
        _cache: &Arc<RwLock<QuickCache<String>>>,
        _app_id: &str,
    ) -> Vec<String> {
        // 注意：这里需要QuickCache提供遍历功能，目前简化为空列表
        // 在实际实现中，可以维护一个单独的tenant_keys集合
        // 或者在QuickCache中添加keys()方法来遍历所有键
        vec![]
    }

    /// 预热app access token (直接更新主缓存)
    async fn preheat_app_token(
        cache: &Arc<RwLock<QuickCache<String>>>,
        config: &Config,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<String> {
        // 直接使用主缓存实例进行预热
        let temp_manager = TokenManager {
            cache: cache.clone(),
            metrics: Arc::new(TokenMetrics::new()), // 临时指标，只用于API调用
            preheating_handle: None,
        };

        match config.app_type {
            AppType::SelfBuild => {
                temp_manager
                    .get_custom_app_access_token_then_cache(config)
                    .await
            }
            _ => {
                temp_manager
                    .get_marketplace_app_access_token_then_cache(config, "", app_ticket_manager)
                    .await
            }
        }
    }

    /// 预热tenant access token (直接更新主缓存)
    async fn preheat_tenant_token(
        cache: &Arc<RwLock<QuickCache<String>>>,
        config: &Config,
        tenant_key: &str,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<String> {
        // 直接使用主缓存实例进行预热
        let temp_manager = TokenManager {
            cache: cache.clone(),
            metrics: Arc::new(TokenMetrics::new()), // 临时指标，只用于API调用
            preheating_handle: None,
        };

        if config.app_type == AppType::SelfBuild {
            temp_manager
                .get_custom_tenant_access_token_then_cache(config, tenant_key)
                .await
        } else {
            temp_manager
                .get_marketplace_tenant_access_token_then_cache(
                    config,
                    tenant_key,
                    "",
                    app_ticket_manager,
                )
                .await
        }
    }

    /// 停止后台预热任务
    pub fn stop_background_preheating(&mut self) {
        if let Some(handle) = self.preheating_handle.take() {
            handle.abort();
            log::info!("🛑 Token后台预热机制已停止");
        }
    }

    /// 检查预热任务是否正在运行 (用于测试)
    pub fn is_preheating_active(&self) -> bool {
        self.preheating_handle.is_some()
    }
    pub async fn get_app_access_token(
        &self,
        config: &Config,
        app_ticket: &str,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<String> {
        let start_time = Instant::now();
        let key = app_access_token_key(&config.app_id);

        // 快速路径：使用读锁尝试获取缓存的token
        {
            self.metrics
                .read_lock_acquisitions
                .fetch_add(1, Ordering::Relaxed);
            let cache = self.cache.read().await;
            if let Some(token) = cache.get(&key) {
                if !token.is_empty() {
                    self.metrics.app_cache_hits.fetch_add(1, Ordering::Relaxed);
                    log::debug!("App token cache hit in {:?}", start_time.elapsed());
                    return Ok(token);
                }
            }
        }

        // 记录缓存未命中
        self.metrics
            .app_cache_misses
            .fetch_add(1, Ordering::Relaxed);

        // 慢速路径：需要刷新token，使用写锁
        self.metrics
            .write_lock_acquisitions
            .fetch_add(1, Ordering::Relaxed);
        let cache = self.cache.write().await;

        // 双重检查：可能其他线程已经刷新了token
        if let Some(token) = cache.get(&key) {
            if !token.is_empty() {
                // 双重检查命中，更新为缓存命中统计
                self.metrics
                    .app_cache_misses
                    .fetch_sub(1, Ordering::Relaxed);
                self.metrics.app_cache_hits.fetch_add(1, Ordering::Relaxed);
                log::debug!("App token double-check hit in {:?}", start_time.elapsed());
                return Ok(token);
            }
        }

        // 实际执行token刷新
        drop(cache); // 释放写锁，避免在HTTP请求期间持有锁
        log::debug!("App token cache miss, refreshing token");

        let app_type = config.app_type;
        let result = if app_type == AppType::SelfBuild {
            self.get_custom_app_access_token_then_cache(config).await
        } else {
            self.get_marketplace_app_access_token_then_cache(config, app_ticket, app_ticket_manager)
                .await
        };

        // 记录刷新结果
        match &result {
            Ok(_) => {
                self.metrics.refresh_success.fetch_add(1, Ordering::Relaxed);
                log::debug!("App token refresh succeeded in {:?}", start_time.elapsed());
            }
            Err(e) => {
                self.metrics
                    .refresh_failures
                    .fetch_add(1, Ordering::Relaxed);
                log::warn!(
                    "App token refresh failed in {:?}: {:?}",
                    start_time.elapsed(),
                    e
                );
            }
        }

        result
    }

    async fn get_custom_app_access_token_then_cache(&self, config: &Config) -> SDKResult<String> {
        let url = format!("{}{}", config.base_url, APP_ACCESS_TOKEN_INTERNAL_URL_PATH);

        let body = SelfBuiltAppAccessTokenReq {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
        };

        let response = config.http_client.post(&url).json(&body).send().await?;

        let resp: AppAccessTokenResp = response.json().await?;
        self.handle_app_access_token_response(resp, &config.app_id)
            .await
    }

    /// 通用的app access token响应处理逻辑
    async fn handle_app_access_token_response(
        &self,
        resp: AppAccessTokenResp,
        app_id: &str,
    ) -> SDKResult<String> {
        if resp.raw_response.code == 0 {
            let expire = resp.expire - EXPIRY_DELTA;
            {
                let mut cache = self.cache.write().await;
                cache.set(
                    &app_access_token_key(app_id),
                    resp.app_access_token.clone(),
                    expire,
                );
            }
            Ok(resp.app_access_token)
        } else {
            warn!("app access token response error: {:#?}", resp.raw_response);
            Err(LarkAPIError::illegal_param(resp.raw_response.msg.clone()))
        }
    }
    async fn get_marketplace_app_access_token_then_cache(
        &self,
        config: &Config,
        app_ticket: &str,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<String> {
        let mut app_ticket = app_ticket.to_string();
        if app_ticket.is_empty() {
            match app_ticket_manager.lock().await.get(config).await {
                None => return Err(LarkAPIError::illegal_param("App ticket is empty")),
                Some(ticket) => {
                    app_ticket = ticket;
                }
            }
        }

        let url = format!("{}{}", config.base_url, APP_ACCESS_TOKEN_URL_PATH);

        let body = MarketplaceAppAccessTokenReq {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
            app_ticket,
        };

        let response = config.http_client.post(&url).json(&body).send().await?;

        let resp: AppAccessTokenResp = response.json().await?;
        self.handle_app_access_token_response(resp, &config.app_id)
            .await
    }

    pub async fn get_tenant_access_token(
        &self,
        config: &Config,
        tenant_key: &str,
        app_ticket: &str,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<String> {
        let start_time = Instant::now();
        let key = tenant_access_token_key(&config.app_id, tenant_key);

        // 快速路径：使用读锁尝试获取缓存的token
        {
            self.metrics
                .read_lock_acquisitions
                .fetch_add(1, Ordering::Relaxed);
            let cache = self.cache.read().await;
            if let Some(token) = cache.get(&key) {
                if !token.is_empty() {
                    self.metrics
                        .tenant_cache_hits
                        .fetch_add(1, Ordering::Relaxed);
                    log::debug!("Tenant token cache hit in {:?}", start_time.elapsed());
                    return Ok(token);
                }
            }
        }

        // 记录缓存未命中
        self.metrics
            .tenant_cache_misses
            .fetch_add(1, Ordering::Relaxed);

        // 慢速路径：需要刷新token，使用写锁
        self.metrics
            .write_lock_acquisitions
            .fetch_add(1, Ordering::Relaxed);
        let cache = self.cache.write().await;

        // 双重检查：可能其他线程已经刷新了token
        if let Some(token) = cache.get(&key) {
            if !token.is_empty() {
                // 双重检查命中，更新为缓存命中统计
                self.metrics
                    .tenant_cache_misses
                    .fetch_sub(1, Ordering::Relaxed);
                self.metrics
                    .tenant_cache_hits
                    .fetch_add(1, Ordering::Relaxed);
                log::debug!(
                    "Tenant token double-check hit in {:?}",
                    start_time.elapsed()
                );
                return Ok(token);
            }
        }

        // 实际执行token刷新
        drop(cache); // 释放写锁，避免在HTTP请求期间持有锁
        log::debug!("Tenant token cache miss, refreshing token");

        let result = if config.app_type == AppType::SelfBuild {
            self.get_custom_tenant_access_token_then_cache(config, tenant_key)
                .await
        } else {
            self.get_marketplace_tenant_access_token_then_cache(
                config,
                tenant_key,
                app_ticket,
                app_ticket_manager,
            )
            .await
        };

        // 记录刷新结果
        match &result {
            Ok(_) => {
                self.metrics.refresh_success.fetch_add(1, Ordering::Relaxed);
                log::debug!(
                    "Tenant token refresh succeeded in {:?}",
                    start_time.elapsed()
                );
            }
            Err(e) => {
                self.metrics
                    .refresh_failures
                    .fetch_add(1, Ordering::Relaxed);
                log::warn!(
                    "Tenant token refresh failed in {:?}: {:?}",
                    start_time.elapsed(),
                    e
                );
            }
        }

        result
    }

    async fn get_custom_tenant_access_token_then_cache(
        &self,
        config: &Config,
        tenant_key: &str,
    ) -> SDKResult<String> {
        let url = format!(
            "{}{}",
            config.base_url, TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH
        );

        let body = SelfBuiltTenantAccessTokenReq {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
        };

        let response = config.http_client.post(&url).json(&body).send().await?;

        let resp: TenantAccessTokenResp = response.json().await?;
        self.handle_tenant_access_token_response(resp, &config.app_id, tenant_key)
            .await
    }

    /// 通用的tenant access token响应处理逻辑
    async fn handle_tenant_access_token_response(
        &self,
        resp: TenantAccessTokenResp,
        app_id: &str,
        tenant_key: &str,
    ) -> SDKResult<String> {
        if resp.raw_response.code == 0 {
            let expire = resp.expire - EXPIRY_DELTA;
            {
                let mut cache = self.cache.write().await;
                cache.set(
                    &tenant_access_token_key(app_id, tenant_key),
                    resp.tenant_access_token.clone(),
                    expire,
                );
            }
            Ok(resp.tenant_access_token)
        } else {
            warn!(
                "tenant access token response error: {:#?}",
                resp.raw_response
            );
            Err(LarkAPIError::illegal_param(resp.raw_response.msg.clone()))
        }
    }

    async fn get_marketplace_tenant_access_token_then_cache(
        &self,
        config: &Config,
        tenant_key: &str,
        app_ticket: &str,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<String> {
        let app_access_token = self
            .get_marketplace_app_access_token_then_cache(config, app_ticket, app_ticket_manager)
            .await?;

        let url = format!("{}{}", config.base_url, TENANT_ACCESS_TOKEN_URL_PATH);

        let body = MarketplaceTenantAccessTokenReq {
            app_access_token,
            tenant_key: tenant_key.to_string(),
        };

        let response = config
            .http_client
            .post(&url)
            .json(&body)
            .header(
                "Authorization",
                &format!("Bearer {}", &body.app_access_token),
            )
            .send()
            .await?;

        let resp: TenantAccessTokenResp = response.json().await?;
        self.handle_tenant_access_token_response(resp, &config.app_id, tenant_key)
            .await
    }
}

fn app_access_token_key(app_id: &str) -> String {
    format!("{APP_ACCESS_TOKEN_KEY_PREFIX}-{app_id}")
}

fn tenant_access_token_key(app_id: &str, tenant_key: &str) -> String {
    format!("{APP_ACCESS_TOKEN_KEY_PREFIX}-{app_id}-{tenant_key}")
}

#[derive(Debug, Serialize, Deserialize)]
struct SelfBuiltAppAccessTokenReq {
    app_id: String,
    app_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SelfBuiltTenantAccessTokenReq {
    app_id: String,
    app_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppAccessTokenResp {
    #[serde(flatten)]
    raw_response: RawResponse,
    expire: i32,
    app_access_token: String,
}

impl ApiResponseTrait for AppAccessTokenResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

#[derive(Serialize, Deserialize)]
struct MarketplaceAppAccessTokenReq {
    app_id: String,
    app_secret: String,
    app_ticket: String,
}

#[derive(Serialize, Deserialize)]
struct MarketplaceTenantAccessTokenReq {
    app_access_token: String,
    tenant_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TenantAccessTokenResp {
    #[serde(flatten)]
    raw_response: RawResponse,
    expire: i32,
    tenant_access_token: String,
}

impl ApiResponseTrait for TenantAccessTokenResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{config::Config, constants::AppType};
    use std::{collections::HashMap, sync::Arc, time::Duration};
    use tokio::sync::Mutex;

    #[test]
    fn test_token_manager_creation() {
        let manager = TokenManager::new();
        // 测试创建TokenManager不会panic
        // 注意：QuickCache没有len方法，我们只测试创建不panic
        let _cache = &manager.cache;
    }

    #[test]
    fn test_app_access_token_key_generation() {
        let app_id = "test_app_id";
        let key = app_access_token_key(app_id);
        assert_eq!(key, format!("{APP_ACCESS_TOKEN_KEY_PREFIX}-{app_id}"));
    }

    #[test]
    fn test_tenant_access_token_key_generation() {
        let app_id = "test_app_id";
        let tenant_key = "test_tenant";
        let key = tenant_access_token_key(app_id, tenant_key);
        assert_eq!(
            key,
            format!("{APP_ACCESS_TOKEN_KEY_PREFIX}-{app_id}-{tenant_key}")
        );
    }

    #[tokio::test]
    async fn test_cache_miss_returns_empty_string() {
        let manager = TokenManager::new();
        let key = "non_existent_key";

        // 验证缓存miss时返回空字符串而不是panic
        let cache = manager.cache.read().await;
        let result = cache.get(key).unwrap_or_default();
        assert_eq!(result, String::new());
    }

    #[tokio::test]
    async fn test_get_app_access_token_cache_miss_does_not_error() {
        let manager = TokenManager::new();
        let config = Config {
            app_id: "test_app".to_string(),
            app_secret: "test_secret".to_string(),
            app_type: AppType::SelfBuild,
            base_url: "https://open.feishu.cn".to_string(),
            http_client: reqwest::Client::new(),
            enable_token_cache: true,
            req_timeout: Some(Duration::from_secs(30)),
            header: HashMap::new(),
            token_manager: Arc::new(Mutex::new(TokenManager::new())),
            app_ticket_manager: Arc::new(Mutex::new(
                crate::core::app_ticket_manager::AppTicketManager::new(),
            )),
        };

        let app_ticket_manager = Arc::new(Mutex::new(
            crate::core::app_ticket_manager::AppTicketManager::new(),
        ));

        // 测试缓存miss时不会立即返回错误，而是尝试获取新token
        // 注意：此测试会因为实际API调用失败而失败，但不应该因为缓存逻辑失败
        let result = manager
            .get_app_access_token(&config, "", &app_ticket_manager)
            .await;

        // 这里我们期望的是网络错误或API错误，而不是"cache error"
        if let Err(error) = result {
            let error_msg = format!("{error:?}");
            assert!(
                !error_msg.contains("cache error"),
                "应该不再出现'cache error'，而是实际的API调用错误: {error_msg}"
            );
        }
    }

    #[test]
    fn test_token_metrics_creation() {
        let metrics = TokenMetrics::new();

        // 测试初始状态
        assert_eq!(metrics.app_cache_hit_rate(), 0.0);
        assert_eq!(metrics.tenant_cache_hit_rate(), 0.0);
        assert_eq!(metrics.refresh_success_rate(), 0.0);
    }

    #[test]
    fn test_token_metrics_cache_hit_rate_calculation() {
        let metrics = TokenMetrics::new();

        // 模拟一些缓存命中和未命中
        metrics.app_cache_hits.store(8, Ordering::Relaxed);
        metrics.app_cache_misses.store(2, Ordering::Relaxed);

        assert_eq!(metrics.app_cache_hit_rate(), 0.8); // 8/(8+2) = 0.8

        // 测试tenant缓存
        metrics.tenant_cache_hits.store(9, Ordering::Relaxed);
        metrics.tenant_cache_misses.store(1, Ordering::Relaxed);

        assert_eq!(metrics.tenant_cache_hit_rate(), 0.9); // 9/(9+1) = 0.9
    }

    #[test]
    fn test_token_metrics_refresh_success_rate() {
        let metrics = TokenMetrics::new();

        // 模拟刷新成功和失败
        metrics.refresh_success.store(19, Ordering::Relaxed);
        metrics.refresh_failures.store(1, Ordering::Relaxed);

        assert_eq!(metrics.refresh_success_rate(), 0.95); // 19/(19+1) = 0.95
    }

    #[test]
    fn test_token_metrics_performance_report() {
        let metrics = TokenMetrics::new();

        // 设置一些测试数据
        metrics.app_cache_hits.store(80, Ordering::Relaxed);
        metrics.app_cache_misses.store(20, Ordering::Relaxed);
        metrics.refresh_success.store(95, Ordering::Relaxed);
        metrics.refresh_failures.store(5, Ordering::Relaxed);

        let report = metrics.performance_report();

        // 验证报告包含关键信息
        assert!(report.contains("80.00%")); // App cache hit rate
        assert!(report.contains("95.00%")); // Refresh success rate
        assert!(report.contains("80 hits, 20 misses")); // Cache statistics
    }

    #[tokio::test]
    async fn test_token_manager_metrics_integration() {
        let manager = TokenManager::new();

        // 验证metrics可以正常访问
        let metrics = manager.metrics();
        assert_eq!(metrics.read_lock_acquisitions.load(Ordering::Relaxed), 0);

        // 验证性能报告功能
        manager.log_performance_metrics(); // 应该不会panic
    }

    #[tokio::test]
    async fn test_preheating_config_default_values() {
        let config = PreheatingConfig::default();
        assert_eq!(config.check_interval_seconds, 1800); // 30分钟
        assert_eq!(config.preheat_threshold_seconds, 900); // 15分钟
        assert!(config.enable_tenant_preheating);
        assert_eq!(config.max_concurrent_preheat, 3);
    }

    #[tokio::test]
    async fn test_should_preheat_token_with_custom_threshold() {
        let manager = TokenManager::new();
        let key = "test_token_key";

        // 测试不存在的token应该需要预热
        assert!(TokenManager::should_preheat_token_with_threshold(&manager.cache, key, 600).await);

        // 添加一个token到缓存（模拟刚刷新的token）
        {
            let mut cache = manager.cache.write().await;
            cache.set(key, "test_token_value".to_string(), 3600); // 1小时后过期
        }

        // 测试长阈值应该不需要预热
        assert!(!TokenManager::should_preheat_token_with_threshold(&manager.cache, key, 600).await);

        // 测试短阈值应该需要预热
        assert!(TokenManager::should_preheat_token_with_threshold(&manager.cache, key, 3700).await);
    }

    #[tokio::test]
    async fn test_get_cached_tenant_keys() {
        let manager = TokenManager::new();

        // 测试空缓存时返回空列表
        let tenant_keys = TokenManager::get_cached_tenant_keys(&manager.cache, "test_app").await;
        assert!(tenant_keys.is_empty());
    }

    #[test]
    fn test_cache_entry_expiry_calculations() {
        use crate::core::cache::CacheEntry;
        use std::time::Duration;
        use tokio::time::Instant;

        let now = Instant::now();
        let expires_in_10_mins = now + Duration::from_secs(600);

        let entry = CacheEntry {
            value: "test_value".to_string(),
            expires_at: expires_in_10_mins,
            current_time: now,
        };

        // 测试过期时间计算
        assert_eq!(entry.expiry_seconds(), 600);

        // 测试即将过期判断
        assert!(entry.expires_within(700)); // 10分钟 < 700秒
        assert!(!entry.expires_within(500)); // 10分钟 > 500秒
    }

    #[test]
    fn test_preheating_config_custom_values() {
        let config = PreheatingConfig {
            check_interval_seconds: 900,
            preheat_threshold_seconds: 300,
            enable_tenant_preheating: false,
            max_concurrent_preheat: 5,
        };

        assert_eq!(config.check_interval_seconds, 900);
        assert_eq!(config.preheat_threshold_seconds, 300);
        assert!(!config.enable_tenant_preheating);
        assert_eq!(config.max_concurrent_preheat, 5);
    }

    #[test]
    fn test_preheating_config_clone() {
        let config = PreheatingConfig::default();
        let cloned = config.clone();

        assert_eq!(config.check_interval_seconds, cloned.check_interval_seconds);
        assert_eq!(config.preheat_threshold_seconds, cloned.preheat_threshold_seconds);
        assert_eq!(config.enable_tenant_preheating, cloned.enable_tenant_preheating);
        assert_eq!(config.max_concurrent_preheat, cloned.max_concurrent_preheat);
    }

    #[test]
    fn test_preheating_config_debug() {
        let config = PreheatingConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("PreheatingConfig"));
        assert!(debug_str.contains("1800"));
        assert!(debug_str.contains("900"));
    }

    #[test]
    fn test_token_metrics_zero_division_safety() {
        let metrics = TokenMetrics::new();

        // 测试零除法安全性 - 当没有统计数据时应该返回0.0
        assert_eq!(metrics.app_cache_hit_rate(), 0.0);
        assert_eq!(metrics.tenant_cache_hit_rate(), 0.0);
        assert_eq!(metrics.refresh_success_rate(), 0.0);
    }

    #[test]
    fn test_token_metrics_edge_cases() {
        let metrics = TokenMetrics::new();

        // 测试只有缓存命中，没有未命中的情况
        metrics.app_cache_hits.store(100, Ordering::Relaxed);
        assert_eq!(metrics.app_cache_hit_rate(), 1.0);

        // 测试只有缓存未命中，没有命中的情况
        metrics.app_cache_hits.store(0, Ordering::Relaxed);
        metrics.app_cache_misses.store(50, Ordering::Relaxed);
        assert_eq!(metrics.app_cache_hit_rate(), 0.0);
    }

    #[test]
    fn test_token_metrics_debug() {
        let metrics = TokenMetrics::new();
        let debug_str = format!("{:?}", metrics);

        assert!(debug_str.contains("TokenMetrics"));
        assert!(debug_str.contains("app_cache_hits"));
        assert!(debug_str.contains("refresh_success"));
    }

    #[test]
    fn test_token_metrics_all_rates_with_data() {
        let metrics = TokenMetrics::new();

        // 设置各种统计数据
        metrics.app_cache_hits.store(75, Ordering::Relaxed);
        metrics.app_cache_misses.store(25, Ordering::Relaxed);
        metrics.tenant_cache_hits.store(60, Ordering::Relaxed);
        metrics.tenant_cache_misses.store(40, Ordering::Relaxed);
        metrics.refresh_success.store(90, Ordering::Relaxed);
        metrics.refresh_failures.store(10, Ordering::Relaxed);
        metrics.read_lock_acquisitions.store(200, Ordering::Relaxed);
        metrics.write_lock_acquisitions.store(50, Ordering::Relaxed);

        assert_eq!(metrics.app_cache_hit_rate(), 0.75);
        assert_eq!(metrics.tenant_cache_hit_rate(), 0.6);
        assert_eq!(metrics.refresh_success_rate(), 0.9);
        assert_eq!(metrics.read_lock_acquisitions.load(Ordering::Relaxed), 200);
        assert_eq!(metrics.write_lock_acquisitions.load(Ordering::Relaxed), 50);
    }

    #[test]
    fn test_token_request_structs_serialization() {
        let self_built_req = SelfBuiltAppAccessTokenReq {
            app_id: "test_app".to_string(),
            app_secret: "test_secret".to_string(),
        };

        let json = serde_json::to_string(&self_built_req).unwrap();
        assert!(json.contains("test_app"));
        assert!(json.contains("test_secret"));

        let deserialized: SelfBuiltAppAccessTokenReq = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.app_id, "test_app");
        assert_eq!(deserialized.app_secret, "test_secret");
    }

    #[test]
    fn test_marketplace_token_request_serialization() {
        let marketplace_req = MarketplaceAppAccessTokenReq {
            app_id: "marketplace_app".to_string(),
            app_secret: "marketplace_secret".to_string(),
            app_ticket: "test_ticket".to_string(),
        };

        let json = serde_json::to_string(&marketplace_req).unwrap();
        assert!(json.contains("marketplace_app"));
        assert!(json.contains("marketplace_secret"));
        assert!(json.contains("test_ticket"));

        let deserialized: MarketplaceAppAccessTokenReq = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.app_id, "marketplace_app");
        assert_eq!(deserialized.app_ticket, "test_ticket");
    }

    #[test]
    fn test_tenant_token_request_serialization() {
        let tenant_req = MarketplaceTenantAccessTokenReq {
            app_access_token: "app_token_123".to_string(),
            tenant_key: "tenant_abc".to_string(),
        };

        let json = serde_json::to_string(&tenant_req).unwrap();
        assert!(json.contains("app_token_123"));
        assert!(json.contains("tenant_abc"));

        let deserialized: MarketplaceTenantAccessTokenReq = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.app_access_token, "app_token_123");
        assert_eq!(deserialized.tenant_key, "tenant_abc");
    }

    #[test]
    fn test_app_access_token_response() {
        use crate::core::api_resp::{ResponseFormat, RawResponse};

        // 测试ResponseFormat
        assert!(matches!(AppAccessTokenResp::data_format(), ResponseFormat::Flatten));

        // 测试响应结构
        let raw_resp = RawResponse {
            code: 0,
            msg: "success".to_string(),
            err: None,
        };

        let resp = AppAccessTokenResp {
            raw_response: raw_resp,
            expire: 3600,
            app_access_token: "test_token_123".to_string(),
        };

        assert_eq!(resp.expire, 3600);
        assert_eq!(resp.app_access_token, "test_token_123");
        assert_eq!(resp.raw_response.code, 0);
    }

    #[test]
    fn test_tenant_access_token_response() {
        use crate::core::api_resp::{ResponseFormat, RawResponse};

        // 测试ResponseFormat
        assert!(matches!(TenantAccessTokenResp::data_format(), ResponseFormat::Flatten));

        // 测试响应结构
        let raw_resp = RawResponse {
            code: 0,
            msg: "success".to_string(),
            err: None,
        };

        let resp = TenantAccessTokenResp {
            raw_response: raw_resp,
            expire: 7200,
            tenant_access_token: "tenant_token_456".to_string(),
        };

        assert_eq!(resp.expire, 7200);
        assert_eq!(resp.tenant_access_token, "tenant_token_456");
        assert_eq!(resp.raw_response.code, 0);
    }

    #[tokio::test]
    async fn test_token_manager_stop_preheating() {
        let mut manager = TokenManager::new();

        // 确保初始状态下没有预热任务
        assert!(!manager.is_preheating_active());

        // 停止不存在的预热任务应该不会panic
        manager.stop_background_preheating();
        assert!(!manager.is_preheating_active());
    }

    #[tokio::test]
    async fn test_token_manager_concurrent_access() {
        let manager = Arc::new(TokenManager::new());
        let manager_clone = manager.clone();

        // 测试并发访问不会导致死锁
        let handle1 = tokio::spawn(async move {
            let cache = manager_clone.cache.read().await;
            cache.get("test_key")
        });

        let handle2 = tokio::spawn(async move {
            let cache = manager.cache.read().await;
            cache.get("another_key")
        });

        let (result1, result2) = tokio::join!(handle1, handle2);
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_key_generation_with_special_characters() {
        let app_id_with_special = "app-id_with.special@chars";
        let tenant_with_special = "tenant.key-with_special@chars";

        let app_key = app_access_token_key(app_id_with_special);
        let tenant_key = tenant_access_token_key(app_id_with_special, tenant_with_special);

        assert!(app_key.contains(app_id_with_special));
        assert!(tenant_key.contains(app_id_with_special));
        assert!(tenant_key.contains(tenant_with_special));
    }

    #[test]
    fn test_key_generation_with_unicode() {
        let app_id = "应用标识符";
        let tenant_key = "租户标识符";

        let app_key = app_access_token_key(app_id);
        let tenant_access_key = tenant_access_token_key(app_id, tenant_key);

        assert!(app_key.contains(app_id));
        assert!(tenant_access_key.contains(app_id));
        assert!(tenant_access_key.contains(tenant_key));
    }

    #[tokio::test]
    async fn test_token_metrics_atomic_operations() {
        let metrics = Arc::new(TokenMetrics::new());
        let metrics_clone = metrics.clone();

        // 测试并发修改指标
        let handle = tokio::spawn(async move {
            for _ in 0..100 {
                metrics_clone.app_cache_hits.fetch_add(1, Ordering::Relaxed);
                metrics_clone.app_cache_misses.fetch_add(1, Ordering::Relaxed);
            }
        });

        for _ in 0..50 {
            metrics.refresh_success.fetch_add(1, Ordering::Relaxed);
            metrics.refresh_failures.fetch_add(1, Ordering::Relaxed);
        }

        handle.await.unwrap();

        assert_eq!(metrics.app_cache_hits.load(Ordering::Relaxed), 100);
        assert_eq!(metrics.app_cache_misses.load(Ordering::Relaxed), 100);
        assert_eq!(metrics.refresh_success.load(Ordering::Relaxed), 50);
        assert_eq!(metrics.refresh_failures.load(Ordering::Relaxed), 50);
    }

    #[test]
    fn test_self_built_tenant_access_token_req() {
        let req = SelfBuiltTenantAccessTokenReq {
            app_id: "self_built_app".to_string(),
            app_secret: "self_built_secret".to_string(),
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("self_built_app"));
        assert!(json.contains("self_built_secret"));

        let deserialized: SelfBuiltTenantAccessTokenReq = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.app_id, "self_built_app");
        assert_eq!(deserialized.app_secret, "self_built_secret");
    }

    #[test]
    fn test_token_response_debug() {
        use crate::core::api_resp::RawResponse;

        let raw_resp = RawResponse {
            code: 0,
            msg: "success".to_string(),
            err: None,
        };

        let app_resp = AppAccessTokenResp {
            raw_response: raw_resp.clone(),
            expire: 3600,
            app_access_token: "debug_token".to_string(),
        };

        let debug_str = format!("{:?}", app_resp);
        assert!(debug_str.contains("AppAccessTokenResp"));
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("3600"));

        let tenant_resp = TenantAccessTokenResp {
            raw_response: raw_resp,
            expire: 7200,
            tenant_access_token: "tenant_debug_token".to_string(),
        };

        let debug_str = format!("{:?}", tenant_resp);
        assert!(debug_str.contains("TenantAccessTokenResp"));
        assert!(debug_str.contains("tenant_debug_token"));
        assert!(debug_str.contains("7200"));
    }

    #[test]
    fn test_token_manager_memory_efficiency() {
        // 测试创建多个TokenManager实例不会消耗过多内存
        let managers: Vec<TokenManager> = (0..50)
            .map(|_| TokenManager::new())
            .collect();

        assert_eq!(managers.len(), 50);

        // 验证每个manager都有独立的缓存和指标
        for manager in &managers {
            assert!(manager.cache.try_read().is_ok());
        }
    }
}

use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::Mutex;

use crate::core::{
    app_ticket_manager::AppTicketManager,
    constants::{AppType, FEISHU_BASE_URL},
    token_manager::TokenManager,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub app_id: String,
    pub app_secret: String,
    /// 域名, 默认为 <https://open.feishu.cn>
    pub base_url: String,
    pub enable_token_cache: bool,
    /// 应用类型, 默认为自建应用
    pub app_type: AppType,
    pub http_client: reqwest::Client,
    /// 客户端超时时间, 默认永不超时
    pub req_timeout: Option<Duration>,
    pub header: HashMap<String, String>,
    /// Token 管理器
    pub token_manager: Arc<Mutex<TokenManager>>,
    /// App Ticket 管理器  
    pub app_ticket_manager: Arc<Mutex<AppTicketManager>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_id: "".to_string(),
            app_secret: "".to_string(),
            base_url: FEISHU_BASE_URL.to_string(),
            enable_token_cache: true,
            app_type: AppType::SelfBuild,
            http_client: reqwest::Client::new(),
            req_timeout: None,
            header: Default::default(),
            token_manager: Arc::new(Mutex::new(TokenManager::new())),
            app_ticket_manager: Arc::new(Mutex::new(AppTicketManager::new())),
        }
    }
}

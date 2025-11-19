//! VC Report API 服务实现
//!
//! 提供视频会议报告相关的API服务，包括：
//! - 每日会议使用报告
//! - Top用户统计报告
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// VC会议报告服务
#[derive(Debug, Clone)]
pub struct VcReportService {
    config: Config,
}

impl VcReportService {
    /// 创建新的VC会议报告服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取每日会议报告
    ///
    /// 获取指定时间范围内的每日会议使用报告
    ///
    /// # 参数
    /// * `request` - 获取每日会议报告请求
    ///
    /// # 返回
    /// 返回每日会议报告数据
    pub async fn get_daily_report(
        &self,
        request: &GetDailyReportRequest,
    ) -> SDKResult<GetDailyReportResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "获取每日会议报告: start_date={}, end_date={}",
            request.start_date,
            request.end_date
        );

        // 构建查询参数
        let mut query_params = HashMap::new();
        query_params.insert("start_date", request.start_date.clone());
        query_params.insert("end_date", request.end_date.clone());

        if let Some(ref user_id_type) = request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/vc/v1/reports/get_daily".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<GetDailyReportResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "获取每日会议报告完成: start_date={}, end_date={}",
            request.start_date,
            request.end_date
        );

        Ok(response)
    }

    /// 获取Top用户报告
    ///
    /// 获取指定时间范围内的会议使用Top用户统计报告
    ///
    /// # 参数
    /// * `request` - 获取Top用户报告请求
    ///
    /// # 返回
    /// 返回Top用户统计报告数据
    pub async fn get_top_user_report(
        &self,
        request: &GetTopUserReportRequest,
    ) -> SDKResult<GetTopUserReportResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "获取Top用户报告: start_date={}, end_date={}, sort_type={:?}",
            request.start_date,
            request.end_date,
            request.sort_type
        );

        // 构建查询参数
        let mut query_params = HashMap::new();
        query_params.insert("start_date", request.start_date.clone());
        query_params.insert("end_date", request.end_date.clone());

        if let Some(ref sort_type) = request.sort_type {
            query_params.insert("sort_type", sort_type.clone());
        }

        if let Some(limit) = request.limit {
            query_params.insert("limit", limit.to_string());
        }

        if let Some(ref user_id_type) = request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/vc/v1/reports/get_top_user".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        // 发送请求
        let resp =
            Transport::<GetTopUserReportResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "获取Top用户报告完成: start_date={}, end_date={}, user_count={}",
            request.start_date,
            request.end_date,
            response
                .data
                .as_ref()
                .and_then(|d| d.top_users.as_ref())
                .map(|u| u.len())
                .unwrap_or(0)
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct GetDailyReportRequestBuilder {
    request: GetDailyReportRequest,
}

impl GetDailyReportRequestBuilder {
    pub fn new(start_date: impl Into<String>, end_date: impl Into<String>) -> Self {
        Self {
            request: GetDailyReportRequest {
                start_date: start_date.into(),
                end_date: end_date.into(),
                user_id_type: None,
                user_ids: None,
            },
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.request.user_ids = Some(user_ids);
        self
    }

    pub async fn execute(self, service: &VcReportService) -> SDKResult<GetDailyReportResponse> {
        service.get_daily_report(&self.request).await
    }
}

pub struct GetTopUserReportRequestBuilder {
    request: GetTopUserReportRequest,
}

impl GetTopUserReportRequestBuilder {
    pub fn new(start_date: impl Into<String>, end_date: impl Into<String>) -> Self {
        Self {
            request: GetTopUserReportRequest {
                start_date: start_date.into(),
                end_date: end_date.into(),
                sort_type: None,
                limit: None,
                user_id_type: None,
            },
        }
    }

    pub fn sort_type(mut self, sort_type: impl Into<String>) -> Self {
        self.request.sort_type = Some(sort_type.into());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.request.limit = Some(limit);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self, service: &VcReportService) -> SDKResult<GetTopUserReportResponse> {
        service.get_top_user_report(&self.request).await
    }
}

//! Bitable App Dashboard API 服务实现
//!
//! 提供多维表格仪表板管理相关的API服务，包括：
//! - 仪表板的创建、查询、更新、删除
//! - 仪表板组件配置和布局管理
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};
use serde_json::Value;

use super::models::*;

/// 多维表格仪表板管理服务
#[derive(Debug, Clone)]
pub struct AppDashboardService {
    config: Config,
}

impl AppDashboardService {
    /// 创建新的仪表板管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建仪表板
    ///
    /// 在指定的应用中创建一个仪表板
    ///
    /// # 参数
    /// * `request` - 创建仪表板请求
    ///
    /// # 返回
    /// 返回新创建的仪表板信息
    pub async fn create_dashboard(
        &self,
        request: &CreateDashboardRequest,
    ) -> SDKResult<CreateDashboardResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "创建仪表板: app_token={}, dashboard_name={}",
            request.app_token,
            request.dashboard_name
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert(
            "dashboard_name",
            serde_json::to_value(&request.dashboard_name)?,
        );

        if let Some(ref description) = request.description {
            body.insert("description", serde_json::to_value(description)?);
        }
        if let Some(ref layout_config) = request.layout_config {
            body.insert("layout_config", serde_json::to_value(layout_config)?);
        }
        if let Some(ref components) = request.components {
            body.insert("components", serde_json::to_value(components)?);
        }

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/dashboards",
                request.app_token
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<CreateDashboardResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "创建仪表板完成: app_token={}, dashboard_name={}",
            request.app_token,
            request.dashboard_name
        );

        Ok(response)
    }

    /// 更新仪表板
    ///
    /// 更新指定应用中的仪表板
    ///
    /// # 参数
    /// * `request` - 更新仪表板请求
    ///
    /// # 返回
    /// 返回更新后的仪表板信息
    pub async fn update_dashboard(
        &self,
        request: &UpdateDashboardRequest,
    ) -> SDKResult<UpdateDashboardResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新仪表板: app_token={}, dashboard_id={}",
            request.app_token,
            request.dashboard_id
        );

        // 构建请求体
        let mut body = HashMap::new();

        if let Some(ref dashboard_name) = request.dashboard_name {
            body.insert("dashboard_name", serde_json::to_value(dashboard_name)?);
        }
        if let Some(ref description) = request.description {
            body.insert("description", serde_json::to_value(description)?);
        }
        if let Some(ref layout_config) = request.layout_config {
            body.insert("layout_config", serde_json::to_value(layout_config)?);
        }
        if let Some(ref components) = request.components {
            body.insert("components", serde_json::to_value(components)?);
        }

        // 构建API请求
        let api_req = ApiRequest::put(format!(
                "/open-apis/bitable/v1/apps/{}/dashboards/{}",
                request.app_token, request.dashboard_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<UpdateDashboardResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新仪表板完成: app_token={}, dashboard_id={}",
            request.app_token,
            request.dashboard_id
        );

        Ok(response)
    }

    /// 删除仪表板
    ///
    /// 删除指定应用中的仪表板
    ///
    /// # 参数
    /// * `request` - 删除仪表板请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_dashboard(
        &self,
        request: &DeleteDashboardRequest,
    ) -> SDKResult<DeleteDashboardResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "删除仪表板: app_token={}, dashboard_id={}",
            request.app_token,
            request.dashboard_id
        );

        // 构建API请求
        let api_req = ApiRequest::delete(format!(
                "/open-apis/bitable/v1/apps/{}/dashboards/{}",
                request.app_token, request.dashboard_id
            ))
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<DeleteDashboardResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "删除仪表板完成: app_token={}, dashboard_id={}",
            request.app_token,
            request.dashboard_id
        );

        Ok(response)
    }

    /// 列出仪表板
    ///
    /// 获取指定应用中的仪表板列表
    ///
    /// # 参数
    /// * `request` - 列出仪表板请求
    ///
    /// # 返回
    /// 返回仪表板列表
    pub async fn list_dashboards(
        &self,
        request: &ListDashboardsRequest,
    ) -> SDKResult<ListDashboardsResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("列出仪表板: app_token={}", request.app_token);

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = request.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/dashboards",
                request.app_token
            ))
            .query(query_params);

        // 发送请求
        let resp =
            Transport::<ListDashboardsResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出仪表板完成: app_token={}, count={}",
            request.app_token,
            response.dashboards.as_ref().map(|d| d.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 复制仪表板
    ///
    /// 复制指定的仪表板
    ///
    /// # 参数
    /// * `request` - 复制仪表板请求
    ///
    /// # 返回
    /// 返回复制的仪表板信息
    pub async fn copy_dashboard(
        &self,
        request: &CopyDashboardRequest,
    ) -> SDKResult<CopyDashboardResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "复制仪表板: app_token={}, dashboard_id={}, new_name={}",
            request.app_token,
            request.dashboard_id,
            request.dashboard_name
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert(
            "dashboard_id".to_string(),
            serde_json::to_value(&request.dashboard_id)?,
        );
        body.insert(
            "dashboard_name".to_string(),
            serde_json::to_value(&request.dashboard_name)?,
        );

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/dashboards/copy",
                request.app_token
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<CopyDashboardResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "复制仪表板完成: app_token={}, dashboard_id={}",
            request.app_token,
            request.dashboard_id
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct CreateDashboardRequestBuilder {
    request: CreateDashboardRequest,
}

impl CreateDashboardRequestBuilder {
    pub fn new(app_token: impl Into<String>, dashboard_name: impl Into<String>) -> Self {
        Self {
            request: CreateDashboardRequest {
                app_token: app_token.into(),
                dashboard_name: dashboard_name.into(),
                description: None,
                layout_config: None,
                components: None,
            },
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn layout_config(mut self, layout_config: Value) -> Self {
        self.request.layout_config = Some(layout_config);
        self
    }

    pub fn components(mut self, components: Vec<DashboardComponent>) -> Self {
        self.request.components = Some(components);
        self
    }

    pub async fn execute(
        self,
        service: &AppDashboardService,
    ) -> SDKResult<CreateDashboardResponse> {
        service.create_dashboard(&self.request).await
    }
}

pub struct UpdateDashboardRequestBuilder {
    request: UpdateDashboardRequest,
}

impl UpdateDashboardRequestBuilder {
    pub fn new(app_token: impl Into<String>, dashboard_id: impl Into<String>) -> Self {
        Self {
            request: UpdateDashboardRequest {
                app_token: app_token.into(),
                dashboard_id: dashboard_id.into(),
                dashboard_name: None,
                description: None,
                layout_config: None,
                components: None,
            },
        }
    }

    pub fn dashboard_name(mut self, dashboard_name: impl Into<String>) -> Self {
        self.request.dashboard_name = Some(dashboard_name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn layout_config(mut self, layout_config: Value) -> Self {
        self.request.layout_config = Some(layout_config);
        self
    }

    pub fn components(mut self, components: Vec<DashboardComponent>) -> Self {
        self.request.components = Some(components);
        self
    }

    pub async fn execute(
        self,
        service: &AppDashboardService,
    ) -> SDKResult<UpdateDashboardResponse> {
        service.update_dashboard(&self.request).await
    }
}

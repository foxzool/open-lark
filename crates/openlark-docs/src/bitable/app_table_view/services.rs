//! Bitable App Table View API 服务实现
//!
//! 提供多维表格视图管理相关的API服务，包括：
//! - 视图的创建、查询、更新、删除
//! - 视图类型配置和样式设置
//! - 完整的错误处理和参数验证
use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 多维表格视图管理服务
#[derive(Debug, Clone)]
pub struct AppTableViewService {
    config: Config,
}

impl AppTableViewService {
    /// 创建新的视图管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建视图
    ///
    /// 在指定的数据表中创建一个视图
    ///
    /// # 参数
    /// * `request` - 创建视图请求
    ///
    /// # 返回
    /// 返回新创建的视图信息
    pub async fn create_view(&self, request: &CreateViewRequest) -> SDKResult<CreateViewResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "创建视图: app_token={}, table_id={}, view_name={}, view_type={}",
            request.app_token,
            request.table_id,
            request.view_name,
            request.view_type
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("view_name", Value::String(request.view_name.clone()));
        body.insert("view_type", Value::String(request.view_type.clone()));

        if let Some(ref view_config) = request.view_config {
            body.insert("view_config", view_config.clone());
        }
        if let Some(ref description) = request.description {
            body.insert("description", Value::String(description.clone()));
        }

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/views",
                request.app_token, request.table_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<CreateViewResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "创建视图完成: app_token={}, table_id={}, view_name={}",
            request.app_token,
            request.table_id,
            request.view_name
        );

        Ok(response)
    }

    /// 更新视图
    ///
    /// 更新指定数据表中的视图
    ///
    /// # 参数
    /// * `request` - 更新视图请求
    ///
    /// # 返回
    /// 返回更新后的视图信息
    pub async fn update_view(&self, request: &UpdateViewRequest) -> SDKResult<UpdateViewResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新视图: app_token={}, table_id={}, view_id={}",
            request.app_token,
            request.table_id,
            request.view_id
        );

        // 构建请求体
        let mut body = HashMap::new();

        if let Some(ref view_name) = request.view_name {
            body.insert("view_name", Value::String(view_name.clone()));
        }
        if let Some(ref view_config) = request.view_config {
            body.insert("view_config", view_config.clone());
        }
        if let Some(ref description) = request.description {
            body.insert("description", Value::String(description.clone()));
        }

        // 构建API请求
        let api_req = ApiRequest::put(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
                request.app_token, request.table_id, request.view_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<UpdateViewResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新视图完成: app_token={}, table_id={}, view_id={}",
            request.app_token,
            request.table_id,
            request.view_id
        );

        Ok(response)
    }

    /// 删除视图
    ///
    /// 删除指定数据表中的视图
    ///
    /// # 参数
    /// * `request` - 删除视图请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_view(&self, request: &DeleteViewRequest) -> SDKResult<DeleteViewResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "删除视图: app_token={}, table_id={}, view_id={}",
            request.app_token,
            request.table_id,
            request.view_id
        );

        // 构建API请求
        let api_req = ApiRequest::delete(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
                request.app_token, request.table_id, request.view_id
            ))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<DeleteViewResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "删除视图完成: app_token={}, table_id={}, view_id={}",
            request.app_token,
            request.table_id,
            request.view_id
        );

        Ok(response)
    }

    /// 列出视图
    ///
    /// 获取指定数据表中的视图列表
    ///
    /// # 参数
    /// * `request` - 列出视图请求
    ///
    /// # 返回
    /// 返回视图列表
    pub async fn list_views(&self, request: &ListViewsRequest) -> SDKResult<ListViewsResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "列出视图: app_token={}, table_id={}",
            request.app_token,
            request.table_id
        );

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
                "/open-apis/bitable/v1/apps/{}/tables/{}/views",
                request.app_token, request.table_id
            ))
            .query(query_params);

        // 发送请求
        let resp = Transport::<ListViewsResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出视图完成: app_token={}, table_id={}, count={}",
            request.app_token,
            request.table_id,
            response.views.as_ref().map(|v| v.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 获取视图
    ///
    /// 获取指定数据表中的单个视图
    ///
    /// # 参数
    /// * `request` - 获取视图请求
    ///
    /// # 返回
    /// 返回视图信息
    pub async fn get_view(&self, request: &GetViewRequest) -> SDKResult<GetViewResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "获取视图: app_token={}, table_id={}, view_id={}",
            request.app_token,
            request.table_id,
            request.view_id
        );

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
                request.app_token, request.table_id, request.view_id
            ))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<GetViewResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "获取视图完成: app_token={}, table_id={}, view_id={}",
            request.app_token,
            request.table_id,
            request.view_id
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct CreateViewRequestBuilder {
    request: CreateViewRequest,
}

impl CreateViewRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        view_name: impl Into<String>,
        view_type: impl Into<String>,
    ) -> Self {
        Self {
            request: CreateViewRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                view_name: view_name.into(),
                view_type: view_type.into(),
                view_config: None,
                description: None,
            },
        }
    }

    pub fn view_config(mut self, view_config: Value) -> Self {
        self.request.view_config = Some(view_config);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub async fn execute(self, service: &AppTableViewService) -> SDKResult<CreateViewResponse> {
        service.create_view(&self.request).await
    }
}

pub struct UpdateViewRequestBuilder {
    request: UpdateViewRequest,
}

impl UpdateViewRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        view_id: impl Into<String>,
    ) -> Self {
        Self {
            request: UpdateViewRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                view_id: view_id.into(),
                view_name: None,
                view_config: None,
                description: None,
            },
        }
    }

    pub fn view_name(mut self, view_name: impl Into<String>) -> Self {
        self.request.view_name = Some(view_name.into());
        self
    }

    pub fn view_config(mut self, view_config: Value) -> Self {
        self.request.view_config = Some(view_config);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub async fn execute(self, service: &AppTableViewService) -> SDKResult<UpdateViewResponse> {
        service.update_view(&self.request).await
    }
}

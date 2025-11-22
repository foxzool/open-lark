//! Bitable App API 服务实现
//!
//! 提供多维表格应用管理相关的API服务，包括：
//! - 应用创建和复制
//! - 应用元数据获取和更新
//! - 应用删除操作

use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// Bitable App API服务
#[derive(Debug, Clone)]
pub struct AppService {
    config: Config,
}

impl AppService {
    /// 创建新的App服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建多维表格
    ///
    /// 在指定目录下创建多维表格
    ///
    /// # 参数
    /// * `request` - 创建多维表格请求
    ///
    /// # 返回
    /// 返回新创建的多维表格信息
    ///
    /// # 示例
    /// ```rust
    /// let request = CreateAppRequest {
    ///     name: "新多维表格".to_string(),
    ///     folder_token: Some("folder_token_xxx".to_string()),
    /// };
    ///
    /// let response = service.create_app(&request).await?;
    /// if let Some(app) = response.app {
    ///     println!("创建成功: {:?}", app.name);
    /// }
    /// ```
    pub async fn create_app(&self, request: &CreateAppRequest) -> SDKResult<CreateAppResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("创建多维表格: name={}", request.name);

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("name", Value::String(request.name.clone()));

        if let Some(ref folder_token) = request.folder_token {
            body.insert("folder_token", Value::String(folder_token.clone()));
        }
        if let Some(ref template_token) = request.app_template_token {
            body.insert("app_template_token", Value::String(template_token.clone()));
        }

        // 构建API请求
        let api_req = ApiRequest::post("/open-apis/bitable/v1/apps".to_string())
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<CreateAppResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("创建多维表格完成: name={}", request.name);

        Ok(response)
    }

    /// 复制多维表格
    ///
    /// 复制一个多维表格，可以指定复制到某个有权限的文件夹下
    ///
    /// # 参数
    /// * `app_token` - 应用token
    /// * `request` - 复制多维表格请求
    ///
    /// # 返回
    /// 返回复制后的多维表格信息
    pub async fn copy_app(
        &self,
        app_token: &str,
        request: &CopyAppRequest,
    ) -> SDKResult<CopyAppResponse> {
        if app_token.trim().is_empty() {
            return Err(LarkAPIError::illegal_param("应用token不能为空".to_string()));
        }

        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("复制多维表格: app_token={}", app_token);

        // 构建请求体
        let mut body = HashMap::new();

        if let Some(ref folder_token) = request.folder_token {
            body.insert("folder_token", Value::String(folder_token.clone()));
        }
        if let Some(ref name) = request.name {
            body.insert("name", Value::String(name.clone()));
        }

        // 构建API请求
        let api_req = ApiRequest::post(format!("/open-apis/bitable/v1/apps/{}/copy", app_token))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<CopyAppResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("复制多维表格完成: app_token={}", app_token);

        Ok(response)
    }

    /// 获取多维表格元数据
    ///
    /// 获取指定多维表格的元数据信息，包括多维表格名称、版本号、是否开启高级权限等
    ///
    /// # 参数
    /// * `request` - 获取多维表格元数据请求
    ///
    /// # 返回
    /// 返回多维表格的元数据信息
    pub async fn get_app(&self, request: &GetAppRequest) -> SDKResult<GetAppResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("获取多维表格元数据: app_token={}", request.app_token);

        // 构建API请求
        let api_req = ApiRequest::get(format!("/open-apis/bitable/v1/apps/{}", request.app_token))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<GetAppResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("获取多维表格元数据完成: app_token={}", request.app_token);

        Ok(response)
    }

    /// 更新多维表格元数据
    ///
    /// 通过app_token更新多维表格元数据
    ///
    /// # 参数
    /// * `request` - 更新多维表格元数据请求
    ///
    /// # 返回
    /// 返回更新后的多维表格信息
    pub async fn update_app(&self, request: &UpdateAppRequest) -> SDKResult<UpdateAppResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("更新多维表格元数据: app_token={}", request.app_token);

        // 构建请求体
        let mut body = HashMap::new();

        if let Some(ref name) = request.name {
            body.insert("name", Value::String(name.clone()));
        }
        if let Some(ref icon) = request.icon {
            body.insert("icon", Value::String(icon.clone()));
        }
        if let Some(ref description) = request.description {
            body.insert("description", Value::String(description.clone()));
        }

        // 构建API请求
        let api_req = ApiRequest::put(format!("/open-apis/bitable/v1/apps/{}", request.app_token))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<UpdateAppResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("更新多维表格元数据完成: app_token={}", request.app_token);

        Ok(response)
    }

    /// 删除多维表格
    ///
    /// 删除指定的多维表格
    ///
    /// # 参数
    /// * `request` - 删除多维表格请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_app(&self, request: &DeleteAppRequest) -> SDKResult<DeleteAppResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("删除多维表格: app_token={}", request.app_token);

        // 构建API请求
        let api_req = ApiRequest::delete(format!("/open-apis/bitable/v1/apps/{}", request.app_token))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<DeleteAppResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("删除多维表格完成: app_token={}", request.app_token);

        Ok(response)
    }
}

// 构建器模式实现
pub struct CreateAppRequestBuilder {
    request: CreateAppRequest,
}

impl CreateAppRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: CreateAppRequest {
                name: String::new(),
                folder_token: None,
                app_template_token: None,
            },
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = Some(folder_token.into());
        self
    }

    pub fn app_template_token(mut self, app_template_token: impl Into<String>) -> Self {
        self.request.app_template_token = Some(app_template_token.into());
        self
    }

    pub async fn execute(self, service: &AppService) -> SDKResult<CreateAppResponse> {
        service.create_app(&self.request).await
    }
}

pub struct UpdateAppRequestBuilder {
    request: UpdateAppRequest,
}

impl UpdateAppRequestBuilder {
    pub fn new(app_token: impl Into<String>) -> Self {
        Self {
            request: UpdateAppRequest {
                app_token: app_token.into(),
                name: None,
                icon: None,
                description: None,
            },
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.request.icon = Some(icon.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub async fn execute(self, service: &AppService) -> SDKResult<UpdateAppResponse> {
        service.update_app(&self.request).await
    }
}

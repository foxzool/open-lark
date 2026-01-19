//! Drive Permission API模块
//!
//! 提供云空间文件权限管理相关的功能，包括：
//! - 协作者权限验证
//! - 所有者转移
//! - 公共权限设置获取
//!
//! # 示例
//!
//! ```rust,no_run
//! use openlark_core::config::Config;
//! use openlark_docs::ccm::drive::permission::PermissionService;
//!
//! #[tokio::main]
//! async fn main() -> openlark_core::SDKResult<()> {
//!     let config = Config::builder()
//!         .app_id("app_id")
//!         .app_secret("app_secret")
//!         .build();
//!     let service = PermissionService::new(config);
//!
//!     // 检查用户权限
//!     let response = service
//!         .check_member_permission_builder()
//!         .file_token("token_xxx")
//!         .permission("view")
//!         .user_id("user_xxx")
//!         .user_id_type("open_id")
//!         .execute(&service)
//!         .await?;
//!
//!     if response.permitted.unwrap_or(false) {
//!         println!("用户有权限");
//!     }
//!
//!     Ok(())
//! }
//! ```

/// 数据模型定义
pub mod models;

// 重新导出主要类型
pub use models::*;

use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, validation_error,
    SDKResult,
};

use crate::common::api_endpoints::DriveApi;

/// Drive Permission API服务
#[derive(Debug, Clone)]
pub struct PermissionService {
    config: Config,
}

impl PermissionService {
    /// 创建新的Permission服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 判断协作者是否有某权限
    ///
    /// 根据filetoken判断当前登录用户是否具有某权限
    ///
    /// # 参数
    /// * `request` - 判断权限请求
    ///
    /// # 返回
    /// 返回权限检查结果
    pub async fn check_member_permission(
        &self,
        request: &CheckMemberPermissionRequest,
    ) -> SDKResult<CheckMemberPermissionResponse> {
        self.check_member_permission_with_options(request, RequestOption::default())
            .await
    }

    /// 判断协作者是否有某权限（带请求选项）
    ///
    /// 根据filetoken判断当前登录用户是否具有某权限
    ///
    /// # 参数
    /// * `request` - 判断权限请求
    /// * `option` - 请求选项
    ///
    /// # 返回
    /// 返回权限检查结果
    pub async fn check_member_permission_with_options(
        &self,
        request: &CheckMemberPermissionRequest,
        option: RequestOption,
    ) -> SDKResult<CheckMemberPermissionResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| validation_error("validation", &format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "判断协作者权限: file_token={}, permission={}",
            request.file_token,
            request.permission
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("file_token", Value::String(request.file_token.clone()));
        body.insert("permission", Value::String(request.permission.clone()));

        if let Some(ref user_id_type) = request.user_id_type {
            body.insert("user_id_type", Value::String(user_id_type.clone()));
        }
        if let Some(ref user_id) = request.user_id {
            body.insert("user_id", Value::String(user_id.clone()));
        }

        // 使用 ApiEndpoint 枚举生成 URL
        let api_endpoint = DriveApi::AuthPermissionMember(request.file_token.clone());

        // 构建API请求
        let api_request: ApiRequest<CheckMemberPermissionResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::json!(&body));

        // 发送请求
        let resp: openlark_core::api::Response<CheckMemberPermissionResponse> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "判断协作者权限完成: file_token={}, permitted={}",
            request.file_token,
            response.permitted.unwrap_or(false)
        );

        Ok(response)
    }

    /// 转移拥有者
    pub async fn transfer_owner(
        &self,
        request: &TransferOwnerRequest,
    ) -> SDKResult<TransferOwnerResponse> {
        self.transfer_owner_with_options(request, RequestOption::default())
            .await
    }

    /// 转移拥有者（带请求选项）
    pub async fn transfer_owner_with_options(
        &self,
        request: &TransferOwnerRequest,
        option: RequestOption,
    ) -> SDKResult<TransferOwnerResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| validation_error("validation", &format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "转移拥有者: file_token={}, user_id={}",
            request.file_token,
            request.user_id
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("file_token", Value::String(request.file_token.clone()));
        body.insert("user_id", Value::String(request.user_id.clone()));

        if let Some(ref user_id_type) = request.user_id_type {
            body.insert("user_id_type", Value::String(user_id_type.clone()));
        }

        // 使用 ApiEndpoint 枚举生成 URL
        let api_endpoint = DriveApi::TransferOwner(request.file_token.clone());

        // 构建API请求
        let api_request: ApiRequest<TransferOwnerResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::json!(&body));

        // 发送请求
        let resp: openlark_core::api::Response<TransferOwnerResponse> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "转移拥有者完成: file_token={}, success={}",
            request.file_token,
            response.success.unwrap_or(false)
        );

        Ok(response)
    }

    /// 获取云文档权限设置V2
    pub async fn get_public_permission(
        &self,
        request: &GetPublicPermissionRequest,
    ) -> SDKResult<GetPublicPermissionResponse> {
        self.get_public_permission_with_options(request, RequestOption::default())
            .await
    }

    /// 获取云文档权限设置V2（带请求选项）
    pub async fn get_public_permission_with_options(
        &self,
        request: &GetPublicPermissionRequest,
        option: RequestOption,
    ) -> SDKResult<GetPublicPermissionResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| validation_error("validation", &format!("请求参数验证失败: {}", e)))?;

        log::info!("获取云文档权限设置V2: file_token={}", request.file_token);

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("file_token", Value::String(request.file_token.clone()));

        // 使用 ApiEndpoint 枚举生成 URL
        let api_endpoint = DriveApi::GetPublicPermissionV2(request.file_token.clone());

        // 构建API请求
        let api_request: ApiRequest<GetPublicPermissionResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::json!(&body));

        // 发送请求
        let resp: openlark_core::api::Response<GetPublicPermissionResponse> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "获取云文档权限设置V2完成: file_token={}",
            request.file_token
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct CheckMemberPermissionRequestBuilder {
    request: CheckMemberPermissionRequest,
}

impl CheckMemberPermissionRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: CheckMemberPermissionRequest {
                file_token: String::new(),
                permission: String::new(),
                user_id_type: None,
                user_id: None,
            },
        }
    }

    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.request.file_token = file_token.into();
        self
    }

    pub fn permission(mut self, permission: impl Into<String>) -> Self {
        self.request.permission = permission.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.request.user_id = Some(user_id.into());
        self
    }

    pub async fn execute(
        self,
        service: &PermissionService,
    ) -> SDKResult<CheckMemberPermissionResponse> {
        service.check_member_permission(&self.request).await
    }
}

impl Default for CheckMemberPermissionRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TransferOwnerRequestBuilder {
    request: TransferOwnerRequest,
}

impl TransferOwnerRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: TransferOwnerRequest {
                file_token: String::new(),
                user_id: String::new(),
                user_id_type: None,
            },
        }
    }

    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.request.file_token = file_token.into();
        self
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.request.user_id = user_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self, service: &PermissionService) -> SDKResult<TransferOwnerResponse> {
        service.transfer_owner(&self.request).await
    }
}

impl Default for TransferOwnerRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GetPublicPermissionRequestBuilder {
    request: GetPublicPermissionRequest,
}

impl GetPublicPermissionRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: GetPublicPermissionRequest {
                file_token: String::new(),
            },
        }
    }

    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.request.file_token = file_token.into();
        self
    }

    pub async fn execute(
        self,
        service: &PermissionService,
    ) -> SDKResult<GetPublicPermissionResponse> {
        service.get_public_permission(&self.request).await
    }
}

impl Default for GetPublicPermissionRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PermissionService {
    pub fn check_member_permission_builder(&self) -> CheckMemberPermissionRequestBuilder {
        CheckMemberPermissionRequestBuilder::new()
    }

    pub fn transfer_owner_builder(&self) -> TransferOwnerRequestBuilder {
        TransferOwnerRequestBuilder::new()
    }

    pub fn get_public_permission_builder(&self) -> GetPublicPermissionRequestBuilder {
        GetPublicPermissionRequestBuilder::new()
    }
}

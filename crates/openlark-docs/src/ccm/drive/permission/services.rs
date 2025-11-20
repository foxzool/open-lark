//! Drive Permission API 服务实现
//!
//! 提供权限管理相关的API服务，包括：
//! - 权限验证
//! - 所有者转移
//! - 公共权限设置
use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

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
    ///
    /// # 示例
    /// ```rust
    /// let request = CheckMemberPermissionRequest {
    ///     file_token: "token_xxx".to_string(),
    ///     permission: "view".to_string(),
    ///     user_id_type: Some("open_id".to_string()),
    ///     user_id: Some("user_xxx".to_string()),
    /// };
    ///
    /// let response = service.check_member_permission(&request).await?;
    /// if response.permitted.unwrap_or(false) {
    ///     println!("用户有权限");
    /// }
    /// ```
    pub async fn check_member_permission(
        &self,
        request: &CheckMemberPermissionRequest,
    ) -> SDKResult<CheckMemberPermissionResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

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

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Post,
            url: "/open-apis/drive/permission/member/permitted".to_string(),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(&body)))?,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp = Transport::<CheckMemberPermissionResponse>::request(api_req, &self.config, None)
            .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "判断协作者权限完成: file_token={}, permitted={}",
            request.file_token,
            response.permitted.unwrap_or(false)
        );

        Ok(response)
    }

    /// 转移拥有者
    ///
    /// 根据文档信息和用户信息转移文档的所有者
    ///
    /// # 参数
    /// * `request` - 转移拥有者请求
    ///
    /// # 返回
    /// 返回转移操作的结果
    ///
    /// # 示例
    /// ```rust
    /// let request = TransferOwnerRequest {
    ///     file_token: "token_xxx".to_string(),
    ///     user_id: "user_xxx".to_string(),
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = service.transfer_owner(&request).await?;
    /// if response.success.unwrap_or(false) {
    ///     println!("拥有者转移成功");
    /// }
    /// ```
    pub async fn transfer_owner(
        &self,
        request: &TransferOwnerRequest,
    ) -> SDKResult<TransferOwnerResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

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

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Post,
            url: "/open-apis/drive/permission/member/transfer".to_string(),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(&body)))?,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp = Transport::<TransferOwnerResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "转移拥有者完成: file_token={}, success={}",
            request.file_token,
            response.success.unwrap_or(false)
        );

        Ok(response)
    }

    /// 获取云文档权限设置V2
    ///
    /// 根据filetoken获取文档的公共设置
    ///
    /// # 参数
    /// * `request` - 获取公共权限请求
    ///
    /// # 返回
    /// 返回文档的公共权限设置
    ///
    /// # 示例
    /// ```rust
    /// let request = GetPublicPermissionRequest {
    ///     file_token: "token_xxx".to_string(),
    /// };
    ///
    /// let response = service.get_public_permission(&request).await?;
    /// if let Some(public_permission) = response.public_permission {
    ///     println!("公共访问权限: {}", public_permission.permission.unwrap_or_else(|| "none".to_string()));
    /// }
    /// ```
    pub async fn get_public_permission(
        &self,
        request: &GetPublicPermissionRequest,
    ) -> SDKResult<GetPublicPermissionResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("获取云文档权限设置V2: file_token={}", request.file_token);

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("file_token", Value::String(request.file_token.clone()));

        // 构建API请求
        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Post,
            url: "/open-apis/drive/permission/v2/public/".to_string(),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(&body)))?,
            query: HashMap::new(),
            
        };

        // 发送请求
        let resp =
            Transport::<GetPublicPermissionResponse>::request(api_req, &self.config, None).await?;
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

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder},
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use super::batch_create::Permission;

/// 判断当前用户是否有某权限请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct AuthPermissionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    token: String,
    /// 文档类型
    #[serde(skip)]
    obj_type: String,
    /// 要检查的权限
    #[serde(skip)]
    perm: String,
}

impl AuthPermissionRequest {
    pub fn builder() -> AuthPermissionRequestBuilder {
        AuthPermissionRequestBuilder::default()
    }

    pub fn new(token: impl ToString, obj_type: impl ToString, perm: Permission) -> Self {
        Self {
            token: token.to_string(),
            obj_type: obj_type.to_string(),
            perm: match perm {
                Permission::FullAccess => "full_access".to_string(),
                Permission::Edit => "edit".to_string(),
                Permission::Comment => "comment".to_string(),
                Permission::View => "view".to_string(),
            },
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct AuthPermissionRequestBuilder {
    request: AuthPermissionRequest,
}

impl AuthPermissionRequestBuilder {
    /// 文档token
    pub fn token(mut self, token: impl ToString) -> Self {
        self.request.token = token.to_string();
        self
    }

    /// 文档类型
    pub fn obj_type(mut self, obj_type: impl ToString) -> Self {
        self.request.obj_type = obj_type.to_string();
        self
    }

    /// 设置为文档类型
    pub fn as_doc(mut self) -> Self {
        self.request.obj_type = "doc".to_string();
        self
    }

    /// 设置为电子表格类型
    pub fn as_sheet(mut self) -> Self {
        self.request.obj_type = "sheet".to_string();
        self
    }

    /// 设置为多维表格类型
    pub fn as_bitable(mut self) -> Self {
        self.request.obj_type = "bitable".to_string();
        self
    }

    /// 设置为知识库类型
    pub fn as_wiki(mut self) -> Self {
        self.request.obj_type = "wiki".to_string();
        self
    }

    /// 要检查的权限
    pub fn permission(mut self, perm: Permission) -> Self {
        self.request.perm = match perm {
            Permission::FullAccess => "full_access".to_string(),
            Permission::Edit => "edit".to_string(),
            Permission::Comment => "comment".to_string(),
            Permission::View => "view".to_string(),
        };
        self
    }

    /// 检查是否有所有者权限
    pub fn check_full_access(mut self) -> Self {
        self.request.perm = "full_access".to_string();
        self
    }

    /// 检查是否有编辑权限
    pub fn check_edit(mut self) -> Self {
        self.request.perm = "edit".to_string();
        self
    }

    /// 检查是否有评论权限
    pub fn check_comment(mut self) -> Self {
        self.request.perm = "comment".to_string();
        self
    }

    /// 检查是否有查看权限
    pub fn check_view(mut self) -> Self {
        self.request.perm = "view".to_string();
        self
    }

    pub fn build(mut self) -> AuthPermissionRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 权限检查结果
#[derive(Debug, Deserialize)]
pub struct PermissionAuth {
    /// 是否有该权限
    pub is_permitted: bool,
    /// 检查的权限类型
    pub perm: String,
    /// 用户实际权限（如果有）
    pub actual_perm: Option<String>,
}

/// 判断当前用户是否有某权限响应
#[derive(Debug, Deserialize)]
pub struct AuthPermissionResponse {
    /// 权限检查结果
    pub auth_result: PermissionAuth,
}

impl ApiResponseTrait for AuthPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 判断当前用户是否有某权限
pub async fn auth_permission(
    request: AuthPermissionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<AuthPermissionResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = format!(
        "{}?type={}&perm={}",
        EndpointBuilder::replace_param(DRIVE_V1_PERMISSIONS_MEMBERS_AUTH, "token", &request.token),
        request.obj_type,
        request.perm
    );

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl PermissionAuth {
    /// 是否有权限
    pub fn has_permission(&self) -> bool {
        self.is_permitted
    }

    /// 获取检查的权限类型
    pub fn checked_permission(&self) -> &str {
        &self.perm
    }

    /// 获取实际权限
    pub fn actual_permission(&self) -> Option<&str> {
        self.actual_perm.as_deref()
    }

    /// 获取权限描述
    pub fn permission_description(&self) -> String {
        let checked = match self.perm.as_str() {
            "full_access" => "所有者",
            "edit" => "编辑",
            "comment" => "评论",
            "view" => "查看",
            _ => "未知",
        };

        if self.is_permitted {
            format!("有{checked}权限")
        } else {
            format!("无{checked}权限")
        }
    }

    /// 权限级别比较
    pub fn actual_permission_level(&self) -> u8 {
        match self.actual_perm.as_deref() {
            Some("view") => 1,
            Some("comment") => 2,
            Some("edit") => 3,
            Some("full_access") => 4,
            _ => 0,
        }
    }

    /// 是否有更高级别的权限
    pub fn has_higher_permission(&self) -> bool {
        if let Some(actual) = &self.actual_perm {
            let checked_level = match self.perm.as_str() {
                "view" => 1,
                "comment" => 2,
                "edit" => 3,
                "full_access" => 4,
                _ => 0,
            };

            let actual_level = match actual.as_str() {
                "view" => 1,
                "comment" => 2,
                "edit" => 3,
                "full_access" => 4,
                _ => 0,
            };

            actual_level > checked_level
        } else {
            false
        }
    }
}

impl AuthPermissionResponse {
    /// 是否有权限
    pub fn has_permission(&self) -> bool {
        self.auth_result.has_permission()
    }

    /// 获取检查的权限
    pub fn checked_permission(&self) -> &str {
        self.auth_result.checked_permission()
    }

    /// 获取实际权限
    pub fn actual_permission(&self) -> Option<&str> {
        self.auth_result.actual_permission()
    }

    /// 获取权限摘要
    pub fn summary(&self) -> String {
        let mut parts = vec![self.auth_result.permission_description()];

        if let Some(actual) = self.actual_permission() {
            if actual != self.checked_permission() {
                let actual_desc = match actual {
                    "full_access" => "所有者",
                    "edit" => "编辑者",
                    "comment" => "评论者",
                    "view" => "阅读者",
                    _ => "未知",
                };
                parts.push(format!("实际权限: {actual_desc}"));
            }
        }

        parts.join(", ")
    }

    /// 是否可以执行指定操作
    pub fn can_perform_action(&self, action: &str) -> bool {
        if !self.has_permission() {
            return false;
        }

        match action {
            "read" | "view" => true, // 有任何权限都能查看
            "comment" => self.auth_result.actual_permission_level() >= 2,
            "edit" | "write" => self.auth_result.actual_permission_level() >= 3,
            "manage" | "admin" => self.auth_result.actual_permission_level() >= 4,
            _ => false,
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_permission_request_builder() {
        let request = AuthPermissionRequest::builder()
            .token("doccnxxxxxx")
            .as_doc()
            .check_edit()
            .build();

        assert_eq!(request.token, "doccnxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.perm, "edit");
    }

    #[test]
    fn test_auth_permission_new() {
        let request = AuthPermissionRequest::new("doccnxxxxxx", "doc", Permission::Edit);
        assert_eq!(request.perm, "edit");
    }
}

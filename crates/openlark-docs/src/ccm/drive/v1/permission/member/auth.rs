/// 判断用户云文档权限
///
/// 该接口用于根据 filetoken 判断当前登录用户是否具有某权限。
/// docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/auth
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 判断用户云文档权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPermissionMemberRequest {
    /// 文件token
    pub token: String,
    /// 权限类型，可选值：view（可见）、edit（可编辑）
    pub r#type: String,
    /// 用户类型，可选值：user（用户）、openid（openid）、unionid（unionid）
    pub user_id_type: Option<String>,
}

impl AuthPermissionMemberRequest {
    /// 创建判断用户云文档权限请求
    ///
    /// # 参数
    /// * `token` - 文件token
    /// * `type` - 权限类型
    pub fn new(
        token: impl Into<String>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            token: token.into(),
            r#type: r#type.into(),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }
}

/// 判断用户云文档权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPermissionMemberResponse {
    /// 是否有权限
    pub has_permission: bool,
    /// 权限类型
    pub r#type: String,
}

impl ApiResponseTrait for AuthPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 判断用户云文档权限
///
/// 该接口用于根据 filetoken 判断当前登录用户是否具有某权限。
/// docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/auth
pub async fn auth_permission_member(
    request: AuthPermissionMemberRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<AuthPermissionMemberResponse>> {
    // 构建查询参数
    let mut query_params = vec![("type", request.r#type.clone())];

    if let Some(user_id_type) = &request.user_id_type {
        query_params.push(("user_id_type", user_id_type.clone()));
    }

    // 创建API请求
    let mut api_request: ApiRequest<AuthPermissionMemberResponse> =
        ApiRequest::get(&format!("/open-apis/drive/v1/permissions/{}/members/auth", request.token));

    // 添加查询参数
    for (key, value) in query_params {
        api_request = api_request.query(key, value);
    }

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_permission_member_request_builder() {
        let request = AuthPermissionMemberRequest::new("file_token", "view")
            .user_id_type("openid");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.r#type, "view");
        assert_eq!(request.user_id_type, Some("openid".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(AuthPermissionMemberResponse::data_format(), ResponseFormat::Data);
    }
}
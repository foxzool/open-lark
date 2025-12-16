/// 关闭云文档密码
///
/// 该接口用于根据 filetoken 关闭云文档密码。
/// docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-public-password/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionPublicPasswordRequest {
    pub token: String,
    pub r#type: String,
}

impl DeletePermissionPublicPasswordRequest {
    pub fn new(token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: r#type.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionPublicPasswordResponse {
    // Empty response body usually implies success
}

impl ApiResponseTrait for DeletePermissionPublicPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn delete_permission_public_password(
    request: DeletePermissionPublicPasswordRequest,
    config: &Config,
) -> SDKResult<Response<DeletePermissionPublicPasswordResponse>> {
    let url = format!("/open-apis/drive/v1/permissions/{}/public/password", request.token);
    let mut api_request: ApiRequest<DeletePermissionPublicPasswordResponse> = ApiRequest::delete(&url);
    api_request = api_request.query_param("type", &request.r#type);
    
    Transport::request(api_request, config, None).await
}

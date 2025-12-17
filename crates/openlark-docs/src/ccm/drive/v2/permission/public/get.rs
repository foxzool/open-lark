/// 获取云文档权限设置
///
/// 该接口用于根据 token 获取云文档的权限设置。
/// docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/get-2
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionPublicRequest {
    pub token: String,
    pub r#type: String,
}

impl GetPermissionPublicRequest {
    pub fn new(token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: r#type.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionPublicResponse {
    pub security_entity: Option<String>,
    pub comment_entity: Option<String>,
    pub share_entity: Option<String>,
    pub link_share_entity: Option<String>,
    pub external_access_entity: Option<String>,
    pub invite_external: Option<bool>,
}

impl ApiResponseTrait for GetPermissionPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn get_permission_public(
    request: GetPermissionPublicRequest,
    config: &Config,
) -> SDKResult<Response<GetPermissionPublicResponse>> {
    let url = format!("/open-apis/drive/v2/permissions/{}/public", request.token);
    let mut api_request: ApiRequest<GetPermissionPublicResponse> = ApiRequest::get(&url);
    api_request = api_request.query_param("type", &request.r#type);

    Transport::request(api_request, config, None).await
}

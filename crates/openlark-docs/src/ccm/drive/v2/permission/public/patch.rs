/// 更新云文档权限设置
///
/// 该接口用于根据 token 更新云文档的权限设置。
/// docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/patch-2
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionPublicRequest {
    pub token: String,
    pub r#type: String,
    pub security_entity: Option<String>,
    pub comment_entity: Option<String>,
    pub share_entity: Option<String>,
    pub link_share_entity: Option<String>,
    pub external_access_entity: Option<String>,
    pub invite_external: Option<bool>,
}

impl UpdatePermissionPublicRequest {
    pub fn new(token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: r#type.into(),
            security_entity: None,
            comment_entity: None,
            share_entity: None,
            link_share_entity: None,
            external_access_entity: None,
            invite_external: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionPublicResponse {
    pub success: bool,
}

impl ApiResponseTrait for UpdatePermissionPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn update_permission_public(
    request: UpdatePermissionPublicRequest,
    config: &Config,
) -> SDKResult<Response<UpdatePermissionPublicResponse>> {
    let url = format!("/open-apis/drive/v2/permissions/{}/public", request.token);
    let mut api_request: ApiRequest<UpdatePermissionPublicResponse> = ApiRequest::patch(&url);

    let mut body = serde_json::Map::new();
    body.insert("type".to_string(), serde_json::json!(request.r#type));
    if let Some(v) = request.security_entity {
        body.insert("security_entity".to_string(), serde_json::json!(v));
    }
    if let Some(v) = request.comment_entity {
        body.insert("comment_entity".to_string(), serde_json::json!(v));
    }
    if let Some(v) = request.share_entity {
        body.insert("share_entity".to_string(), serde_json::json!(v));
    }
    if let Some(v) = request.link_share_entity {
        body.insert("link_share_entity".to_string(), serde_json::json!(v));
    }
    if let Some(v) = request.external_access_entity {
        body.insert("external_access_entity".to_string(), serde_json::json!(v));
    }
    if let Some(v) = request.invite_external {
        body.insert("invite_external".to_string(), serde_json::json!(v));
    }

    api_request = api_request.body(serde_json::Value::Object(body));

    Transport::request(api_request, config, None).await
}

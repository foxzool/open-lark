/// 更新云文档权限设置
///
/// 该接口用于根据 token 更新云文档的权限设置。
/// docPath: /document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/permission-public/patch
/// doc: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/patch-2
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};
use super::models::PermissionPublic;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionPublicRequest {
    pub token: String,
    pub r#type: String,
    pub security_entity: Option<String>,
    pub comment_entity: Option<String>,
    pub share_entity: Option<String>,
    pub link_share_entity: Option<String>,
    /// 是否允许内容被分享到组织外
    pub external_access: Option<bool>,
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
            external_access: None,
            invite_external: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionPublicResponse {
    pub permission_public: PermissionPublic,
}

impl ApiResponseTrait for UpdatePermissionPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdatePermissionPublicBody {
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_share_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invite_external: Option<bool>,
}

pub async fn update_permission_public(
    request: UpdatePermissionPublicRequest,
    config: &Config,
) -> SDKResult<UpdatePermissionPublicResponse> {
    if request.token.is_empty() {
        return Err(openlark_core::error::validation_error("token", "token 不能为空"));
    }
    if request.r#type.is_empty() {
        return Err(openlark_core::error::validation_error("type", "type 不能为空"));
    }

    let api_endpoint = DriveApi::UpdatePublicPermissionV2(request.token);

    let body = UpdatePermissionPublicBody {
        type_: request.r#type,
        security_entity: request.security_entity,
        comment_entity: request.comment_entity,
        share_entity: request.share_entity,
        link_share_entity: request.link_share_entity,
        external_access: request.external_access,
        invite_external: request.invite_external,
    };

    let api_request: ApiRequest<UpdatePermissionPublicResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serialize_params(&body, "更新云文档权限设置")?);

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新云文档权限设置")
}

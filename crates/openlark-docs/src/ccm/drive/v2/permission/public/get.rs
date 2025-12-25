/// 获取云文档权限设置
///
/// 该接口用于根据 token 获取云文档的权限设置。
/// docPath: /document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/permission-public/get
/// doc: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/get-2
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
    pub permission_public: PermissionPublic,
}

impl ApiResponseTrait for GetPermissionPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn get_permission_public(
    request: GetPermissionPublicRequest,
    config: &Config,
) -> SDKResult<GetPermissionPublicResponse> {
    if request.token.is_empty() {
        return Err(openlark_core::error::validation_error("token", "token 不能为空"));
    }
    if request.r#type.is_empty() {
        return Err(openlark_core::error::validation_error("type", "type 不能为空"));
    }

    let api_endpoint = DriveApi::GetPublicPermissionV2(request.token);
    let api_request: ApiRequest<GetPermissionPublicResponse> =
        ApiRequest::get(&api_endpoint.to_url()).query("type", &request.r#type);

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取云文档权限设置")
}

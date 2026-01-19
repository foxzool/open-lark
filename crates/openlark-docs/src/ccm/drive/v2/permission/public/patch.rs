/// 更新云文档权限设置
///
/// 该接口用于根据 token 更新云文档的权限设置。
/// docPath: /document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/permission-public/patch
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::PermissionPublic;
use crate::common::{api_endpoints::DriveApi, api_utils::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionPublicRequest {
    pub token: String,
    /// 云文档类型（query 参数 `type`），需要与 token 匹配
    pub r#type: String,
    /// 是否允许内容被分享到组织外
    pub external_access_entity: Option<String>,
    /// 谁可以创建副本、打印、下载
    pub security_entity: Option<String>,
    /// 谁可以评论
    pub comment_entity: Option<String>,
    /// 从组织维度，设置谁可以查看、添加、移除协作者
    pub share_entity: Option<String>,
    /// 从协作者维度，设置谁可以查看、添加、移除协作者
    pub manage_collaborator_entity: Option<String>,
    /// 链接分享设置
    pub link_share_entity: Option<String>,
    /// 谁可以复制内容
    pub copy_entity: Option<String>,
}

impl UpdatePermissionPublicRequest {
    pub fn new(token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: r#type.into(),
            external_access_entity: None,
            security_entity: None,
            comment_entity: None,
            share_entity: None,
            manage_collaborator_entity: None,
            link_share_entity: None,
            copy_entity: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionPublicResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_public: Option<PermissionPublic>,
}

impl ApiResponseTrait for UpdatePermissionPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdatePermissionPublicBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_access_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manage_collaborator_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_share_entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy_entity: Option<String>,
}

pub async fn update_permission_public(
    request: UpdatePermissionPublicRequest,
    config: &Config,
) -> SDKResult<UpdatePermissionPublicResponse> {
    update_permission_public_with_options(
        request,
        config,
        openlark_core::req_option::RequestOption::default(),
    )
    .await
}

pub async fn update_permission_public_with_options(
    request: UpdatePermissionPublicRequest,
    config: &Config,
    option: openlark_core::req_option::RequestOption,
) -> SDKResult<UpdatePermissionPublicResponse> {
    if request.token.is_empty() {
        return Err(openlark_core::error::validation_error(
            "token",
            "token 不能为空",
        ));
    }
    if request.r#type.is_empty() {
        return Err(openlark_core::error::validation_error(
            "type",
            "type 不能为空",
        ));
    }

    let api_endpoint = DriveApi::UpdatePublicPermissionV2(request.token);

    let body = UpdatePermissionPublicBody {
        external_access_entity: request.external_access_entity,
        security_entity: request.security_entity,
        comment_entity: request.comment_entity,
        share_entity: request.share_entity,
        manage_collaborator_entity: request.manage_collaborator_entity,
        link_share_entity: request.link_share_entity,
        copy_entity: request.copy_entity,
    };

    let api_request: ApiRequest<UpdatePermissionPublicResponse> =
        ApiRequest::patch(&api_endpoint.to_url())
            .query("type", &request.r#type)
            .body(serialize_params(&body, "更新云文档权限设置")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新云文档权限设置")
}

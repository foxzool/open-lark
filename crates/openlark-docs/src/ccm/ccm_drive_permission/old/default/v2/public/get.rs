use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDrivePermissionApiOld;

/// 获取云文档权限设置V2请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicRequest {
    /// 令牌
    pub token: String,
    /// 类型
    pub r#type: String,
}

impl GetPublicRequest {
    /// 创建新的 GetPublicRequest
    pub fn new(token: impl Into<String>, type_: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: type_.into(),
        }
    }
}

/// 获取云文档权限设置V2响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicResponse {
    /// 安全设置
    pub security_entity: Option<serde_json::Value>,
    /// 评论设置
    pub comment_entity: Option<serde_json::Value>,
    /// 共享设置
    pub share_entity: Option<serde_json::Value>,
    /// 管理设置
    pub manage_collaborator_entity: Option<serde_json::Value>,
    /// 链接共享设置
    pub link_share_entity: Option<serde_json::Value>,
    /// 外部访问设置
    pub external_access_entity: Option<serde_json::Value>,
    /// 邀请人设置
    pub invite_external: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档权限设置V2
///
/// 根据 filetoken 获取文档的公共设置。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/get-document-sharing-settings-v2
pub async fn get(
    request: GetPublicRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetPublicResponse>> {
    let api_endpoint = CcmDrivePermissionApiOld::Public;
    let mut api_request: ApiRequest<GetPublicResponse> = ApiRequest::post(&api_endpoint.to_url())
        .json_body(&request);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_public_request() {
        let request = GetPublicRequest::new("token", "type");
        assert_eq!(request.token, "token");
    }
}

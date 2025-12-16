use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDrivePermissionApiOld;

/// 判断协作者是否有某权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermittedRequest {
    /// 令牌
    pub token: String,
    /// 类型
    pub r#type: String,
    /// 权限
    pub perm: String,
}

impl PermittedRequest {
    /// 创建新的 PermittedRequest
    pub fn new(token: impl Into<String>, type_: impl Into<String>, perm: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: type_.into(),
            perm: perm.into(),
        }
    }
}

/// 判断协作者是否有某权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermittedResponse {
    /// 是否有权限
    pub permitted: bool,
}

impl ApiResponseTrait for PermittedResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 判断协作者是否有某权限
///
/// 根据 filetoken 判断当前登录用户是否具有某权限。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/querying-if-a-collaborator-has-a-specific-permission
pub async fn permitted(
    request: PermittedRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<PermittedResponse>> {
    let api_endpoint = CcmDrivePermissionApiOld::MemberPermitted;
    let mut api_request: ApiRequest<PermittedResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_permitted_request() {
        let request = PermittedRequest::new("token", "type", "perm");
        assert_eq!(request.token, "token");
        assert_eq!(request.perm, "perm");
    }
}

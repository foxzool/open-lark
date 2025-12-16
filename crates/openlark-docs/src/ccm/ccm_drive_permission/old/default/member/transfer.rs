use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDrivePermissionApiOld;

/// 转移拥有者请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    /// 令牌
    pub token: String,
    /// 类型
    pub r#type: String,
    /// 所有者ID
    pub owner_id: String,
    /// 所有者ID类型
    pub owner_type: String, // open_id, user_id, union_id
    /// 是否移除旧所有者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_old_owner: Option<bool>,
    /// 评论通知
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 是否通知
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
}

impl TransferRequest {
    /// 创建新的 TransferRequest
    pub fn new(token: impl Into<String>, type_: impl Into<String>, owner_id: impl Into<String>, owner_type: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: type_.into(),
            owner_id: owner_id.into(),
            owner_type: owner_type.into(),
            remove_old_owner: None,
            comment: None,
            notify: None,
        }
    }
}

/// 转移拥有者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResponse {
    /// 是否成功
    pub is_success: bool,
    /// 拥有者类型
    pub r#type: String,
    /// 拥有者ID
    pub id: String,
    /// 名称
    pub name: String,
}

impl ApiResponseTrait for TransferResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 转移拥有者
///
/// 根据文档信息和用户信息转移文档的所有者。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/transfer-ownership
pub async fn transfer(
    request: TransferRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<TransferResponse>> {
    let api_endpoint = CcmDrivePermissionApiOld::MemberTransfer;
    let mut api_request: ApiRequest<TransferResponse> = ApiRequest::post(&api_endpoint.to_url())
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
    fn test_transfer_request() {
        let request = TransferRequest::new("token", "doc", "owner_id", "open_id");
        assert_eq!(request.token, "token");
        assert_eq!(request.owner_id, "owner_id");
    }
}

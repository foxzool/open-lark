//! 转移拥有者
//!
//! docPath: /document/ukTMukTMukTM/uQzNzUjL0czM14CN3MTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uQzNzUjL0czM14CN3MTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TransferReq {
    /// 文件 token
    pub token: String,
    /// 文档类型
    #[serde(rename = "type")]
    pub type_: String,
    /// 新 owner
    pub owner: Owner,
    /// 转移后删除旧 owner 的权限（默认 false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_old_owner: Option<bool>,
    /// 不通知新 owner（默认 false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_notify: Option<bool>,
}

/// 文档所有者
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Owner {
    /// 用户类型：email/openid/userid/unionid
    pub member_type: String,
    /// 用户类型下的值
    pub member_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TransferResponse {
    pub is_success: bool,
    pub token: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub owner: Owner,
}

impl ApiResponseTrait for TransferResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 转移拥有者请求
pub struct TransferRequest {
    config: Config,
    req: TransferReq,
}

impl TransferRequest {
    pub fn new(
        config: Config,
        token: impl Into<String>,
        type_: impl Into<String>,
        owner: Owner,
    ) -> Self {
        Self {
            config,
            req: TransferReq {
                token: token.into(),
                type_: type_.into(),
                owner,
                remove_old_owner: None,
                cancel_notify: None,
            },
        }
    }

    pub fn remove_old_owner(mut self, remove: bool) -> Self {
        self.req.remove_old_owner = Some(remove);
        self
    }

    pub fn cancel_notify(mut self, cancel: bool) -> Self {
        self.req.cancel_notify = Some(cancel);
        self
    }

    pub async fn execute(self) -> SDKResult<TransferResponse> {
        validate_required!(self.req.token, "token 不能为空");
        validate_required!(self.req.type_, "type 不能为空");
        validate_required!(self.req.owner.member_type, "owner.member_type 不能为空");
        validate_required!(self.req.owner.member_id, "owner.member_id 不能为空");

        use crate::common::api_endpoints::CcmDrivePermissionApiOld;

        let api_request: ApiRequest<TransferResponse> =
            ApiRequest::post(&CcmDrivePermissionApiOld::MemberTransfer.to_url())
                .body(serialize_params(&self.req, "转移拥有者")?);
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "转移拥有者")
    }
}

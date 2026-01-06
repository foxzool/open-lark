//! 通过手机号或邮箱获取用户 ID
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/user/batch_get_id

use std::collections::HashMap;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::user::models::UserIdType,
    endpoints::CONTACT_V3_USERS_BATCH_GET_ID,
};

/// 通过手机号或邮箱获取用户 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchGetIdBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobiles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resigned: Option<bool>,
}

/// 用户状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatus {
    #[serde(default)]
    pub is_frozen: bool,
    #[serde(default)]
    pub is_resigned: bool,
    #[serde(default)]
    pub is_activated: bool,
    #[serde(default)]
    pub is_exited: bool,
    #[serde(default)]
    pub is_unjoin: bool,
}

/// 用户 ID 查询结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetIdItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 通过手机号或邮箱获取用户 ID 响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetIdResponse {
    #[serde(default)]
    pub user_list: Vec<BatchGetIdItem>,
}

impl ApiResponseTrait for BatchGetIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 通过手机号或邮箱获取用户 ID 请求
pub struct BatchGetIdRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
}

impl BatchGetIdRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
        }
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/user/batch_get_id
    pub async fn execute(self, body: BatchGetIdBody) -> SDKResult<BatchGetIdResponse> {
        let has_emails = body.emails.as_ref().map(|v| !v.is_empty()).unwrap_or(false);
        let has_mobiles = body
            .mobiles
            .as_ref()
            .map(|v| !v.is_empty())
            .unwrap_or(false);
        if !has_emails && !has_mobiles {
            return Err(error::validation_error(
                "emails/mobiles 不能为空".to_string(),
                "emails 与 mobiles 至少需要提供其中一个".to_string(),
            ));
        }

        // url: POST:/open-apis/contact/v3/users/batch_get_id
        let mut req: ApiRequest<BatchGetIdResponse> =
            ApiRequest::post(CONTACT_V3_USERS_BATCH_GET_ID)
                .body(serialize_params(&body, "通过手机号或邮箱获取用户 ID")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "通过手机号或邮箱获取用户 ID")
    }
}

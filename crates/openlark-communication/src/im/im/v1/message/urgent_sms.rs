//! 发送短信加急
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/buzz-messages/urgent_sms

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
    im::im::v1::message::models::UserIdType,
};

/// 发送短信加急请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrgentSmsBody {
    pub user_id_list: Vec<String>,
}

/// 发送短信加急响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrgentSmsResponse {
    #[serde(default)]
    pub invalid_user_id_list: Vec<String>,
}

impl ApiResponseTrait for UrgentSmsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 发送短信加急请求
pub struct UrgentSmsRequest {
    config: Config,
    message_id: String,
    user_id_type: Option<UserIdType>,
}

impl UrgentSmsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
            user_id_type: None,
        }
    }

    /// 待加急的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，必填）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/buzz-messages/urgent_sms
    pub async fn execute(self, body: UrgentSmsBody) -> SDKResult<UrgentSmsResponse> {
        validate_required!(self.message_id, "message_id 不能为空");
        if body.user_id_list.is_empty() {
            return Err(error::validation_error(
                "user_id_list 不能为空".to_string(),
                "加急用户列表至少包含 1 项".to_string(),
            ));
        }
        let user_id_type = self.user_id_type.ok_or_else(|| {
            error::validation_error(
                "user_id_type 不能为空".to_string(),
                "发送短信加急需要指定 user_id_type".to_string(),
            )
        })?;

        // url: PATCH:/open-apis/im/v1/messages/:message_id/urgent_sms
        let req: ApiRequest<UrgentSmsResponse> =
            ApiRequest::patch(format!("{}/{}/urgent_sms", IM_V1_MESSAGES, self.message_id))
                .query("user_id_type", user_id_type.as_str())
                .body(serialize_params(&body, "发送短信加急")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "发送短信加急")
    }
}

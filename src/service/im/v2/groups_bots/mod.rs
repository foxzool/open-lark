use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::im::v2::models::{ButtonInfo, UserIdType},
};

/// 群聊或机器人消息服务
pub struct GroupsBotsService {
    pub config: Config,
}

/// 机器人单聊即时提醒请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BotTimeSentiveRequest {
    /// 接收用户ID
    pub receive_id: String,
    /// 提醒消息内容
    pub content: Value,
    /// 消息类型
    pub msg_type: String,
}

/// 机器人单聊即时提醒响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotTimeSentiveResponse {
    /// 消息ID
    pub message_id: String,
    /// 发送时间
    pub send_time: String,
}

impl ApiResponseTrait for BotTimeSentiveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新消息流卡片按钮请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFeedCardButtonRequest {
    /// 按钮信息列表
    pub buttons: Vec<ButtonInfo>,
    /// 更新原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 更新消息流卡片按钮响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFeedCardButtonResponse {
    /// 消息ID
    pub message_id: String,
    /// 更新时间
    pub update_time: String,
    /// 更新的按钮数量
    pub updated_button_count: i32,
}

impl ApiResponseTrait for UpdateFeedCardButtonResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 即时提醒请求
#[derive(Debug, Serialize, Deserialize)]
pub struct TimelyReminderRequest {
    /// 提醒消息内容
    pub content: Value,
    /// 目标用户ID列表
    pub target_users: Vec<String>,
    /// 提醒类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_type: Option<String>,
}

/// 即时提醒响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelyReminderResponse {
    /// 提醒ID
    pub reminder_id: String,
    /// 发送时间
    pub send_time: String,
    /// 成功发送的用户数量
    pub success_count: i32,
    /// 失败的用户ID列表
    pub failed_users: Vec<String>,
}

impl ApiResponseTrait for TimelyReminderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GroupsBotsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 机器人单聊即时提醒
    pub async fn bot_time_sentive(
        &self,
        receive_id_type: UserIdType,
        request: BotTimeSentiveRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BotTimeSentiveResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: crate::core::endpoints::im::IM_V2_GROUPS_BOTS_TIME_SENSITIVE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params: HashMap::from([(
                "receive_id_type",
                receive_id_type.as_str().to_string(),
            )]),
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新消息流卡片按钮
    pub async fn update(
        &self,
        message_id: &str,
        request: UpdateFeedCardButtonRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateFeedCardButtonResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V2_GROUPS_BOTS_UPDATE,
                "message_id",
                message_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 即时提醒
    pub async fn patch(
        &self,
        receive_id_type: UserIdType,
        request: TimelyReminderRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<TimelyReminderResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: crate::core::endpoints::im::IM_V2_GROUPS_BOTS_PATCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params: HashMap::from([(
                "receive_id_type",
                receive_id_type.as_str().to_string(),
            )]),
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

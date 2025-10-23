use reqwest::Method;
use open_lark_core::core::{constants::AccessTokenType, http::Transport};
use serde::{Deserialize, Serialize },
use serde_json::Value;
use std::collections::HashMap;

use open_lark_core::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
},
use crate::im::models::{ButtonInfo, UserIdTypeV2 as UserIdType },
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
/// 机器人单聊即时提醒响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotTimeSentiveResponse {
    /// 消息ID
    pub message_id: String,
    /// 发送时间
    pub send_time: String,
impl ApiResponseTrait for BotTimeSentiveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use open_lark_core::core::{
    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
},
use crate::impl_full_service;
/// 消息卡片服务
pub struct MessageCardService {
    pub config: Config,
}
/// 更新消息卡片请求
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchMessageCardRequest {
    /// 卡片内容
    pub card: Value,
    /// 令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
// 接入统一 Service 抽象（IM v1 - MessageCardService）
impl_full_service!(MessageCardService, "im.message_card", "v1");
/// 延时更新消息卡片请求
pub struct DelayUpdateMessageCardRequest {
    /// 延时时间(秒)
    pub delay: i32,
/// 发送仅特定人可见的消息卡片请求
pub struct SendVisibleMessageCardRequest {
    /// 可见用户ID列表
    pub open_ids: Vec<String>,
/// 发送仅特定人可见的消息卡片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendVisibleMessageCardResponse {
    /// 消息ID
    pub message_id: String,
impl ApiResponseTrait for SendVisibleMessageCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

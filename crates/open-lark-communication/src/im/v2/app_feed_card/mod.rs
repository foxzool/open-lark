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
/// 应用消息流卡片服务
pub struct AppFeedCardService {
    pub config: Config,
}
/// 创建应用消息流卡片请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppFeedCardRequest {
    /// 卡片内容
    pub card_content: Value,
    /// 目标用户ID列表
    pub target_users: Vec<String>,
    /// 卡片标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 卡片描述
    pub description: Option<String>,
/// 创建应用消息流卡片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAppFeedCardResponse {
    /// 卡片ID
    pub card_id: String,
    /// 创建时间
    pub create_time: String,
impl ApiResponseTrait for CreateAppFeedCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
/// 更新应用消息流卡片请求

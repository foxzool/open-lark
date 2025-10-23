use reqwest::Method;
use open_lark_core::core::{constants::AccessTokenType, http::Transport};
use serde::{Deserialize, Serialize },
use std::collections::HashMap;

use crate::impl_full_service;
use open_lark_core::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
        SDKResult,
    },
},
use crate::im::models::UserIdTypeV1 as UserIdType;
/// 消息加急服务
pub struct BuzzMessagesService {
    pub config: Config,
}
/// 发送应用内加急请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UrgentAppRequest {
    /// 用户ID列表
    pub user_id_list: Vec<String>,
// 接入统一 Service 抽象（IM v1 - BuzzMessagesService）
impl_full_service!(BuzzMessagesService, "im.buzz_messages", "v1");
/// 发送短信加急请求
pub struct UrgentSmsRequest {
/// 发送电话加急请求
pub struct UrgentPhoneRequest {
/// 消息加急响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrgentResponse {
    /// 无效的用户ID列表
    pub invalid_user_id_list: Vec<String>,
impl ApiResponseTrait for UrgentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data

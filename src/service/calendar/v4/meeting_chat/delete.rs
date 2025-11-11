#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 会议群删除功能,
//!,
//! ## 实现状态,
//! ❌ **未实现** - 此功能等待实现,
//!
//! ## API文档,
/// # API文档,
///
/// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/portal_apply_schema/list>,
use openlark_core::api_req::ApiRequest;
use crate::{
use openlark_core::{api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat}},
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use super::MeetingChatService;
/// 删除会议群请求 (未实现)
#[derive(Debug, Clone)]
pub struct DeleteMeetingChatRequest {
    pub api_req: ApiRequest,
    // TODO: 添加具体的请求字段,
/// 删除会议群响应 (未实现),
#[derive(Debug, Clone)]
pub struct DeleteMeetingChatResponse {
    // TODO: 添加具体的响应字段,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl MeetingChatService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
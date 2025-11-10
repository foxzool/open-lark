#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::SDKResult;use reqwest::Method;
use serde::Deserialize;
use open_lark_core::core::api_req::ApiRequest;
use crate::{,
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::calendar::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    }
    impl_executable_builder_owned,
    service::calendar::v4::models::{Calendar, UserIdType}
};
use super::CalendarManagementService;
/// 查询日历信息请求
#[derive(Debug, Clone)]
pub struct GetCalendarRequest {
    pub api_req: ApiRequest,
    /// 日历ID
    pub calendar_id: String,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>}
impl GetCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 查询日历信息请求构建器,
pub struct GetCalendarRequestBuilder {
    request: GetCalendarRequest}
impl GetCalendarRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.request,
    }
/// 查询日历信息响应数据,
#[derive(Debug, Clone)]
pub struct GetCalendarResponseData {
    /// 日历信息
    pub calendar: Calendar,
/// 查询日历信息响应,
#[derive(Debug, Clone)]
pub struct GetCalendarResponse {
    /// 响应数据
    pub data: GetCalendarResponseData,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl CalendarManagementService {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder宏,
impl_executable_builder_owned!(
    GetCalendarRequestBuilder,
    CalendarManagementService,
    GetCalendarRequest,
    BaseResponse<GetCalendarResponse>,
    get,
);

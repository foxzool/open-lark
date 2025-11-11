#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use reqwest::Method;
use serde::Deserialize;
use open_lark_core::api_req::ApiRequest;
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
/// 获取日历列表请求
#[derive(Debug, Clone)]
pub struct ListCalendarRequest {
    pub api_req: ApiRequest,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 分页大小，最大值 50
    pub page_size: Option<i32>,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    pub page_token: Option<String>,
    /// 是否同步日历的权限信息
    pub sync_events: Option<bool>}
impl ListCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取日历列表请求构建器,
#[derive(Default)]
pub struct ListCalendarRequestBuilder {
    request: ListCalendarRequest}
impl ListCalendarRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_size) = self.request.page_size {,
            self.request,
.api_req
                .query_params
                .insert("page_size", page_size.to_string());
if let Some(ref page_token) = self.request.page_token {,
            self.request,
.api_req
                .query_params
                .insert("page_token", page_token.clone());
if let Some(sync_events) = self.request.sync_events {,
            self.request,
.api_req
                .query_params
                .insert("sync_events", sync_events.to_string());
self.request,
    }
/// 获取日历列表响应数据,
#[derive(Debug, Clone)]
pub struct ListCalendarResponseData {
    /// 是否还有更多项
    pub has_more: bool,
    /// 下次遍历的分页标记
    pub page_token: Option<String>,
    /// 日历列表
    pub calendars: Vec<Calendar>}
/// 获取日历列表响应,
#[derive(Debug, Clone)]
pub struct ListCalendarResponse {
    /// 响应数据
    pub data: ListCalendarResponseData,
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
    ListCalendarRequestBuilder,
    CalendarManagementService,
    ListCalendarRequest,
    BaseResponse<ListCalendarResponse>,
    list,
);

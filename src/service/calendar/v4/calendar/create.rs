#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::SDKResult;use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use open_lark_core::core::api_req::ApiRequest;
use crate::{,
core::{,
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
/// 创建共享日历请求
#[derive(Debug, Clone)]
pub struct CreateCalendarRequest {
    pub api_req: ApiRequest,
    /// 日历标题
    pub summary: Option<String>,
    /// 日历描述  
    pub description: Option<String>,
    /// 日历权限
    pub permissions: Option<String>,
    /// 日历颜色
    pub color: Option<i32>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>}
impl CreateCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建共享日历请求构建器,
#[derive(Default)]
pub struct CreateCalendarRequestBuilder {
    request: CreateCalendarRequest}
impl CreateCalendarRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 构建请求体,
        let mut body = json!({});
if let Some(ref summary) = self.request.summary {,
            body["summary"] = json!(summary);
if let Some(ref description) = self.request.description {,
            body["description"] = json!(description);
if let Some(ref permissions) = self.request.permissions {,
            body["permissions"] = json!(permissions);
if let Some(color) = self.request.color {,
            body["color"] = json!(color);
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 创建共享日历响应数据,
#[derive(Debug, Clone)]
pub struct CreateCalendarResponseData {
    /// 创建的日历信息
    pub calendar: Calendar,
/// 创建共享日历响应,
#[derive(Debug, Clone)]
pub struct CreateCalendarResponse {
    /// 响应数据
    pub data: CreateCalendarResponseData,
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
    CreateCalendarRequestBuilder,
    CalendarManagementService,
    CreateCalendarRequest,
    BaseResponse<CreateCalendarResponse>,
    create,
);

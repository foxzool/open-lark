use reqwest::Method;
use serde::Deserialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::calendar::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::calendar::v4::models::{Calendar, UserIdType},
};

use super::CalendarManagementService;

/// 获取日历列表请求
#[derive(Default, Clone)]
pub struct ListCalendarRequest {
    pub api_req: ApiRequest,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 分页大小，最大值 50
    pub page_size: Option<i32>,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    pub page_token: Option<String>,
    /// 是否同步日历的权限信息
    pub sync_events: Option<bool>,
}

impl ListCalendarRequest {
    /// 创建获取日历列表请求的构建器
    pub fn builder() -> ListCalendarRequestBuilder {
        ListCalendarRequestBuilder::default()
    }
}

/// 获取日历列表请求构建器
#[derive(Default)]
pub struct ListCalendarRequestBuilder {
    request: ListCalendarRequest,
}

impl ListCalendarRequestBuilder {
    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 设置分页大小，最大值 50
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置是否同步日历的权限信息
    pub fn sync_events(mut self, sync_events: bool) -> Self {
        self.request.sync_events = Some(sync_events);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> ListCalendarRequest {
        // 构建查询参数
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_req
                .query_params
                .insert("user_id_type", user_id_type.to_string());
        }

        if let Some(page_size) = self.request.page_size {
            self.request
                .api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(ref page_token) = self.request.page_token {
            self.request
                .api_req
                .query_params
                .insert("page_token", page_token.clone());
        }

        if let Some(sync_events) = self.request.sync_events {
            self.request
                .api_req
                .query_params
                .insert("sync_events", sync_events.to_string());
        }

        self.request
    }
}

/// 获取日历列表响应数据
#[derive(Debug, Deserialize)]
pub struct ListCalendarResponseData {
    /// 是否还有更多项
    pub has_more: bool,
    /// 下次遍历的分页标记
    pub page_token: Option<String>,
    /// 日历列表
    pub calendars: Vec<Calendar>,
}

/// 获取日历列表响应
#[derive(Debug, Deserialize)]
pub struct ListCalendarResponse {
    /// 响应数据
    pub data: ListCalendarResponseData,
}

impl ApiResponseTrait for ListCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CalendarManagementService {
    /// 获取日历列表
    ///
    /// 获取应用在企业内的日历列表。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 获取日历列表请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 日历列表
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.calendar.v4.calendar.list(
    ///     ListCalendarRequest::builder()
    ///         .page_size(20)
    ///         .user_id_type(UserIdType::UserId)
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/list>
    pub async fn list(
        &self,
        request: ListCalendarRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListCalendarResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = CALENDAR_V4_CALENDARS.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    ListCalendarRequestBuilder,
    CalendarManagementService,
    ListCalendarRequest,
    BaseResponse<ListCalendarResponse>,
    list
);

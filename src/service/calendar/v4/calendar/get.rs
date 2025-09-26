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

/// 查询日历信息请求
#[derive(Default, Clone)]
pub struct GetCalendarRequest {
    pub api_req: ApiRequest,
    /// 日历ID
    pub calendar_id: String,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl GetCalendarRequest {
    /// 创建查询日历请求的构建器
    pub fn builder(calendar_id: impl ToString) -> GetCalendarRequestBuilder {
        GetCalendarRequestBuilder {
            request: GetCalendarRequest {
                calendar_id: calendar_id.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 查询日历信息请求构建器
pub struct GetCalendarRequestBuilder {
    request: GetCalendarRequest,
}

impl GetCalendarRequestBuilder {
    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> GetCalendarRequest {
        // 构建查询参数
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_req
                .query_params
                .insert("user_id_type", user_id_type.to_string());
        }

        self.request
    }
}

/// 查询日历信息响应数据
#[derive(Debug, Deserialize)]
pub struct GetCalendarResponseData {
    /// 日历信息
    pub calendar: Calendar,
}

/// 查询日历信息响应
#[derive(Debug, Deserialize)]
pub struct GetCalendarResponse {
    /// 响应数据
    pub data: GetCalendarResponseData,
}

impl ApiResponseTrait for GetCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CalendarManagementService {
    /// 查询日历信息
    ///
    /// 根据日历ID查询日历信息。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 查询日历请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 日历信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.calendar.v4.calendar.get(
    ///     GetCalendarRequest::builder("calendar_id")
    ///         .user_id_type(UserIdType::UserId)
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/get>
    pub async fn get(
        &self,
        request: GetCalendarRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetCalendarResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path =
            CALENDAR_V4_CALENDAR_OPERATION.replace("{calendar_id}", &request.calendar_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    GetCalendarRequestBuilder,
    CalendarManagementService,
    GetCalendarRequest,
    BaseResponse<GetCalendarResponse>,
    get
);

use reqwest::Method;
use serde::Deserialize;
use serde_json::json;

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

/// 创建共享日历请求
#[derive(Default, Clone)]
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
    pub user_id_type: Option<UserIdType>,
}

impl CreateCalendarRequest {
    /// 创建创建日历请求的构建器
    pub fn builder() -> CreateCalendarRequestBuilder {
        CreateCalendarRequestBuilder::default()
    }
}

/// 创建共享日历请求构建器
#[derive(Default)]
pub struct CreateCalendarRequestBuilder {
    request: CreateCalendarRequest,
}

impl CreateCalendarRequestBuilder {
    /// 设置日历标题
    pub fn summary(mut self, summary: impl ToString) -> Self {
        self.request.summary = Some(summary.to_string());
        self
    }

    /// 设置日历描述
    pub fn description(mut self, description: impl ToString) -> Self {
        self.request.description = Some(description.to_string());
        self
    }

    /// 设置日历权限
    pub fn permissions(mut self, permissions: impl ToString) -> Self {
        self.request.permissions = Some(permissions.to_string());
        self
    }

    /// 设置日历颜色
    pub fn color(mut self, color: i32) -> Self {
        self.request.color = Some(color);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> CreateCalendarRequest {
        // 构建查询参数
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_req
                .query_params
                .insert("user_id_type", user_id_type.to_string());
        }

        // 构建请求体
        let mut body = json!({});

        if let Some(ref summary) = self.request.summary {
            body["summary"] = json!(summary);
        }

        if let Some(ref description) = self.request.description {
            body["description"] = json!(description);
        }

        if let Some(ref permissions) = self.request.permissions {
            body["permissions"] = json!(permissions);
        }

        if let Some(color) = self.request.color {
            body["color"] = json!(color);
        }

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 创建共享日历响应数据
#[derive(Debug, Deserialize)]
pub struct CreateCalendarResponseData {
    /// 创建的日历信息
    pub calendar: Calendar,
}

/// 创建共享日历响应
#[derive(Debug, Deserialize)]
pub struct CreateCalendarResponse {
    /// 响应数据
    pub data: CreateCalendarResponseData,
}

impl ApiResponseTrait for CreateCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CalendarManagementService {
    /// 创建共享日历
    ///
    /// 创建一个共享日历。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 创建日历请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 创建的日历信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.calendar.v4.calendar.create(
    ///     CreateCalendarRequest::builder()
    ///         .summary("团队日历")
    ///         .description("团队日程安排")
    ///         .color(1)
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/create>
    pub async fn create(
        &self,
        request: CreateCalendarRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateCalendarResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = CALENDAR_V4_CALENDARS.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    CreateCalendarRequestBuilder,
    CalendarManagementService,
    CreateCalendarRequest,
    BaseResponse<CreateCalendarResponse>,
    create
);

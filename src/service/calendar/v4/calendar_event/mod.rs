use crate::{core::config::Config, core::SDKResult, core::ApiRequest, impl_full_service};
pub mod create;
pub mod delete;
pub mod get;
pub mod instance_view;
pub mod instances;
pub mod list;
pub mod patch;
pub mod reply;
pub mod search;
pub mod subscription;
pub mod unsubscription;

// 重新导出所有请求和响应类型
pub use get::{GetCalendarEventRequest, GetCalendarEventResponse, GetCalendarEventBuilder};
pub use create::{CreateCalendarEventRequest, CreateCalendarEventResponse, CreateCalendarEventBuilder};

/// 日程管理服务
///
/// 提供日程的创建、删除、查询、更新、搜索等功能
pub struct CalendarEventService {
    pub config: Config,
}

// Service 抽象接入：Calendar v4 CalendarEventService
impl_full_service!(CalendarEventService, "calendar.event", "v4");

impl CalendarEventService {
    /// 创建日程事件服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::CalendarEventService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = CalendarEventService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取日程事件
    ///
    /// 获取指定日程事件的详细信息，包括事件标题、时间、地点、参与者等。
    ///
    /// # 参数
    /// - `req`: 获取日程事件请求
    ///
    /// # 返回
    /// - `Ok(GetCalendarEventResponse)`: 获取成功，返回日程事件详细信息
    /// - `Err(SDKError)`: 获取失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::GetCalendarEventRequest;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let request = GetCalendarEventRequest::new("calendar_123", "event_456");
    /// let response = client.calendar.v4.calendar_event.get(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, req: &GetCalendarEventRequest) -> SDKResult<GetCalendarEventResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始获取日程事件: calendar_id={}, event_id={}", req.calendar_id, req.event_id);

        // 构建动态端点路径
        let endpoint = crate::core::endpoints_original::Endpoints::CALENDAR_EVENT_GET
            .replace("{}", &req.calendar_id)
            .replace("{}", &req.event_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: endpoint,
            supported_access_token_types: vec![
                crate::core::constants::AccessTokenType::Tenant,
                crate::core::constants::AccessTokenType::User
            ],
            body: Vec::new(), // GET请求无body
            ..Default::default()
        };

        let resp = crate::core::http::Transport::<GetCalendarEventResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("日程事件获取完成: calendar_id={}, event_id={}, title={:?}",
                   req.calendar_id, req.event_id, response.event.as_ref().and_then(|e| e.summary.as_ref()));

        Ok(response)
    }

    /// 获取日程事件构建器
    ///
    /// 创建一个用于获取日程事件的构建器实例，支持流式API设计。
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    /// - `event_id`: 日程ID
    ///
    /// # 返回
    /// - `GetCalendarEventBuilder`: 日程事件获取构建器实例
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let response = client
    ///     .calendar
    ///     .v4
    ///     .calendar_event
    ///     .get_calendar_event_builder("calendar_123", "event_456")
    ///     .user_id_type("open_id")
    ///     .execute(&client.calendar.v4.calendar_event)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_calendar_event_builder(
        &self,
        calendar_id: impl Into<String>,
        event_id: impl Into<String>,
    ) -> GetCalendarEventBuilder {
        GetCalendarEventBuilder::new(calendar_id, event_id)
    }

    /// 创建日程事件
    ///
    /// 在指定日历中创建新的日程事件，支持设置事件标题、时间、地点、参与者等。
    ///
    /// # 参数
    /// - `req`: 创建日程事件请求
    ///
    /// # 返回
    /// - `Ok(CreateCalendarEventResponse)`: 创建成功，返回日程事件详细信息
    /// - `Err(SDKError)`: 创建失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::CreateCalendarEventRequest;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let request = CreateCalendarEventRequest::new("calendar_123");
    /// let response = client.calendar.v4.calendar_event.create(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, req: &CreateCalendarEventRequest) -> SDKResult<CreateCalendarEventResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始创建日程事件: calendar_id={}, summary={:?}", req.calendar_id, req.summary);

        // 构建动态端点路径
        let endpoint = crate::core::endpoints_original::Endpoints::CALENDAR_EVENT_CREATE
            .replace("{}", &req.calendar_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: endpoint,
            supported_access_token_types: vec![
                crate::core::constants::AccessTokenType::Tenant,
                crate::core::constants::AccessTokenType::User
            ],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = crate::core::http::Transport::<CreateCalendarEventResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("日程事件创建完成: calendar_id={}, event_id={:?}, title={:?}",
                   req.calendar_id, response.event.as_ref().and_then(|e| e.event_id.as_ref()), response.event.as_ref().and_then(|e| e.summary.as_ref()));

        Ok(response)
    }

    /// 创建日程事件构建器
    ///
    /// 创建一个用于创建日程事件的构建器实例，支持流式API设计。
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    ///
    /// # 返回
    /// - `CreateCalendarEventBuilder`: 日程事件创建构建器实例
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let response = client
    ///     .calendar
    ///     .v4
    ///     .calendar_event
    ///     .create_calendar_event_builder("calendar_123")
    ///     .summary("团队会议")
    ///     .description("讨论项目进展")
    ///     .user_id_type("open_id")
    ///     .execute(&client.calendar.v4.calendar_event)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_calendar_event_builder(
        &self,
        calendar_id: impl Into<String>,
    ) -> CreateCalendarEventBuilder {
        CreateCalendarEventBuilder::new(calendar_id)
    }
}
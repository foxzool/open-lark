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
pub use delete::{DeleteCalendarEventRequest, DeleteCalendarEventResponse, DeleteCalendarEventBuilder};
pub use list::{ListCalendarEventsRequest, ListCalendarEventsResponse, ListCalendarEventsBuilder};
pub use reply::{ReplyCalendarEventRequest, ReplyCalendarEventResponse, ReplyCalendarEventBuilder, EventReplyStatus};
pub use patch::{PatchCalendarEventRequest, PatchCalendarEventResponse, PatchCalendarEventBuilder};

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

    /// 获取日程事件列表
    ///
    /// 分页获取指定日历中的日程事件列表，支持按时间范围、关键词等条件过滤，
    /// 支持多种排序方式。
    ///
    /// # 参数
    /// - `req`: 获取日程事件列表请求
    ///
    /// # 返回
    /// - `Ok(ListCalendarEventsResponse)`: 获取成功，返回日程事件列表和分页信息
    /// - `Err(SDKError)`: 获取失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsRequest;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let request = ListCalendarEventsRequest::new("calendar_123")
    ///     .page_size(20)
    ///     .time_min("2024-01-01T00:00:00Z")
    ///     .time_max("2024-01-31T23:59:59Z")
    ///     .sort_by("start_time")
    ///     .user_id_type("open_id");
    /// let response = client.calendar.v4.calendar_event.list(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, req: &ListCalendarEventsRequest) -> SDKResult<ListCalendarEventsResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始获取日程事件列表: calendar_id={}, page_size={:?}",
                   req.calendar_id, req.page_size);

        // 构建动态端点路径和查询参数
        let base_endpoint = crate::core::endpoints_original::Endpoints::CALENDAR_EVENT_LIST
            .replace("{}", &req.calendar_id);
        let query_string = req.build_query_string();
        let endpoint = format!("{}{}", base_endpoint, query_string);

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

        let resp = crate::core::http::Transport::<ListCalendarEventsResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        let event_count = response.events.as_ref().map_or(0, |e| e.len());
        log::info!("日程事件列表获取完成: calendar_id={}, event_count={}, has_more={:?}",
                   req.calendar_id, event_count, response.has_more);

        Ok(response)
    }

    /// 获取日程事件列表构建器
    ///
    /// 创建一个用于获取日程事件列表的构建器实例，支持流式API设计。
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    ///
    /// # 返回
    /// - `ListCalendarEventsBuilder`: 日程事件列表获取构建器实例
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
    ///     .list_calendar_events_builder("calendar_123")
    ///     .page_size(20)
    ///     .time_min("2024-01-01T00:00:00Z")
    ///     .time_max("2024-01-31T23:59:59Z")
    ///     .sort_by("start_time")
    ///     .sort_order("asc")
    ///     .user_id_type("open_id")
    ///     .execute(&client.calendar.v4.calendar_event)
    ///     .await?;
    ///
    /// if let Some(events) = response.events {
    ///     println!("获取到 {} 个日程事件", events.len());
    ///     for event in events {
    ///         if let Some(summary) = event.summary {
    ///             println!("- {}", summary);
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_calendar_events_builder(
        &self,
        calendar_id: impl Into<String>,
    ) -> ListCalendarEventsBuilder {
        ListCalendarEventsBuilder::new(calendar_id)
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

    /// 删除日程事件
    ///
    /// 删除指定的日程事件，删除后不可恢复，请谨慎使用。
    ///
    /// # 参数
    /// - `req`: 删除日程事件请求
    ///
    /// # 返回
    /// - `Ok(DeleteCalendarEventResponse)`: 删除成功，返回操作结果
    /// - `Err(SDKError)`: 删除失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::DeleteCalendarEventRequest;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let request = DeleteCalendarEventRequest::new("calendar_123", "event_456");
    /// let response = client.calendar.v4.calendar_event.delete(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, req: &DeleteCalendarEventRequest) -> SDKResult<DeleteCalendarEventResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始删除日程事件: calendar_id={}, event_id={}", req.calendar_id, req.event_id);

        // 构建动态端点路径
        let endpoint = crate::core::endpoints_original::Endpoints::CALENDAR_EVENT_DELETE
            .replace("{}", &req.calendar_id)
            .replace("{}", &req.event_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: endpoint,
            supported_access_token_types: vec![
                crate::core::constants::AccessTokenType::Tenant,
                crate::core::constants::AccessTokenType::User
            ],
            body: Vec::new(), // DELETE请求无body
            ..Default::default()
        };

        let resp = crate::core::http::Transport::<DeleteCalendarEventResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("日程事件删除完成: calendar_id={}, event_id={}, success={:?}",
                   req.calendar_id, req.event_id, response.success);

        Ok(response)
    }

    /// 删除日程事件构建器
    ///
    /// 创建一个用于删除日程事件的构建器实例，支持流式API设计。
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    /// - `event_id`: 日程ID
    ///
    /// # 返回
    /// - `DeleteCalendarEventBuilder`: 日程事件删除构建器实例
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
    ///     .delete_calendar_event_builder("calendar_123", "event_456")
    ///     .user_id_type("open_id")
    ///     .execute(&client.calendar.v4.calendar_event)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_calendar_event_builder(
        &self,
        calendar_id: impl Into<String>,
        event_id: impl Into<String>,
    ) -> DeleteCalendarEventBuilder {
        DeleteCalendarEventBuilder::new(calendar_id, event_id)
    }

    /// 更新日程事件
    ///
    /// 部分更新指定日程事件的信息，支持更新事件标题、描述、时间、地点、参与者等信息的任意组合。
    ///
    /// # 参数
    /// - `req`: 更新日程事件请求
    ///
    /// # 返回
    /// - `Ok(PatchCalendarEventResponse)`: 更新成功，返回日程事件详细信息
    /// - `Err(SDKError)`: 更新失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::PatchCalendarEventRequest;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let request = PatchCalendarEventRequest::new("calendar_123", "event_456");
    /// let response = client.calendar.v4.calendar_event.patch(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn patch(&self, req: &PatchCalendarEventRequest) -> SDKResult<PatchCalendarEventResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始更新日程事件: calendar_id={}, event_id={}", req.calendar_id, req.event_id);

        // 构建动态端点路径
        let endpoint = crate::core::endpoints_original::Endpoints::CALENDAR_EVENT_UPDATE
            .replace("{}", &req.calendar_id)
            .replace("{}", &req.event_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path: endpoint,
            supported_access_token_types: vec![
                crate::core::constants::AccessTokenType::Tenant,
                crate::core::constants::AccessTokenType::User
            ],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = crate::core::http::Transport::<PatchCalendarEventResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("日程事件更新完成: calendar_id={}, event_id={}, title={:?}",
                   req.calendar_id, req.event_id, response.event.as_ref().and_then(|e| e.summary.as_ref()));

        Ok(response)
    }

    /// 更新日程事件构建器
    ///
    /// 创建一个用于更新日程事件的构建器实例，支持流式API设计。
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    /// - `event_id`: 日程ID
    ///
    /// # 返回
    /// - `PatchCalendarEventBuilder`: 日程事件更新构建器实例
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
    ///     .patch_calendar_event_builder("calendar_123", "event_456")
    ///     .summary("更新的会议标题")
    ///     .user_id_type("open_id")
    ///     .execute(&client.calendar.v4.calendar_event)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn patch_calendar_event_builder(
        &self,
        calendar_id: impl Into<String>,
        event_id: impl Into<String>,
    ) -> PatchCalendarEventBuilder {
        PatchCalendarEventBuilder::new(calendar_id, event_id)
    }

    /// 回复日程邀请
    ///
    /// 回复日程事件的邀请，支持接受、拒绝或暂定回复。
    /// 用户可以根据自己的日程安排对收到的日程邀请进行响应。
    ///
    /// # 参数
    /// - `req`: 回复日程事件请求
    ///
    /// # 返回
    /// - `Ok(ReplyCalendarEventResponse)`: 回复成功，返回操作结果
    /// - `Err(SDKError)`: 回复失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::{ReplyCalendarEventRequest, EventReplyStatus};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let request = ReplyCalendarEventRequest::new(
    ///     "calendar_123",
    ///     "event_456",
    ///     EventReplyStatus::Accept
    /// );
    /// let response = client.calendar.v4.calendar_event.reply(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reply(&self, req: &ReplyCalendarEventRequest) -> SDKResult<ReplyCalendarEventResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始回复日程邀请: calendar_id={}, event_id={}, reply_status={:?}",
                   req.calendar_id, req.event_id, req.reply_status);

        // 构建动态端点路径
        let endpoint = crate::core::endpoints_original::Endpoints::CALENDAR_EVENT_REPLY
            .replace("{}", &req.calendar_id)
            .replace("{}", &req.event_id);

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

        let resp = crate::core::http::Transport::<ReplyCalendarEventResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("日程邀请回复完成: calendar_id={}, event_id={}, reply_status={:?}, success={:?}",
                   req.calendar_id, req.event_id, response.reply_status, response.success);

        Ok(response)
    }

    /// 回复日程邀请构建器
    ///
    /// 创建一个用于回复日程邀请的构建器实例，支持流式API设计。
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    /// - `event_id`: 日程ID
    /// - `reply_status`: 回复状态
    ///
    /// # 返回
    /// - `ReplyCalendarEventBuilder`: 日程邀请回复构建器实例
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::{ReplyCalendarEventBuilder, EventReplyStatus};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let response = client
    ///     .calendar
    ///     .v4
    ///     .calendar_event
    ///     .reply_calendar_event_builder("calendar_123", "event_456", EventReplyStatus::Accept)
    ///     .comment("我会准时参加这个重要会议")
    ///     .send_notifications(true)
    ///     .user_id_type("open_id")
    ///     .execute(&client.calendar.v4.calendar_event)
    ///     .await?;
    ///
    /// if let Some(success) = response.success {
    ///     if success {
    ///         println!("日程邀请回复成功");
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn reply_calendar_event_builder(
        &self,
        calendar_id: impl Into<String>,
        event_id: impl Into<String>,
        reply_status: EventReplyStatus,
    ) -> ReplyCalendarEventBuilder {
        ReplyCalendarEventBuilder::new(calendar_id, event_id, reply_status)
    }
}
//! # 回复日程邀请
//!
//! 回复日程事件的邀请（接受/拒绝/暂定）。
//!
//! ## 功能描述
//!
//! 此接口用于回复日程事件的邀请，支持接受、拒绝或暂定回复。
//! 用户可以根据自己的日程安排对收到的日程邀请进行响应。
//!
//! ## API端点
//!
//! POST `/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/reply`
//!
//! ## 权限要求
//!
//! 需要 `calendar:calendar` 权限。
//!
//! ## 回复状态
//!
//! - `accept`: 接受邀请
//! - `decline`: 拒绝邀请
//! - `tentative`: 暂定回复
//!
//! ## 注意事项
//!
//! - 只有被邀请的参与者才能回复日程邀请
//! - 组织者通常不需要回复自己的日程
//! - 回复后可能需要同步到其他日历系统

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    http::Transport,
    ApiRequest, SDKResult,
};
use crate::service::calendar::v4::models::CalendarEvent;
use serde::{Deserialize, Serialize};

/// 日程邀请回复状态
///
/// 表示用户对日程邀请的回复状态。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventReplyStatus {
    /// 接受邀请
    Accept,
    /// 拒绝邀请
    Decline,
    /// 暂定回复
    Tentative,
}

/// 回复日程事件请求
///
/// 用于回复日程事件邀请的请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyCalendarEventRequest {
    /// 日历ID
    pub calendar_id: String,
    /// 日程ID
    pub event_id: String,
    /// 回复状态
    ///
    /// 可选值：
    /// - `accept`: 接受邀请
    /// - `decline`: 拒绝邀请
    /// - `tentative`: 暂定回复
    pub reply_status: EventReplyStatus,
    /// 回复留言
    ///
    /// 可选的回复留言，向组织者说明回复原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 是否发送通知
    ///
    /// true: 发送通知给组织者（默认）
    /// false: 不发送通知
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_notifications: Option<bool>,
    /// 用户ID类型
    ///
    /// 可选值：
    /// - `open_id`：用户的开放应用ID
    /// - `user_id`：用户的用户ID
    /// - `union_id`：用户的联合ID
    ///
    /// 默认值：`open_id`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl ReplyCalendarEventRequest {
    /// 创建回复日程事件请求
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    /// - `event_id`: 日程ID
    /// - `reply_status`: 回复状态
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::calendar::v4::calendar_event::{ReplyCalendarEventRequest, EventReplyStatus};
    ///
    /// let request = ReplyCalendarEventRequest::new(
    ///     "calendar_123",
    ///     "event_456",
    ///     EventReplyStatus::Accept
    /// );
    /// ```
    pub fn new(
        calendar_id: impl Into<String>,
        event_id: impl Into<String>,
        reply_status: EventReplyStatus,
    ) -> Self {
        Self {
            calendar_id: calendar_id.into(),
            event_id: event_id.into(),
            reply_status,
            comment: None,
            send_notifications: None,
            user_id_type: None,
        }
    }

    /// 设置回复留言
    ///
    /// # 参数
    /// - `comment`: 回复留言
    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.comment = Some(comment.into());
        self
    }

    /// 设置是否发送通知
    ///
    /// # 参数
    /// - `send_notifications`: 是否发送通知
    pub fn set_send_notifications(&mut self, send_notifications: bool) -> &mut Self {
        self.send_notifications = Some(send_notifications);
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    pub fn set_user_id_type(&mut self, user_id_type: impl Into<String>) -> &mut Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    ///
    /// # 返回
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.calendar_id.trim().is_empty() {
            return Err("日历ID不能为空".to_string());
        }

        if self.event_id.trim().is_empty() {
            return Err("日程ID不能为空".to_string());
        }

        if let Some(user_id_type) = &self.user_id_type {
            if user_id_type.trim().is_empty() {
                return Err("用户ID类型不能为空".to_string());
            }
        }

        // 验证回复留言长度
        if let Some(comment) = &self.comment {
            if comment.len() > 1000 {
                return Err("回复留言长度不能超过1000个字符".to_string());
            }
        }

        Ok(())
    }
}

/// 回复日程事件响应
///
/// 包含回复操作的结果信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyCalendarEventResponse {
    /// 日程事件信息
    pub event: Option<CalendarEvent>,
    /// 回复状态
    pub reply_status: Option<EventReplyStatus>,
    /// 是否回复成功
    pub success: Option<bool>,
    /// 操作结果消息
    pub message: Option<String>,
}

impl Default for ReplyCalendarEventResponse {
    fn default() -> Self {
        Self {
            event: None,
            reply_status: None,
            success: None,
            message: None,
        }
    }
}

impl ApiResponseTrait for ReplyCalendarEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 回复日程事件构建器
///
/// 提供流式API来构建回复日程事件的请求。
#[derive(Debug, Clone)]
pub struct ReplyCalendarEventBuilder {
    request: ReplyCalendarEventRequest,
}

impl ReplyCalendarEventBuilder {
    /// 创建回复日程事件构建器
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    /// - `event_id`: 日程ID
    /// - `reply_status`: 回复状态
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::calendar::v4::calendar_event::{ReplyCalendarEventBuilder, EventReplyStatus};
    ///
    /// let builder = ReplyCalendarEventBuilder::new(
    ///     "calendar_123",
    ///     "event_456",
    ///     EventReplyStatus::Accept
    /// );
    /// ```
    pub fn new(
        calendar_id: impl Into<String>,
        event_id: impl Into<String>,
        reply_status: EventReplyStatus,
    ) -> Self {
        Self {
            request: ReplyCalendarEventRequest::new(calendar_id, event_id, reply_status),
        }
    }

    /// 设置回复留言
    ///
    /// # 参数
    /// - `comment`: 回复留言
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::{ReplyCalendarEventBuilder, EventReplyStatus};
    /// let builder = ReplyCalendarEventBuilder::new(
    ///     "calendar_123",
    ///     "event_456",
    ///     EventReplyStatus::Accept
    /// ).comment("我会准时参加");
    /// ```
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.request.set_comment(comment);
        self
    }

    /// 设置是否发送通知
    ///
    /// # 参数
    /// - `send_notifications`: 是否发送通知
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::{ReplyCalendarEventBuilder, EventReplyStatus};
    /// let builder = ReplyCalendarEventBuilder::new(
    ///     "calendar_123",
    ///     "event_456",
    ///     EventReplyStatus::Accept
    /// ).send_notifications(true);
    /// ```
    pub fn send_notifications(mut self, send_notifications: bool) -> Self {
        self.request.set_send_notifications(send_notifications);
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::{ReplyCalendarEventBuilder, EventReplyStatus};
    /// let builder = ReplyCalendarEventBuilder::new(
    ///     "calendar_123",
    ///     "event_456",
    ///     EventReplyStatus::Accept
    /// ).user_id_type("open_id");
    /// ```
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.set_user_id_type(user_id_type);
        self
    }

    /// 构建请求对象
    ///
    /// # 返回
    /// - `Ok(ReplyCalendarEventRequest)`: 构建成功的请求对象
    /// - `Err(String)`: 构建失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::{ReplyCalendarEventBuilder, EventReplyStatus};
    /// let request = ReplyCalendarEventBuilder::new(
    ///     "calendar_123",
    ///     "event_456",
    ///     EventReplyStatus::Accept
    /// )
    /// .comment("我会准时参加")
    /// .user_id_type("open_id")
    /// .build()
    /// .unwrap();
    /// ```
    pub fn build(self) -> Result<ReplyCalendarEventRequest, String> {
        self.request.validate()?;
        Ok(self.request)
    }

    /// 执行回复日程事件请求
    ///
    /// # 参数
    /// - `service`: 日程事件服务实例
    ///
    /// # 返回
    /// - `Ok(ReplyCalendarEventResponse)`: 回复成功，返回操作结果
    /// - `Err(SDKError)`: 回复失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::{ReplyCalendarEventBuilder, EventReplyStatus};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let response = ReplyCalendarEventBuilder::new(
    ///     "calendar_123",
    ///     "event_456",
    ///     EventReplyStatus::Accept
    /// )
    /// .comment("我会准时参加这个重要会议")
    /// .send_notifications(true)
    /// .user_id_type("open_id")
    /// .execute(&client.calendar.v4.calendar_event)
    /// .await?;
    ///
    /// if let Some(success) = response.success {
    ///     if success {
    ///         println!("日程邀请回复成功");
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(
        self,
        service: &crate::service::calendar::v4::calendar_event::CalendarEventService,
    ) -> SDKResult<ReplyCalendarEventResponse> {
        let request = self
            .build()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        service.reply(&request).await
    }
}

impl Default for ReplyCalendarEventBuilder {
    fn default() -> Self {
        Self {
            request: ReplyCalendarEventRequest {
                calendar_id: String::new(),
                event_id: String::new(),
                reply_status: EventReplyStatus::Accept, // 默认值
                comment: None,
                send_notifications: None,
                user_id_type: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reply_calendar_event_request_creation() {
        let request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.event_id, "event_456");
        assert!(matches!(request.reply_status, EventReplyStatus::Accept));
        assert!(request.comment.is_none());
        assert!(request.send_notifications.is_none());
    }

    #[test]
    fn test_set_comment() {
        let mut request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        request.set_comment("我会准时参加");
        assert_eq!(request.comment, Some("我会准时参加".to_string()));
    }

    #[test]
    fn test_set_send_notifications() {
        let mut request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        request.set_send_notifications(false);
        assert_eq!(request.send_notifications, Some(false));
    }

    #[test]
    fn test_set_user_id_type() {
        let mut request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        request.set_user_id_type("user_id");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_request_validation_success() {
        let request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_request_validation_empty_calendar_id() {
        let request = ReplyCalendarEventRequest::new("", "event_456", EventReplyStatus::Accept);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_request_validation_empty_event_id() {
        let request = ReplyCalendarEventRequest::new("calendar_123", "", EventReplyStatus::Accept);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日程ID不能为空");
    }

    #[test]
    fn test_request_validation_empty_user_id_type() {
        let mut request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_request_validation_comment_too_long() {
        let mut request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        let long_comment = "a".repeat(1001);
        request.set_comment(&long_comment);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "回复留言长度不能超过1000个字符");
    }

    #[test]
    fn test_request_validation_comment_max_length() {
        let mut request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        let long_comment = "a".repeat(1000);
        request.set_comment(&long_comment);
        let result = request.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_reply_status_variants() {
        // Test all reply status variants
        let accept_request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        assert!(matches!(
            accept_request.reply_status,
            EventReplyStatus::Accept
        ));

        let decline_request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Decline);
        assert!(matches!(
            decline_request.reply_status,
            EventReplyStatus::Decline
        ));

        let tentative_request = ReplyCalendarEventRequest::new(
            "calendar_123",
            "event_456",
            EventReplyStatus::Tentative,
        );
        assert!(matches!(
            tentative_request.reply_status,
            EventReplyStatus::Tentative
        ));
    }

    #[test]
    fn test_reply_calendar_event_builder_creation() {
        let builder =
            ReplyCalendarEventBuilder::new("calendar_123", "event_456", EventReplyStatus::Accept);
        assert_eq!(builder.request.calendar_id, "calendar_123");
        assert_eq!(builder.request.event_id, "event_456");
        assert!(matches!(
            builder.request.reply_status,
            EventReplyStatus::Accept
        ));
        assert!(builder.request.comment.is_none());
    }

    #[test]
    fn test_builder_comment() {
        let builder =
            ReplyCalendarEventBuilder::new("calendar_123", "event_456", EventReplyStatus::Accept)
                .comment("确认参加");
        assert_eq!(builder.request.comment, Some("确认参加".to_string()));
    }

    #[test]
    fn test_builder_send_notifications() {
        let builder =
            ReplyCalendarEventBuilder::new("calendar_123", "event_456", EventReplyStatus::Accept)
                .send_notifications(false);
        assert_eq!(builder.request.send_notifications, Some(false));
    }

    #[test]
    fn test_builder_user_id_type() {
        let builder =
            ReplyCalendarEventBuilder::new("calendar_123", "event_456", EventReplyStatus::Accept)
                .user_id_type("union_id");
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_builder_build_success() {
        let result =
            ReplyCalendarEventBuilder::new("calendar_123", "event_456", EventReplyStatus::Accept)
                .comment("我会准时参加")
                .send_notifications(true)
                .user_id_type("open_id")
                .build();
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.event_id, "event_456");
        assert!(matches!(request.reply_status, EventReplyStatus::Accept));
        assert_eq!(request.comment, Some("我会准时参加".to_string()));
        assert_eq!(request.send_notifications, Some(true));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_builder_build_failure_empty_calendar_id() {
        let result =
            ReplyCalendarEventBuilder::new("", "event_456", EventReplyStatus::Accept).build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_builder_build_failure_empty_event_id() {
        let result =
            ReplyCalendarEventBuilder::new("calendar_123", "", EventReplyStatus::Accept).build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日程ID不能为空");
    }

    #[test]
    fn test_builder_default() {
        let builder = ReplyCalendarEventBuilder::default();
        assert_eq!(builder.request.calendar_id, "");
        assert_eq!(builder.request.event_id, "");
        assert!(matches!(
            builder.request.reply_status,
            EventReplyStatus::Accept
        ));
        assert!(builder.request.comment.is_none());
        assert!(builder.request.send_notifications.is_none());
    }

    #[test]
    fn test_serialization_request() {
        let request = ReplyCalendarEventRequest {
            calendar_id: "calendar_123".to_string(),
            event_id: "event_456".to_string(),
            reply_status: EventReplyStatus::Accept,
            comment: Some("我会准时参加".to_string()),
            send_notifications: Some(true),
            user_id_type: Some("open_id".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("calendar_123"));
        assert!(json.contains("event_456"));
        assert!(json.contains("accept"));
        assert!(json.contains("我会准时参加"));
        assert!(json.contains("open_id"));
    }

    #[test]
    fn test_serialization_request_minimal() {
        let request = ReplyCalendarEventRequest {
            calendar_id: "calendar_123".to_string(),
            event_id: "event_456".to_string(),
            reply_status: EventReplyStatus::Decline,
            comment: None,
            send_notifications: None,
            user_id_type: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("calendar_123"));
        assert!(json.contains("event_456"));
        assert!(json.contains("decline"));
        assert!(!json.contains("comment"));
        assert!(!json.contains("send_notifications"));
    }

    #[test]
    fn test_serialization_response() {
        let response = ReplyCalendarEventResponse {
            event: None,
            reply_status: Some(EventReplyStatus::Accept),
            success: Some(true),
            message: Some("回复成功".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("accept"));
        assert!(json.contains("true"));
        assert!(json.contains("回复成功"));
    }

    #[test]
    fn test_deserialization_request() {
        let json = r#"
        {
            "calendar_id": "calendar_123",
            "event_id": "event_456",
            "reply_status": "accept",
            "comment": "确认参加",
            "send_notifications": true,
            "user_id_type": "open_id"
        }
        "#;
        let request: ReplyCalendarEventRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.event_id, "event_456");
        assert!(matches!(request.reply_status, EventReplyStatus::Accept));
        assert_eq!(request.comment, Some("确认参加".to_string()));
        assert_eq!(request.send_notifications, Some(true));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_deserialization_response() {
        let json = r#"
        {
            "event": {
                "event_id": "event_456",
                "summary": "Team Meeting"
            },
            "reply_status": "accept",
            "success": true,
            "message": "回复成功"
        }
        "#;
        let response: ReplyCalendarEventResponse = serde_json::from_str(json).unwrap();
        assert!(response.event.is_some());
        assert!(matches!(
            response.reply_status,
            Some(EventReplyStatus::Accept)
        ));
        assert_eq!(response.success, Some(true));
        assert_eq!(response.message, Some("回复成功".to_string()));
    }

    #[test]
    fn test_default_response() {
        let response = ReplyCalendarEventResponse::default();
        assert!(response.event.is_none());
        assert!(response.reply_status.is_none());
        assert!(response.success.is_none());
        assert!(response.message.is_none());
    }

    #[test]
    fn test_request_with_whitespace_calendar_id() {
        let request = ReplyCalendarEventRequest::new("   ", "event_456", EventReplyStatus::Accept);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_request_with_whitespace_event_id() {
        let request =
            ReplyCalendarEventRequest::new("calendar_123", "   ", EventReplyStatus::Accept);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日程ID不能为空");
    }

    #[test]
    fn test_request_with_whitespace_user_id_type() {
        let mut request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        request.set_user_id_type("   ");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_validation_edge_cases() {
        // Test with very long calendar_id and event_id
        let long_id = "a".repeat(1000);
        let request = ReplyCalendarEventRequest::new(&long_id, &long_id, EventReplyStatus::Accept);
        assert!(request.validate().is_ok());

        // Test with Unicode characters in comment
        let mut request =
            ReplyCalendarEventRequest::new("calendar_123", "event_456", EventReplyStatus::Accept);
        request.set_comment("很高兴参加这个重要的会议，期待与大家交流合作！");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_builder_fluent_with_all_options() {
        let builder = ReplyCalendarEventBuilder::new(
            "calendar_123",
            "event_456",
            EventReplyStatus::Tentative,
        )
        .comment("暂时确认，可能需要调整时间")
        .send_notifications(true)
        .user_id_type("union_id");

        assert_eq!(builder.request.calendar_id, "calendar_123");
        assert_eq!(builder.request.event_id, "event_456");
        assert!(matches!(
            builder.request.reply_status,
            EventReplyStatus::Tentative
        ));
        assert_eq!(
            builder.request.comment,
            Some("暂时确认，可能需要调整时间".to_string())
        );
        assert_eq!(builder.request.send_notifications, Some(true));
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));

        let request = builder.build().unwrap();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_fluent_api_chain() {
        let result =
            ReplyCalendarEventBuilder::new("calendar_123", "event_456", EventReplyStatus::Decline)
                .comment("抱歉，时间冲突无法参加")
                .send_notifications(false)
                .user_id_type("user_id")
                .build();
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.event_id, "event_456");
        assert!(matches!(request.reply_status, EventReplyStatus::Decline));
        assert_eq!(request.comment, Some("抱歉，时间冲突无法参加".to_string()));
        assert_eq!(request.send_notifications, Some(false));
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_all_reply_status_builders() {
        // Test accept builder
        let accept_builder =
            ReplyCalendarEventBuilder::new("calendar_123", "event_456", EventReplyStatus::Accept);
        assert!(matches!(
            accept_builder.request.reply_status,
            EventReplyStatus::Accept
        ));

        // Test decline builder
        let decline_builder =
            ReplyCalendarEventBuilder::new("calendar_123", "event_456", EventReplyStatus::Decline);
        assert!(matches!(
            decline_builder.request.reply_status,
            EventReplyStatus::Decline
        ));

        // Test tentative builder
        let tentative_builder = ReplyCalendarEventBuilder::new(
            "calendar_123",
            "event_456",
            EventReplyStatus::Tentative,
        );
        assert!(matches!(
            tentative_builder.request.reply_status,
            EventReplyStatus::Tentative
        ));
    }
}

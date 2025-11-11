//! # 获取日程事件列表
//!
//! 分页获取指定日历中的日程事件列表。
//!
//! ## 功能描述
//!
//! 此接口用于分页获取指定日历中的日程事件列表，支持按时间范围、状态、
//! 关键词等条件过滤，支持多种排序方式。
//!
//! ## API端点
//!
//! GET `/open-apis/calendar/v4/calendars/{calendar_id}/events`
//!
//! ## 权限要求
//!
//! 需要 `calendar:calendar` 权限。
//!
//! ## 查询参数
//!
//! - `page_token`: 分页令牌，用于获取下一页数据
//! - `page_size`: 每页大小，默认20，最大100
//! - `time_min`: 开始时间过滤
//! - `time_max`: 结束时间过滤
//! - `query`: 关键词搜索
//! - `sort_by`: 排序字段（start_time, updated_time, created_time）
//! - `sort_order`: 排序方向（asc, desc）
//! - `user_id_type`: 用户ID类型

use crate::service::calendar::v4::models::CalendarEvent;
use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取日程事件列表请求
///
/// 用于分页获取指定日历中的日程事件列表的请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCalendarEventsRequest {
    /// 日历ID
    pub calendar_id: String,
    /// 分页令牌
    ///
    /// 用于获取下一页数据，第一次请求时为空
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页大小
    ///
    /// 取值范围：1-100，默认值：20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 开始时间过滤
    ///
    /// 格式：RFC3339格式的UTC时间戳，如 "2024-01-01T00:00:00Z"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_min: Option<String>,
    /// 结束时间过滤
    ///
    /// 格式：RFC3339格式的UTC时间戳，如 "2024-12-31T23:59:59Z"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_max: Option<String>,
    /// 关键词搜索
    ///
    /// 在日程标题和描述中搜索关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 排序字段
    ///
    /// 可选值：
    /// - `start_time`：按开始时间排序（默认）
    /// - `updated_time`：按更新时间排序
    /// - `created_time`：按创建时间排序
    /// - `summary`：按标题排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// 排序方向
    ///
    /// 可选值：
    /// - `asc`：升序（默认）
    /// - `desc`：降序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
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

impl ListCalendarEventsRequest {
    /// 创建获取日程事件列表请求
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsRequest;
    ///
    /// let request = ListCalendarEventsRequest::new("calendar_123");
    /// ```
    pub fn new(calendar_id: impl Into<String>) -> Self {
        Self {
            calendar_id: calendar_id.into(),
            page_token: None,
            page_size: None,
            time_min: None,
            time_max: None,
            query: None,
            sort_by: None,
            sort_order: None,
            user_id_type: None,
        }
    }

    /// 设置分页令牌
    ///
    /// # 参数
    /// - `page_token`: 分页令牌
    pub fn set_page_token(&mut self, page_token: impl Into<String>) -> &mut Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置每页大小
    ///
    /// # 参数
    /// - `page_size`: 每页大小（1-100）
    pub fn set_page_size(&mut self, page_size: i32) -> &mut Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置开始时间过滤
    ///
    /// # 参数
    /// - `time_min`: 开始时间
    pub fn set_time_min(&mut self, time_min: impl Into<String>) -> &mut Self {
        self.time_min = Some(time_min.into());
        self
    }

    /// 设置结束时间过滤
    ///
    /// # 参数
    /// - `time_max`: 结束时间
    pub fn set_time_max(&mut self, time_max: impl Into<String>) -> &mut Self {
        self.time_max = Some(time_max.into());
        self
    }

    /// 设置关键词搜索
    ///
    /// # 参数
    /// - `query`: 关键词
    pub fn set_query(&mut self, query: impl Into<String>) -> &mut Self {
        self.query = Some(query.into());
        self
    }

    /// 设置排序字段
    ///
    /// # 参数
    /// - `sort_by`: 排序字段
    pub fn set_sort_by(&mut self, sort_by: impl Into<String>) -> &mut Self {
        self.sort_by = Some(sort_by.into());
        self
    }

    /// 设置排序方向
    ///
    /// # 参数
    /// - `sort_order`: 排序方向
    pub fn set_sort_order(&mut self, sort_order: impl Into<String>) -> &mut Self {
        self.sort_order = Some(sort_order.into());
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

        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err("每页大小必须在1-100之间".to_string());
            }
        }

        if let Some(user_id_type) = &self.user_id_type {
            if user_id_type.trim().is_empty() {
                return Err("用户ID类型不能为空".to_string());
            }
        }

        // 验证排序字段
        if let Some(sort_by) = &self.sort_by {
            if !["start_time", "updated_time", "created_time", "summary"]
                .contains(&sort_by.as_str())
            {
                return Err(
                    "排序字段必须是 start_time、updated_time、created_time 或 summary".to_string(),
                );
            }
        }

        // 验证排序方向
        if let Some(sort_order) = &self.sort_order {
            if !["asc", "desc"].contains(&sort_order.as_str()) {
                return Err("排序方向必须是 asc 或 desc".to_string());
            }
        }

        Ok(())
    }

    /// 构建查询字符串
    ///
    /// # 返回
    /// - `String`: 查询字符串
    pub fn build_query_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(page_token) = &self.page_token {
            params.push(format!("page_token={}", urlencoding::encode(page_token)));
        }

        if let Some(page_size) = self.page_size {
            params.push(format!("page_size={}", page_size));
        }

        if let Some(time_min) = &self.time_min {
            params.push(format!("time_min={}", urlencoding::encode(time_min)));
        }

        if let Some(time_max) = &self.time_max {
            params.push(format!("time_max={}", urlencoding::encode(time_max)));
        }

        if let Some(query) = &self.query {
            params.push(format!("query={}", urlencoding::encode(query)));
        }

        if let Some(sort_by) = &self.sort_by {
            params.push(format!("sort_by={}", urlencoding::encode(sort_by)));
        }

        if let Some(sort_order) = &self.sort_order {
            params.push(format!("sort_order={}", urlencoding::encode(sort_order)));
        }

        if let Some(user_id_type) = &self.user_id_type {
            params.push(format!(
                "user_id_type={}",
                urlencoding::encode(user_id_type)
            ));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}

/// 获取日程事件列表响应
///
/// 包含日程事件列表和分页信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCalendarEventsResponse {
    /// 日程事件列表
    pub events: Option<Vec<CalendarEvent>>,
    /// 分页令牌
    ///
    /// 用于获取下一页数据，为空表示已到最后一页
    pub next_page_token: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 总数（如果有）
    pub total: Option<i32>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
}

impl Default for ListCalendarEventsResponse {
    fn default() -> Self {
        Self {
            events: None,
            next_page_token: None,
            page_size: None,
            total: None,
            has_more: None,
        }
    }
}

impl ApiResponseTrait for ListCalendarEventsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取日程事件列表构建器
///
/// 提供流式API来构建获取日程事件列表的请求。
#[derive(Debug, Clone)]
pub struct ListCalendarEventsBuilder {
    request: ListCalendarEventsRequest,
}

impl ListCalendarEventsBuilder {
    /// 创建获取日程事件列表构建器
    ///
    /// # 参数
    /// - `calendar_id`: 日历ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    ///
    /// let builder = ListCalendarEventsBuilder::new("calendar_123");
    /// ```
    pub fn new(calendar_id: impl Into<String>) -> Self {
        Self {
            request: ListCalendarEventsRequest::new(calendar_id),
        }
    }

    /// 设置分页令牌
    ///
    /// # 参数
    /// - `page_token`: 分页令牌
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    /// let builder = ListCalendarEventsBuilder::new("calendar_123")
    ///     .page_token("next_page_token_123");
    /// ```
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.set_page_token(page_token);
        self
    }

    /// 设置每页大小
    ///
    /// # 参数
    /// - `page_size`: 每页大小（1-100）
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    /// let builder = ListCalendarEventsBuilder::new("calendar_123")
    ///     .page_size(50);
    /// ```
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.set_page_size(page_size);
        self
    }

    /// 设置开始时间过滤
    ///
    /// # 参数
    /// - `time_min`: 开始时间
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    /// let builder = ListCalendarEventsBuilder::new("calendar_123")
    ///     .time_min("2024-01-01T00:00:00Z");
    /// ```
    pub fn time_min(mut self, time_min: impl Into<String>) -> Self {
        self.request.set_time_min(time_min);
        self
    }

    /// 设置结束时间过滤
    ///
    /// # 参数
    /// - `time_max`: 结束时间
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    /// let builder = ListCalendarEventsBuilder::new("calendar_123")
    ///     .time_max("2024-12-31T23:59:59Z");
    /// ```
    pub fn time_max(mut self, time_max: impl Into<String>) -> Self {
        self.request.set_time_max(time_max);
        self
    }

    /// 设置关键词搜索
    ///
    /// # 参数
    /// - `query`: 关键词
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    /// let builder = ListCalendarEventsBuilder::new("calendar_123")
    ///     .query("会议");
    /// ```
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.request.set_query(query);
        self
    }

    /// 设置排序字段
    ///
    /// # 参数
    /// - `sort_by`: 排序字段
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    /// let builder = ListCalendarEventsBuilder::new("calendar_123")
    ///     .sort_by("start_time");
    /// ```
    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.request.set_sort_by(sort_by);
        self
    }

    /// 设置排序方向
    ///
    /// # 参数
    /// - `sort_order`: 排序方向
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    /// let builder = ListCalendarEventsBuilder::new("calendar_123")
    ///     .sort_order("desc");
    /// ```
    pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
        self.request.set_sort_order(sort_order);
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
    /// # use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    /// let builder = ListCalendarEventsBuilder::new("calendar_123")
    ///     .user_id_type("open_id");
    /// ```
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.set_user_id_type(user_id_type);
        self
    }

    /// 构建请求对象
    ///
    /// # 返回
    /// - `Ok(ListCalendarEventsRequest)`: 构建成功的请求对象
    /// - `Err(String)`: 构建失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// # use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    /// let request = ListCalendarEventsBuilder::new("calendar_123")
    ///     .page_size(20)
    ///     .sort_by("start_time")
    ///     .user_id_type("open_id")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<ListCalendarEventsRequest, String> {
        self.request.validate()?;
        Ok(self.request)
    }

    /// 执行获取日程事件列表请求
    ///
    /// # 参数
    /// - `service`: 日程事件服务实例
    ///
    /// # 返回
    /// - `Ok(ListCalendarEventsResponse)`: 获取成功，返回日程事件列表
    /// - `Err(SDKError)`: 获取失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::calendar::v4::calendar_event::ListCalendarEventsBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LarkClient::builder("app_id", "app_secret").build()?;
    /// let response = ListCalendarEventsBuilder::new("calendar_123")
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
    pub async fn execute(
        self,
        service: &crate::service::calendar::v4::calendar_event::CalendarEventService,
    ) -> SDKResult<ListCalendarEventsResponse> {
        let request = self
            .build()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        service.list(&request).await
    }
}

impl Default for ListCalendarEventsBuilder {
    fn default() -> Self {
        Self {
            request: ListCalendarEventsRequest {
                calendar_id: String::new(),
                page_token: None,
                page_size: None,
                time_min: None,
                time_max: None,
                query: None,
                sort_by: None,
                sort_order: None,
                user_id_type: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_calendar_events_request_creation() {
        let request = ListCalendarEventsRequest::new("calendar_123");
        assert_eq!(request.calendar_id, "calendar_123");
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_set_page_token() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_page_token("token_456");
        assert_eq!(request.page_token, Some("token_456".to_string()));
    }

    #[test]
    fn test_set_page_size() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_page_size(50);
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_set_time_min() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_time_min("2024-01-01T00:00:00Z");
        assert_eq!(request.time_min, Some("2024-01-01T00:00:00Z".to_string()));
    }

    #[test]
    fn test_set_time_max() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_time_max("2024-12-31T23:59:59Z");
        assert_eq!(request.time_max, Some("2024-12-31T23:59:59Z".to_string()));
    }

    #[test]
    fn test_set_query() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_query("团队会议");
        assert_eq!(request.query, Some("团队会议".to_string()));
    }

    #[test]
    fn test_set_sort_by() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_sort_by("start_time");
        assert_eq!(request.sort_by, Some("start_time".to_string()));
    }

    #[test]
    fn test_set_sort_order() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_sort_order("desc");
        assert_eq!(request.sort_order, Some("desc".to_string()));
    }

    #[test]
    fn test_set_user_id_type() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_user_id_type("user_id");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_request_validation_success() {
        let request = ListCalendarEventsRequest::new("calendar_123");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_request_validation_empty_calendar_id() {
        let request = ListCalendarEventsRequest::new("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_request_validation_page_size_too_small() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_page_size(0);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "每页大小必须在1-100之间");
    }

    #[test]
    fn test_request_validation_page_size_too_large() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_page_size(101);
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "每页大小必须在1-100之间");
    }

    #[test]
    fn test_request_validation_empty_user_id_type() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_user_id_type("");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_request_validation_invalid_sort_by() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_sort_by("invalid_field");
        let result = request.validate();
        assert!(result.is_err());
        assert!(
            result.unwrap_err(),
            "排序字段必须是 start_time、updated_time、created_time 或 summary"
        );
    }

    #[test]
    fn test_request_validation_invalid_sort_order() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_sort_order("invalid_order");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "排序方向必须是 asc 或 desc");
    }

    #[test]
    fn test_build_query_string_empty() {
        let request = ListCalendarEventsRequest::new("calendar_123");
        let query_string = request.build_query_string();
        assert_eq!(query_string, "");
    }

    #[test]
    fn test_build_query_string_with_params() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_page_size(20);
        request.set_query("团队会议");
        request.set_sort_by("start_time");
        request.set_sort_order("desc");
        request.set_user_id_type("open_id");

        let query_string = request.build_query_string();
        assert!(query_string.contains("page_size=20"));
        assert!(query_string.contains("query="));
        assert!(query_string.contains("sort_by=start_time"));
        assert!(query_string.contains("sort_order=desc"));
        assert!(query_string.contains("user_id_type=open_id"));
        assert!(query_string.starts_with("?"));
    }

    #[test]
    fn test_build_query_string_with_special_chars() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_query("会议 & 讨论");
        request.set_time_min("2024-01-01T00:00:00Z");

        let query_string = request.build_query_string();
        assert!(query_string.contains("query=%E4%BC%9A%E8%AE%AE%20%26%20%E8%AE%A8%E8%AE%BA"));
        assert!(query_string.contains("time_min=2024-01-01T00%3A00%3A00Z"));
    }

    #[test]
    fn test_list_calendar_events_builder_creation() {
        let builder = ListCalendarEventsBuilder::new("calendar_123");
        assert_eq!(builder.request.calendar_id, "calendar_123");
        assert!(builder.request.page_token.is_none());
        assert!(builder.request.page_size.is_none());
    }

    #[test]
    fn test_builder_page_token() {
        let builder = ListCalendarEventsBuilder::new("calendar_123").page_token("next_page_token");
        assert_eq!(
            builder.request.page_token,
            Some("next_page_token".to_string())
        );
    }

    #[test]
    fn test_builder_page_size() {
        let builder = ListCalendarEventsBuilder::new("calendar_123").page_size(50);
        assert_eq!(builder.request.page_size, Some(50));
    }

    #[test]
    fn test_builder_time_min() {
        let builder =
            ListCalendarEventsBuilder::new("calendar_123").time_min("2024-01-01T00:00:00Z");
        assert_eq!(
            builder.request.time_min,
            Some("2024-01-01T00:00:00Z".to_string())
        );
    }

    #[test]
    fn test_builder_time_max() {
        let builder =
            ListCalendarEventsBuilder::new("calendar_123").time_max("2024-12-31T23:59:59Z");
        assert_eq!(
            builder.request.time_max,
            Some("2024-12-31T23:59:59Z".to_string())
        );
    }

    #[test]
    fn test_builder_query() {
        let builder = ListCalendarEventsBuilder::new("calendar_123").query("周会");
        assert_eq!(builder.request.query, Some("周会".to_string()));
    }

    #[test]
    fn test_builder_sort_by() {
        let builder = ListCalendarEventsBuilder::new("calendar_123").sort_by("updated_time");
        assert_eq!(builder.request.sort_by, Some("updated_time".to_string()));
    }

    #[test]
    fn test_builder_sort_order() {
        let builder = ListCalendarEventsBuilder::new("calendar_123").sort_order("desc");
        assert_eq!(builder.request.sort_order, Some("desc".to_string()));
    }

    #[test]
    fn test_builder_user_id_type() {
        let builder = ListCalendarEventsBuilder::new("calendar_123").user_id_type("union_id");
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_builder_build_success() {
        let result = ListCalendarEventsBuilder::new("calendar_123")
            .page_size(20)
            .sort_by("start_time")
            .sort_order("asc")
            .user_id_type("open_id")
            .build();
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.sort_by, Some("start_time".to_string()));
        assert_eq!(request.sort_order, Some("asc".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_builder_build_failure_empty_calendar_id() {
        let result = ListCalendarEventsBuilder::new("").build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_builder_build_failure_invalid_page_size() {
        let result = ListCalendarEventsBuilder::new("calendar_123")
            .page_size(0)
            .build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "每页大小必须在1-100之间");
    }

    #[test]
    fn test_builder_default() {
        let builder = ListCalendarEventsBuilder::default();
        assert_eq!(builder.request.calendar_id, "");
        assert!(builder.request.page_token.is_none());
        assert!(builder.request.page_size.is_none());
    }

    #[test]
    fn test_serialization_request() {
        let request = ListCalendarEventsRequest {
            calendar_id: "calendar_123".to_string(),
            page_token: Some("token_456".to_string()),
            page_size: Some(20),
            time_min: Some("2024-01-01T00:00:00Z".to_string()),
            time_max: Some("2024-12-31T23:59:59Z".to_string()),
            query: Some("团队会议".to_string()),
            sort_by: Some("start_time".to_string()),
            sort_order: Some("asc".to_string()),
            user_id_type: Some("open_id".to_string()),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("calendar_123"));
        assert!(json.contains("token_456"));
        assert!(json.contains("团队会议"));
        assert!(json.contains("open_id"));
    }

    #[test]
    fn test_serialization_request_minimal() {
        let request = ListCalendarEventsRequest {
            calendar_id: "calendar_123".to_string(),
            page_token: None,
            page_size: None,
            time_min: None,
            time_max: None,
            query: None,
            sort_by: None,
            sort_order: None,
            user_id_type: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("calendar_123"));
        assert!(!json.contains("page_token"));
        assert!(!json.contains("page_size"));
    }

    #[test]
    fn test_deserialization_response() {
        let json = r#"
        {
            "events": [
                {
                    "event_id": "event_456",
                    "summary": "Team Meeting",
                    "description": "Weekly team sync"
                }
            ],
            "next_page_token": "next_page_token_789",
            "page_size": 20,
            "total": 1,
            "has_more": false
        }
        "#;
        let response: ListCalendarEventsResponse = serde_json::from_str(json).unwrap();
        assert!(response.events.is_some());
        assert_eq!(response.events.unwrap().len(), 1);
        assert_eq!(
            response.next_page_token,
            Some("next_page_token_789".to_string())
        );
        assert_eq!(response.page_size, Some(20));
        assert_eq!(response.total, Some(1));
        assert_eq!(response.has_more, Some(false));
    }

    #[test]
    fn test_default_response() {
        let response = ListCalendarEventsResponse::default();
        assert!(response.events.is_none());
        assert!(response.next_page_token.is_none());
        assert!(response.page_size.is_none());
        assert!(response.total.is_none());
        assert!(response.has_more.is_none());
    }

    #[test]
    fn test_request_with_whitespace_calendar_id() {
        let request = ListCalendarEventsRequest::new("   ");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "日历ID不能为空");
    }

    #[test]
    fn test_request_with_whitespace_user_id_type() {
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_user_id_type("   ");
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户ID类型不能为空");
    }

    #[test]
    fn test_validation_edge_cases() {
        // Test with very long values
        let long_query = "a".repeat(1000);
        let mut request = ListCalendarEventsRequest::new("calendar_123");
        request.set_query(&long_query);
        assert!(request.validate().is_ok());

        // Test with valid sort_by values
        let valid_sort_fields = ["start_time", "updated_time", "created_time", "summary"];
        for sort_field in &valid_sort_fields {
            request.set_sort_by(sort_field);
            assert!(request.validate().is_ok());
        }

        // Test with valid sort_order values
        let valid_sort_orders = ["asc", "desc"];
        for sort_order in &valid_sort_orders {
            request.set_sort_order(sort_order);
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_builder_fluent_with_all_options() {
        let builder = ListCalendarEventsBuilder::new("calendar_123")
            .page_token("next_token")
            .page_size(50)
            .time_min("2024-01-01T00:00:00Z")
            .time_max("2024-12-31T23:59:59Z")
            .query("年度会议")
            .sort_by("summary")
            .sort_order("desc")
            .user_id_type("union_id");

        assert_eq!(builder.request.calendar_id, "calendar_123");
        assert_eq!(builder.request.page_token, Some("next_token".to_string()));
        assert_eq!(builder.request.page_size, Some(50));
        assert_eq!(
            builder.request.time_min,
            Some("2024-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            builder.request.time_max,
            Some("2024-12-31T23:59:59Z".to_string())
        );
        assert_eq!(builder.request.query, Some("年度会议".to_string()));
        assert_eq!(builder.request.sort_by, Some("summary".to_string()));
        assert_eq!(builder.request.sort_order, Some("desc".to_string()));
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));

        let request = builder.build().unwrap();
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_fluent_api_chain() {
        let result = ListCalendarEventsBuilder::new("calendar_123")
            .page_size(20)
            .query("重要会议")
            .sort_by("start_time")
            .sort_order("asc")
            .user_id_type("user_id")
            .build();
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(request.calendar_id, "calendar_123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.query, Some("重要会议".to_string()));
        assert_eq!(request.sort_by, Some("start_time".to_string()));
        assert_eq!(request.sort_order, Some("asc".to_string()));
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }
}

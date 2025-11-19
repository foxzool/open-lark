//! OpenLark Collaboration 服务端点定义
//!
//! 此模块包含协作相关的所有API端点，从 openlark-core 迁移而来。
//! 包含项目看板、日程管理、会议服务、任务协作等完整功能。
//!
//! # 服务模块包含
//!
//! - **board**: 项目看板（看板管理、任务卡片、工作流）
//! - **calendar**: 日程管理（日历安排、会议预订、时间协调）
//! - **meeting_room**: 会议室管理（会议室预订、设备管理）
//! - **minutes**: 会议纪要（纪要管理、行动项跟踪）
//! - **task**: 任务协作（任务分配、进度跟踪）
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_collab::endpoints::*;
//!
//! // 项目看板
//! let boards_endpoint = BOARD_V1_BOARDS;
//! let cards_endpoint = BOARD_V1_CARDS;
//!
//! // 日程管理
//! let calendar_endpoint = CALENDAR_V4_CALENDARS;
//! let events_endpoint = CALENDAR_V4_EVENTS;
//!
//! // 会议服务
//! let meeting_rooms_endpoint = MEETING_ROOM_V1_ROOMS;
//! let minutes_endpoint = MINUTES_V1_MINUTES;
//!
//! // 任务协作
//! let tasks_endpoint = TASK_V1_TASKS;
//! ```

// 导入核心端点（auth, application等基础端点）
pub use openlark_core::endpoints::{apass, application, auth, platform_integration};

// ==================== Board (项目看板) v1 ====================
// 看板系统 - 看板管理、任务卡片、工作流

/// Board看板管理 v1
/// 项目看板和任务管理
pub const BOARD_V1_BOARDS: &str = "/open-apis/board/v1/boards";
pub const BOARD_V1_BOARD_GET: &str = "/open-apis/board/v1/boards/{board_id}";
pub const BOARD_V1_BOARD_CREATE: &str = "/open-apis/board/v1/boards";
pub const BOARD_V1_BOARD_UPDATE: &str = "/open-apis/board/v1/boards/{board_id}";
pub const BOARD_V1_BOARD_DELETE: &str = "/open-apis/board/v1/boards/{board_id}";

/// Board卡片管理 v1
/// 任务卡片和工作项管理
pub const BOARD_V1_CARDS: &str = "/open-apis/board/v1/cards";
pub const BOARD_V1_CARD_GET: &str = "/open-apis/board/v1/cards/{card_id}";
pub const BOARD_V1_CARD_CREATE: &str = "/open-apis/board/v1/cards";
pub const BOARD_V1_CARD_UPDATE: &str = "/open-apis/board/v1/cards/{card_id}";
pub const BOARD_V1_CARD_DELETE: &str = "/open-apis/board/v1/cards/{card_id}";
pub const BOARD_V1_CARD_MOVE: &str = "/open-apis/board/v1/cards/{card_id}/move";

/// Board列表管理 v1
/// 看板列表和分组管理
pub const BOARD_V1_LISTS: &str = "/open-apis/board/v1/lists";
pub const BOARD_V1_LIST_GET: &str = "/open-apis/board/v1/lists/{list_id}";
pub const BOARD_V1_LIST_CREATE: &str = "/open-apis/board/v1/lists";
pub const BOARD_V1_LIST_UPDATE: &str = "/open-apis/board/v1/lists/{list_id}";
pub const BOARD_V1_LIST_DELETE: &str = "/open-apis/board/v1/lists/{list_id}";
pub const BOARD_V1_LIST_MOVE: &str = "/open-apis/board/v1/lists/{list_id}/move";

// ==================== Calendar (日程管理) v4 ====================
// 日历系统 - 日历安排、会议预订、时间协调

/// Calendar日历管理 v4
/// 日历创建和管理
pub const CALENDAR_V4_CALENDARS: &str = "/open-apis/calendar/v4/calendars";
pub const CALENDAR_V4_CALENDAR_GET: &str = "/open-apis/calendar/v4/calendars/{calendar_id}";
pub const CALENDAR_V4_CALENDAR_CREATE: &str = "/open-apis/calendar/v4/calendars";
pub const CALENDAR_V4_CALENDAR_UPDATE: &str = "/open-apis/calendar/v4/calendars/{calendar_id}";
pub const CALENDAR_V4_CALENDAR_DELETE: &str = "/open-apis/calendar/v4/calendars/{calendar_id}";
pub const CALENDAR_V4_CALENDAR_SEARCH: &str = "/open-apis/calendar/v4/calendars/search";
pub const CALENDAR_V4_CALENDAR_SUBSCRIBE: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/subscribe";
pub const CALENDAR_V4_CALENDAR_UNSUBSCRIBE: &str =
    "/open-apis/calendar/v4/calendars/{calendar_id}/unsubscribe";

/// Calendar事件管理 v4
/// 日程事件和会议安排
pub const CALENDAR_V4_EVENTS: &str = "/open-apis/calendar/v4/events";
pub const CALENDAR_V4_EVENT_GET: &str = "/open-apis/calendar/v4/events/{event_id}";
pub const CALENDAR_V4_EVENT_CREATE: &str = "/open-apis/calendar/v4/events";
pub const CALENDAR_V4_EVENT_UPDATE: &str = "/open-apis/calendar/v4/events/{event_id}";
pub const CALENDAR_V4_EVENT_DELETE: &str = "/open-apis/calendar/v4/events/{event_id}";
pub const CALENDAR_V4_EVENT_SEARCH: &str = "/open-apis/calendar/v4/events/search";
pub const CALENDAR_V4_EVENT_RSVP: &str = "/open-apis/calendar/v4/events/{event_id}/rsvp";

/// Calendar时间安排 v4
/// 忙闲查询和时间协调
pub const CALENDAR_V4_FREEBUSY: &str = "/open-apis/calendar/v4/freebusy";
pub const CALENDAR_V4_SCHEDULE: &str = "/open-apis/calendar/v4/schedule";
pub const CALENDAR_V4_AVAILABILITY: &str = "/open-apis/calendar/v4/availability";

// ==================== Meeting Room (会议室管理) v1 ====================
// 会议室系统 - 会议室预订、设备管理

/// Meeting Room会议室管理 v1
/// 会议室预订和管理
pub const MEETING_ROOM_V1_ROOMS: &str = "/open-apis/meeting_room/v1/rooms";
pub const MEETING_ROOM_V1_ROOM_GET: &str = "/open-apis/meeting_room/v1/rooms/{room_id}";
pub const MEETING_ROOM_V1_ROOM_CREATE: &str = "/open-apis/meeting_room/v1/rooms";
pub const MEETING_ROOM_V1_ROOM_UPDATE: &str = "/open-apis/meeting_room/v1/rooms/{room_id}";
pub const MEETING_ROOM_V1_ROOM_DELETE: &str = "/open-apis/meeting_room/v1/rooms/{room_id}";
pub const MEETING_ROOM_V1_ROOM_SEARCH: &str = "/open-apis/meeting_room/v1/rooms/search";
pub const MEETING_ROOM_V1_ROOM_AVAILABLE: &str = "/open-apis/meeting_room/v1/rooms/available";

/// Meeting Room预订管理 v1
/// 会议室预订和取消
pub const MEETING_ROOM_V1_BOOKINGS: &str = "/open-apis/meeting_room/v1/bookings";
pub const MEETING_ROOM_V1_BOOKING_GET: &str = "/open-apis/meeting_room/v1/bookings/{booking_id}";
pub const MEETING_ROOM_V1_BOOKING_CREATE: &str = "/open-apis/meeting_room/v1/bookings";
pub const MEETING_ROOM_V1_BOOKING_UPDATE: &str = "/open-apis/meeting_room/v1/bookings/{booking_id}";
pub const MEETING_ROOM_V1_BOOKING_DELETE: &str = "/open-apis/meeting_room/v1/bookings/{booking_id}";
pub const MEETING_ROOM_V1_BOOKING_SEARCH: &str = "/open-apis/meeting_room/v1/bookings/search";

// ==================== Minutes (会议纪要) v1 ====================
// 会议纪要系统 - 纪要管理、行动项跟踪

/// Minutes会议纪要 v1
/// 会议纪要创建和管理
pub const MINUTES_V1_MINUTES: &str = "/open-apis/minutes/v1/minutes";
pub const MINUTES_V1_MINUTE_GET: &str = "/open-apis/minutes/v1/minutes/{minute_id}";
pub const MINUTES_V1_MINUTE_CREATE: &str = "/open-apis/minutes/v1/minutes";
pub const MINUTES_V1_MINUTE_UPDATE: &str = "/open-apis/minutes/v1/minutes/{minute_id}";
pub const MINUTES_V1_MINUTE_DELETE: &str = "/open-apis/minutes/v1/minutes/{minute_id}";
pub const MINUTES_V1_MINUTE_SEARCH: &str = "/open-apis/minutes/v1/minutes/search";

/// Minutes行动项管理 v1
/// 会议行动项和任务跟踪
pub const MINUTES_V1_ACTION_ITEMS: &str = "/open-apis/minutes/v1/action_items";
pub const MINUTES_V1_ACTION_ITEM_GET: &str = "/open-apis/minutes/v1/action_items/{action_item_id}";
pub const MINUTES_V1_ACTION_ITEM_CREATE: &str = "/open-apis/minutes/v1/action_items";
pub const MINUTES_V1_ACTION_ITEM_UPDATE: &str =
    "/open-apis/minutes/v1/action_items/{action_item_id}";
pub const MINUTES_V1_ACTION_ITEM_DELETE: &str =
    "/open-apis/minutes/v1/action_items/{action_item_id}";

/// Minutes模板管理 v1
/// 会议纪要模板管理
pub const MINUTES_V1_TEMPLATES: &str = "/open-apis/minutes/v1/templates";
pub const MINUTES_V1_TEMPLATE_GET: &str = "/open-apis/minutes/v1/templates/{template_id}";
pub const MINUTES_V1_TEMPLATE_CREATE: &str = "/open-apis/minutes/v1/templates";
pub const MINUTES_V1_TEMPLATE_UPDATE: &str = "/open-apis/minutes/v1/templates/{template_id}";
pub const MINUTES_V1_TEMPLATE_DELETE: &str = "/open-apis/minutes/v1/templates/{template_id}";

// ==================== Task (任务协作) v1 ====================
// 任务系统 - 任务分配、进度跟踪、团队协作

/// Task任务管理 v1
/// 任务创建和管理
pub const TASK_V1_TASKS: &str = "/open-apis/task/v1/tasks";
pub const TASK_V1_TASK_GET: &str = "/open-apis/task/v1/tasks/{task_id}";
pub const TASK_V1_TASK_CREATE: &str = "/open-apis/task/v1/tasks";
pub const TASK_V1_TASK_UPDATE: &str = "/open-apis/task/v1/tasks/{task_id}";
pub const TASK_V1_TASK_DELETE: &str = "/open-apis/task/v1/tasks/{task_id}";
pub const TASK_V1_TASK_SEARCH: &str = "/open-apis/task/v1/tasks/search";
pub const TASK_V1_TASK_BATCH_CREATE: &str = "/open-apis/task/v1/tasks/batch_create";
pub const TASK_V1_TASK_BATCH_UPDATE: &str = "/open-apis/task/v1/tasks/batch_update";
pub const TASK_V1_TASK_BATCH_DELETE: &str = "/open-apis/task/v1/tasks/batch_delete";

/// Task任务列表管理 v1
/// 任务列表和项目管理
pub const TASK_V1_TASK_LISTS: &str = "/open-apis/task/v1/task_lists";
pub const TASK_V1_TASK_LIST_GET: &str = "/open-apis/task/v1/task_lists/{task_list_id}";
pub const TASK_V1_TASK_LIST_CREATE: &str = "/open-apis/task/v1/task_lists";
pub const TASK_V1_TASK_LIST_UPDATE: &str = "/open-apis/task/v1/task_lists/{task_list_id}";
pub const TASK_V1_TASK_LIST_DELETE: &str = "/open-apis/task/v1/task_lists/{task_list_id}";

/// Task任务分配管理 v1
/// 任务分配和协作管理
pub const TASK_V1_TASK_ASSIGNEES: &str = "/open-apis/task/v1/task_assignees";
pub const TASK_V1_TASK_ASSIGNEE_GET: &str = "/open-apis/task/v1/task_assignees/{assignee_id}";
pub const TASK_V1_TASK_ASSIGNEE_CREATE: &str = "/open-apis/task/v1/task_assignees";
pub const TASK_V1_TASK_ASSIGNEE_UPDATE: &str = "/open-apis/task/v1/task_assignees/{assignee_id}";
pub const TASK_V1_TASK_ASSIGNEE_DELETE: &str = "/open-apis/task/v1/task_assignees/{assignee_id}";

// ==================== 兼容性别名 ====================

/// 为保持向后兼容性，提供一些简短的别名
/// Calendar别名
pub const CALENDAR_LIST: &str = CALENDAR_V4_CALENDARS;
pub const CALENDAR_CREATE: &str = CALENDAR_V4_CALENDAR_CREATE;
pub const EVENT_CREATE: &str = CALENDAR_V4_EVENT_CREATE;

/// Task别名
pub const TASK_LIST: &str = TASK_V1_TASKS;
pub const TASK_CREATE: &str = TASK_V1_TASK_CREATE;

/// Minutes别名
pub const MINUTES_LIST: &str = MINUTES_V1_MINUTES;
pub const MINUTES_CREATE: &str = MINUTES_V1_MINUTE_CREATE;

/// Board别名
pub const BOARD_LIST: &str = BOARD_V1_BOARDS;
pub const BOARD_CREATE: &str = BOARD_V1_BOARD_CREATE;

/// Meeting Room别名
pub const ROOM_LIST: &str = MEETING_ROOM_V1_ROOMS;
pub const ROOM_BOOK: &str = MEETING_ROOM_V1_BOOKING_CREATE;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_endpoints() {
        // 验证Board端点
        assert!(BOARD_V1_BOARDS.starts_with("/open-apis/board/v1/"));
        assert!(BOARD_V1_BOARDS.contains("boards"));
        assert!(BOARD_V1_CARDS.contains("cards"));
        assert!(BOARD_V1_LISTS.contains("lists"));
    }

    #[test]
    fn test_calendar_endpoints() {
        // 验证Calendar端点
        assert!(CALENDAR_V4_CALENDARS.starts_with("/open-apis/calendar/v4/"));
        assert!(CALENDAR_V4_CALENDARS.contains("calendars"));
        assert!(CALENDAR_V4_EVENTS.contains("events"));
        assert!(CALENDAR_V4_FREEBUSY.contains("freebusy"));
    }

    #[test]
    fn test_meeting_room_endpoints() {
        // 验证Meeting Room端点
        assert!(MEETING_ROOM_V1_ROOMS.starts_with("/open-apis/meeting_room/v1/"));
        assert!(MEETING_ROOM_V1_ROOMS.contains("rooms"));
        assert!(MEETING_ROOM_V1_BOOKINGS.contains("bookings"));
        assert!(MEETING_ROOM_V1_ROOM_AVAILABLE.contains("available"));
    }

    #[test]
    fn test_minutes_endpoints() {
        // 验证Minutes端点
        assert!(MINUTES_V1_MINUTES.starts_with("/open-apis/minutes/v1/"));
        assert!(MINUTES_V1_MINUTES.contains("minutes"));
        assert!(MINUTES_V1_ACTION_ITEMS.contains("action_items"));
        assert!(MINUTES_V1_TEMPLATES.contains("templates"));
    }

    #[test]
    fn test_task_endpoints() {
        // 验证Task端点
        assert!(TASK_V1_TASKS.starts_with("/open-apis/task/v1/"));
        assert!(TASK_V1_TASKS.contains("tasks"));
        assert!(TASK_V1_TASK_LISTS.contains("task_lists"));
        assert!(TASK_V1_TASK_ASSIGNEES.contains("task_assignees"));
    }

    #[test]
    fn test_backward_compatibility() {
        // 验证兼容性别名
        assert_eq!(CALENDAR_LIST, CALENDAR_V4_CALENDARS);
        assert_eq!(TASK_LIST, TASK_V1_TASKS);
        assert_eq!(BOARD_LIST, BOARD_V1_BOARDS);
        assert_eq!(ROOM_LIST, MEETING_ROOM_V1_ROOMS);
        assert_eq!(MINUTES_LIST, MINUTES_V1_MINUTES);
    }

    #[test]
    fn test_endpoint_parameter_placeholders() {
        // 测试关键端点是否包含正确的参数占位符
        assert!(BOARD_V1_BOARD_GET.contains("{board_id}"));
        assert!(BOARD_V1_BOARD_UPDATE.contains("{board_id}"));
        assert!(BOARD_V1_BOARD_DELETE.contains("{board_id}"));
        assert!(BOARD_V1_CARD_GET.contains("{card_id}"));
        assert!(BOARD_V1_CARD_UPDATE.contains("{card_id}"));
        assert!(BOARD_V1_CARD_DELETE.contains("{card_id}"));
        assert!(BOARD_V1_CARD_MOVE.contains("{card_id}"));
        assert!(BOARD_V1_LIST_GET.contains("{list_id}"));
        assert!(BOARD_V1_LIST_UPDATE.contains("{list_id}"));
        assert!(BOARD_V1_LIST_DELETE.contains("{list_id}"));
        assert!(BOARD_V1_LIST_MOVE.contains("{list_id}"));

        assert!(CALENDAR_V4_CALENDAR_GET.contains("{calendar_id}"));
        assert!(CALENDAR_V4_CALENDAR_UPDATE.contains("{calendar_id}"));
        assert!(CALENDAR_V4_CALENDAR_DELETE.contains("{calendar_id}"));
        assert!(CALENDAR_V4_CALENDAR_SUBSCRIBE.contains("{calendar_id}"));
        assert!(CALENDAR_V4_CALENDAR_UNSUBSCRIBE.contains("{calendar_id}"));
        assert!(CALENDAR_V4_EVENT_GET.contains("{event_id}"));
        assert!(CALENDAR_V4_EVENT_UPDATE.contains("{event_id}"));
        assert!(CALENDAR_V4_EVENT_DELETE.contains("{event_id}"));
        assert!(CALENDAR_V4_EVENT_RSVP.contains("{event_id}"));

        assert!(MEETING_ROOM_V1_ROOM_GET.contains("{room_id}"));
        assert!(MEETING_ROOM_V1_ROOM_UPDATE.contains("{room_id}"));
        assert!(MEETING_ROOM_V1_ROOM_DELETE.contains("{room_id}"));
        assert!(MEETING_ROOM_V1_BOOKING_GET.contains("{booking_id}"));
        assert!(MEETING_ROOM_V1_BOOKING_UPDATE.contains("{booking_id}"));
        assert!(MEETING_ROOM_V1_BOOKING_DELETE.contains("{booking_id}"));

        assert!(MINUTES_V1_MINUTE_GET.contains("{minute_id}"));
        assert!(MINUTES_V1_MINUTE_UPDATE.contains("{minute_id}"));
        assert!(MINUTES_V1_MINUTE_DELETE.contains("{minute_id}"));
        assert!(MINUTES_V1_ACTION_ITEM_GET.contains("{action_item_id}"));
        assert!(MINUTES_V1_ACTION_ITEM_UPDATE.contains("{action_item_id}"));
        assert!(MINUTES_V1_ACTION_ITEM_DELETE.contains("{action_item_id}"));
        assert!(MINUTES_V1_TEMPLATE_GET.contains("{template_id}"));
        assert!(MINUTES_V1_TEMPLATE_UPDATE.contains("{template_id}"));
        assert!(MINUTES_V1_TEMPLATE_DELETE.contains("{template_id}"));

        assert!(TASK_V1_TASK_GET.contains("{task_id}"));
        assert!(TASK_V1_TASK_UPDATE.contains("{task_id}"));
        assert!(TASK_V1_TASK_DELETE.contains("{task_id}"));
        assert!(TASK_V1_TASK_LIST_GET.contains("{task_list_id}"));
        assert!(TASK_V1_TASK_LIST_UPDATE.contains("{task_list_id}"));
        assert!(TASK_V1_TASK_LIST_DELETE.contains("{task_list_id}"));
        assert!(TASK_V1_TASK_ASSIGNEE_GET.contains("{assignee_id}"));
        assert!(TASK_V1_TASK_ASSIGNEE_UPDATE.contains("{assignee_id}"));
        assert!(TASK_V1_TASK_ASSIGNEE_DELETE.contains("{assignee_id}"));
    }

    #[test]
    fn test_service_grouping() {
        // 测试服务分组的正确性
        let board_endpoints = [BOARD_V1_BOARDS, BOARD_V1_CARDS, BOARD_V1_LISTS];
        for endpoint in board_endpoints {
            assert!(
                endpoint.contains("/board/"),
                "{} 应该包含 /board/",
                endpoint
            );
        }

        let calendar_endpoints = [
            CALENDAR_V4_CALENDARS,
            CALENDAR_V4_EVENTS,
            CALENDAR_V4_FREEBUSY,
        ];
        for endpoint in calendar_endpoints {
            assert!(
                endpoint.contains("/calendar/"),
                "{} 应该包含 /calendar/",
                endpoint
            );
        }

        let meeting_room_endpoints = [MEETING_ROOM_V1_ROOMS, MEETING_ROOM_V1_BOOKINGS];
        for endpoint in meeting_room_endpoints {
            assert!(
                endpoint.contains("/meeting_room/"),
                "{} 应该包含 /meeting_room/",
                endpoint
            );
        }

        let minutes_endpoints = [
            MINUTES_V1_MINUTES,
            MINUTES_V1_ACTION_ITEMS,
            MINUTES_V1_TEMPLATES,
        ];
        for endpoint in minutes_endpoints {
            assert!(
                endpoint.contains("/minutes/"),
                "{} 应该包含 /minutes/",
                endpoint
            );
        }

        let task_endpoints = [TASK_V1_TASKS, TASK_V1_TASK_LISTS, TASK_V1_TASK_ASSIGNEES];
        for endpoint in task_endpoints {
            assert!(endpoint.contains("/task/"), "{} 应该包含 /task/", endpoint);
        }
    }

    #[test]
    fn test_version_consistency() {
        // 测试版本一致性
        let v4_endpoints = [CALENDAR_V4_CALENDARS, CALENDAR_V4_EVENTS];
        for endpoint in v4_endpoints {
            assert!(endpoint.contains("/v4/"), "{} 应该包含 /v4/", endpoint);
        }

        let v1_endpoints = [
            BOARD_V1_BOARDS,
            MEETING_ROOM_V1_ROOMS,
            MINUTES_V1_MINUTES,
            TASK_V1_TASKS,
        ];
        for endpoint in v1_endpoints {
            assert!(endpoint.contains("/v1/"), "{} 应该包含 /v1/", endpoint);
        }
    }
} // Endpoints and EndpointBuilder are now available directly from openlark_core::endpoints

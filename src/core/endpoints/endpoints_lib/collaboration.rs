//! 协作工具服务端点
//!
//! 包含日历、审批、任务等协作相关的API端点。

/// 协作工具相关端点
pub struct Collaboration;

impl Collaboration {
    // ==================== 日历管理 ====================

    /// 创建日历
    pub const CALENDAR_CREATE: &'static str = "/open-apis/calendar/v4/calendars";

    /// 获取日历信息
    pub const CALENDAR_GET: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";

    /// 创建日历事件
    pub const CALENDAR_EVENT_CREATE: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}/events";

    // ==================== 审批管理 ====================

    /// 审批流程
    pub const APPROVAL_FLOWS: &'static str = "/open-apis/approval/v4/flows";

    /// 审批实例
    pub const APPROVAL_INSTANCES: &'static str = "/open-apis/approval/v4/instances";

    // ==================== 任务管理 ====================

    /// 任务列表
    pub const TASK_LISTS: &'static str = "/open-apis/task/v2/tasklists";

    /// 任务
    pub const TASKS: &'static str = "/open-apis/task/v2/tasks";
}
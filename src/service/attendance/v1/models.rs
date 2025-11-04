//! Attendance服务数据模型
//!
//! 定义考勤相关的数据结构，包括班次、打卡任务、统计等核心实体。

use serde::{Deserialize, Serialize};

/// 班次信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shift {
    /// 班次ID
    pub shift_id: String,
    /// 班次名称
    pub shift_name: String,
    /// 打卡次数
    pub punch_times: i32,
    /// 是否弹性打卡
    pub is_flexible: Option<bool>,
    /// 弹性打卡时间，单位：分钟
    pub flexible_minutes: Option<i32>,
    /// 弹性打卡规则
    pub flexible_rule: Option<Vec<FlexibleRule>>,
    /// 不需要打下班卡
    pub no_need_off: Option<bool>,
    /// 打卡时间
    pub punch_time_rule: Option<Vec<PunchTimeRule>>,
    /// 晚走晚到时间，单位：分钟
    pub late_minutes_as_late: Option<i32>,
    /// 晚走晚到规则
    pub late_minutes_as_lack: Option<i32>,
    /// 早走早到时间，单位：分钟
    pub early_minutes_as_early: Option<i32>,
    /// 早走早到规则
    pub early_minutes_as_lack: Option<i32>,
    /// 是否允许在家办公
    pub allow_outside_apply: Option<bool>,
    /// 在家办公限制
    pub outside_apply_limit: Option<i32>,
    /// 是否开启人脸识别打卡
    pub allow_face_punch: Option<bool>,
    /// 人脸识别打卡限制
    pub face_punch_cfg: Option<FacePunchConfig>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
}

/// 弹性打卡规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlexibleRule {
    /// 弹性打卡时间，单位：分钟
    pub flexible_early_minutes: i32,
    /// 弹性打卡时间，单位：分钟
    pub flexible_late_minutes: i32,
}

/// 打卡时间规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunchTimeRule {
    /// 上班时间，格式：HH:mm
    pub on_time: String,
    /// 下班时间，格式：HH:mm
    pub off_time: String,
    /// 上班提前打卡时间，单位：分钟
    pub on_advance_minutes: i32,
    /// 下班延迟打卡时间，单位：分钟
    pub off_delay_minutes: i32,
    /// 晚走晚到时间，单位：分钟
    pub late_minutes_as_late: Option<i32>,
    /// 晚走晚到规则
    pub late_minutes_as_lack: Option<i32>,
    /// 早走早到时间，单位：分钟
    pub early_minutes_as_early: Option<i32>,
    /// 早走早到规则
    pub early_minutes_as_lack: Option<i32>,
}

/// 人脸识别打卡配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacePunchConfig {
    // TODO: Add fields
}

/// 用户打卡任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTask {
    /// 任务ID
    pub task_id: String,
    /// 用户ID
    pub user_id: String,
    /// 打卡日期
    pub check_date: String,
    /// 班次名称
    pub shift_name: String,
    /// 上班打卡时间
    pub check_in_time: Option<String>,
    /// 下班打卡时间
    pub check_out_time: Option<String>,
    /// 工作时长（小时）
    pub work_hours: Option<String>,
    /// 考勤状态
    pub status: TaskStatus,
    /// 打卡位置
    pub location: Option<String>,
    /// 创建时间
    pub create_time: String,
}

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// 正常
    Normal,
    /// 迟到
    Late,
    /// 早退
    Early,
    /// 缺卡
    Missing,
    /// 外勤
    Outside,
}

/// 用户统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    /// 用户ID
    pub user_id: String,
    /// 统计周期
    pub stats_period: String,
    /// 总工作日
    pub total_work_days: i32,
    /// 实际工作日
    pub actual_work_days: i32,
    /// 请假天数
    pub leave_days: i32,
    /// 迟到天数
    pub late_days: i32,
    /// 早退天数
    pub early_leave_days: i32,
    /// 缺勤天数
    pub absent_days: i32,
    /// 加班时长
    pub overtime_hours: Option<String>,
    /// 平均工作时长
    pub average_work_hours: Option<String>,
    /// 总工作时长
    pub total_work_hours: Option<String>,
}

/// 班次组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftGroup {
    /// 班次组ID
    pub group_id: String,
    /// 班次组名称
    pub group_name: String,
    /// 包含的班次ID列表
    pub shift_ids: Vec<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 请假记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRecord {
    /// 请假记录ID
    pub record_id: String,
    /// 用户ID
    pub user_id: String,
    /// 请假类型
    pub leave_type: String,
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 请假天数
    pub leave_days: f64,
    /// 请假原因
    pub reason: Option<String>,
    /// 审批状态
    pub approval_status: ApprovalStatus,
    /// 创建时间
    pub create_time: String,
}

/// 审批状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    /// 待审批
    Pending,
    /// 已通过
    Approved,
    /// 已拒绝
    Rejected,
    /// 已撤销
    Cancelled,
}

/// 考勤设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceSetting {
    /// 用户ID
    pub user_id: String,
    /// 是否启用考勤
    pub attendance_enabled: bool,
    /// 默认班次ID
    pub default_shift_id: Option<String>,
    /// 打卡范围设置
    pub punch_range_settings: Option<PunchRangeSettings>,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
}

/// 打卡范围设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunchRangeSettings {
    /// 允许打卡范围（米）
    pub allowed_range: Option<i32>,
    /// 启用WiFi打卡
    pub wifi_enabled: Option<bool>,
    /// 允许的WiFi列表
    pub allowed_wifi_list: Option<Vec<String>>,
    /// 启用位置打卡
    pub location_enabled: Option<bool>,
    /// 允许的地理位置
    pub allowed_locations: Option<Vec<Location>>,
}

/// 地理位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// 纬度
    pub latitude: f64,
    /// 经度
    pub longitude: f64,
    /// 地址
    pub address: Option<String>,
    /// 半径（米）
    pub radius: Option<i32>,
}

// 为各个结构体实现Default trait
impl Default for Shift {
    fn default() -> Self {
        Self {
            shift_id: String::new(),
            shift_name: String::new(),
            punch_times: 0,
            is_flexible: None,
            flexible_minutes: None,
            flexible_rule: None,
            no_need_off: None,
            punch_time_rule: None,
            late_minutes_as_late: None,
            late_minutes_as_lack: None,
            early_minutes_as_early: None,
            early_minutes_as_lack: None,
            allow_outside_apply: None,
            outside_apply_limit: None,
            allow_face_punch: None,
            face_punch_cfg: None,
            create_time: None,
            update_time: None,
        }
    }
}

impl Default for PunchTimeRule {
    fn default() -> Self {
        Self {
            on_time: String::new(),
            off_time: String::new(),
            on_advance_minutes: 0,
            off_delay_minutes: 0,
            late_minutes_as_late: None,
            late_minutes_as_lack: None,
            early_minutes_as_early: None,
            early_minutes_as_lack: None,
        }
    }
}

impl Default for FlexibleRule {
    fn default() -> Self {
        Self {
            flexible_early_minutes: 0,
            flexible_late_minutes: 0,
        }
    }
}

impl Default for FacePunchConfig {
    fn default() -> Self {
        Self {}
    }
}

impl Default for UserTask {
    fn default() -> Self {
        Self {
            task_id: String::new(),
            user_id: String::new(),
            check_date: String::new(),
            shift_name: String::new(),
            check_in_time: None,
            check_out_time: None,
            work_hours: None,
            status: TaskStatus::Normal,
            location: None,
            create_time: String::new(),
        }
    }
}

impl Default for UserStats {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            stats_period: String::new(),
            total_work_days: 0,
            actual_work_days: 0,
            leave_days: 0,
            late_days: 0,
            early_leave_days: 0,
            absent_days: 0,
            overtime_hours: None,
            average_work_hours: None,
            total_work_hours: None,
        }
    }
}

impl Default for ShiftGroup {
    fn default() -> Self {
        Self {
            group_id: String::new(),
            group_name: String::new(),
            shift_ids: Vec::new(),
            create_time: None,
            update_time: None,
        }
    }
}

impl Default for LeaveRecord {
    fn default() -> Self {
        Self {
            record_id: String::new(),
            user_id: String::new(),
            leave_type: String::new(),
            start_date: String::new(),
            end_date: String::new(),
            leave_days: 0.0,
            reason: None,
            approval_status: ApprovalStatus::Pending,
            create_time: String::new(),
        }
    }
}

impl Default for AttendanceSetting {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            attendance_enabled: false,
            default_shift_id: None,
            punch_range_settings: None,
            create_time: String::new(),
            update_time: String::new(),
        }
    }
}

impl Default for PunchRangeSettings {
    fn default() -> Self {
        Self {
            allowed_range: None,
            wifi_enabled: None,
            allowed_wifi_list: None,
            location_enabled: None,
            allowed_locations: None,
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            address: None,
            radius: None,
        }
    }
}

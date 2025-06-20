use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, ResponseFormat},
};
use serde::{Deserialize, Serialize};

/// 班次信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shift {
    /// 班次 ID
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
    /// 是否允许人脸识别打卡
    pub face_punch: bool,
    /// 人脸识别打卡限制距离，单位：米
    pub face_live_need_action: Option<bool>,
    /// 是否需要活体检测
    pub face_downgrade: Option<bool>,
}

/// 创建班次请求
#[derive(Default)]
pub struct CreateShiftRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型，作为查询参数。如果没有后台管理权限，可使用通过手机号或邮箱获取用户 ID
    ///
    /// 示例值："employee_id"
    ///
    /// 可选值有：
    ///
    /// employee_id：员工 employee ID，即飞书管理后台 > 组织架构 > 成员与部门 > 成员详情中的用户 ID，或者通过手机号或邮箱获取用户 ID获取的userid。
    /// employee_no：员工工号，即飞书管理后台 > 组织架构 > 成员与部门 > 成员详情中的工号
    pub employee_type: String,
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
}

/// 创建班次响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShiftRespData {
    /// 班次信息
    pub shift: Shift,
}

impl ApiResponseTrait for CreateShiftRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for Shift {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除班次请求
#[derive(Default)]
pub struct DeleteShiftRequest {
    pub api_req: ApiRequest,
    /// 班次 ID
    pub shift_id: String,
}

/// 获取班次请求
#[derive(Default)]
pub struct GetShiftRequest {
    pub api_req: ApiRequest,
    /// 班次 ID
    pub shift_id: String,
}

/// 按名称查询班次请求
#[derive(Default)]
pub struct QueryShiftRequest {
    pub api_req: ApiRequest,
    /// 员工类型，作为查询参数，同创建班次接口
    pub employee_type: String,
    /// 班次名称
    pub shift_name: String,
}

/// 查询班次响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryShiftRespData {
    /// 班次列表
    pub shift_list: Vec<Shift>,
}

impl ApiResponseTrait for QueryShiftRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询所有班次请求
#[derive(Default)]
pub struct ListShiftRequest {
    pub api_req: ApiRequest,
    /// 分页大小，最大值：20
    pub page_size: Option<i32>,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    pub page_token: Option<String>,
}

/// 班次列表响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftListData {
    /// 班次列表
    pub shift_list: Vec<Shift>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ShiftListData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 空响应（用于删除等操作）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {}

impl ApiResponseTrait for EmptyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户排班信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDailyShift {
    /// 用户 ID
    pub user_id: String,
    /// 日期，格式：YYYY-MM-DD
    pub shift_date: String,
    /// 班次 ID
    pub shift_id: String,
}

/// 创建或修改排班表请求
#[derive(Default)]
pub struct BatchCreateUserDailyShiftRequest {
    pub api_req: ApiRequest,
    /// 员工类型，用于指定 user_daily_shifts 中的 user_id 类型
    /// 可选值：
    /// - employee_id：员工 employee ID
    /// - employee_no：员工工号
    pub employee_type: String,
    /// 用户排班信息列表
    pub user_daily_shifts: Vec<UserDailyShift>,
}

/// 批量操作响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateUserDailyShiftRespData {
    /// 失败的用户排班信息
    pub failed_user_daily_shifts: Option<Vec<UserDailyShift>>,
    /// 成功的数量
    pub success_count: Option<i32>,
    /// 失败的数量
    pub failed_count: Option<i32>,
}

impl ApiResponseTrait for BatchCreateUserDailyShiftRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询排班表请求
#[derive(Default)]
pub struct QueryUserDailyShiftRequest {
    pub api_req: ApiRequest,
    /// 员工类型，用于指定 user_ids 中的用户 ID 类型
    pub employee_type: String,
    /// 用户 ID 列表，一次最多 50 个
    pub user_ids: Vec<String>,
    /// 查询的起始时间，精确到日期
    pub check_date_from: String,
    /// 查询的结束时间，精确到日期
    pub check_date_to: String,
}

/// 查询排班表响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserDailyShiftRespData {
    /// 用户排班信息列表
    pub user_daily_shift_list: Vec<UserDailyShiftData>,
}

impl ApiResponseTrait for QueryUserDailyShiftRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户排班详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDailyShiftData {
    /// 用户 ID
    pub user_id: String,
    /// 日期，格式：YYYY-MM-DD
    pub shift_date: String,
    /// 班次 ID
    pub shift_id: String,
    /// 班次名称
    pub shift_name: String,
    /// 是否为临时班次
    pub is_temp: Option<bool>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
}

/// 创建或修改临时排班请求
#[derive(Default)]
pub struct BatchCreateTempUserDailyShiftRequest {
    pub api_req: ApiRequest,
    /// 员工类型，用于指定 user_daily_shifts 中的 user_id 类型
    pub employee_type: String,
    /// 用户排班信息列表
    pub user_daily_shifts: Vec<UserDailyShift>,
}

/// 创建或修改临时排班响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateTempUserDailyShiftRespData {
    /// 失败的用户排班信息
    pub failed_user_daily_shifts: Option<Vec<UserDailyShift>>,
    /// 成功的数量
    pub success_count: Option<i32>,
    /// 失败的数量
    pub failed_count: Option<i32>,
}

impl ApiResponseTrait for BatchCreateTempUserDailyShiftRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 考勤组信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// 考勤组 ID
    pub group_id: String,
    /// 考勤组名称
    pub group_name: String,
    /// 时区
    pub time_zone: Option<String>,
    /// 绑定的部门列表
    pub bind_dept_ids: Option<Vec<String>>,
    /// 例外日期设置
    pub except_date_rule: Option<Vec<ExceptDateRule>>,
    /// 考勤方式
    pub attendance_type: Option<i32>,
    /// 打卡方式
    pub punch_type: Option<i32>,
    /// 允许迟到时间，单位：分钟
    pub allow_late_minutes: Option<i32>,
    /// 允许早退时间，单位：分钟
    pub allow_early_leave_minutes: Option<i32>,
    /// 工作日设置
    pub work_day_rule: Option<Vec<WorkDayRule>>,
    /// 班次设置
    pub shift_rule: Option<Vec<ShiftRule>>,
    /// 成员设置
    pub member_rule: Option<MemberRule>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
}

/// 例外日期规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptDateRule {
    /// 例外日期，格式：YYYY-MM-DD
    pub date: String,
    /// 例外类型：1-工作日 2-休息日
    pub except_type: i32,
    /// 班次 ID（当例外类型为工作日时必填）
    pub shift_id: Option<String>,
}

/// 工作日规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDayRule {
    /// 星期几：1-7，1代表周一
    pub week_day: i32,
    /// 班次 ID
    pub shift_id: String,
}

/// 班次规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftRule {
    /// 班次 ID
    pub shift_id: String,
    /// 班次名称
    pub shift_name: Option<String>,
}

/// 成员规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberRule {
    /// 成员类型：1-部门 2-用户
    pub member_type: i32,
    /// 成员 ID 列表
    pub member_ids: Vec<String>,
}

/// 考勤组成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupUser {
    /// 用户 ID
    pub user_id: String,
    /// 用户姓名
    pub user_name: Option<String>,
    /// 员工工号
    pub employee_no: Option<String>,
    /// 所属部门 ID
    pub department_id: Option<String>,
    /// 加入时间
    pub join_time: Option<String>,
}

/// 查询考勤组下所有成员请求
#[derive(Default)]
pub struct ListGroupUserRequest {
    pub api_req: ApiRequest,
    /// 考勤组 ID
    pub group_id: String,
    /// 员工 ID 类型
    pub employee_type: String,
    /// 部门 ID 类型
    pub dept_type: Option<String>,
    /// 分页大小，最大值：100
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 查询考勤组成员响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupUserRespData {
    /// 成员列表
    pub user_list: Vec<GroupUser>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListGroupUserRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建或修改考勤组请求
#[derive(Default)]
pub struct CreateGroupRequest {
    pub api_req: ApiRequest,
    /// 员工 ID 类型
    pub employee_type: String,
    /// 部门 ID 类型
    pub dept_type: Option<String>,
    /// 考勤组名称
    pub group_name: String,
    /// 时区
    pub time_zone: Option<String>,
    /// 绑定的部门列表
    pub bind_dept_ids: Option<Vec<String>>,
    /// 例外日期设置
    pub except_date_rule: Option<Vec<ExceptDateRule>>,
    /// 考勤方式
    pub attendance_type: Option<i32>,
    /// 打卡方式
    pub punch_type: Option<i32>,
    /// 允许迟到时间，单位：分钟
    pub allow_late_minutes: Option<i32>,
    /// 允许早退时间，单位：分钟
    pub allow_early_leave_minutes: Option<i32>,
    /// 工作日设置
    pub work_day_rule: Option<Vec<WorkDayRule>>,
    /// 班次设置
    pub shift_rule: Option<Vec<ShiftRule>>,
    /// 成员设置
    pub member_rule: Option<MemberRule>,
}

/// 创建考勤组响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRespData {
    /// 考勤组信息
    pub group: Group,
}

impl ApiResponseTrait for CreateGroupRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除考勤组请求
#[derive(Default)]
pub struct DeleteGroupRequest {
    pub api_req: ApiRequest,
    /// 考勤组 ID
    pub group_id: String,
}

/// 获取考勤组请求
#[derive(Default)]
pub struct GetGroupRequest {
    pub api_req: ApiRequest,
    /// 考勤组 ID
    pub group_id: String,
    /// 员工 ID 类型
    pub employee_type: String,
    /// 部门 ID 类型
    pub dept_type: Option<String>,
}

/// 按名称查询考勤组请求
#[derive(Default)]
pub struct SearchGroupRequest {
    pub api_req: ApiRequest,
    /// 员工 ID 类型
    pub employee_type: String,
    /// 部门 ID 类型
    pub dept_type: Option<String>,
    /// 考勤组名称
    pub group_name: String,
}

/// 查询考勤组响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchGroupRespData {
    /// 考勤组列表
    pub group_list: Vec<Group>,
}

impl ApiResponseTrait for SearchGroupRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询所有考勤组请求
#[derive(Default)]
pub struct ListGroupRequest {
    pub api_req: ApiRequest,
    /// 员工 ID 类型
    pub employee_type: String,
    /// 部门 ID 类型
    pub dept_type: Option<String>,
    /// 分页大小，最大值：100
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 考勤组列表响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupRespData {
    /// 考勤组列表
    pub group_list: Vec<Group>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListGroupRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for Group {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

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

/// 用户人脸识别设置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSetting {
    /// 用户 ID
    pub user_id: String,
    /// 是否开启人脸识别打卡
    pub face_key_open: Option<bool>,
    /// 人脸识别照片文件 key
    pub face_key: Option<String>,
    /// 活体检测等级
    pub face_live_need_action: Option<bool>,
    /// 人脸识别降级开关
    pub face_downgrade: Option<bool>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
}

/// 修改用户人脸识别信息请求
#[derive(Default)]
pub struct ModifyUserSettingRequest {
    pub api_req: ApiRequest,
    /// 员工 ID 类型
    pub employee_type: String,
    /// 用户 ID
    pub user_id: String,
    /// 是否开启人脸识别打卡
    pub face_key_open: Option<bool>,
    /// 人脸识别照片文件 key
    pub face_key: Option<String>,
    /// 活体检测等级
    pub face_live_need_action: Option<bool>,
    /// 人脸识别降级开关
    pub face_downgrade: Option<bool>,
}

/// 修改用户人脸识别信息响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyUserSettingRespData {
    /// 用户设置信息
    pub user_setting: UserSetting,
}

impl ApiResponseTrait for ModifyUserSettingRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询用户人脸识别信息请求
#[derive(Default)]
pub struct QueryUserSettingRequest {
    pub api_req: ApiRequest,
    /// 员工 ID 类型
    pub employee_type: String,
    /// 用户 ID 列表，一次最多 50 个
    pub user_ids: Vec<String>,
}

/// 批量查询用户人脸识别信息响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserSettingRespData {
    /// 用户设置信息列表
    pub user_setting_list: Vec<UserSetting>,
}

impl ApiResponseTrait for QueryUserSettingRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传用户人脸识别照片请求
#[derive(Default)]
pub struct UploadUserPhotoRequest {
    pub api_req: ApiRequest,
    /// 员工 ID 类型
    pub employee_type: String,
    /// 用户 ID
    pub user_id: String,
    /// 人脸识别照片的二进制数据
    pub photo_data: Vec<u8>,
    /// 照片文件名
    pub photo_name: String,
}

/// 上传用户人脸识别照片响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadUserPhotoRespData {
    /// 人脸识别照片文件 key
    pub face_key: String,
}

impl ApiResponseTrait for UploadUserPhotoRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载用户人脸识别照片请求
#[derive(Default)]
pub struct DownloadUserPhotoRequest {
    pub api_req: ApiRequest,
    /// 员工 ID 类型
    pub employee_type: String,
    /// 用户 ID
    pub user_id: String,
    /// 人脸识别照片Key
    pub face_key: String,
}

/// 下载用户人脸识别照片响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUserPhotoRespData {
    /// 照片的二进制数据
    pub photo_data: Vec<u8>,
}

impl ApiResponseTrait for DownloadUserPhotoRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UserSetting {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 考勤统计相关数据结构 ====================

/// 更新统计设置请求
#[derive(Default)]
pub struct UpdateUserStatsDataRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 统计设置
    pub stats_setting: StatsSettings,
}

/// 统计设置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatsSettings {
    /// 统计范围 1：自定义范围, 2：全部
    pub stats_scope: i32,
    /// 统计起始日期，格式YYYY-MM-DD
    pub start_date: String,
    /// 统计结束日期，格式YYYY-MM-DD
    pub end_date: String,
    /// 需要查询的用户ID列表
    pub user_ids: Vec<String>,
    /// 需要查询的字段列表
    pub need_fields: Vec<String>,
}

/// 更新统计设置响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserStatsDataRespData {
    /// 更新是否成功
    pub success: bool,
}

/// 查询统计设置请求
#[derive(Default)]
pub struct QueryStatsSettingsRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型  
    pub employee_type: String,
}

/// 查询统计设置响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryStatsSettingsRespData {
    /// 统计设置
    pub stats_setting: StatsSettings,
}

/// 查询统计表头请求
#[derive(Default)]
pub struct QueryStatsFieldsRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 语言类型，zh-CN：中文，en-US：英文，ja-JP：日文
    pub locale: Option<String>,
}

/// 统计字段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsField {
    /// 字段标识
    pub field_key: String,
    /// 字段名称
    pub field_name: String,
    /// 字段中文名称
    pub field_name_zh: Option<String>,
    /// 字段英文名称  
    pub field_name_en: Option<String>,
    /// 字段日文名称
    pub field_name_ja: Option<String>,
    /// 字段类型：0-文本，1-数字，2-时间
    pub field_type: i32,
}

/// 查询统计表头响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryStatsFieldsRespData {
    /// 统计字段列表
    pub fields: Vec<StatsField>,
}

/// 查询统计数据请求
#[derive(Default)]
pub struct QueryUserStatsDataRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 统计起始日期，格式YYYY-MM-DD
    pub start_date: String,
    /// 统计结束日期，格式YYYY-MM-DD
    pub end_date: String,
    /// 需要查询的用户ID列表
    pub user_ids: Vec<String>,
    /// 需要查询的字段列表
    pub need_fields: Vec<String>,
    /// 语言类型，zh-CN：中文，en-US：英文，ja-JP：日文
    pub locale: Option<String>,
}

/// 用户统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatsData {
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub user_name: Option<String>,
    /// 统计数据字段
    pub datas: std::collections::HashMap<String, serde_json::Value>,
}

/// 查询统计数据响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserStatsDataRespData {
    /// 用户统计数据列表
    pub datas: Vec<UserStatsData>,
}

// 实现 ApiResponseTrait

impl ApiResponseTrait for UpdateUserStatsDataRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for QueryStatsSettingsRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for QueryStatsFieldsRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for QueryUserStatsDataRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 假勤审批相关数据结构 ====================

/// 获取审批数据请求
#[derive(Default)]
pub struct QueryUserApprovalRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 审批状态，1：待审批，2：已通过，3：已拒绝
    pub status: Option<i32>,
    /// 审批开始时间，格式YYYY-MM-DD
    pub date_from: Option<String>,
    /// 审批结束时间，格式YYYY-MM-DD  
    pub date_to: Option<String>,
    /// 审批员工ID列表
    pub user_ids: Option<Vec<String>>,
    /// 分页大小，最大100
    pub page_size: Option<i32>,
    /// 分页偏移量
    pub page_token: Option<String>,
}

/// 审批数据项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserApproval {
    /// 审批ID
    pub approval_id: String,
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub user_name: Option<String>,
    /// 审批类型，1：请假，2：出差，3：外出，4：加班，5：调休
    pub approval_type: i32,
    /// 审批状态，1：待审批，2：已通过，3：已拒绝
    pub status: i32,
    /// 申请开始时间
    pub start_time: String,
    /// 申请结束时间
    pub end_time: String,
    /// 申请时长（小时）
    pub duration: Option<f64>,
    /// 申请理由
    pub reason: Option<String>,
    /// 审批备注
    pub approval_note: Option<String>,
    /// 提交时间
    pub created_at: Option<String>,
    /// 审批时间
    pub approved_at: Option<String>,
}

/// 获取审批数据响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserApprovalRespData {
    /// 审批数据列表
    pub approvals: Vec<UserApproval>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页令牌
    pub page_token: Option<String>,
}

/// 写入审批结果请求
#[derive(Default)]
pub struct CreateUserApprovalRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 审批ID
    pub approval_id: String,
    /// 审批状态，2：已通过，3：已拒绝
    pub status: i32,
    /// 审批备注
    pub approval_note: Option<String>,
}

/// 写入审批结果响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserApprovalRespData {
    /// 处理是否成功
    pub success: bool,
    /// 审批ID
    pub approval_id: String,
}

/// 通知审批状态更新请求
#[derive(Default)]
pub struct ProcessUserApprovalRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 审批ID
    pub approval_id: String,
    /// 通知类型，1：审批通过，2：审批拒绝，3：撤回申请
    pub action: i32,
    /// 通知消息
    pub message: Option<String>,
}

/// 通知审批状态更新响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessUserApprovalRespData {
    /// 通知是否成功
    pub success: bool,
    /// 审批ID
    pub approval_id: String,
}

// 实现 ApiResponseTrait

impl ApiResponseTrait for QueryUserApprovalRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateUserApprovalRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ProcessUserApprovalRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 考勤补卡相关数据结构 ====================

/// 通知补卡审批发起请求
#[derive(Default)]
pub struct CreateUserTaskRemedyRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 补卡申请信息
    pub remedy_application: UserTaskRemedyApplication,
}

/// 补卡申请信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserTaskRemedyApplication {
    /// 用户ID
    pub user_id: String,
    /// 补卡日期，格式：yyyy-MM-dd
    pub remedy_date: String,
    /// 补卡时间，格式：HH:mm
    pub remedy_time: String,
    /// 补卡类型，1：上班补卡，2：下班补卡
    pub remedy_type: i32,
    /// 补卡原因
    pub reason: String,
    /// 补卡备注
    pub comment: Option<String>,
}

/// 通知补卡审批发起响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserTaskRemedyRespData {
    /// 补卡申请ID
    pub remedy_id: String,
    /// 申请是否成功提交
    pub success: bool,
}

/// 获取可补卡时间请求
#[derive(Default)]
pub struct QueryUserAllowedRemedysRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 用户ID
    pub user_id: String,
    /// 查询开始日期，格式：yyyy-MM-dd
    pub date_from: Option<String>,
    /// 查询结束日期，格式：yyyy-MM-dd
    pub date_to: Option<String>,
}

/// 可补卡时间数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAllowedRemedy {
    /// 日期，格式：yyyy-MM-dd
    pub date: String,
    /// 班次ID
    pub shift_id: String,
    /// 班次名称
    pub shift_name: String,
    /// 可补卡时间段列表
    pub remedy_periods: Vec<RemedyPeriod>,
}

/// 补卡时间段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemedyPeriod {
    /// 补卡类型，1：上班补卡，2：下班补卡
    pub remedy_type: i32,
    /// 补卡类型名称
    pub remedy_type_name: String,
    /// 标准打卡时间，格式：HH:mm
    pub standard_time: String,
    /// 可补卡开始时间，格式：HH:mm
    pub remedy_start_time: String,
    /// 可补卡结束时间，格式：HH:mm
    pub remedy_end_time: String,
    /// 是否可以补卡
    pub can_remedy: bool,
    /// 不可补卡原因
    pub block_reason: Option<String>,
}

/// 获取可补卡时间响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserAllowedRemedysRespData {
    /// 可补卡时间列表
    pub allowed_remedys: Vec<UserAllowedRemedy>,
}

/// 获取补卡记录请求
#[derive(Default)]
pub struct QueryUserTaskRemedyRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 用户ID列表
    pub user_ids: Option<Vec<String>>,
    /// 查询开始日期，格式：yyyy-MM-dd
    pub date_from: Option<String>,
    /// 查询结束日期，格式：yyyy-MM-dd
    pub date_to: Option<String>,
    /// 补卡状态，1：待审批，2：已通过，3：已拒绝
    pub status: Option<i32>,
    /// 分页大小，最大100
    pub page_size: Option<i32>,
    /// 分页偏移量
    pub page_token: Option<String>,
}

/// 补卡记录数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTaskRemedy {
    /// 补卡记录ID
    pub remedy_id: String,
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub user_name: Option<String>,
    /// 补卡日期，格式：yyyy-MM-dd
    pub remedy_date: String,
    /// 补卡时间，格式：HH:mm
    pub remedy_time: String,
    /// 补卡类型，1：上班补卡，2：下班补卡
    pub remedy_type: i32,
    /// 补卡状态，1：待审批，2：已通过，3：已拒绝
    pub status: i32,
    /// 申请原因
    pub reason: String,
    /// 补卡备注
    pub comment: Option<String>,
    /// 申请时间戳（毫秒）
    pub apply_time: String,
    /// 审批时间戳（毫秒）
    pub approve_time: Option<String>,
    /// 审批人ID
    pub approver_id: Option<String>,
    /// 审批备注
    pub approve_comment: Option<String>,
}

/// 获取补卡记录响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserTaskRemedyRespData {
    /// 补卡记录列表
    pub remedys: Vec<UserTaskRemedy>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页令牌
    pub page_token: Option<String>,
}

// 实现 ApiResponseTrait

impl ApiResponseTrait for CreateUserTaskRemedyRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for QueryUserAllowedRemedysRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for QueryUserTaskRemedyRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 打卡信息管理相关数据结构 ====================

/// 导入打卡流水请求
#[derive(Default)]
pub struct BatchCreateUserTaskRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 打卡记录列表
    pub user_tasks: Vec<UserTaskCreate>,
}

/// 打卡记录创建信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserTaskCreate {
    /// 用户ID
    pub user_id: String,
    /// 考勤组ID
    pub group_id: String,
    /// 班次ID
    pub shift_id: String,
    /// 打卡日期，格式：yyyy-MM-dd
    pub check_date: String,
    /// 打卡时间，格式：yyyy-MM-dd HH:mm:ss
    pub check_time: String,
    /// 打卡类型，1：上班打卡，2：下班打卡
    pub check_type: i32,
    /// 打卡结果，1：正常，2：早退，3：迟到，4：严重迟到，5：缺卡，6：无效，7：无班次，8：休息
    pub check_result: i32,
    /// 位置信息
    pub location: Option<UserTaskLocation>,
    /// 是否外勤打卡
    pub is_field: Option<bool>,
    /// 是否补卡
    pub is_remedy: Option<bool>,
    /// 打卡备注
    pub comment: Option<String>,
}

/// 打卡位置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTaskLocation {
    /// 纬度
    pub latitude: f64,
    /// 经度
    pub longitude: f64,
    /// 位置名称
    pub address: Option<String>,
}

/// 导入打卡流水响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateUserTaskRespData {
    /// 导入成功的记录数
    pub success_count: i32,
    /// 导入失败的记录数
    pub failed_count: i32,
    /// 失败记录详情
    pub failed_records: Option<Vec<UserTaskCreateFailure>>,
}

/// 打卡记录创建失败信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTaskCreateFailure {
    /// 用户ID
    pub user_id: String,
    /// 失败原因
    pub reason: String,
    /// 错误代码
    pub error_code: Option<String>,
}

/// 查询打卡流水请求
#[derive(Default)]
pub struct GetUserTaskRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 用户ID
    pub user_id: String,
    /// 查询日期，格式：yyyy-MM-dd
    pub check_date: String,
}

/// 打卡流水信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTask {
    /// 打卡记录ID
    pub task_id: String,
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub user_name: Option<String>,
    /// 考勤组ID
    pub group_id: String,
    /// 班次ID
    pub shift_id: String,
    /// 班次名称
    pub shift_name: String,
    /// 打卡日期，格式：yyyy-MM-dd
    pub check_date: String,
    /// 打卡时间，格式：yyyy-MM-dd HH:mm:ss
    pub check_time: String,
    /// 打卡类型，1：上班打卡，2：下班打卡
    pub check_type: i32,
    /// 打卡结果，1：正常，2：早退，3：迟到，4：严重迟到，5：缺卡，6：无效，7：无班次，8：休息
    pub check_result: i32,
    /// 位置信息
    pub location: Option<UserTaskLocation>,
    /// 是否外勤打卡
    pub is_field: bool,
    /// 是否补卡
    pub is_remedy: bool,
    /// 打卡备注
    pub comment: Option<String>,
    /// 记录创建时间戳（毫秒）
    pub create_time: String,
    /// 记录更新时间戳（毫秒）
    pub update_time: String,
}

/// 查询打卡流水响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTaskRespData {
    /// 打卡流水列表
    pub user_tasks: Vec<UserTask>,
}

/// 批量查询打卡流水请求
#[derive(Default)]
pub struct QueryUserTaskRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 用户ID列表
    pub user_ids: Option<Vec<String>>,
    /// 查询开始日期，格式：yyyy-MM-dd
    pub check_date_from: Option<String>,
    /// 查询结束日期，格式：yyyy-MM-dd
    pub check_date_to: Option<String>,
    /// 打卡类型，1：上班打卡，2：下班打卡
    pub check_type: Option<i32>,
    /// 分页大小，最大100
    pub page_size: Option<i32>,
    /// 分页偏移量
    pub page_token: Option<String>,
}

/// 批量查询打卡流水响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserTaskRespData {
    /// 打卡流水列表
    pub user_tasks: Vec<UserTask>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页令牌
    pub page_token: Option<String>,
}

/// 删除打卡流水请求
#[derive(Default)]
pub struct BatchDelUserTaskRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 打卡记录ID列表
    pub task_ids: Vec<String>,
}

/// 删除打卡流水响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDelUserTaskRespData {
    /// 删除成功的记录数
    pub success_count: i32,
    /// 删除失败的记录数
    pub failed_count: i32,
    /// 失败记录详情
    pub failed_records: Option<Vec<UserTaskDeleteFailure>>,
}

/// 打卡记录删除失败信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTaskDeleteFailure {
    /// 打卡记录ID
    pub task_id: String,
    /// 失败原因
    pub reason: String,
    /// 错误代码
    pub error_code: Option<String>,
}

/// 查询打卡结果请求
#[derive(Default)]
pub struct QueryUserTaskResultRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 用户ID列表
    pub user_ids: Option<Vec<String>>,
    /// 查询开始日期，格式：yyyy-MM-dd
    pub check_date_from: Option<String>,
    /// 查询结束日期，格式：yyyy-MM-dd
    pub check_date_to: Option<String>,
    /// 分页大小，最大100
    pub page_size: Option<i32>,
    /// 分页偏移量
    pub page_token: Option<String>,
}

/// 打卡结果信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTaskResult {
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub user_name: Option<String>,
    /// 考勤日期，格式：yyyy-MM-dd
    pub attendance_date: String,
    /// 考勤状态，1：正常，2：迟到，3：早退，4：缺勤，5：休息，6：加班
    pub attendance_status: i32,
    /// 上班打卡记录
    pub check_in_records: Option<Vec<UserTask>>,
    /// 下班打卡记录
    pub check_out_records: Option<Vec<UserTask>>,
    /// 工作时长（小时）
    pub work_duration: Option<f64>,
    /// 加班时长（小时）
    pub overtime_duration: Option<f64>,
    /// 考勤异常信息
    pub exceptions: Option<Vec<String>>,
}

/// 查询打卡结果响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserTaskResultRespData {
    /// 打卡结果列表
    pub user_task_results: Vec<UserTaskResult>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页令牌
    pub page_token: Option<String>,
}

// 实现 ApiResponseTrait

impl ApiResponseTrait for BatchCreateUserTaskRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetUserTaskRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for QueryUserTaskRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for BatchDelUserTaskRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for QueryUserTaskResultRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 归档报表相关数据结构 ====================

/// 查询归档报表表头请求
#[derive(Default)]
pub struct QueryArchiveStatsFieldsRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 归档规则ID
    pub archive_rule_id: String,
}

/// 归档报表字段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveStatsField {
    /// 字段ID
    pub field_id: String,
    /// 字段名称
    pub field_name: String,
    /// 字段类型，1：文本，2：数值，3：日期，4：布尔值
    pub field_type: i32,
    /// 字段描述
    pub field_description: Option<String>,
    /// 是否必填
    pub is_required: bool,
    /// 字段默认值
    pub default_value: Option<String>,
    /// 字段选项（用于枚举类型）
    pub field_options: Option<Vec<ArchiveFieldOption>>,
}

/// 归档报表字段选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveFieldOption {
    /// 选项值
    pub value: String,
    /// 选项显示名称
    pub label: String,
}

/// 查询归档报表表头响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryArchiveStatsFieldsRespData {
    /// 报表字段列表
    pub fields: Vec<ArchiveStatsField>,
    /// 报表名称
    pub report_name: String,
    /// 归档规则名称
    pub archive_rule_name: String,
}

/// 写入归档报表结果请求
#[derive(Default)]
pub struct UploadArchiveReportRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 归档规则ID
    pub archive_rule_id: String,
    /// 报表数据
    pub report_data: Vec<ArchiveReportRecord>,
}

/// 归档报表记录
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArchiveReportRecord {
    /// 记录ID（用于更新或删除）
    pub record_id: Option<String>,
    /// 用户ID
    pub user_id: String,
    /// 归档日期，格式：yyyy-MM-dd
    pub archive_date: String,
    /// 报表字段数据，key为字段ID，value为字段值
    pub field_data: std::collections::HashMap<String, String>,
}

/// 写入归档报表结果响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadArchiveReportRespData {
    /// 上传成功的记录数
    pub success_count: i32,
    /// 上传失败的记录数
    pub failed_count: i32,
    /// 失败记录详情
    pub failed_records: Option<Vec<ArchiveReportFailure>>,
}

/// 归档报表上传失败信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveReportFailure {
    /// 用户ID
    pub user_id: String,
    /// 归档日期
    pub archive_date: String,
    /// 失败原因
    pub reason: String,
    /// 错误代码
    pub error_code: Option<String>,
}

/// 删除归档报表行数据请求
#[derive(Default)]
pub struct DelArchiveReportRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 归档规则ID
    pub archive_rule_id: String,
    /// 要删除的记录ID列表
    pub record_ids: Vec<String>,
}

/// 删除归档报表行数据响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelArchiveReportRespData {
    /// 删除成功的记录数
    pub success_count: i32,
    /// 删除失败的记录数
    pub failed_count: i32,
    /// 失败记录详情
    pub failed_records: Option<Vec<ArchiveReportDeleteFailure>>,
}

/// 归档报表删除失败信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveReportDeleteFailure {
    /// 记录ID
    pub record_id: String,
    /// 失败原因
    pub reason: String,
    /// 错误代码
    pub error_code: Option<String>,
}

/// 查询所有归档规则请求
#[derive(Default)]
pub struct ListArchiveRulesRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 分页大小，最大100
    pub page_size: Option<i32>,
    /// 分页偏移量
    pub page_token: Option<String>,
}

/// 归档规则信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveRule {
    /// 归档规则ID
    pub archive_rule_id: String,
    /// 归档规则名称
    pub archive_rule_name: String,
    /// 归档规则描述
    pub description: Option<String>,
    /// 归档周期类型，1：日，2：周，3：月，4：季，5：年
    pub archive_period_type: i32,
    /// 是否启用
    pub is_enabled: bool,
    /// 创建时间戳（毫秒）
    pub create_time: String,
    /// 更新时间戳（毫秒）
    pub update_time: String,
    /// 关联的统计字段数量
    pub field_count: i32,
}

/// 查询所有归档规则响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListArchiveRulesRespData {
    /// 归档规则列表
    pub archive_rules: Vec<ArchiveRule>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页令牌
    pub page_token: Option<String>,
}

// 实现 ApiResponseTrait

impl ApiResponseTrait for QueryArchiveStatsFieldsRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for UploadArchiveReportRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DelArchiveReportRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ListArchiveRulesRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

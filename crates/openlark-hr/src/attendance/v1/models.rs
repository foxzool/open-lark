use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
},
use serde::{Deserialize, Serialize },

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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FacePunchConfig {
    // TODO: Add fields
}

/// 创建班次请求
#[derive(Default, Debug)]
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
#[derive(Default, Debug)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptDateRule {
    /// 例外日期，格式：YYYY-MM-DD
    pub date: String,
    /// 例外类型：1-工作日 2-休息日
    pub except_type: i32,
    /// 班次 ID（当例外类型为工作日时必填）
    pub shift_id: Option<String>,
}

/// 工作日规则
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkDayRule {
    /// 星期几：1-7，1代表周一
    pub week_day: i32,
    /// 班次 ID
    pub shift_id: String,
}

/// 班次规则
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShiftRule {
    /// 班次 ID
    pub shift_id: String,
    /// 班次名称
    pub shift_name: Option<String>,
}

/// 成员规则
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Default, Debug)]
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
#[derive(Default, Debug)]
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
#[derive(Default, Clone)]
pub struct UpdateUserStatsDataRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 统计设置
    pub stats_setting: StatsSettings,
}

impl UpdateUserStatsDataRequest {
    pub fn build(self) -> Self {
        self
    }
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
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
#[derive(Default, Debug)]
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
#[derive(Default, Clone, Debug)]
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
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
#[derive(Debug, Default, Clone)]
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
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

// ==================== 休假相关数据结构 ====================

/// 通过过期时间获取发放记录请求
#[derive(Default, Clone)]
pub struct GetLeaveEmployExpireRecordRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 过期时间开始（毫秒级时间戳）
    pub start_time: i64,
    /// 过期时间结束（毫秒级时间戳）
    pub end_time: i64,
    /// 分页大小，最大100
    pub page_size: Option<i32>,
    /// 分页偏移量
    pub page_token: Option<String>,
}

/// 休假发放记录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveEmployExpireRecord {
    /// 记录ID
    pub record_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 员工名称
    pub employee_name: Option<String>,
    /// 休假类型ID
    pub leave_type_id: String,
    /// 休假类型名称
    pub leave_type_name: String,
    /// 发放数量（小时）
    pub granted_amount: f64,
    /// 过期时间（毫秒级时间戳）
    pub expire_time: i64,
    /// 发放时间（毫秒级时间戳）
    pub granted_time: i64,
    /// 发放原因
    pub granted_reason: Option<String>,
    /// 剩余数量（小时）
    pub remaining_amount: f64,
    /// 状态：1-有效，2-已过期，3-已使用完
    pub status: i32,
    /// 创建时间（毫秒级时间戳）
    pub create_time: i64,
    /// 更新时间（毫秒级时间戳）
    pub update_time: i64,
}

/// 通过过期时间获取发放记录响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLeaveEmployExpireRecordRespData {
    /// 发放记录列表
    pub records: Vec<LeaveEmployExpireRecord>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 下一页令牌
    pub page_token: Option<String>,
    /// 总记录数
    pub total_count: i32,
}

/// 修改发放记录请求
#[derive(Default, Clone)]
pub struct PatchLeaveAccrualRecordRequest {
    pub api_req: ApiRequest,
    /// 员工ID类型
    pub employee_type: String,
    /// 发放记录ID
    pub leave_accrual_record_id: String,
    /// 修改的发放记录信息
    pub leave_accrual_record: LeaveAccrualRecordPatch,
}

/// 发放记录修改信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LeaveAccrualRecordPatch {
    /// 员工ID
    pub employee_id: Option<String>,
    /// 休假类型ID
    pub leave_type_id: Option<String>,
    /// 发放数量（小时）
    pub granted_amount: Option<f64>,
    /// 过期时间（毫秒级时间戳）
    pub expire_time: Option<i64>,
    /// 发放时间（毫秒级时间戳）
    pub granted_time: Option<i64>,
    /// 发放原因
    pub granted_reason: Option<String>,
    /// 有效期类型：1-永久有效，2-指定过期时间
    pub validity_type: Option<i32>,
    /// 发放类型：1-系统自动发放，2-管理员手动发放，3-员工申请发放
    pub granted_type: Option<i32>,
    /// 发放说明
    pub granted_description: Option<String>,
}

/// 修改发放记录响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchLeaveAccrualRecordRespData {
    /// 修改后的发放记录信息
    pub leave_accrual_record: LeaveAccrualRecord,
}

/// 完整的发放记录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveAccrualRecord {
    /// 记录ID
    pub record_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 员工名称
    pub employee_name: Option<String>,
    /// 休假类型ID
    pub leave_type_id: String,
    /// 休假类型名称
    pub leave_type_name: String,
    /// 发放数量（小时）
    pub granted_amount: f64,
    /// 过期时间（毫秒级时间戳）
    pub expire_time: Option<i64>,
    /// 发放时间（毫秒级时间戳）
    pub granted_time: i64,
    /// 发放原因
    pub granted_reason: Option<String>,
    /// 剩余数量（小时）
    pub remaining_amount: f64,
    /// 已使用数量（小时）
    pub used_amount: f64,
    /// 状态：1-有效，2-已过期，3-已使用完
    pub status: i32,
    /// 有效期类型：1-永久有效，2-指定过期时间
    pub validity_type: i32,
    /// 发放类型：1-系统自动发放，2-管理员手动发放，3-员工申请发放
    pub granted_type: i32,
    /// 发放说明
    pub granted_description: Option<String>,
    /// 创建时间（毫秒级时间戳）
    pub create_time: i64,
    /// 更新时间（毫秒级时间戳）
    pub update_time: i64,
}

// 实现 ApiResponseTrait

impl ApiResponseTrait for GetLeaveEmployExpireRecordRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for PatchLeaveAccrualRecordRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// Build method implementations for trait support
impl QueryStatsSettingsRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl QueryStatsFieldsRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl QueryUserStatsDataRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl CreateUserTaskRemedyRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl QueryUserAllowedRemedysRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl QueryUserTaskRemedyRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl QueryArchiveStatsFieldsRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl UploadArchiveReportRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl DelArchiveReportRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl ListArchiveRulesRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl GetLeaveEmployExpireRecordRequest {
    pub fn build(self) -> Self {
        self
    }
}

impl PatchLeaveAccrualRecordRequest {
    pub fn build(self) -> Self {
        self
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_shift_serialization() {
        let shift = Shift {
            shift_id: "shift_001".to_string(),
            shift_name: "Morning Shift".to_string(),
            punch_times: 2,
            is_flexible: Some(true),
            flexible_minutes: Some(30),
            flexible_rule: Some(vec![FlexibleRule {
                flexible_early_minutes: 15,
                flexible_late_minutes: 15,
            }]),
            no_need_off: Some(false),
            punch_time_rule: Some(vec![PunchTimeRule {
                on_time: "09:00".to_string(),
                off_time: "18:00".to_string(),
                on_advance_minutes: 30,
                off_delay_minutes: 30,
                late_minutes_as_late: Some(10),
                late_minutes_as_lack: Some(30),
                early_minutes_as_early: Some(10),
                early_minutes_as_lack: Some(30),
            }]),
            late_minutes_as_late: Some(10),
            late_minutes_as_lack: Some(30),
            early_minutes_as_early: Some(10),
            early_minutes_as_lack: Some(30),
            allow_outside_apply: Some(true),
            outside_apply_limit: Some(2),
            allow_face_punch: Some(true),
            face_punch_cfg: Some(FacePunchConfig::default()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        },

        let serialized = serde_json::to_string(&shift).unwrap();
        let deserialized: Shift = serde_json::from_str(&serialized).unwrap();
        assert_eq!(shift.shift_id, deserialized.shift_id);
        assert_eq!(shift.shift_name, deserialized.shift_name);
        assert_eq!(shift.punch_times, deserialized.punch_times);
    }

    #[test]
    fn test_flexible_rule_serialization() {
        let rule = FlexibleRule {
            flexible_early_minutes: 15,
            flexible_late_minutes: 20,
        },

        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: FlexibleRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            rule.flexible_early_minutes,
            deserialized.flexible_early_minutes
        );
        assert_eq!(
            rule.flexible_late_minutes,
            deserialized.flexible_late_minutes
        );
    }

    #[test]
    fn test_punch_time_rule_serialization() {
        let rule = PunchTimeRule {
            on_time: "09:00".to_string(),
            off_time: "17:30".to_string(),
            on_advance_minutes: 30,
            off_delay_minutes: 60,
            late_minutes_as_late: Some(15),
            late_minutes_as_lack: Some(45),
            early_minutes_as_early: Some(15),
            early_minutes_as_lack: Some(45),
        },

        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: PunchTimeRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rule.on_time, deserialized.on_time);
        assert_eq!(rule.off_time, deserialized.off_time);
        assert_eq!(rule.on_advance_minutes, deserialized.on_advance_minutes);
    }

    #[test]
    fn test_face_punch_config_serialization() {
        let config = FacePunchConfig::default();

        let serialized = serde_json::to_string(&config).unwrap();
        let _deserialized: FacePunchConfig = serde_json::from_str(&serialized).unwrap();
        // Test should succeed without panicking
    }

    #[test]
    fn test_create_shift_request_default() {
        let request = CreateShiftRequest::default();
        assert_eq!(request.employee_type, "");
        assert_eq!(request.shift_name, "");
        assert_eq!(request.punch_times, 0);
        assert!(request.is_flexible.is_none());
    }

    #[test]
    fn test_create_shift_resp_data_api_response_trait() {
        assert_eq!(CreateShiftRespData::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_user_daily_shift_serialization() {
        let shift = UserDailyShift {
            user_id: "user_123".to_string(),
            shift_date: "2023-10-15".to_string(),
            shift_id: "shift_456".to_string(),
        },

        let serialized = serde_json::to_string(&shift).unwrap();
        let deserialized: UserDailyShift = serde_json::from_str(&serialized).unwrap();
        assert_eq!(shift.user_id, deserialized.user_id);
        assert_eq!(shift.shift_date, deserialized.shift_date);
        assert_eq!(shift.shift_id, deserialized.shift_id);
    }

    #[test]
    fn test_batch_create_user_daily_shift_resp_data_serialization() {
        let resp_data = BatchCreateUserDailyShiftRespData {
            failed_user_daily_shifts: Some(vec![UserDailyShift {
                user_id: "user_789".to_string(),
                shift_date: "2023-10-16".to_string(),
                shift_id: "shift_101".to_string(),
            }]),
            success_count: Some(5),
            failed_count: Some(1),
        },

        let serialized = serde_json::to_string(&resp_data).unwrap();
        let deserialized: BatchCreateUserDailyShiftRespData =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(resp_data.success_count, deserialized.success_count);
        assert_eq!(resp_data.failed_count, deserialized.failed_count);
    }

    #[test]
    fn test_group_serialization() {
        let group = Group {
            group_id: "group_001".to_string(),
            group_name: "Engineering Team".to_string(),
            time_zone: Some("Asia/Shanghai".to_string()),
            bind_dept_ids: Some(vec!["dept_001".to_string(), "dept_002".to_string()]),
            except_date_rule: Some(vec![ExceptDateRule {
                date: "2023-12-25".to_string(),
                except_type: 2,
                shift_id: None,
            }]),
            attendance_type: Some(1),
            punch_type: Some(1),
            allow_late_minutes: Some(10),
            allow_early_leave_minutes: Some(10),
            work_day_rule: Some(vec![WorkDayRule {
                week_day: 1,
                shift_id: "shift_001".to_string(),
            }]),
            shift_rule: Some(vec![ShiftRule {
                shift_id: "shift_001".to_string(),
                shift_name: Some("Morning".to_string()),
            }]),
            member_rule: Some(MemberRule {
                member_type: 1,
                member_ids: vec!["member_001".to_string()],
            }),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        },

        let serialized = serde_json::to_string(&group).unwrap();
        let deserialized: Group = serde_json::from_str(&serialized).unwrap();
        assert_eq!(group.group_id, deserialized.group_id);
        assert_eq!(group.group_name, deserialized.group_name);
        assert_eq!(group.time_zone, deserialized.time_zone);
    }

    #[test]
    fn test_except_date_rule_serialization() {
        let rule = ExceptDateRule {
            date: "2023-05-01".to_string(),
            except_type: 1,
            shift_id: Some("shift_special".to_string()),
        },

        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: ExceptDateRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rule.date, deserialized.date);
        assert_eq!(rule.except_type, deserialized.except_type);
        assert_eq!(rule.shift_id, deserialized.shift_id);
    }

    #[test]
    fn test_work_day_rule_serialization() {
        let rule = WorkDayRule {
            week_day: 2,
            shift_id: "shift_tuesday".to_string(),
        },

        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: WorkDayRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rule.week_day, deserialized.week_day);
        assert_eq!(rule.shift_id, deserialized.shift_id);
    }

    #[test]
    fn test_user_setting_serialization() {
        let setting = UserSetting {
            user_id: "user_001".to_string(),
            face_key_open: Some(true),
            face_key: Some("face_key_abc123".to_string()),
            face_live_need_action: Some(false),
            face_downgrade: Some(true),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        },

        let serialized = serde_json::to_string(&setting).unwrap();
        let deserialized: UserSetting = serde_json::from_str(&serialized).unwrap();
        assert_eq!(setting.user_id, deserialized.user_id);
        assert_eq!(setting.face_key_open, deserialized.face_key_open);
        assert_eq!(setting.face_key, deserialized.face_key);
    }

    #[test]
    fn test_stats_settings_serialization() {
        let settings = StatsSettings {
            stats_scope: 1,
            start_date: "2023-01-01".to_string(),
            end_date: "2023-12-31".to_string(),
            user_ids: vec!["user_001".to_string(), "user_002".to_string()],
            need_fields: vec!["attendance_days".to_string(), "late_count".to_string()],
        },

        let serialized = serde_json::to_string(&settings).unwrap();
        let deserialized: StatsSettings = serde_json::from_str(&serialized).unwrap();
        assert_eq!(settings.stats_scope, deserialized.stats_scope);
        assert_eq!(settings.start_date, deserialized.start_date);
        assert_eq!(settings.end_date, deserialized.end_date);
        assert_eq!(settings.user_ids, deserialized.user_ids);
    }

    #[test]
    fn test_stats_field_serialization() {
        let field = StatsField {
            field_key: "attendance_rate".to_string(),
            field_name: "Attendance Rate".to_string(),
            field_name_zh: Some("出勤率".to_string()),
            field_name_en: Some("Attendance Rate".to_string()),
            field_name_ja: Some("出席率".to_string()),
            field_type: 1,
        },

        let serialized = serde_json::to_string(&field).unwrap();
        let deserialized: StatsField = serde_json::from_str(&serialized).unwrap();
        assert_eq!(field.field_key, deserialized.field_key);
        assert_eq!(field.field_name, deserialized.field_name);
        assert_eq!(field.field_type, deserialized.field_type);
    }

    #[test]
    fn test_user_stats_data_serialization() {
        let mut datas = std::collections::HashMap::new();
        datas.insert(
            "attendance_days".to_string(),
            serde_json::Value::Number(22.into()),
        );
        datas.insert(
            "late_count".to_string(),
            serde_json::Value::Number(3.into()),
        );

        let stats_data = UserStatsData {
            user_id: "user_001".to_string(),
            user_name: Some("John Doe".to_string()),
            datas,
        },

        let serialized = serde_json::to_string(&stats_data).unwrap();
        let deserialized: UserStatsData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stats_data.user_id, deserialized.user_id);
        assert_eq!(stats_data.user_name, deserialized.user_name);
        assert_eq!(stats_data.datas.len(), deserialized.datas.len());
    }

    #[test]
    fn test_user_approval_serialization() {
        let approval = UserApproval {
            approval_id: "approval_001".to_string(),
            user_id: "user_001".to_string(),
            user_name: Some("Jane Smith".to_string()),
            approval_type: 1,
            status: 2,
            start_time: "2023-10-01T09:00:00Z".to_string(),
            end_time: "2023-10-01T18:00:00Z".to_string(),
            duration: Some(8.0),
            reason: Some("Personal leave".to_string()),
            approval_note: Some("Approved by manager".to_string()),
            created_at: Some("2023-09-30T10:00:00Z".to_string()),
            approved_at: Some("2023-09-30T11:00:00Z".to_string()),
        },

        let serialized = serde_json::to_string(&approval).unwrap();
        let deserialized: UserApproval = serde_json::from_str(&serialized).unwrap();
        assert_eq!(approval.approval_id, deserialized.approval_id);
        assert_eq!(approval.user_id, deserialized.user_id);
        assert_eq!(approval.approval_type, deserialized.approval_type);
        assert_eq!(approval.duration, deserialized.duration);
    }

    #[test]
    fn test_user_task_remedy_application_serialization() {
        let application = UserTaskRemedyApplication {
            user_id: "user_001".to_string(),
            remedy_date: "2023-10-15".to_string(),
            remedy_time: "09:15".to_string(),
            remedy_type: 1,
            reason: "Traffic jam".to_string(),
            comment: Some("Heavy traffic due to weather".to_string()),
        },

        let serialized = serde_json::to_string(&application).unwrap();
        let deserialized: UserTaskRemedyApplication = serde_json::from_str(&serialized).unwrap();
        assert_eq!(application.user_id, deserialized.user_id);
        assert_eq!(application.remedy_date, deserialized.remedy_date);
        assert_eq!(application.remedy_type, deserialized.remedy_type);
        assert_eq!(application.reason, deserialized.reason);
    }

    #[test]
    fn test_user_task_location_serialization() {
        let location = UserTaskLocation {
            latitude: 39.9042,
            longitude: 116.4074,
            address: Some("Beijing, China".to_string()),
        },

        let serialized = serde_json::to_string(&location).unwrap();
        let deserialized: UserTaskLocation = serde_json::from_str(&serialized).unwrap();
        assert_eq!(location.latitude, deserialized.latitude);
        assert_eq!(location.longitude, deserialized.longitude);
        assert_eq!(location.address, deserialized.address);
    }

    #[test]
    fn test_user_task_create_serialization() {
        let task_create = UserTaskCreate {
            user_id: "user_001".to_string(),
            group_id: "group_001".to_string(),
            shift_id: "shift_001".to_string(),
            check_date: "2023-10-15".to_string(),
            check_time: "2023-10-15 09:00:00".to_string(),
            check_type: 1,
            check_result: 1,
            location: Some(UserTaskLocation {
                latitude: 39.9042,
                longitude: 116.4074,
                address: Some("Office Building".to_string()),
            }),
            is_field: Some(false),
            is_remedy: Some(false),
            comment: Some("Normal check-in".to_string()),
        },

        let serialized = serde_json::to_string(&task_create).unwrap();
        let deserialized: UserTaskCreate = serde_json::from_str(&serialized).unwrap();
        assert_eq!(task_create.user_id, deserialized.user_id);
        assert_eq!(task_create.check_type, deserialized.check_type);
        assert_eq!(task_create.check_result, deserialized.check_result);
    }

    #[test]
    fn test_user_task_serialization() {
        let task = UserTask {
            task_id: "task_001".to_string(),
            user_id: "user_001".to_string(),
            user_name: Some("Alice Johnson".to_string()),
            group_id: "group_001".to_string(),
            shift_id: "shift_001".to_string(),
            shift_name: "Morning Shift".to_string(),
            check_date: "2023-10-15".to_string(),
            check_time: "2023-10-15 09:00:00".to_string(),
            check_type: 1,
            check_result: 1,
            location: Some(UserTaskLocation {
                latitude: 39.9042,
                longitude: 116.4074,
                address: Some("Main Office".to_string()),
            }),
            is_field: false,
            is_remedy: false,
            comment: Some("On time arrival".to_string()),
            create_time: "1634284800000".to_string(),
            update_time: "1634284800000".to_string(),
        },

        let serialized = serde_json::to_string(&task).unwrap();
        let deserialized: UserTask = serde_json::from_str(&serialized).unwrap();
        assert_eq!(task.task_id, deserialized.task_id);
        assert_eq!(task.user_name, deserialized.user_name);
        assert_eq!(task.shift_name, deserialized.shift_name);
        assert_eq!(task.is_field, deserialized.is_field);
    }

    #[test]
    fn test_archive_field_option_serialization() {
        let option = ArchiveFieldOption {
            value: "active".to_string(),
            label: "Active Status".to_string(),
        },

        let serialized = serde_json::to_string(&option).unwrap();
        let deserialized: ArchiveFieldOption = serde_json::from_str(&serialized).unwrap();
        assert_eq!(option.value, deserialized.value);
        assert_eq!(option.label, deserialized.label);
    }

    #[test]
    fn test_archive_stats_field_serialization() {
        let field = ArchiveStatsField {
            field_id: "field_001".to_string(),
            field_name: "Employee Status".to_string(),
            field_type: 1,
            field_description: Some("Current employment status".to_string()),
            is_required: true,
            default_value: Some("active".to_string()),
            field_options: Some(vec![ArchiveFieldOption {
                value: "active".to_string(),
                label: "Active".to_string(),
            }]),
        },

        let serialized = serde_json::to_string(&field).unwrap();
        let deserialized: ArchiveStatsField = serde_json::from_str(&serialized).unwrap();
        assert_eq!(field.field_id, deserialized.field_id);
        assert_eq!(field.field_name, deserialized.field_name);
        assert_eq!(field.is_required, deserialized.is_required);
    }

    #[test]
    fn test_archive_report_record_serialization() {
        let mut field_data = std::collections::HashMap::new();
        field_data.insert("status".to_string(), "active".to_string());
        field_data.insert("department".to_string(), "engineering".to_string());

        let record = ArchiveReportRecord {
            record_id: Some("record_001".to_string()),
            user_id: "user_001".to_string(),
            archive_date: "2023-10-15".to_string(),
            field_data,
        },

        let serialized = serde_json::to_string(&record).unwrap();
        let deserialized: ArchiveReportRecord = serde_json::from_str(&serialized).unwrap();
        assert_eq!(record.record_id, deserialized.record_id);
        assert_eq!(record.user_id, deserialized.user_id);
        assert_eq!(record.archive_date, deserialized.archive_date);
        assert_eq!(record.field_data.len(), deserialized.field_data.len());
    }

    #[test]
    fn test_leave_employ_expire_record_serialization() {
        let record = LeaveEmployExpireRecord {
            record_id: "leave_001".to_string(),
            employee_id: "emp_001".to_string(),
            employee_name: Some("Bob Wilson".to_string()),
            leave_type_id: "annual_leave".to_string(),
            leave_type_name: "Annual Leave".to_string(),
            granted_amount: 40.0,
            expire_time: 1672531200000,
            granted_time: 1640995200000,
            granted_reason: Some("Annual allocation".to_string()),
            remaining_amount: 32.0,
            status: 1,
            create_time: 1640995200000,
            update_time: 1640995200000,
        },

        let serialized = serde_json::to_string(&record).unwrap();
        let deserialized: LeaveEmployExpireRecord = serde_json::from_str(&serialized).unwrap();
        assert_eq!(record.record_id, deserialized.record_id);
        assert_eq!(record.employee_id, deserialized.employee_id);
        assert_eq!(record.granted_amount, deserialized.granted_amount);
        assert_eq!(record.remaining_amount, deserialized.remaining_amount);
    }

    #[test]
    fn test_leave_accrual_record_patch_serialization() {
        let patch = LeaveAccrualRecordPatch {
            employee_id: Some("emp_002".to_string()),
            leave_type_id: Some("sick_leave".to_string()),
            granted_amount: Some(20.0),
            expire_time: Some(1703980800000),
            granted_time: Some(1672444800000),
            granted_reason: Some("Sick leave allocation".to_string()),
            validity_type: Some(2),
            granted_type: Some(1),
            granted_description: Some("System generated".to_string()),
        },

        let serialized = serde_json::to_string(&patch).unwrap();
        let deserialized: LeaveAccrualRecordPatch = serde_json::from_str(&serialized).unwrap();
        assert_eq!(patch.employee_id, deserialized.employee_id);
        assert_eq!(patch.granted_amount, deserialized.granted_amount);
        assert_eq!(patch.validity_type, deserialized.validity_type);
    }

    #[test]
    fn test_leave_accrual_record_serialization() {
        let record = LeaveAccrualRecord {
            record_id: "accrual_001".to_string(),
            employee_id: "emp_001".to_string(),
            employee_name: Some("Charlie Brown".to_string()),
            leave_type_id: "vacation".to_string(),
            leave_type_name: "Vacation Leave".to_string(),
            granted_amount: 80.0,
            expire_time: Some(1703980800000),
            granted_time: 1672444800000,
            granted_reason: Some("Annual vacation allocation".to_string()),
            remaining_amount: 64.0,
            used_amount: 16.0,
            status: 1,
            validity_type: 2,
            granted_type: 1,
            granted_description: Some("Automatically granted".to_string()),
            create_time: 1672444800000,
            update_time: 1672444800000,
        },

        let serialized = serde_json::to_string(&record).unwrap();
        let deserialized: LeaveAccrualRecord = serde_json::from_str(&serialized).unwrap();
        assert_eq!(record.record_id, deserialized.record_id);
        assert_eq!(record.granted_amount, deserialized.granted_amount);
        assert_eq!(record.used_amount, deserialized.used_amount);
        assert_eq!(record.validity_type, deserialized.validity_type);
    }

    #[test]
    fn test_default_request_structs() {
        let _delete_shift = DeleteShiftRequest::default();
        let _get_shift = GetShiftRequest::default();
        let _query_shift = QueryShiftRequest::default();
        let _list_shift = ListShiftRequest::default();
        let _batch_create = BatchCreateUserDailyShiftRequest::default();
        let _query_daily_shift = QueryUserDailyShiftRequest::default();
        let _batch_create_temp = BatchCreateTempUserDailyShiftRequest::default();
        let _list_group_user = ListGroupUserRequest::default();
        let _create_group = CreateGroupRequest::default();
        let _delete_group = DeleteGroupRequest::default();
        let _get_group = GetGroupRequest::default();
        let _search_group = SearchGroupRequest::default();
        let _list_group = ListGroupRequest::default();
        let _modify_user_setting = ModifyUserSettingRequest::default();
        let _query_user_setting = QueryUserSettingRequest::default();
        let _upload_photo = UploadUserPhotoRequest::default();
        let _download_photo = DownloadUserPhotoRequest::default();
    }

    #[test]
    fn test_api_response_trait_implementations() {
        assert_eq!(Shift::data_format(), ResponseFormat::Data);
        assert_eq!(EmptyResponse::data_format(), ResponseFormat::Data);
        assert_eq!(QueryShiftRespData::data_format(), ResponseFormat::Data);
        assert_eq!(ShiftListData::data_format(), ResponseFormat::Data);
        assert_eq!(
            BatchCreateUserDailyShiftRespData::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            QueryUserDailyShiftRespData::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            BatchCreateTempUserDailyShiftRespData::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(ListGroupUserRespData::data_format(), ResponseFormat::Data);
        assert_eq!(CreateGroupRespData::data_format(), ResponseFormat::Data);
        assert_eq!(SearchGroupRespData::data_format(), ResponseFormat::Data);
        assert_eq!(ListGroupRespData::data_format(), ResponseFormat::Data);
        assert_eq!(Group::data_format(), ResponseFormat::Data);
        assert_eq!(
            ModifyUserSettingRespData::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            QueryUserSettingRespData::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(UploadUserPhotoRespData::data_format(), ResponseFormat::Data);
        assert_eq!(
            DownloadUserPhotoRespData::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(UserSetting::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_build_methods() {
        let stats_request = UpdateUserStatsDataRequest::default().build();
        let query_stats = QueryStatsSettingsRequest::default().build();
        let query_fields = QueryStatsFieldsRequest::default().build();
        let query_data = QueryUserStatsDataRequest::default().build();
        let remedy_request = CreateUserTaskRemedyRequest::default().build();
        let allowed_remedys = QueryUserAllowedRemedysRequest::default().build();
        let task_remedy = QueryUserTaskRemedyRequest::default().build();

        // Just ensure build methods work without errors
        assert_eq!(stats_request.employee_type, "");
        assert_eq!(query_stats.employee_type, "");
        assert_eq!(query_fields.employee_type, "");
        assert_eq!(query_data.employee_type, "");
        assert_eq!(remedy_request.employee_type, "");
        assert_eq!(allowed_remedys.employee_type, "");
        assert_eq!(task_remedy.employee_type, "");
    }

    #[test]
    fn test_empty_response_serialization() {
        let empty = EmptyResponse {};
        let serialized = serde_json::to_string(&empty).unwrap();
        let deserialized: EmptyResponse = serde_json::from_str(&serialized).unwrap();
        // EmptyResponse should serialize/deserialize successfully
        let _unused = deserialized;
    }

    #[test]
    fn test_complex_nested_structures() {
        let group_user = GroupUser {
            user_id: "user_456".to_string(),
            user_name: Some("Test User".to_string()),
            employee_no: Some("EMP001".to_string()),
            department_id: Some("dept_123".to_string()),
            join_time: Some("2023-01-01T00:00:00Z".to_string()),
        },

        let serialized = serde_json::to_string(&group_user).unwrap();
        let deserialized: GroupUser = serde_json::from_str(&serialized).unwrap();
        assert_eq!(group_user.user_id, deserialized.user_id);
        assert_eq!(group_user.employee_no, deserialized.employee_no);
    }

    #[test]
    fn test_member_rule_serialization() {
        let rule = MemberRule {
            member_type: 2,
            member_ids: vec!["member_001".to_string(), "member_002".to_string()],
        },

        let serialized = serde_json::to_string(&rule).unwrap();
        let deserialized: MemberRule = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rule.member_type, deserialized.member_type);
        assert_eq!(rule.member_ids, deserialized.member_ids);
    }

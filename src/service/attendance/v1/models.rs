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
    /// 请求体中的 user_ids 和响应体中的 user_id 的员工ID类型。如果没有后台管理权限，可使用通过手机号或邮箱获取用户 ID
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
    /// 员工类型，同创建班次接口
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

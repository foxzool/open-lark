//! 考勤模块数据模型

use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// 用户考勤任务查询请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserTaskQueryRequest {
    /// 用户ID列表，最多100个
    pub user_ids: Vec<String>,
    /// 查询的开始日期
    pub check_date_from: NaiveDate,
    /// 查询的结束日期  
    pub check_date_to: NaiveDate,
    /// 是否需要请假数据，默认false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_absent_info: Option<bool>,
    /// 是否需要补卡数据，默认false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_supplement_info: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 分页大小，默认100，最大500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl UserTaskQueryRequest {
    pub fn builder() -> UserTaskQueryRequestBuilder {
        UserTaskQueryRequestBuilder::default()
    }
}

/// 用户考勤任务查询请求构建器
#[derive(Default)]
pub struct UserTaskQueryRequestBuilder {
    user_ids: Option<Vec<String>>,
    check_date_from: Option<NaiveDate>,
    check_date_to: Option<NaiveDate>,
    need_absent_info: Option<bool>,
    need_supplement_info: Option<bool>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl UserTaskQueryRequestBuilder {
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn check_date_from(mut self, date: NaiveDate) -> Self {
        self.check_date_from = Some(date);
        self
    }

    pub fn check_date_to(mut self, date: NaiveDate) -> Self {
        self.check_date_to = Some(date);
        self
    }

    pub fn need_absent_info(mut self, need: bool) -> Self {
        self.need_absent_info = Some(need);
        self
    }

    pub fn need_supplement_info(mut self, need: bool) -> Self {
        self.need_supplement_info = Some(need);
        self
    }

    pub fn page_token(mut self, token: String) -> Self {
        self.page_token = Some(token);
        self
    }

    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }

    pub fn build(self) -> UserTaskQueryRequest {
        UserTaskQueryRequest {
            user_ids: self.user_ids.unwrap_or_default(),
            check_date_from: self.check_date_from.expect("check_date_from is required"),
            check_date_to: self.check_date_to.expect("check_date_to is required"),
            need_absent_info: self.need_absent_info,
            need_supplement_info: self.need_supplement_info,
            page_token: self.page_token,
            page_size: self.page_size,
        }
    }
}

/// 用户考勤任务查询响应
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserTaskQueryResponse {
    /// 考勤记录列表
    pub records: Vec<UserTaskRecord>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: bool,
}

impl ApiResponseTrait for UserTaskQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户考勤任务记录
#[derive(Debug, Serialize, Deserialize)]
pub struct UserTaskRecord {
    /// 用户ID
    pub user_id: String,
    /// 任务ID
    pub task_id: String,
    /// 考勤日期
    pub date: NaiveDate,
    /// 班次ID
    pub shift_id: String,
    /// 班次名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_name: Option<String>,
    /// 上班打卡结果
    pub check_in_result: CheckInResult,
    /// 下班打卡结果
    pub check_out_result: CheckOutResult,
    /// 请假信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent_info: Option<AbsentInfo>,
    /// 补卡信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplement_info: Option<SupplementInfo>,
}

/// 打卡结果
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckInResult {
    /// 结果状态
    pub result: CheckInStatus,
    /// 打卡时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,
    /// 打卡位置信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_info: Option<LocationInfo>,
    /// 异常类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_type: Option<ExceptionType>,
}

/// 下班打卡结果
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckOutResult {
    /// 结果状态
    pub result: CheckInStatus,
    /// 打卡时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,
    /// 打卡位置信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_info: Option<LocationInfo>,
    /// 异常类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_type: Option<ExceptionType>,
}

/// 打卡状态
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CheckInStatus {
    /// 正常
    #[serde(rename = "Normal")]
    Normal,
    /// 早退
    #[serde(rename = "Early")]
    Early,
    /// 迟到
    #[serde(rename = "Late")]
    Late,
    /// 缺卡
    #[serde(rename = "Lack")]
    Lack,
    /// 无需打卡
    #[serde(rename = "NoNeed")]
    NoNeed,
}

/// 位置信息
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationInfo {
    /// 位置名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 经度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// 纬度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// 异常类型
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ExceptionType {
    /// 无异常
    #[serde(rename = "NoException")]
    NoException,
    /// 迟到
    #[serde(rename = "Late")]
    Late,
    /// 早退
    #[serde(rename = "Early")]
    Early,
    /// 旷工
    #[serde(rename = "Absent")]
    Absent,
    /// 请假
    #[serde(rename = "Leave")]
    Leave,
}

/// 请假信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AbsentInfo {
    /// 请假类型
    pub absent_type: String,
    /// 请假开始时间
    pub start_time: DateTime<Utc>,
    /// 请假结束时间
    pub end_time: DateTime<Utc>,
    /// 请假时长（小时）
    pub duration: f64,
}

/// 补卡信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SupplementInfo {
    /// 补卡状态
    pub status: SupplementStatus,
    /// 补卡时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplement_time: Option<DateTime<Utc>>,
    /// 补卡原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 补卡状态
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SupplementStatus {
    /// 无需补卡
    #[serde(rename = "NoNeed")]
    NoNeed,
    /// 待补卡
    #[serde(rename = "Pending")]
    Pending,
    /// 已补卡
    #[serde(rename = "Completed")]
    Completed,
}

/// 用户打卡流水查询请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserFlowQueryRequest {
    /// 用户ID列表，最多100个
    pub user_ids: Vec<String>,
    /// 查询的开始时间
    pub check_time_from: DateTime<Utc>,
    /// 查询的结束时间
    pub check_time_to: DateTime<Utc>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 分页大小，默认100，最大500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl UserFlowQueryRequest {
    pub fn builder() -> UserFlowQueryRequestBuilder {
        UserFlowQueryRequestBuilder::default()
    }
}

/// 用户打卡流水查询请求构建器
#[derive(Default)]
pub struct UserFlowQueryRequestBuilder {
    user_ids: Option<Vec<String>>,
    check_time_from: Option<DateTime<Utc>>,
    check_time_to: Option<DateTime<Utc>>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl UserFlowQueryRequestBuilder {
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn check_time_from(mut self, time: DateTime<Utc>) -> Self {
        self.check_time_from = Some(time);
        self
    }

    pub fn check_time_to(mut self, time: DateTime<Utc>) -> Self {
        self.check_time_to = Some(time);
        self
    }

    pub fn page_token(mut self, token: String) -> Self {
        self.page_token = Some(token);
        self
    }

    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }

    pub fn build(self) -> UserFlowQueryRequest {
        UserFlowQueryRequest {
            user_ids: self.user_ids.unwrap_or_default(),
            check_time_from: self.check_time_from.expect("check_time_from is required"),
            check_time_to: self.check_time_to.expect("check_time_to is required"),
            page_token: self.page_token,
            page_size: self.page_size,
        }
    }
}

/// 用户打卡流水查询响应
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserFlowQueryResponse {
    /// 打卡流水记录列表
    pub records: Vec<UserFlowRecord>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: bool,
}

impl ApiResponseTrait for UserFlowQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户打卡流水记录
#[derive(Debug, Serialize, Deserialize)]
pub struct UserFlowRecord {
    /// 用户ID
    pub user_id: String,
    /// 打卡时间
    pub check_time: DateTime<Utc>,
    /// 打卡类型（上班/下班）
    pub check_type: CheckType,
    /// 打卡方式
    pub check_method: CheckMethod,
    /// 位置信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_info: Option<LocationInfo>,
    /// 设备信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<DeviceInfo>,
    /// 照片信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_info: Option<PhotoInfo>,
}

/// 打卡类型
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CheckType {
    /// 上班打卡
    #[serde(rename = "CheckIn")]
    CheckIn,
    /// 下班打卡
    #[serde(rename = "CheckOut")]
    CheckOut,
}

/// 打卡方式
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CheckMethod {
    /// GPS打卡
    #[serde(rename = "GPS")]
    GPS,
    /// WiFi打卡
    #[serde(rename = "WiFi")]
    WiFi,
    /// 考勤机打卡
    #[serde(rename = "Machine")]
    Machine,
    /// PC打卡
    #[serde(rename = "PC")]
    PC,
    /// 手机打卡
    #[serde(rename = "Mobile")]
    Mobile,
}

/// 设备信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// 设备名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// 设备类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

/// 照片信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoInfo {
    /// 照片文件key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_key: Option<String>,
    /// 照片URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_user_task_query_request_builder() {
        let req = UserTaskQueryRequest::builder()
            .user_ids(vec!["user_1".to_string(), "user_2".to_string()])
            .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
            .need_absent_info(true)
            .page_size(50)
            .build();

        assert_eq!(req.user_ids.len(), 2);
        assert_eq!(
            req.check_date_from,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
        );
        assert_eq!(req.need_absent_info, Some(true));
        assert_eq!(req.page_size, Some(50));
    }

    #[test]
    fn test_user_flow_query_request_builder() {
        let from_time = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let to_time = Utc.with_ymd_and_hms(2025, 1, 31, 23, 59, 59).unwrap();

        let req = UserFlowQueryRequest::builder()
            .user_ids(vec!["user_1".to_string()])
            .check_time_from(from_time)
            .check_time_to(to_time)
            .page_size(100)
            .build();

        assert_eq!(req.user_ids.len(), 1);
        assert_eq!(req.check_time_from, from_time);
        assert_eq!(req.check_time_to, to_time);
        assert_eq!(req.page_size, Some(100));
    }

    #[test]
    fn test_check_in_status_serialization() {
        let status = CheckInStatus::Normal;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"Normal\"");

        let deserialized: CheckInStatus = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, CheckInStatus::Normal));
    }

    #[test]
    fn test_check_method_serialization() {
        let method = CheckMethod::GPS;
        let json = serde_json::to_string(&method).unwrap();
        assert_eq!(json, "\"GPS\"");

        let deserialized: CheckMethod = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, CheckMethod::GPS));
    }
}

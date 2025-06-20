//! 排班信息查询服务
//!
//! 提供查询排班信息、班次详情等功能。

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use chrono::NaiveDate;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 排班信息服务
pub struct ShiftService {
    config: Config,
}

impl ShiftService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询排班信息
    ///
    /// # 权限要求
    /// - `attendance:readonly` - 基础考勤数据读取权限
    ///
    /// # 参数
    /// - `req` - 查询请求参数
    /// - `option` - 请求选项
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use chrono::NaiveDate;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret").build();
    ///     let req = ShiftQueryRequest::builder()
    ///         .shift_ids(vec!["shift_123".to_string()])
    ///         .date(NaiveDate::from_ymd_opt(2025, 1, 20).unwrap())
    ///         .build();
    ///
    ///     let response = client.attendance.v1.shift.query(req, None).await?;
    ///     for shift in response.shifts {
    ///         println!("班次: {} - {}", shift.shift_name, shift.punch_day_shift_ids.len());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn query(
        &self,
        req: ShiftQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ShiftQueryResponse> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/attendance/v1/shift/query".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&req)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<ShiftQueryResponse> =
            Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp.data.unwrap_or_default())
    }
}

/// 排班查询请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShiftQueryRequest {
    /// 班次ID列表
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shift_ids: Vec<String>,
    /// 查询日期
    pub date: NaiveDate,
}

impl ShiftQueryRequest {
    pub fn builder() -> ShiftQueryRequestBuilder {
        ShiftQueryRequestBuilder::default()
    }
}

/// 排班查询请求构建器
#[derive(Default)]
pub struct ShiftQueryRequestBuilder {
    shift_ids: Vec<String>,
    date: Option<NaiveDate>,
}

impl ShiftQueryRequestBuilder {
    pub fn shift_ids(mut self, shift_ids: Vec<String>) -> Self {
        self.shift_ids = shift_ids;
        self
    }

    pub fn date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }

    pub fn build(self) -> ShiftQueryRequest {
        ShiftQueryRequest {
            shift_ids: self.shift_ids,
            date: self.date.expect("date is required"),
        }
    }
}

/// 排班查询响应
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShiftQueryResponse {
    /// 班次信息列表
    pub shifts: Vec<ShiftInfo>,
}

impl ApiResponseTrait for ShiftQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 班次信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ShiftInfo {
    /// 班次ID
    pub shift_id: String,
    /// 班次名称
    pub shift_name: String,
    /// 打卡次数
    pub punch_times: i32,
    /// 是否弹性打卡
    pub is_flexible: bool,
    /// 弹性打卡时间(分钟)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_minutes: Option<i32>,
    /// 弹性班次允许迟到时间(分钟)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_rule: Option<FlexibleRule>,
    /// 班次时间配置
    pub punch_day_shift_ids: Vec<String>,
    /// 班次时间段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_day_shifts: Option<Vec<PunchDayShift>>,
}

/// 弹性规则
#[derive(Debug, Serialize, Deserialize)]
pub struct FlexibleRule {
    /// 允许迟到时间(分钟)
    pub late_minutes_as_lack: i32,
    /// 早退时间设置(分钟)
    pub late_minutes_as_late: i32,
}

/// 打卡时间段
#[derive(Debug, Serialize, Deserialize)]
pub struct PunchDayShift {
    /// 时间段ID
    pub punch_day_shift_id: String,
    /// 上班时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_start_time: Option<String>,
    /// 下班时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_end_time: Option<String>,
    /// 是否跨天
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_across_day: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_query_request_builder() {
        let req = ShiftQueryRequest::builder()
            .shift_ids(vec!["shift_1".to_string(), "shift_2".to_string()])
            .date(NaiveDate::from_ymd_opt(2025, 1, 20).unwrap())
            .build();

        assert_eq!(req.shift_ids.len(), 2);
        assert_eq!(req.date, NaiveDate::from_ymd_opt(2025, 1, 20).unwrap());
    }

    #[test]
    fn test_shift_service_creation() {
        let config = Config::default();
        let service = ShiftService::new(config);

        // 验证服务创建成功
        assert_eq!(
            std::mem::size_of_val(&service),
            std::mem::size_of::<Config>()
        );
    }
}

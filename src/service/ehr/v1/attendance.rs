//! Attendance v1 - 考勤管理API
//!
//! 提供完整的企业考勤管理功能，包括：
//! - 考勤记录的增删改查和批量处理
//! - 打卡记录管理和异常处理
//! - 考勤统计分析和报表生成
//! - 请假加班关联和审批流程
//! - 考勤规则配置和策略管理
//! - 外勤打卡和位置服务支持
//!
//! # 示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::ehr::v1::attendance::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 获取员工考勤记录
//!     let response = client.ehr.v1.attendance
//!         .get_attendance_records_builder("emp_001")
//!         .start_date("2024-01-01")
//!         .end_date("2024-01-31")
//!         .page_size(50)
//!         .execute(&client.ehr.v1.attendance)
//!         .await?;
//!
//!     println!("获取到 {} 条考勤记录", response.data.items.len());
//!
//!     // 创建打卡记录
//!     let checkin_data = CheckinRecordCreateData {
//!         employee_id: "emp_001".to_string(),
//!         checkin_type: CheckinType::OnDuty,
//!         checkin_time: "2024-01-15T09:00:00Z".to_string(),
//!         location: Some(CheckinLocation {
//!             address: "北京市朝阳区xxx大厦".to_string(),
//!             latitude: 39.9042,
//!             longitude: 116.4074,
//!             ..Default::default()
//!         }),
//!         ..Default::default()
//!     };
//!
//!     let create_response = client.ehr.v1.attendance
//!         .create_checkin_record_builder()
//!         .checkin_data(checkin_data)
//!         .execute(&client.ehr.v1.attendance)
//!         .await?;
//!
//!     println!("打卡记录创建成功，ID: {}", create_response.data.record_id);
//!
//!     // 获取考勤统计
//!     let stats_response = client.ehr.v1.attendance
//!         .get_attendance_statistics_builder()
//!         .employee_id("emp_001")
//!         .start_date("2024-01-01")
//!         .end_date("2024-01-31")
//!         .execute(&client.ehr.v1.attendance)
//!         .await?;
//!
//!     println!("出勤天数: {}", stats_response.data.statistics.work_days);
//!     println!("迟到次数: {}", stats_response.data.statistics.late_count);
//!
//!     Ok(())
//! }
//! ```

use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    SDKResult,
};
use open_lark_core::core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};

/// 考勤管理服务
///
/// 提供完整的企业考勤管理功能，支持考勤记录管理、打卡处理、统计分析等。
/// 适用于各种考勤场景，包括办公室考勤、外勤管理、弹性工作制等。
///
/// # 核心功能
///
/// - **考勤记录管理**: 员工考勤记录的查询、创建、更新、删除
/// - **打卡记录处理**: 上班打卡、下班打卡、外勤打卡等记录管理
/// - **异常处理**: 迟到、早退、缺卡等异常情况的识别和处理
/// - **统计分析**: 考勤数据统计、出勤率分析、异常统计
/// - **请假关联**: 请假记录与考勤的关联处理
/// - **规则配置**: 考勤规则、工作时间、弹性制度等配置管理
///
/// # 使用示例
///
/// ```rust
/// use open_lark::prelude::*;
/// use open_lark::service::ehr::v1::attendance::AttendanceService;
///
/// let config = Config::new("app_id", "app_secret");
/// let service = AttendanceService::new(config);
/// ```
#[derive(Debug, Clone)]
pub struct AttendanceService {
    pub config: Config,
}

impl AttendanceService {
    /// 创建考勤服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::AttendanceService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = AttendanceService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取员工考勤记录
    ///
    /// 根据员工ID和日期范围获取考勤记录列表，支持分页查询。
    /// 返回指定时间段内的所有考勤记录，包括正常出勤、异常情况等。
    ///
    /// # API文档
    ///
    /// 根据员工ID和日期范围查询考勤记录。
    /// 支持多种过滤条件，返回详细的考勤信息和统计数据。
    ///
    /// # 参数
    ///
    /// * `employee_id` - 员工ID
    /// * `request` - 获取考勤记录的请求参数，包含日期范围和过滤条件
    ///
    /// # 返回值
    ///
    /// 返回考勤记录列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let request = GetAttendanceRecordsRequest {
    ///     start_date: "2024-01-01".to_string(),
    ///     end_date: "2024-01-31".to_string(),
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     attendance_status: None,
    /// };
    ///
    /// let response = client.ehr.v1.attendance
    ///     .get_attendance_records("emp_001", &request).await?;
    /// println!("获取到 {} 条考勤记录", response.data.items.len());
    /// ```
    pub async fn get_attendance_records(
        &self,
        employee_id: &str,
        request: &GetAttendanceRecordsRequest,
    ) -> SDKResult<BaseResponse<GetAttendanceRecordsResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("employee_id", employee_id.to_string());
        query_params.insert("start_date", request.start_date.clone());
        query_params.insert("end_date", request.end_date.clone());

        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            query_params.insert("page_token", page_token.clone());
        }
        if let Some(attendance_status) = &request.attendance_status {
            query_params.insert("attendance_status", attendance_status.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/ehr/v1/attendance/records".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 批量获取员工考勤记录
    ///
    /// 根据员工ID列表批量获取多个员工的考勤记录。
    /// 适用于团队考勤查询、部门考勤统计等场景。
    ///
    /// # API文档
    ///
    /// 根据员工ID列表批量获取考勤记录。
    /// 最多支持50个员工ID的批量查询，提高查询效率。
    ///
    /// # 参数
    ///
    /// * `request` - 批量获取考勤记录的请求参数
    ///
    /// # 返回值
    ///
    /// 返回批量员工的考勤记录列表和统计信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let request = BatchGetAttendanceRecordsRequest {
    ///     employee_ids: vec!["emp_001".to_string(), "emp_002".to_string()],
    ///     start_date: "2024-01-01".to_string(),
    ///     end_date: "2024-01-31".to_string(),
    ///     page_size: Some(50),
    /// };
    ///
    /// let response = client.ehr.v1.attendance
    ///     .batch_get_attendance_records(&request).await?;
    /// println!("批量获取到 {} 条记录", response.data.total_records);
    /// ```
    pub async fn batch_get_attendance_records(
        &self,
        request: &BatchGetAttendanceRecordsRequest,
    ) -> SDKResult<BaseResponse<BatchGetAttendanceRecordsResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/ehr/v1/attendance/records/batch_get".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 创建打卡记录
    ///
    /// 为员工创建打卡记录，支持上班打卡、下班打卡、外勤打卡等类型。
    /// 可以包含位置信息、设备信息、备注等详细数据。
    ///
    /// # API文档
    ///
    /// 创建新的打卡记录，支持多种打卡类型。
    /// 系统会自动验证打卡时间和规则，记录位置信息。
    ///
    /// # 参数
    ///
    /// * `request` - 创建打卡记录的请求参数
    ///
    /// # 返回值
    ///
    /// 返回创建成功的打卡记录信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let checkin_data = CheckinRecordCreateData {
    ///     employee_id: "emp_001".to_string(),
    ///     checkin_type: CheckinType::OnDuty,
    ///     checkin_time: "2024-01-15T09:00:00Z".to_string(),
    ///     location: Some(CheckinLocation {
    ///         address: "北京市朝阳区xxx大厦".to_string(),
    ///         latitude: 39.9042,
    ///         longitude: 116.4074,
    ///         ..Default::default()
    ///     }),
    ///     device_info: Some("iPhone 15 Pro".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let request = CreateCheckinRecordRequest {
    ///     checkin_data,
    /// };
    ///
    /// let response = client.ehr.v1.attendance
    ///     .create_checkin_record(&request).await?;
    /// println!("打卡记录创建成功，ID: {}", response.data.record_id);
    /// ```
    pub async fn create_checkin_record(
        &self,
        request: &CreateCheckinRecordRequest,
    ) -> SDKResult<BaseResponse<CreateCheckinRecordResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/ehr/v1/attendance/checkins".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 更新打卡记录
    ///
    /// 更新已存在的打卡记录信息，支持修正打卡时间、位置、备注等。
    /// 通常用于处理打卡异常或数据修正的情况。
    ///
    /// # API文档
    ///
    /// 修改打卡记录信息，支持更新时间和备注等字段。
    /// 系统会记录修改历史，确保数据可追溯。
    ///
    /// # 参数
    ///
    /// * `record_id` - 打卡记录ID
    /// * `request` - 更新打卡记录的请求参数
    ///
    /// # 返回值
    ///
    /// 返回更新后的打卡记录信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let update_data = CheckinRecordUpdateData {
    ///     checkin_time: Some("2024-01-15T08:58:00Z".to_string()),
    ///     remarks: Some("修正打卡时间".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let request = UpdateCheckinRecordRequest {
    ///     checkin_data: update_data,
    /// };
    ///
    /// let response = client.ehr.v1.attendance
    ///     .update_checkin_record("checkin_001", &request).await?;
    /// println!("打卡记录更新成功");
    /// ```
    pub async fn update_checkin_record(
        &self,
        record_id: &str,
        request: &UpdateCheckinRecordRequest,
    ) -> SDKResult<BaseResponse<UpdateCheckinRecordResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path: format!("/open-apis/ehr/v1/attendance/checkins/{}", record_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 删除打卡记录
    ///
    /// 删除指定的打卡记录，删除操作不可逆。
    /// 通常用于处理重复打卡或错误记录的情况。
    ///
    /// # API文档
    ///
    /// 删除打卡记录，操作不可逆。
    /// 删除前请确认记录确实需要删除，相关统计将重新计算。
    ///
    /// # 参数
    ///
    /// * `record_id` - 要删除的打卡记录ID
    ///
    /// # 返回值
    ///
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let response = client.ehr.v1.attendance
    ///     .delete_checkin_record("checkin_001").await?;
    /// println!("打卡记录删除成功");
    /// ```
    pub async fn delete_checkin_record(
        &self,
        record_id: &str,
    ) -> SDKResult<BaseResponse<DeleteCheckinRecordResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: format!("/open-apis/ehr/v1/attendance/checkins/{}", record_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取考勤统计信息
    ///
    /// 获取指定员工或部门的考勤统计数据，包括出勤天数、异常次数、工作时长等。
    /// 支持按不同时间段进行统计分析。
    ///
    /// # API文档
    ///
    /// 获取考勤统计信息，支持按员工或部门统计。
    /// 返回详细的考勤分析数据和趋势信息。
    ///
    /// # 参数
    ///
    /// * `request` - 获取考勤统计的请求参数
    ///
    /// # 返回值
    ///
    /// 返回详细的考勤统计信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let request = GetAttendanceStatisticsRequest {
    ///     employee_id: Some("emp_001".to_string()),
    ///     department_id: None,
    ///     start_date: "2024-01-01".to_string(),
    ///     end_date: "2024-01-31".to_string(),
    ///     statistics_type: StatisticsType::Monthly,
    /// };
    ///
    /// let response = client.ehr.v1.attendance
    ///     .get_attendance_statistics(&request).await?;
    /// println!("出勤天数: {}", response.data.statistics.work_days);
    /// println!("迟到次数: {}", response.data.statistics.late_count);
    /// println!("平均工作时长: {:.1}小时", response.data.statistics.avg_work_hours);
    /// ```
    pub async fn get_attendance_statistics(
        &self,
        request: &GetAttendanceStatisticsRequest,
    ) -> SDKResult<BaseResponse<GetAttendanceStatisticsResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("start_date", request.start_date.clone());
        query_params.insert("end_date", request.end_date.clone());
        query_params.insert("statistics_type", format!("{:?}", request.statistics_type));

        if let Some(employee_id) = &request.employee_id {
            query_params.insert("employee_id", employee_id.clone());
        }
        if let Some(department_id) = &request.department_id {
            query_params.insert("department_id", department_id.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/ehr/v1/attendance/statistics".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取考勤异常记录
    ///
    /// 获取考勤异常记录列表，包括迟到、早退、缺卡等异常情况。
    /// 支持按异常类型、严重程度等条件筛选。
    ///
    /// # API文档
    ///
    /// 获取考勤异常记录，支持多种筛选条件。
    /// 返回异常详情和处理建议，支持批量处理。
    ///
    /// # 参数
    ///
    /// * `request` - 获取考勤异常记录的请求参数
    ///
    /// # 返回值
    ///
    /// 返回考勤异常记录列表和处理建议
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let request = GetAttendanceExceptionsRequest {
    ///     employee_id: Some("emp_001".to_string()),
    ///     start_date: "2024-01-01".to_string(),
    ///     end_date: "2024-01-31".to_string(),
    ///     exception_types: Some(vec!["late".to_string(), "absent".to_string()]),
    ///     page_size: Some(20),
    /// };
    ///
    /// let response = client.ehr.v1.attendance
    ///     .get_attendance_exceptions(&request).await?;
    /// println!("发现 {} 条异常记录", response.data.items.len());
    /// ```
    pub async fn get_attendance_exceptions(
        &self,
        request: &GetAttendanceExceptionsRequest,
    ) -> SDKResult<BaseResponse<GetAttendanceExceptionsResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("start_date", request.start_date.clone());
        query_params.insert("end_date", request.end_date.clone());

        if let Some(employee_id) = &request.employee_id {
            query_params.insert("employee_id", employee_id.clone());
        }
        if let Some(department_id) = &request.department_id {
            query_params.insert("department_id", department_id.clone());
        }
        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/ehr/v1/attendance/exceptions".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 处理考勤异常
    ///
    /// 对考勤异常记录进行处理，包括确认异常、豁免处理、备注说明等。
    /// 支持批量处理多条异常记录。
    ///
    /// # API文档
    ///
    /// 处理考勤异常记录，支持多种处理方式。
    /// 系统会记录处理历史和处理人信息。
    ///
    /// # 参数
    ///
    /// * `request` - 处理考勤异常的请求参数
    ///
    /// # 返回值
    ///
    /// 返回异常处理的结果和状态
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let process_data = AttendanceExceptionProcessData {
    ///     exception_id: "exception_001".to_string(),
    ///     action: ExceptionAction::Confirm,
    ///     remarks: Some("因客户会议迟到，已申请豁免".to_string()),
    ///     process_time: "2024-01-16T10:00:00Z".to_string(),
    /// };
    ///
    /// let request = ProcessAttendanceExceptionRequest {
    ///     process_data,
    /// };
    ///
    /// let response = client.ehr.v1.attendance
    ///     .process_attendance_exception(&request).await?;
    /// println!("异常处理完成，状态: {}", response.data.status);
    /// ```
    pub async fn process_attendance_exception(
        &self,
        request: &ProcessAttendanceExceptionRequest,
    ) -> SDKResult<BaseResponse<ProcessAttendanceExceptionResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/ehr/v1/attendance/exceptions/process".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取考勤报表
    ///
    /// 生成各种类型的考勤报表，包括日报、周报、月报等。
    /// 支持自定义报表格式和数据维度。
    ///
    /// # API文档
    ///
    /// 生成考勤报表，支持多种报表类型。
    /// 返回结构化的报表数据和图表信息。
    ///
    /// # 参数
    ///
    /// * `request` - 生成考勤报表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回考勤报表数据和统计信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let request = GetAttendanceReportRequest {
    ///     report_type: ReportType::Monthly,
    ///     employee_id: Some("emp_001".to_string()),
    ///     department_id: None,
    ///     start_date: "2024-01-01".to_string(),
    ///     end_date: "2024-01-31".to_string(),
    ///     format: ReportFormat::Json,
    /// };
    ///
    /// let response = client.ehr.v1.attendance
    ///     .get_attendance_report(&request).await?;
    /// println!("报表生成完成，类型: {}", response.data.report_type);
    /// ```
    pub async fn get_attendance_report(
        &self,
        request: &GetAttendanceReportRequest,
    ) -> SDKResult<BaseResponse<GetAttendanceReportResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("report_type", format!("{:?}", request.report_type));
        query_params.insert("start_date", request.start_date.clone());
        query_params.insert("end_date", request.end_date.clone());
        query_params.insert("format", format!("{:?}", request.format));

        if let Some(employee_id) = &request.employee_id {
            query_params.insert("employee_id", employee_id.clone());
        }
        if let Some(department_id) = &request.department_id {
            query_params.insert("department_id", department_id.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/ehr/v1/attendance/reports".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    // ==================== Builder模式实现 ====================

    /// 获取员工考勤记录构建器
    ///
    /// 提供流式API来构建获取考勤记录的请求参数。
    /// 支持链式调用，方便配置查询条件。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let builder = client.ehr.v1.attendance
    ///     .get_attendance_records_builder("emp_001")
    ///     .start_date("2024-01-01")
    ///     .end_date("2024-01-31")
    ///     .page_size(50);
    ///
    /// let response = builder.execute(&client.ehr.v1.attendance).await?;
    /// ```
    pub fn get_attendance_records_builder(&self, employee_id: &str) -> GetAttendanceRecordsBuilder {
        GetAttendanceRecordsBuilder::new(employee_id)
    }

    /// 批量获取考勤记录构建器
    ///
    /// 提供流式API来构建批量获取考勤记录的请求参数。
    /// 支持链式调用，方便配置批量查询条件。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let builder = client.ehr.v1.attendance
    ///     .batch_get_attendance_records_builder()
    ///     .employee_ids(vec!["emp_001".to_string(), "emp_002".to_string()])
    ///     .start_date("2024-01-01")
    ///     .end_date("2024-01-31");
    ///
    /// let response = builder.execute(&client.ehr.v1.attendance).await?;
    /// ```
    pub fn batch_get_attendance_records_builder(&self) -> BatchGetAttendanceRecordsBuilder {
        BatchGetAttendanceRecordsBuilder::new()
    }

    /// 创建打卡记录构建器
    ///
    /// 提供流式API来构建创建打卡记录的请求参数。
    /// 支持链式调用，方便构建复杂的打卡记录创建请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let checkin_data = CheckinRecordCreateData {
    ///     employee_id: "emp_001".to_string(),
    ///     checkin_type: CheckinType::OnDuty,
    ///     checkin_time: "2024-01-15T09:00:00Z".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let builder = client.ehr.v1.attendance
    ///     .create_checkin_record_builder()
    ///     .checkin_data(checkin_data);
    ///
    /// let response = builder.execute(&client.ehr.v1.attendance).await?;
    /// ```
    pub fn create_checkin_record_builder(&self) -> CreateCheckinRecordBuilder {
        CreateCheckinRecordBuilder::new()
    }

    /// 更新打卡记录构建器
    ///
    /// 提供流式API来构建更新打卡记录的请求参数。
    /// 支持链式调用，方便构建打卡记录更新请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let update_data = CheckinRecordUpdateData {
    ///     checkin_time: Some("2024-01-15T08:58:00Z".to_string()),
    ///     remarks: Some("修正打卡时间".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let builder = client.ehr.v1.attendance
    ///     .update_checkin_record_builder("checkin_001")
    ///     .checkin_data(update_data);
    ///
    /// let response = builder.execute(&client.ehr.v1.attendance).await?;
    /// ```
    pub fn update_checkin_record_builder(&self, record_id: &str) -> UpdateCheckinRecordBuilder {
        UpdateCheckinRecordBuilder::new(record_id)
    }

    /// 删除打卡记录构建器
    ///
    /// 提供流式API来构建删除打卡记录的请求参数。
    /// 支持链式调用，方便配置删除参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let builder = client.ehr.v1.attendance
    ///     .delete_checkin_record_builder("checkin_001");
    ///
    /// let response = builder.execute(&client.ehr.v1.attendance).await?;
    /// ```
    pub fn delete_checkin_record_builder(&self, record_id: &str) -> DeleteCheckinRecordBuilder {
        DeleteCheckinRecordBuilder::new(record_id)
    }

    /// 获取考勤统计信息构建器
    ///
    /// 提供流式API来构建获取考勤统计信息的请求参数。
    /// 支持链式调用，方便配置统计条件和维度。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let builder = client.ehr.v1.attendance
    ///     .get_attendance_statistics_builder()
    ///     .employee_id("emp_001")
    ///     .start_date("2024-01-01")
    ///     .end_date("2024-01-31")
    ///     .statistics_type(StatisticsType::Monthly);
    ///
    /// let response = builder.execute(&client.ehr.v1.attendance).await?;
    /// ```
    pub fn get_attendance_statistics_builder(&self) -> GetAttendanceStatisticsBuilder {
        GetAttendanceStatisticsBuilder::new()
    }

    /// 获取考勤异常记录构建器
    ///
    /// 提供流式API来构建获取考勤异常记录的请求参数。
    /// 支持链式调用，方便配置异常查询条件。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let builder = client.ehr.v1.attendance
    ///     .get_attendance_exceptions_builder()
    ///     .employee_id("emp_001")
    ///     .start_date("2024-01-01")
    ///     .end_date("2024-01-31")
    ///     .page_size(20);
    ///
    /// let response = builder.execute(&client.ehr.v1.attendance).await?;
    /// ```
    pub fn get_attendance_exceptions_builder(&self) -> GetAttendanceExceptionsBuilder {
        GetAttendanceExceptionsBuilder::new()
    }

    /// 处理考勤异常构建器
    ///
    /// 提供流式API来构建处理考勤异常的请求参数。
    /// 支持链式调用，方便配置异常处理参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let process_data = AttendanceExceptionProcessData {
    ///     exception_id: "exception_001".to_string(),
    ///     action: ExceptionAction::Confirm,
    ///     process_time: "2024-01-16T10:00:00Z".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let builder = client.ehr.v1.attendance
    ///     .process_attendance_exception_builder()
    ///     .process_data(process_data);
    ///
    /// let response = builder.execute(&client.ehr.v1.attendance).await?;
    /// ```
    pub fn process_attendance_exception_builder(&self) -> ProcessAttendanceExceptionBuilder {
        ProcessAttendanceExceptionBuilder::new()
    }

    /// 获取考勤报表构建器
    ///
    /// 提供流式API来构建获取考勤报表的请求参数。
    /// 支持链式调用，方便配置报表参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::ehr::v1::attendance::*;
    ///
    /// let builder = client.ehr.v1.attendance
    ///     .get_attendance_report_builder()
    ///     .report_type(ReportType::Monthly)
    ///     .employee_id("emp_001")
    ///     .start_date("2024-01-01")
    ///     .end_date("2024-01-31")
    ///     .format(ReportFormat::Json);
    ///
    /// let response = builder.execute(&client.ehr.v1.attendance).await?;
    /// ```
    pub fn get_attendance_report_builder(&self) -> GetAttendanceReportBuilder {
        GetAttendanceReportBuilder::new()
    }
}

// ==================== Builder结构体实现 ====================

/// 获取员工考勤记录构建器
#[derive(Debug, Clone)]
pub struct GetAttendanceRecordsBuilder {
    employee_id: String,
    request: GetAttendanceRecordsRequest,
}

impl GetAttendanceRecordsBuilder {
    /// 创建新的Builder实例
    pub fn new(employee_id: &str) -> Self {
        Self {
            employee_id: employee_id.to_string(),
            request: GetAttendanceRecordsRequest::default(),
        }
    }

    /// 设置开始日期
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.request.start_date = start_date.to_string();
        self
    }

    /// 设置结束日期
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.request.end_date = end_date.to_string();
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置考勤状态筛选
    pub fn attendance_status(mut self, attendance_status: &str) -> Self {
        self.request.attendance_status = Some(attendance_status.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, GetAttendanceRecordsRequest) {
        (self.employee_id, self.request)
    }
}

impl Default for GetAttendanceRecordsBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 标准实现
crate::impl_executable_builder!(
    GetAttendanceRecordsBuilder,
    AttendanceService,
    (String, GetAttendanceRecordsRequest),
    BaseResponse<GetAttendanceRecordsResponse>,
    get_attendance_records_with_tuple
);

/// 批量获取考勤记录构建器
#[derive(Debug, Clone)]
pub struct BatchGetAttendanceRecordsBuilder {
    request: BatchGetAttendanceRecordsRequest,
}

impl BatchGetAttendanceRecordsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: BatchGetAttendanceRecordsRequest::default(),
        }
    }

    /// 设置员工ID列表
    pub fn employee_ids(mut self, employee_ids: Vec<String>) -> Self {
        self.request.employee_ids = employee_ids;
        self
    }

    /// 设置开始日期
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.request.start_date = start_date.to_string();
        self
    }

    /// 设置结束日期
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.request.end_date = end_date.to_string();
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> BatchGetAttendanceRecordsRequest {
        self.request
    }
}

impl Default for BatchGetAttendanceRecordsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait - 标准实现
crate::impl_executable_builder!(
    BatchGetAttendanceRecordsBuilder,
    AttendanceService,
    BatchGetAttendanceRecordsRequest,
    BaseResponse<BatchGetAttendanceRecordsResponse>,
    batch_get_attendance_records_with_tuple
);

/// 创建打卡记录构建器
#[derive(Debug, Clone)]
pub struct CreateCheckinRecordBuilder {
    request: CreateCheckinRecordRequest,
}

impl CreateCheckinRecordBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: CreateCheckinRecordRequest::default(),
        }
    }

    /// 设置打卡数据
    pub fn checkin_data(mut self, checkin_data: CheckinRecordCreateData) -> Self {
        self.request.checkin_data = checkin_data;
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> CreateCheckinRecordRequest {
        self.request
    }
}

impl Default for CreateCheckinRecordBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
crate::impl_executable_builder!(
    CreateCheckinRecordBuilder,
    AttendanceService,
    CreateCheckinRecordRequest,
    BaseResponse<CreateCheckinRecordResponse>,
    create_checkin_record_with_tuple
);

/// 更新打卡记录构建器
#[derive(Debug, Clone)]
pub struct UpdateCheckinRecordBuilder {
    record_id: String,
    request: UpdateCheckinRecordRequest,
}

impl UpdateCheckinRecordBuilder {
    /// 创建新的Builder实例
    pub fn new(record_id: &str) -> Self {
        Self {
            record_id: record_id.to_string(),
            request: UpdateCheckinRecordRequest::default(),
        }
    }

    /// 设置打卡数据
    pub fn checkin_data(mut self, checkin_data: CheckinRecordUpdateData) -> Self {
        self.request.checkin_data = checkin_data;
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, UpdateCheckinRecordRequest) {
        (self.record_id, self.request)
    }
}

impl Default for UpdateCheckinRecordBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 标准实现
crate::impl_executable_builder!(
    UpdateCheckinRecordBuilder,
    AttendanceService,
    (String, UpdateCheckinRecordRequest),
    BaseResponse<UpdateCheckinRecordResponse>,
    update_checkin_record_with_tuple
);

/// 删除打卡记录构建器
#[derive(Debug, Clone)]
pub struct DeleteCheckinRecordBuilder {
    record_id: String,
}

impl DeleteCheckinRecordBuilder {
    /// 创建新的Builder实例
    pub fn new(record_id: &str) -> Self {
        Self {
            record_id: record_id.to_string(),
        }
    }

    /// 构建最终的请求对象
    pub fn build(self) -> String {
        self.record_id
    }
}

impl Default for DeleteCheckinRecordBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 标准实现
crate::impl_executable_builder!(
    DeleteCheckinRecordBuilder,
    AttendanceService,
    String,
    BaseResponse<DeleteCheckinRecordResponse>,
    delete_checkin_record_with_string
);

/// 获取考勤统计信息构建器
#[derive(Debug, Clone)]
pub struct GetAttendanceStatisticsBuilder {
    request: GetAttendanceStatisticsRequest,
}

impl GetAttendanceStatisticsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetAttendanceStatisticsRequest::default(),
        }
    }

    /// 设置员工ID
    pub fn employee_id(mut self, employee_id: &str) -> Self {
        self.request.employee_id = Some(employee_id.to_string());
        self
    }

    /// 设置部门ID
    pub fn department_id(mut self, department_id: &str) -> Self {
        self.request.department_id = Some(department_id.to_string());
        self
    }

    /// 设置开始日期
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.request.start_date = start_date.to_string();
        self
    }

    /// 设置结束日期
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.request.end_date = end_date.to_string();
        self
    }

    /// 设置统计类型
    pub fn statistics_type(mut self, statistics_type: StatisticsType) -> Self {
        self.request.statistics_type = statistics_type;
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetAttendanceStatisticsRequest {
        self.request
    }
}

impl Default for GetAttendanceStatisticsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
crate::impl_executable_builder!(
    GetAttendanceStatisticsBuilder,
    AttendanceService,
    GetAttendanceStatisticsRequest,
    BaseResponse<GetAttendanceStatisticsResponse>,
    get_attendance_statistics_with_tuple
);

/// 获取考勤异常记录构建器
#[derive(Debug, Clone)]
pub struct GetAttendanceExceptionsBuilder {
    request: GetAttendanceExceptionsRequest,
}

impl GetAttendanceExceptionsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetAttendanceExceptionsRequest::default(),
        }
    }

    /// 设置员工ID
    pub fn employee_id(mut self, employee_id: &str) -> Self {
        self.request.employee_id = Some(employee_id.to_string());
        self
    }

    /// 设置部门ID
    pub fn department_id(mut self, department_id: &str) -> Self {
        self.request.department_id = Some(department_id.to_string());
        self
    }

    /// 设置开始日期
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.request.start_date = start_date.to_string();
        self
    }

    /// 设置结束日期
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.request.end_date = end_date.to_string();
        self
    }

    /// 设置异常类型列表
    pub fn exception_types(mut self, exception_types: Vec<String>) -> Self {
        self.request.exception_types = Some(exception_types);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetAttendanceExceptionsRequest {
        self.request
    }
}

impl Default for GetAttendanceExceptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
crate::impl_executable_builder!(
    GetAttendanceExceptionsBuilder,
    AttendanceService,
    GetAttendanceExceptionsRequest,
    BaseResponse<GetAttendanceExceptionsResponse>,
    get_attendance_exceptions_with_tuple
);

/// 处理考勤异常构建器
#[derive(Debug, Clone)]
pub struct ProcessAttendanceExceptionBuilder {
    request: ProcessAttendanceExceptionRequest,
}

impl ProcessAttendanceExceptionBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: ProcessAttendanceExceptionRequest::default(),
        }
    }

    /// 设置处理数据
    pub fn process_data(mut self, process_data: AttendanceExceptionProcessData) -> Self {
        self.request.process_data = process_data;
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> ProcessAttendanceExceptionRequest {
        self.request
    }
}

impl Default for ProcessAttendanceExceptionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
crate::impl_executable_builder!(
    ProcessAttendanceExceptionBuilder,
    AttendanceService,
    ProcessAttendanceExceptionRequest,
    BaseResponse<ProcessAttendanceExceptionResponse>,
    process_attendance_exception_with_tuple
);

/// 获取考勤报表构建器
#[derive(Debug, Clone)]
pub struct GetAttendanceReportBuilder {
    request: GetAttendanceReportRequest,
}

impl GetAttendanceReportBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetAttendanceReportRequest::default(),
        }
    }

    /// 设置报表类型
    pub fn report_type(mut self, report_type: ReportType) -> Self {
        self.request.report_type = report_type;
        self
    }

    /// 设置员工ID
    pub fn employee_id(mut self, employee_id: &str) -> Self {
        self.request.employee_id = Some(employee_id.to_string());
        self
    }

    /// 设置部门ID
    pub fn department_id(mut self, department_id: &str) -> Self {
        self.request.department_id = Some(department_id.to_string());
        self
    }

    /// 设置开始日期
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.request.start_date = start_date.to_string();
        self
    }

    /// 设置结束日期
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.request.end_date = end_date.to_string();
        self
    }

    /// 设置报表格式
    pub fn format(mut self, format: ReportFormat) -> Self {
        self.request.format = format;
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetAttendanceReportRequest {
        self.request
    }
}

impl Default for GetAttendanceReportBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
crate::impl_executable_builder!(
    GetAttendanceReportBuilder,
    AttendanceService,
    GetAttendanceReportRequest,
    BaseResponse<GetAttendanceReportResponse>,
    get_attendance_report_with_tuple
);

// 为AttendanceService实现辅助方法，处理Builder的参数
impl AttendanceService {
    async fn get_attendance_records_with_tuple(
        &self,
        params: (String, GetAttendanceRecordsRequest),
    ) -> SDKResult<BaseResponse<GetAttendanceRecordsResponse>> {
        self.get_attendance_records(&params.0, &params.1).await
    }

    async fn batch_get_attendance_records_with_tuple(
        &self,
        params: BatchGetAttendanceRecordsRequest,
    ) -> SDKResult<BaseResponse<BatchGetAttendanceRecordsResponse>> {
        self.batch_get_attendance_records(&params).await
    }

    async fn create_checkin_record_with_tuple(
        &self,
        params: CreateCheckinRecordRequest,
    ) -> SDKResult<BaseResponse<CreateCheckinRecordResponse>> {
        self.create_checkin_record(&params).await
    }

    async fn get_attendance_statistics_with_tuple(
        &self,
        params: GetAttendanceStatisticsRequest,
    ) -> SDKResult<BaseResponse<GetAttendanceStatisticsResponse>> {
        self.get_attendance_statistics(&params).await
    }

    async fn get_attendance_exceptions_with_tuple(
        &self,
        params: GetAttendanceExceptionsRequest,
    ) -> SDKResult<BaseResponse<GetAttendanceExceptionsResponse>> {
        self.get_attendance_exceptions(&params).await
    }

    async fn process_attendance_exception_with_tuple(
        &self,
        params: ProcessAttendanceExceptionRequest,
    ) -> SDKResult<BaseResponse<ProcessAttendanceExceptionResponse>> {
        self.process_attendance_exception(&params).await
    }

    async fn get_attendance_report_with_tuple(
        &self,
        params: GetAttendanceReportRequest,
    ) -> SDKResult<BaseResponse<GetAttendanceReportResponse>> {
        self.get_attendance_report(&params).await
    }

    async fn update_checkin_record_with_tuple(
        &self,
        params: (String, UpdateCheckinRecordRequest),
    ) -> SDKResult<BaseResponse<UpdateCheckinRecordResponse>> {
        self.update_checkin_record(&params.0, &params.1).await
    }

    async fn delete_checkin_record_with_string(
        &self,
        record_id: String,
    ) -> SDKResult<BaseResponse<DeleteCheckinRecordResponse>> {
        self.delete_checkin_record(&record_id).await
    }
}

// ==================== 数据模型 ====================

/// 获取考勤记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceRecordsRequest {
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 考勤状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendance_status: Option<String>,
}

impl Default for GetAttendanceRecordsRequest {
    fn default() -> Self {
        Self {
            start_date: String::new(),
            end_date: String::new(),
            page_size: None,
            page_token: None,
            attendance_status: None,
        }
    }
}

/// 获取考勤记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceRecordsResponse {
    /// 考勤记录列表
    pub items: Vec<AttendanceRecord>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

/// 考勤记录列表响应类型
pub type AttendanceListResponse = BaseResponse<GetAttendanceRecordsResponse>;

impl ApiResponseTrait for GetAttendanceRecordsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 批量获取考勤记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetAttendanceRecordsRequest {
    /// 员工ID列表
    pub employee_ids: Vec<String>,
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl Default for BatchGetAttendanceRecordsRequest {
    fn default() -> Self {
        Self {
            employee_ids: vec![],
            start_date: String::new(),
            end_date: String::new(),
            page_size: None,
        }
    }
}

/// 批量获取考勤记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetAttendanceRecordsResponse {
    /// 考勤记录列表
    pub items: Vec<AttendanceRecord>,
    /// 总记录数
    pub total_records: i32,
    /// 员工统计
    pub employee_statistics: Vec<EmployeeAttendanceSummary>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 创建打卡记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckinRecordRequest {
    /// 打卡数据
    pub checkin_data: CheckinRecordCreateData,
}

impl Default for CreateCheckinRecordRequest {
    fn default() -> Self {
        Self {
            checkin_data: CheckinRecordCreateData::default(),
        }
    }
}

/// 创建打卡记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckinRecordResponse {
    /// 打卡记录ID
    pub record_id: String,
    /// 打卡记录信息
    pub record: CheckinRecord,
}

/// 更新打卡记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckinRecordRequest {
    /// 打卡数据
    pub checkin_data: CheckinRecordUpdateData,
}

impl Default for UpdateCheckinRecordRequest {
    fn default() -> Self {
        Self {
            checkin_data: CheckinRecordUpdateData::default(),
        }
    }
}

/// 更新打卡记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckinRecordResponse {
    /// 打卡记录信息
    pub record: CheckinRecord,
}

/// 删除打卡记录响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteCheckinRecordResponse {}

/// 获取考勤统计信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceStatisticsRequest {
    /// 员工ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 统计类型
    pub statistics_type: StatisticsType,
}

impl Default for GetAttendanceStatisticsRequest {
    fn default() -> Self {
        Self {
            employee_id: None,
            department_id: None,
            start_date: String::new(),
            end_date: String::new(),
            statistics_type: StatisticsType::Daily,
        }
    }
}

/// 获取考勤统计信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceStatisticsResponse {
    /// 统计信息
    pub statistics: AttendanceStatistics,
}

/// 获取考勤异常记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceExceptionsRequest {
    /// 员工ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 异常类型列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_types: Option<Vec<String>>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for GetAttendanceExceptionsRequest {
    fn default() -> Self {
        Self {
            employee_id: None,
            department_id: None,
            start_date: String::new(),
            end_date: String::new(),
            exception_types: None,
            page_size: None,
            page_token: None,
        }
    }
}

/// 获取考勤异常记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceExceptionsResponse {
    /// 异常记录列表
    pub items: Vec<AttendanceException>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

/// 处理考勤异常请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessAttendanceExceptionRequest {
    /// 处理数据
    pub process_data: AttendanceExceptionProcessData,
}

impl Default for ProcessAttendanceExceptionRequest {
    fn default() -> Self {
        Self {
            process_data: AttendanceExceptionProcessData::default(),
        }
    }
}

/// 处理考勤异常响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessAttendanceExceptionResponse {
    /// 处理状态
    pub status: String,
    /// 处理时间
    pub process_time: String,
    /// 处理人
    pub processor: String,
}

/// 获取考勤报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceReportRequest {
    /// 报表类型
    pub report_type: ReportType,
    /// 员工ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 报表格式
    pub format: ReportFormat,
}

impl Default for GetAttendanceReportRequest {
    fn default() -> Self {
        Self {
            report_type: ReportType::Daily,
            employee_id: None,
            department_id: None,
            start_date: String::new(),
            end_date: String::new(),
            format: ReportFormat::Json,
        }
    }
}

/// 获取考勤报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceReportResponse {
    /// 报表类型
    pub report_type: String,
    /// 报表数据
    pub report_data: serde_json::Value,
    /// 生成时间
    pub generated_at: String,
}

// ==================== 枚举类型定义 ====================

/// 打卡类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckinType {
    /// 上班打卡
    OnDuty,
    /// 下班打卡
    OffDuty,
    /// 外勤打卡
    FieldWork,
    /// 加班打卡
    Overtime,
}

impl Default for CheckinType {
    fn default() -> Self {
        CheckinType::OnDuty
    }
}

/// 统计类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatisticsType {
    /// 日报
    Daily,
    /// 周报
    Weekly,
    /// 月报
    Monthly,
    /// 季报
    Quarterly,
    /// 年报
    Yearly,
}

impl Default for StatisticsType {
    fn default() -> Self {
        StatisticsType::Daily
    }
}

/// 异常处理动作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExceptionAction {
    /// 确认异常
    Confirm,
    /// 豁免处理
    Waive,
    /// 需要说明
    RequireExplanation,
    /// 批准休假
    ApproveLeave,
}

impl Default for ExceptionAction {
    fn default() -> Self {
        ExceptionAction::Confirm
    }
}

/// 报表类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportType {
    /// 日报
    Daily,
    /// 周报
    Weekly,
    /// 月报
    Monthly,
    /// 出勤统计
    AttendanceSummary,
    /// 异常统计
    ExceptionSummary,
    /// 加班统计
    OvertimeSummary,
}

impl Default for ReportType {
    fn default() -> Self {
        ReportType::Daily
    }
}

/// 报表格式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportFormat {
    /// JSON格式
    Json,
    /// Excel格式
    Excel,
    /// PDF格式
    Pdf,
}

impl Default for ReportFormat {
    fn default() -> Self {
        ReportFormat::Json
    }
}

// ==================== 数据结构体定义 ====================

/// 考勤记录
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttendanceRecord {
    /// 记录ID
    pub record_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 考勤日期
    pub attendance_date: String,
    /// 上班打卡时间
    pub check_in_time: Option<String>,
    /// 下班打卡时间
    pub check_out_time: Option<String>,
    /// 工作时长（小时）
    pub work_hours: Option<f64>,
    /// 考勤状态
    pub status: AttendanceStatus,
    /// 迟到分钟数
    pub late_minutes: Option<i32>,
    /// 早退分钟数
    pub early_leave_minutes: Option<i32>,
    /// 请假类型
    pub leave_type: Option<String>,
    /// 备注
    pub remarks: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 考勤状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttendanceStatus {
    /// 正常
    Normal,
    /// 迟到
    Late,
    /// 早退
    Early,
    /// 缺卡
    Absent,
    /// 请假
    Leave,
    /// 外勤
    FieldWork,
}

impl Default for AttendanceStatus {
    fn default() -> Self {
        AttendanceStatus::Normal
    }
}

/// 打卡记录
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckinRecord {
    /// 记录ID
    pub record_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 打卡类型
    pub checkin_type: CheckinType,
    /// 打卡时间
    pub checkin_time: String,
    /// 打卡位置
    pub location: Option<CheckinLocation>,
    /// 设备信息
    pub device_info: Option<String>,
    /// 打卡照片
    pub photo_url: Option<String>,
    /// 备注
    pub remarks: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 打卡位置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckinLocation {
    /// 地址
    pub address: String,
    /// 纬度
    pub latitude: f64,
    /// 经度
    pub longitude: f64,
    /// 精度（米）
    pub accuracy: Option<f64>,
    /// 位置描述
    pub description: Option<String>,
}

/// 打卡记录创建数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckinRecordCreateData {
    /// 员工ID
    pub employee_id: String,
    /// 打卡类型
    pub checkin_type: CheckinType,
    /// 打卡时间
    pub checkin_time: String,
    /// 打卡位置
    pub location: Option<CheckinLocation>,
    /// 设备信息
    pub device_info: Option<String>,
    /// 打卡照片
    pub photo_url: Option<String>,
    /// 备注
    pub remarks: Option<String>,
}

/// 打卡记录更新数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckinRecordUpdateData {
    /// 打卡时间
    pub checkin_time: Option<String>,
    /// 打卡位置
    pub location: Option<CheckinLocation>,
    /// 设备信息
    pub device_info: Option<String>,
    /// 打卡照片
    pub photo_url: Option<String>,
    /// 备注
    pub remarks: Option<String>,
}

/// 员工考勤汇总
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmployeeAttendanceSummary {
    /// 员工ID
    pub employee_id: String,
    /// 员工姓名
    pub employee_name: String,
    /// 出勤天数
    pub work_days: i32,
    /// 迟到次数
    pub late_count: i32,
    /// 早退次数
    pub early_leave_count: i32,
    /// 缺卡次数
    pub absent_count: i32,
    /// 平均工作时长
    pub avg_work_hours: f64,
    /// 总工作时长
    pub total_work_hours: f64,
}

/// 考勤统计信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttendanceStatistics {
    /// 统计范围（员工或部门）
    pub scope: String,
    /// 统计周期
    pub period: String,
    /// 出勤天数
    pub work_days: i32,
    /// 应出勤天数
    pub expected_work_days: i32,
    /// 出勤率
    pub attendance_rate: f64,
    /// 迟到次数
    pub late_count: i32,
    /// 早退次数
    pub early_leave_count: i32,
    /// 缺卡次数
    pub absent_count: i32,
    /// 请假天数
    pub leave_days: i32,
    /// 平均工作时长
    pub avg_work_hours: f64,
    /// 总工作时长
    pub total_work_hours: f64,
    /// 加班时长
    pub overtime_hours: f64,
    /// 异常统计
    pub exception_summary: Option<ExceptionSummary>,
}

/// 异常汇总
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExceptionSummary {
    /// 迟到统计
    pub late_summary: LateSummary,
    /// 早退统计
    pub early_leave_summary: EarlyLeaveSummary,
    /// 缺卡统计
    pub absent_summary: AbsentSummary,
}

/// 迟到统计
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LateSummary {
    /// 迟到次数
    pub count: i32,
    /// 总迟到分钟数
    pub total_minutes: i32,
    /// 平均迟到分钟数
    pub avg_minutes: f64,
}

/// 早退统计
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EarlyLeaveSummary {
    /// 早退次数
    pub count: i32,
    /// 总早退分钟数
    pub total_minutes: i32,
    /// 平均早退分钟数
    pub avg_minutes: f64,
}

/// 缺卡统计
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AbsentSummary {
    /// 缺卡次数
    pub count: i32,
    /// 上班缺卡次数
    pub on_duty_count: i32,
    /// 下班缺卡次数
    pub off_duty_count: i32,
}

/// 考勤异常
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttendanceException {
    /// 异常ID
    pub exception_id: String,
    /// 员工ID
    pub employee_id: String,
    /// 异常日期
    pub exception_date: String,
    /// 异常类型
    pub exception_type: String,
    /// 异常描述
    pub description: String,
    /// 严重程度
    pub severity: String,
    /// 处理状态
    pub process_status: String,
    /// 处理时间
    pub process_time: Option<String>,
    /// 处理人
    pub processor: Option<String>,
    /// 处理备注
    pub process_remarks: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
}

/// 考勤异常处理数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttendanceExceptionProcessData {
    /// 异常ID
    pub exception_id: String,
    /// 处理动作
    pub action: ExceptionAction,
    /// 处理备注
    pub remarks: Option<String>,
    /// 处理时间
    pub process_time: String,
    /// 处理人
    pub processor: String,
}

// ==================== ApiResponseTrait实现 ====================

impl ApiResponseTrait for BatchGetAttendanceRecordsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateCheckinRecordResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdateCheckinRecordResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteCheckinRecordResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetAttendanceStatisticsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetAttendanceExceptionsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for ProcessAttendanceExceptionResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetAttendanceReportResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

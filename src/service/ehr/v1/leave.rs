//! Leave v1 - 请假管理API
//!
//! 提供完整的企业请假管理功能，包括：
//! - 请假申请的创建、查询、更新和取消
//! - 请假审批流程和状态管理
//! - 请假余额查询和统计
//! - 请假规则配置和管理
//! - 请假统计分析和报表
//! - 与考勤系统的数据集成
//!
//! # 示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::ehr::v1::leave::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 创建请假申请
//!     let response = client.ehr.v1.leave
//!         .create_leave_builder()
//!         .employee_id("emp_001")
//!         .leave_type(LeaveType::Annual)
//!         .start_time("2024-01-15T09:00:00Z")
//!         .end_time("2024-01-17T18:00:00Z")
//!         .reason("家庭聚会")
//!         .execute(&client.ehr.v1.leave)
//!         .await?;
//!
//!     println!("请假申请创建成功，ID: {}", response.data.leave_id);
//!
//!     // 查询请假记录
//!     let records_response = client.ehr.v1.leave
//!         .query_leave_records_builder()
//!         .employee_id("emp_001")
//!         .status(LeaveStatus::Approved)
//!         .page_size(20)
//!         .execute(&client.ehr.v1.leave)
//!         .await?;
//!
//!     println!("获取到 {} 条请假记录", records_response.data.items.len());
//!
//!     // 查询请假余额
//!     let balance_response = client.ehr.v1.leave
//!         .query_leave_balance_builder("emp_001")
//!         .leave_type(LeaveType::Annual)
//!         .year(2024)
//!         .execute(&client.ehr.v1.leave)
//!         .await?;
//!
//!     println!("年假余额: {} 天", balance_response.data.remaining_days);
//!
//!     // 获取请假统计
//!     let stats_response = client.ehr.v1.leave
//!         .get_leave_statistics_builder()
//!         .employee_id("emp_001")
//!         .year(2024)
//!         .execute(&client.ehr.v1.leave)
//!         .await?;
//!
//!     println!("总请假天数: {}", stats_response.data.total_leave_days);
//!
//!     Ok(())
//! }
//! ```

use super::models::*;
use crate::core::{config::Config, constants::AccessTokenType, http::Transport, SDKResult};
use open_lark_core::core::api_req::ApiRequest;

// ==================== Builder实现 ====================

/// 创建请假申请构建器
///
/// 提供流畅的API来构建请假申请请求，支持方法链式调用。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = CreateLeaveBuilder::new()
///     .employee_id("emp_001")
///     .leave_type(LeaveType::Annual)
///     .start_time("2024-01-15T09:00:00Z")
///     .end_time("2024-01-17T18:00:00Z")
///     .reason("家庭聚会")
///     .remarks("需要提前安排工作交接");
/// ```
#[derive(Debug, Clone, Default)]
pub struct CreateLeaveBuilder {
    request: CreateLeaveRequest,
}

impl CreateLeaveBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self {
            request: CreateLeaveRequest {
                employee_id: String::new(),
                leave_type: LeaveType::Personal,
                start_time: String::new(),
                end_time: String::new(),
                reason: String::new(),
                attachments: None,
                remarks: None,
                approver_id: None,
            },
        }
    }

    /// 设置员工ID
    pub fn employee_id(mut self, employee_id: impl Into<String>) -> Self {
        self.request.employee_id = employee_id.into();
        self
    }

    /// 设置请假类型
    pub fn leave_type(mut self, leave_type: LeaveType) -> Self {
        self.request.leave_type = leave_type;
        self
    }

    /// 设置请假开始时间
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.request.start_time = start_time.into();
        self
    }

    /// 设置请假结束时间
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.request.end_time = end_time.into();
        self
    }

    /// 设置请假原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.request.reason = reason.into();
        self
    }

    /// 设置附件列表
    pub fn attachments(mut self, attachments: Vec<String>) -> Self {
        self.request.attachments = Some(attachments);
        self
    }

    /// 添加单个附件
    pub fn add_attachment(mut self, attachment: impl Into<String>) -> Self {
        let mut attachments = self.request.attachments.unwrap_or_default();
        attachments.push(attachment.into());
        self.request.attachments = Some(attachments);
        self
    }

    /// 设置备注
    pub fn remarks(mut self, remarks: impl Into<String>) -> Self {
        self.request.remarks = Some(remarks.into());
        self
    }

    /// 设置审批人ID
    pub fn approver_id(mut self, approver_id: impl Into<String>) -> Self {
        self.request.approver_id = Some(approver_id.into());
        self
    }

    /// 执行请假申请创建
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveRecordResponse> {
        service.create_leave(&self.request).await
    }

    /// 获取构建的请求对象
    pub fn build(self) -> CreateLeaveRequest {
        self.request
    }
}

/// 查询请假记录构建器
///
/// 提供流畅的API来构建请假记录查询请求，支持多种筛选条件。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = QueryLeaveRecordsBuilder::new()
///     .employee_id("emp_001")
///     .leave_type(LeaveType::Annual)
///     .status(LeaveStatus::Approved)
///     .start_date("2024-01-01")
///     .end_date("2024-12-31")
///     .page_size(50);
/// ```
#[derive(Debug, Clone, Default)]
pub struct QueryLeaveRecordsBuilder {
    request: QueryLeaveRecordsRequest,
}

impl QueryLeaveRecordsBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self {
            request: QueryLeaveRecordsRequest {
                employee_id: None,
                leave_type: None,
                status: None,
                start_date: None,
                end_date: None,
                applicant_id: None,
                approver_id: None,
                department_id: None,
                page_size: None,
                page_token: None,
            },
        }
    }

    /// 设置员工ID筛选
    pub fn employee_id(mut self, employee_id: impl Into<String>) -> Self {
        self.request.employee_id = Some(employee_id.into());
        self
    }

    /// 设置请假类型筛选
    pub fn leave_type(mut self, leave_type: LeaveType) -> Self {
        self.request.leave_type = Some(leave_type);
        self
    }

    /// 设置请假状态筛选
    pub fn status(mut self, status: LeaveStatus) -> Self {
        self.request.status = Some(status);
        self
    }

    /// 设置开始日期筛选
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.request.start_date = Some(start_date.into());
        self
    }

    /// 设置结束日期筛选
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.request.end_date = Some(end_date.into());
        self
    }

    /// 设置申请人ID筛选
    pub fn applicant_id(mut self, applicant_id: impl Into<String>) -> Self {
        self.request.applicant_id = Some(applicant_id.into());
        self
    }

    /// 设置审批人ID筛选
    pub fn approver_id(mut self, approver_id: impl Into<String>) -> Self {
        self.request.approver_id = Some(approver_id.into());
        self
    }

    /// 设置部门ID筛选
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.request.department_id = Some(department_id.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 执行请假记录查询
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveRecordListResponse> {
        service.query_leave_records(&self.request).await
    }

    /// 获取构建的请求对象
    pub fn build(self) -> QueryLeaveRecordsRequest {
        self.request
    }
}

/// 更新请假记录构建器
///
/// 提供流畅的API来构建请假记录更新请求。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = UpdateLeaveBuilder::new("leave_001")
///     .reason("修改后的请假原因")
///     .remarks("补充说明")
///     .add_attachment("证明文件.pdf");
/// ```
#[derive(Debug, Clone, Default)]
pub struct UpdateLeaveBuilder {
    leave_id: String,
    update_fields: LeaveUpdateFields,
}

impl UpdateLeaveBuilder {
    /// 创建新的构建器实例
    pub fn new(leave_id: impl Into<String>) -> Self {
        Self {
            leave_id: leave_id.into(),
            update_fields: LeaveUpdateFields::default(),
        }
    }

    /// 设置请假原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.update_fields.reason = Some(reason.into());
        self
    }

    /// 设置附件列表
    pub fn attachments(mut self, attachments: Vec<String>) -> Self {
        self.update_fields.attachments = Some(attachments);
        self
    }

    /// 添加单个附件
    pub fn add_attachment(mut self, attachment: impl Into<String>) -> Self {
        let mut attachments = self.update_fields.attachments.unwrap_or_default();
        attachments.push(attachment.into());
        self.update_fields.attachments = Some(attachments);
        self
    }

    /// 设置备注
    pub fn remarks(mut self, remarks: impl Into<String>) -> Self {
        self.update_fields.remarks = Some(remarks.into());
        self
    }

    /// 设置请假状态（仅限草稿状态下的撤销）
    pub fn status(mut self, status: LeaveStatus) -> Self {
        self.update_fields.status = Some(status);
        self
    }

    /// 执行请假记录更新
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveRecordResponse> {
        let request = UpdateLeaveRequest {
            leave_id: self.leave_id,
            update_fields: self.update_fields,
        };
        service.update_leave_record(&request).await
    }

    /// 获取构建的请求对象
    pub fn build(self) -> UpdateLeaveRequest {
        UpdateLeaveRequest {
            leave_id: self.leave_id,
            update_fields: self.update_fields,
        }
    }
}

/// 请假审批构建器
///
/// 提供流畅的API来构建请假审批请求。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = ApproveLeaveBuilder::new("leave_001")
///     .decision(LeaveApprovalDecision::Approve)
///     .comment("同意请假，请安排好工作交接");
/// ```
#[derive(Debug, Clone, Default)]
pub struct ApproveLeaveBuilder {
    leave_id: String,
    decision: LeaveApprovalDecision,
    comment: Option<String>,
    forward_to_user_id: Option<String>,
}

impl ApproveLeaveBuilder {
    /// 创建新的构建器实例
    pub fn new(leave_id: impl Into<String>) -> Self {
        Self {
            leave_id: leave_id.into(),
            decision: LeaveApprovalDecision::Approve,
            comment: None,
            forward_to_user_id: None,
        }
    }

    /// 设置审批决定
    pub fn decision(mut self, decision: LeaveApprovalDecision) -> Self {
        self.decision = decision;
        self
    }

    /// 设置审批意见
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// 设置转交目标用户ID（转交操作时使用）
    pub fn forward_to_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.forward_to_user_id = Some(user_id.into());
        self
    }

    /// 执行请假审批
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveApprovalResponse> {
        let request = ApproveLeaveRequest {
            leave_id: self.leave_id,
            decision: self.decision,
            comment: self.comment,
            forward_to_user_id: self.forward_to_user_id,
        };
        service.approve_leave_request(&request).await
    }

    /// 获取构建的请求对象
    pub fn build(self) -> ApproveLeaveRequest {
        ApproveLeaveRequest {
            leave_id: self.leave_id,
            decision: self.decision,
            comment: self.comment,
            forward_to_user_id: self.forward_to_user_id,
        }
    }
}

/// 查询请假余额构建器
///
/// 提供流畅的API来构建请假余额查询请求。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = QueryLeaveBalanceBuilder::new("emp_001")
///     .leave_type(LeaveType::Annual)
///     .year(2024);
/// ```
#[derive(Debug, Clone, Default)]
pub struct QueryLeaveBalanceBuilder {
    request: QueryLeaveBalanceRequest,
}

impl QueryLeaveBalanceBuilder {
    /// 创建新的构建器实例
    pub fn new(employee_id: impl Into<String>) -> Self {
        Self {
            request: QueryLeaveBalanceRequest {
                employee_id: employee_id.into(),
                leave_type: None,
                year: None,
            },
        }
    }

    /// 设置请假类型筛选
    pub fn leave_type(mut self, leave_type: LeaveType) -> Self {
        self.request.leave_type = Some(leave_type);
        self
    }

    /// 设置年份
    pub fn year(mut self, year: i32) -> Self {
        self.request.year = Some(year);
        self
    }

    /// 执行请假余额查询
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveBalanceListResponse> {
        service.query_leave_balance(&self.request).await
    }

    /// 获取构建的请求对象
    pub fn build(self) -> QueryLeaveBalanceRequest {
        self.request
    }
}

/// 获取请假统计构建器
///
/// 提供流畅的API来构建请假统计请求。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = GetLeaveStatisticsBuilder::new()
///     .employee_id("emp_001")
///     .year(2024)
///     .month(1)
///     .leave_type(LeaveType::Annual);
/// ```
#[derive(Debug, Clone, Default)]
pub struct GetLeaveStatisticsBuilder {
    request: GetLeaveStatisticsRequest,
}

impl GetLeaveStatisticsBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self {
            request: GetLeaveStatisticsRequest {
                employee_id: None,
                department_id: None,
                year: chrono::Utc::now().year(),
                month: None,
                leave_type: None,
            },
        }
    }

    /// 设置员工ID
    pub fn employee_id(mut self, employee_id: impl Into<String>) -> Self {
        self.request.employee_id = Some(employee_id.into());
        self
    }

    /// 设置部门ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.request.department_id = Some(department_id.into());
        self
    }

    /// 设置年份
    pub fn year(mut self, year: i32) -> Self {
        self.request.year = year;
        self
    }

    /// 设置月份
    pub fn month(mut self, month: i32) -> Self {
        self.request.month = Some(month);
        self
    }

    /// 设置请假类型筛选
    pub fn leave_type(mut self, leave_type: LeaveType) -> Self {
        self.request.leave_type = Some(leave_type);
        self
    }

    /// 执行请假统计查询
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveStatisticsResponse> {
        service.get_leave_statistics(&self.request).await
    }

    /// 获取构建的请求对象
    pub fn build(self) -> GetLeaveStatisticsRequest {
        self.request
    }
}

/// 创建请假规则构建器
///
/// 提供流畅的API来构建请假规则创建请求。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = CreateLeaveRuleBuilder::new()
///     .leave_type(LeaveType::Annual)
///     .name("年假管理规则")
///     .requires_approval(true)
///     .max_leave_days(15.0)
///     .allow_partial_days(true);
/// ```
#[derive(Debug, Clone, Default)]
pub struct CreateLeaveRuleBuilder {
    request: CreateLeaveRuleRequest,
}

impl CreateLeaveRuleBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self {
            request: CreateLeaveRuleRequest {
                leave_type: LeaveType::Personal,
                name: String::new(),
                description: None,
                requires_approval: true,
                min_duration_hours: None,
                max_leave_days: None,
                allow_partial_days: false,
                requires_attachment: false,
                advance_notice_days: None,
                applicable_departments: None,
            },
        }
    }

    /// 设置请假类型
    pub fn leave_type(mut self, leave_type: LeaveType) -> Self {
        self.request.leave_type = leave_type;
        self
    }

    /// 设置规则名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    /// 设置规则描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置是否需要审批
    pub fn requires_approval(mut self, requires_approval: bool) -> Self {
        self.request.requires_approval = requires_approval;
        self
    }

    /// 设置最小请假时长（小时）
    pub fn min_duration_hours(mut self, min_duration_hours: f64) -> Self {
        self.request.min_duration_hours = Some(min_duration_hours);
        self
    }

    /// 设置最大请假天数
    pub fn max_leave_days(mut self, max_leave_days: f64) -> Self {
        self.request.max_leave_days = Some(max_leave_days);
        self
    }

    /// 设置是否支持部分请假
    pub fn allow_partial_days(mut self, allow_partial_days: bool) -> Self {
        self.request.allow_partial_days = allow_partial_days;
        self
    }

    /// 设置是否需要附件
    pub fn requires_attachment(mut self, requires_attachment: bool) -> Self {
        self.request.requires_attachment = requires_attachment;
        self
    }

    /// 设置提前申请天数
    pub fn advance_notice_days(mut self, advance_notice_days: i32) -> Self {
        self.request.advance_notice_days = Some(advance_notice_days);
        self
    }

    /// 设置适用部门列表
    pub fn applicable_departments(mut self, departments: Vec<String>) -> Self {
        self.request.applicable_departments = Some(departments);
        self
    }

    /// 添加适用部门
    pub fn add_applicable_department(mut self, department: impl Into<String>) -> Self {
        let mut departments = self.request.applicable_departments.unwrap_or_default();
        departments.push(department.into());
        self.request.applicable_departments = Some(departments);
        self
    }

    /// 执行请假规则创建
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveRuleResponse> {
        service.create_leave_rule(&self.request).await
    }

    /// 获取构建的请求对象
    pub fn build(self) -> CreateLeaveRuleRequest {
        self.request
    }
}

/// 更新请假规则构建器
///
/// 提供流畅的API来构建请假规则更新请求。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = UpdateLeaveRuleBuilder::new("rule_001")
///     .name("更新后的规则名称")
///     .max_leave_days(20.0)
///     .requires_approval(false);
/// ```
#[derive(Debug, Clone, Default)]
pub struct UpdateLeaveRuleBuilder {
    rule_id: String,
    update_fields: LeaveRuleUpdateFields,
}

impl UpdateLeaveRuleBuilder {
    /// 创建新的构建器实例
    pub fn new(rule_id: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
            update_fields: LeaveRuleUpdateFields::default(),
        }
    }

    /// 设置规则名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.update_fields.name = Some(name.into());
        self
    }

    /// 设置规则描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.update_fields.description = Some(description.into());
        self
    }

    /// 设置是否需要审批
    pub fn requires_approval(mut self, requires_approval: bool) -> Self {
        self.update_fields.requires_approval = Some(requires_approval);
        self
    }

    /// 设置最小请假时长（小时）
    pub fn min_duration_hours(mut self, min_duration_hours: f64) -> Self {
        self.update_fields.min_duration_hours = Some(min_duration_hours);
        self
    }

    /// 设置最大请假天数
    pub fn max_leave_days(mut self, max_leave_days: f64) -> Self {
        self.update_fields.max_leave_days = Some(max_leave_days);
        self
    }

    /// 设置是否支持部分请假
    pub fn allow_partial_days(mut self, allow_partial_days: bool) -> Self {
        self.update_fields.allow_partial_days = Some(allow_partial_days);
        self
    }

    /// 设置是否需要附件
    pub fn requires_attachment(mut self, requires_attachment: bool) -> Self {
        self.update_fields.requires_attachment = Some(requires_attachment);
        self
    }

    /// 设置提前申请天数
    pub fn advance_notice_days(mut self, advance_notice_days: i32) -> Self {
        self.update_fields.advance_notice_days = Some(advance_notice_days);
        self
    }

    /// 设置适用部门列表
    pub fn applicable_departments(mut self, departments: Vec<String>) -> Self {
        self.update_fields.applicable_departments = Some(departments);
        self
    }

    /// 添加适用部门
    pub fn add_applicable_department(mut self, department: impl Into<String>) -> Self {
        let mut departments = self
            .update_fields
            .applicable_departments
            .unwrap_or_default();
        departments.push(department.into());
        self.update_fields.applicable_departments = Some(departments);
        self
    }

    /// 设置状态
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.update_fields.status = Some(status.into());
        self
    }

    /// 执行请假规则更新
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveRuleResponse> {
        let request = UpdateLeaveRuleRequest {
            rule_id: self.rule_id,
            update_fields: self.update_fields,
        };
        service.update_leave_rule(&request).await
    }

    /// 获取构建的请求对象
    pub fn build(self) -> UpdateLeaveRuleRequest {
        UpdateLeaveRuleRequest {
            rule_id: self.rule_id,
            update_fields: self.update_fields,
        }
    }
}

/// 删除请假规则构建器
///
/// 提供流畅的API来删除请假规则。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = DeleteLeaveRuleBuilder::new("rule_001");
/// let response = builder.execute(service).await?;
/// ```
#[derive(Debug, Clone)]
pub struct DeleteLeaveRuleBuilder {
    rule_id: String,
}

impl DeleteLeaveRuleBuilder {
    /// 创建新的构建器实例
    pub fn new(rule_id: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
        }
    }

    /// 执行请假规则删除
    pub async fn execute(self, service: &LeaveService) -> SDKResult<EmptyResponse> {
        service.delete_leave_rule(&self.rule_id).await
    }
}

/// 取消请假申请构建器
///
/// 提供流畅的API来取消请假申请。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = CancelLeaveBuilder::new("leave_001")
///     .reason("计划变更");
/// let response = builder.execute(service).await?;
/// ```
#[derive(Debug, Clone)]
pub struct CancelLeaveBuilder {
    leave_id: String,
    reason: String,
}

impl CancelLeaveBuilder {
    /// 创建新的构建器实例
    pub fn new(leave_id: impl Into<String>) -> Self {
        Self {
            leave_id: leave_id.into(),
            reason: String::new(),
        }
    }

    /// 设置取消原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = reason.into();
        self
    }

    /// 执行请假申请取消
    pub async fn execute(self, service: &LeaveService) -> SDKResult<EmptyResponse> {
        service
            .cancel_leave_request(&self.leave_id, &self.reason)
            .await
    }
}

/// 调整请假余额构建器
///
/// 提供流畅的API来调整员工请假余额。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = AdjustLeaveBalanceBuilder::new("emp_001")
///     .leave_type(LeaveType::Annual)
///     .year(2024)
///     .adjustment_days(2.0)
///     .reason("年度福利调整");
/// let response = builder.execute(service).await?;
/// ```
#[derive(Debug, Clone)]
pub struct AdjustLeaveBalanceBuilder {
    employee_id: String,
    leave_type: LeaveType,
    year: i32,
    adjustment_days: f64,
    reason: String,
}

impl AdjustLeaveBalanceBuilder {
    /// 创建新的构建器实例
    pub fn new(employee_id: impl Into<String>) -> Self {
        Self {
            employee_id: employee_id.into(),
            leave_type: LeaveType::Personal,
            year: chrono::Utc::now().year(),
            adjustment_days: 0.0,
            reason: String::new(),
        }
    }

    /// 设置请假类型
    pub fn leave_type(mut self, leave_type: LeaveType) -> Self {
        self.leave_type = leave_type;
        self
    }

    /// 设置年份
    pub fn year(mut self, year: i32) -> Self {
        self.year = year;
        self
    }

    /// 设置调整天数
    pub fn adjustment_days(mut self, adjustment_days: f64) -> Self {
        self.adjustment_days = adjustment_days;
        self
    }

    /// 设置调整原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = reason.into();
        self
    }

    /// 执行请假余额调整
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveBalanceResponse> {
        service
            .adjust_leave_balance(
                &self.employee_id,
                self.leave_type,
                self.year,
                self.adjustment_days,
                &self.reason,
            )
            .await
    }
}

/// 获取待审批列表构建器
///
/// 提供流畅的API来获取待审批的请假申请列表。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = GetPendingApprovalsBuilder::new()
///     .page_size(50)
///     .page_token("next_page_token");
/// let response = builder.execute(service).await?;
/// ```
#[derive(Debug, Clone, Default)]
pub struct GetPendingApprovalsBuilder {
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl GetPendingApprovalsBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行待审批列表查询
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveRecordListResponse> {
        service
            .get_pending_approvals(self.page_size, self.page_token)
            .await
    }
}

/// 获取请假规则列表构建器
///
/// 提供流畅的API来获取请假规则列表。
///
/// # 示例
///
/// ```rust,no_run
/// let builder = GetLeaveRulesBuilder::new()
///     .leave_type(LeaveType::Annual)
///     .page_size(50);
/// let response = builder.execute(service).await?;
/// ```
#[derive(Debug, Clone, Default)]
pub struct GetLeaveRulesBuilder {
    leave_type: Option<LeaveType>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl GetLeaveRulesBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self {
            leave_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置请假类型筛选
    pub fn leave_type(mut self, leave_type: LeaveType) -> Self {
        self.leave_type = Some(leave_type);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请假规则列表查询
    pub async fn execute(self, service: &LeaveService) -> SDKResult<LeaveRuleListResponse> {
        service
            .get_leave_rules(self.leave_type, self.page_size, self.page_token)
            .await
    }
}

/// 请假管理服务
///
/// 提供完整的企业请假管理功能，支持请假申请、审批、统计等全流程管理。
/// 适用于各种请假场景，包括年假、病假、事假、婚假等各类假期管理。
///
/// # 核心功能
///
/// - **请假申请管理**: 员工请假申请的创建、查询、更新、取消
/// - **审批流程管理**: 请假审批流程的执行、转交、撤销
/// - **余额查询管理**: 员工各类假期余额的查询和统计
/// - **规则配置管理**: 请假规则、审批流程、限制条件等配置
/// - **统计分析功能**: 请假数据统计、趋势分析、异常监控
/// - **系统集成支持**: 与考勤、薪资、人事等系统的数据集成
///
/// # 使用示例
///
/// ```rust
/// use open_lark::prelude::*;
/// use open_lark::service::ehr::v1::leave::*;
///
/// let client = LarkClient::builder()
///     .app_id("app_id")
///     .app_secret("app_secret")
///     .build()?;
///
/// // 创建请假申请
/// let response = client.ehr.v1.leave
///     .create_leave_builder()
///     .employee_id("emp_001")
///     .leave_type(LeaveType::Annual)
///     .start_time("2024-01-15T09:00:00Z")
///     .end_time("2024-01-17T18:00:00Z")
///     .reason("家庭聚会")
///     .execute(&client.ehr.v1.leave)
///     .await?;
/// ```
#[derive(Debug, Clone)]
pub struct LeaveService {
    pub config: Config,
}

impl LeaveService {
    /// 创建新的请假服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 请假申请管理 ====================

    /// 创建请假申请
    ///
    /// 为员工创建新的请假申请，支持各类假期类型。
    /// 系统会根据请假规则验证申请的有效性，并启动相应的审批流程。
    ///
    /// # 参数
    ///
    /// * `request` - 请假申请请求，包含员工ID、请假类型、时间等信息
    ///
    /// # 返回
    ///
    /// 返回创建成功的请假记录信息，包含请假ID和初始状态
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = CreateLeaveRequest {
    ///     employee_id: "emp_001".to_string(),
    ///     leave_type: LeaveType::Annual,
    ///     start_time: "2024-01-15T09:00:00Z".to_string(),
    ///     end_time: "2024-01-17T18:00:00Z".to_string(),
    ///     reason: "家庭聚会".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let response = leave_service.create_leave(&request).await?;
    /// println!("请假申请ID: {}", response.data.leave_id);
    /// ```
    pub async fn create_leave(
        &self,
        request: &CreateLeaveRequest,
    ) -> SDKResult<LeaveRecordResponse> {
        // 参数验证
        self.validate_create_leave_request(request)?;

        let url = format!("{}/open-apis/ehr/v1/leaves", self.config.base_url);
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("POST")
            .url(&url)
            .body(serde_json::to_value(request)?)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveRecordResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    /// 获取请假记录详情
    ///
    /// 根据请假记录ID获取详细的请假信息，包括申请详情、审批状态、相关附件等。
    ///
    /// # 参数
    ///
    /// * `leave_id` - 请假记录ID
    ///
    /// # 返回
    ///
    /// 返回请假记录的详细信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = leave_service.get_leave_record("leave_001").await?;
    /// println!("请假状态: {:?}", response.data.status);
    /// println!("请假天数: {}", response.data.leave_days);
    /// ```
    pub async fn get_leave_record(&self, leave_id: &str) -> SDKResult<LeaveRecordResponse> {
        if leave_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "请假记录ID不能为空".to_string(),
            ));
        }

        let url = format!(
            "{}/open-apis/ehr/v1/leaves/{}",
            self.config.base_url, leave_id
        );
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("GET")
            .url(&url)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveRecordResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    /// 更新请假记录
    ///
    /// 更新已提交但尚未审批的请假申请信息，如修改请假原因、添加附件等。
    /// 已审批或已完成的请假记录不允许修改。
    ///
    /// # 参数
    ///
    /// * `request` - 更新请假记录请求
    ///
    /// # 返回
    ///
    /// 返回更新后的请假记录信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let update_fields = LeaveUpdateFields {
    ///     reason: Some("修改后的请假原因".to_string()),
    ///     remarks: Some("补充说明".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let request = UpdateLeaveRequest {
    ///     leave_id: "leave_001".to_string(),
    ///     update_fields,
    /// };
    ///
    /// let response = leave_service.update_leave_record(&request).await?;
    /// ```
    pub async fn update_leave_record(
        &self,
        request: &UpdateLeaveRequest,
    ) -> SDKResult<LeaveRecordResponse> {
        // 参数验证
        self.validate_update_leave_request(request)?;

        let url = format!(
            "{}/open-apis/ehr/v1/leaves/{}",
            self.config.base_url, request.leave_id
        );
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("PUT")
            .url(&url)
            .body(serde_json::to_value(&request.update_fields)?)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveRecordResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    /// 取消请假申请
    ///
    /// 取消已提交但尚未开始执行的请假申请。
    /// 已开始执行或已完成的请假记录不允许取消，只能通过撤销操作处理。
    ///
    /// # 参数
    ///
    /// * `leave_id` - 请假记录ID
    /// * `reason` - 取消原因
    ///
    /// # 返回
    ///
    /// 返回操作结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = leave_service.cancel_leave_request("leave_001", "计划变更").await?;
    /// println!("请假申请已取消");
    /// ```
    pub async fn cancel_leave_request(
        &self,
        leave_id: &str,
        reason: &str,
    ) -> SDKResult<EmptyResponse> {
        if leave_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "请假记录ID不能为空".to_string(),
            ));
        }

        if reason.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "取消原因不能为空".to_string(),
            ));
        }

        let cancel_data = serde_json::json!({
            "reason": reason,
            "action": "cancel"
        });

        let url = format!(
            "{}/open-apis/ehr/v1/leaves/{}/cancel",
            self.config.base_url, leave_id
        );
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("POST")
            .url(&url)
            .body(cancel_data)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: EmptyResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    /// 查询请假记录列表
    ///
    /// 根据查询条件获取员工请假记录列表，支持多种筛选条件和分页查询。
    ///
    /// # 参数
    ///
    /// * `request` - 查询请假记录请求
    ///
    /// # 返回
    ///
    /// 返回符合条件的请假记录列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = QueryLeaveRecordsRequest {
    ///     employee_id: Some("emp_001".to_string()),
    ///     status: Some(LeaveStatus::Approved),
    ///     start_date: Some("2024-01-01".to_string()),
    ///     end_date: Some("2024-12-31".to_string()),
    ///     page_size: Some(50),
    ///     ..Default::default()
    /// };
    ///
    /// let response = leave_service.query_leave_records(&request).await?;
    /// println!("找到 {} 条请假记录", response.data.items.len());
    /// ```
    pub async fn query_leave_records(
        &self,
        request: &QueryLeaveRecordsRequest,
    ) -> SDKResult<LeaveRecordListResponse> {
        // 参数验证
        self.validate_query_leave_records_request(request)?;

        let url = format!("{}/open-apis/ehr/v1/leaves/query", self.config.base_url);
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("POST")
            .url(&url)
            .body(serde_json::to_value(request)?)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveRecordListResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    // ==================== 请假审批管理 ====================

    /// 审批请假申请
    ///
    /// 对待审批的请假申请进行审批操作，支持批准、拒绝、转交等决定。
    /// 系统会记录审批历史和意见，并更新请假状态。
    ///
    /// # 参数
    ///
    /// * `request` - 请假审批请求
    ///
    /// # 返回
    ///
    /// 返回审批操作结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = ApproveLeaveRequest {
    ///     leave_id: "leave_001".to_string(),
    ///     decision: LeaveApprovalDecision::Approve,
    ///     comment: Some("同意请假，请注意安排好工作交接".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = leave_service.approve_leave_request(&request).await?;
    /// println!("请假审批完成");
    /// ```
    pub async fn approve_leave_request(
        &self,
        request: &ApproveLeaveRequest,
    ) -> SDKResult<LeaveApprovalResponse> {
        // 参数验证
        self.validate_approve_leave_request(request)?;

        let url = format!(
            "{}/open-apis/ehr/v1/leaves/{}/approve",
            self.config.base_url, request.leave_id
        );
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("POST")
            .url(&url)
            .body(serde_json::to_value(request)?)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveApprovalResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    /// 获取待审批请假列表
    ///
    /// 获取当前用户需要审批的请假申请列表，支持分页查询。
    ///
    /// # 参数
    ///
    /// * `page_size` - 分页大小，默认20
    /// * `page_token` - 分页标记，用于获取下一页
    ///
    /// # 返回
    ///
    /// 返回待审批的请假申请列表
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = leave_service.get_pending_approvals(Some(50), None).await?;
    /// println!("待审批请假申请: {} 个", response.data.items.len());
    /// ```
    pub async fn get_pending_approvals(
        &self,
        page_size: Option<i32>,
        page_token: Option<String>,
    ) -> SDKResult<LeaveRecordListResponse> {
        let query_params = serde_json::json!({
            "status": LeaveStatus::PendingApproval,
            "page_size": page_size.unwrap_or(20),
            "page_token": page_token
        });

        let url = format!(
            "{}/open-apis/ehr/v1/leaves/pending_approvals",
            self.config.base_url
        );
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("GET")
            .url(&url)
            .query_params(Some(query_params))
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveRecordListResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    // ==================== 请假余额管理 ====================

    /// 查询员工请假余额
    ///
    /// 查询指定员工各类假期的余额情况，包括总额度、已使用、剩余天数等信息。
    ///
    /// # 参数
    ///
    /// * `request` - 查询请假余额请求
    ///
    /// # 返回
    ///
    /// 返回员工的假期余额信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = QueryLeaveBalanceRequest {
    ///     employee_id: "emp_001".to_string(),
    ///     leave_type: Some(LeaveType::Annual),
    ///     year: Some(2024),
    /// };
    ///
    /// let response = leave_service.query_leave_balance(&request).await?;
    /// println!("年假余额: {} 天", response.data.remaining_days);
    /// ```
    pub async fn query_leave_balance(
        &self,
        request: &QueryLeaveBalanceRequest,
    ) -> SDKResult<LeaveBalanceListResponse> {
        // 参数验证
        self.validate_query_leave_balance_request(request)?;

        let url = format!(
            "{}/open-apis/ehr/v1/leave_balances/query",
            self.config.base_url
        );
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("POST")
            .url(&url)
            .body(serde_json::to_value(request)?)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveBalanceListResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    /// 更新请假余额
    ///
    /// 手动调整员工的假期余额，通常用于HR管理员进行余额修正。
    ///
    /// # 参数
    ///
    /// * `employee_id` - 员工ID
    /// * `leave_type` - 请假类型
    /// * `year` - 年份
    /// * `adjustment_days` - 调整天数（正数为增加，负数为减少）
    /// * `reason` - 调整原因
    ///
    /// # 返回
    ///
    /// 返回调整后的假期余额信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = leave_service.adjust_leave_balance(
    ///     "emp_001",
    ///     LeaveType::Annual,
    ///     2024,
    ///     2.0,
    ///     "年度福利调整"
    /// ).await?;
    /// println!("假期余额已调整");
    /// ```
    pub async fn adjust_leave_balance(
        &self,
        employee_id: &str,
        leave_type: LeaveType,
        year: i32,
        adjustment_days: f64,
        reason: &str,
    ) -> SDKResult<LeaveBalanceResponse> {
        if employee_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "员工ID不能为空".to_string(),
            ));
        }

        if reason.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "调整原因不能为空".to_string(),
            ));
        }

        let adjustment_data = serde_json::json!({
            "employee_id": employee_id,
            "leave_type": leave_type,
            "year": year,
            "adjustment_days": adjustment_days,
            "reason": reason
        });

        let url = format!(
            "{}/open-apis/ehr/v1/leave_balances/adjust",
            self.config.base_url
        );
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("POST")
            .url(&url)
            .body(adjustment_data)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveBalanceResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    // ==================== 请假统计管理 ====================

    /// 获取请假统计数据
    ///
    /// 获取指定员工或部门的请假统计信息，支持按时间、类型等多维度分析。
    ///
    /// # 参数
    ///
    /// * `request` - 获取请假统计请求
    ///
    /// # 返回
    ///
    /// 返回请假统计信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = GetLeaveStatisticsRequest {
    ///     employee_id: Some("emp_001".to_string()),
    ///     year: 2024,
    ///     month: None,
    ///     ..Default::default()
    /// };
    ///
    /// let response = leave_service.get_leave_statistics(&request).await?;
    /// println!("2024年总请假天数: {}", response.data.total_leave_days);
    /// ```
    pub async fn get_leave_statistics(
        &self,
        request: &GetLeaveStatisticsRequest,
    ) -> SDKResult<LeaveStatisticsResponse> {
        // 参数验证
        self.validate_get_leave_statistics_request(request)?;

        let url = format!("{}/open-apis/ehr/v1/leave_statistics", self.config.base_url);
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("POST")
            .url(&url)
            .body(serde_json::to_value(request)?)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveStatisticsResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    /// 获取部门请假统计
    ///
    /// 获取指定部门的请假统计信息，用于管理层了解部门请假情况。
    ///
    /// # 参数
    ///
    /// * `department_id` - 部门ID
    /// * `year` - 年份
    /// * `month` - 月份（可选）
    ///
    /// # 返回
    ///
    /// 返回部门请假统计信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = leave_service.get_department_leave_statistics("dept_001", 2024, None).await?;
    /// println!("部门请假统计: {:?}", response.data);
    /// ```
    pub async fn get_department_leave_statistics(
        &self,
        department_id: &str,
        year: i32,
        month: Option<i32>,
    ) -> SDKResult<LeaveStatisticsResponse> {
        if department_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "部门ID不能为空".to_string(),
            ));
        }

        let request = GetLeaveStatisticsRequest {
            employee_id: None,
            department_id: Some(department_id.to_string()),
            year,
            month,
            leave_type: None,
        };

        self.get_leave_statistics(&request).await
    }

    // ==================== 请假规则管理 ====================

    /// 创建请假规则
    ///
    /// 创建新的请假规则，定义各类假期的申请条件、审批流程、限制条件等。
    ///
    /// # 参数
    ///
    /// * `request` - 创建请假规则请求
    ///
    /// # 返回
    ///
    /// 返回创建成功的请假规则信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let request = CreateLeaveRuleRequest {
    ///     leave_type: LeaveType::Annual,
    ///     name: "年假管理规则".to_string(),
    ///     requires_approval: true,
    ///     max_leave_days: Some(15.0),
    ///     allow_partial_days: true,
    ///     ..Default::default()
    /// };
    ///
    /// let response = leave_service.create_leave_rule(&request).await?;
    /// println!("请假规则创建成功，ID: {}", response.data.rule_id);
    /// ```
    pub async fn create_leave_rule(
        &self,
        request: &CreateLeaveRuleRequest,
    ) -> SDKResult<LeaveRuleResponse> {
        // 参数验证
        self.validate_create_leave_rule_request(request)?;

        let url = format!("{}/open-apis/ehr/v1/leave_rules", self.config.base_url);
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("POST")
            .url(&url)
            .body(serde_json::to_value(request)?)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveRuleResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    /// 获取请假规则列表
    ///
    /// 获取系统中的请假规则列表，支持按请假类型筛选。
    ///
    /// # 参数
    ///
    /// * `leave_type` - 请假类型筛选（可选）
    /// * `page_size` - 分页大小
    /// * `page_token` - 分页标记
    ///
    /// # 返回
    ///
    /// 返回请假规则列表
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = leave_service.get_leave_rules(Some(LeaveType::Annual), Some(50), None).await?;
    /// println!("找到 {} 条年假规则", response.data.items.len());
    /// ```
    pub async fn get_leave_rules(
        &self,
        leave_type: Option<LeaveType>,
        page_size: Option<i32>,
        page_token: Option<String>,
    ) -> SDKResult<LeaveRuleListResponse> {
        let mut query_params = serde_json::Map::new();
        if let Some(leave_type) = leave_type {
            query_params.insert("leave_type".to_string(), serde_json::to_value(leave_type)?);
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size".to_string(), serde_json::to_value(page_size)?);
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token".to_string(), serde_json::to_value(page_token)?);
        }

        let url = format!("{}/open-apis/ehr/v1/leave_rules", self.config.base_url);
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("GET")
            .url(&url)
            .query_params(Some(serde_json::Value::Object(query_params)))
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveRuleListResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    /// 更新请假规则
    ///
    /// 更新现有的请假规则配置，修改规则参数和适用范围。
    ///
    /// # 参数
    ///
    /// * `request` - 更新请假规则请求
    ///
    /// # 返回
    ///
    /// 返回更新后的请假规则信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let update_fields = LeaveRuleUpdateFields {
    ///     max_leave_days: Some(20.0),
    ///     requires_approval: Some(false),
    ///     ..Default::default()
    /// };
    ///
    /// let request = UpdateLeaveRuleRequest {
    ///     rule_id: "rule_001".to_string(),
    ///     update_fields,
    /// };
    ///
    /// let response = leave_service.update_leave_rule(&request).await?;
    /// println!("请假规则已更新");
    /// ```
    pub async fn update_leave_rule(
        &self,
        request: &UpdateLeaveRuleRequest,
    ) -> SDKResult<LeaveRuleResponse> {
        // 参数验证
        self.validate_update_leave_rule_request(request)?;

        let url = format!(
            "{}/open-apis/ehr/v1/leave_rules/{}",
            self.config.base_url, request.rule_id
        );
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("PUT")
            .url(&url)
            .body(serde_json::to_value(&request.update_fields)?)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: LeaveRuleResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    /// 删除请假规则
    ///
    /// 删除指定的请假规则，删除后不再对新申请生效。
    ///
    /// # 参数
    ///
    /// * `rule_id` - 规则ID
    ///
    /// # 返回
    ///
    /// 返回操作结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let response = leave_service.delete_leave_rule("rule_001").await?;
    /// println!("请假规则已删除");
    /// ```
    pub async fn delete_leave_rule(&self, rule_id: &str) -> SDKResult<EmptyResponse> {
        if rule_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "规则ID不能为空".to_string(),
            ));
        }

        let url = format!(
            "{}/open-apis/ehr/v1/leave_rules/{}",
            self.config.base_url, rule_id
        );
        let api_req = ApiRequest::with_method_and_path(reqwest::Method::GET, "")
            .method("DELETE")
            .url(&url)
            .access_token_type(AccessTokenType::Tenant);

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        let response: EmptyResponse = api_resp.data.ok_or_else(|| LarkAPIError::from_response(&api_resp))??;

        Ok(response)
    }

    // ==================== 参数验证方法 ====================

    /// 验证创建请假申请请求参数
    fn validate_create_leave_request(&self, request: &CreateLeaveRequest) -> SDKResult<()> {
        if request.employee_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "员工ID不能为空".to_string(),
            ));
        }

        if request.start_time.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "请假开始时间不能为空".to_string(),
            ));
        }

        if request.end_time.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "请假结束时间不能为空".to_string(),
            ));
        }

        if request.reason.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "请假原因不能为空".to_string(),
            ));
        }

        // 验证时间格式和逻辑
        if let Err(_) = chrono::DateTime::parse_from_rfc3339(&request.start_time) {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "请假开始时间格式无效，请使用RFC3339格式".to_string(),
            ));
        }

        if let Err(_) = chrono::DateTime::parse_from_rfc3339(&request.end_time) {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "请假结束时间格式无效，请使用RFC3339格式".to_string(),
            ));
        }

        // 验证时间逻辑
        let start_time = chrono::DateTime::parse_from_rfc3339(&request.start_time).unwrap();
        let end_time = chrono::DateTime::parse_from_rfc3339(&request.end_time).unwrap();

        if start_time >= end_time {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "请假结束时间必须晚于开始时间".to_string(),
            ));
        }

        Ok(())
    }

    /// 验证更新请假记录请求参数
    fn validate_update_leave_request(&self, request: &UpdateLeaveRequest) -> SDKResult<()> {
        if request.leave_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "请假记录ID不能为空".to_string(),
            ));
        }

        // 如果有更新原因，不能为空
        if let Some(ref reason) = request.update_fields.reason {
            if reason.is_empty() {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "更新后的请假原因不能为空".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// 验证查询请假记录请求参数
    fn validate_query_leave_records_request(
        &self,
        request: &QueryLeaveRecordsRequest,
    ) -> SDKResult<()> {
        // 验证日期格式
        if let Some(ref start_date) = request.start_date {
            if let Err(_) = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "开始日期格式无效，请使用YYYY-MM-DD格式".to_string(),
                ));
            }
        }

        if let Some(ref end_date) = request.end_date {
            if let Err(_) = chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d") {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "结束日期格式无效，请使用YYYY-MM-DD格式".to_string(),
                ));
            }
        }

        // 验证分页参数
        if let Some(page_size) = request.page_size {
            if page_size <= 0 || page_size > 1000 {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "分页大小必须在1-1000之间".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// 验证请假审批请求参数
    fn validate_approve_leave_request(&self, request: &ApproveLeaveRequest) -> SDKResult<()> {
        if request.leave_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "请假记录ID不能为空".to_string(),
            ));
        }

        // 如果是转交操作，必须提供转交对象
        if matches!(request.decision, LeaveApprovalDecision::Forward) {
            if request.forward_to_user_id.is_none()
                || request.forward_to_user_id.as_ref().unwrap().is_empty()
            {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "转交操作必须提供转交对象用户ID".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// 验证查询请假余额请求参数
    fn validate_query_leave_balance_request(
        &self,
        request: &QueryLeaveBalanceRequest,
    ) -> SDKResult<()> {
        if request.employee_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "员工ID不能为空".to_string(),
            ));
        }

        if let Some(year) = request.year {
            if year < 2000 || year > 2100 {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "年份必须在2000-2100之间".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// 验证获取请假统计请求参数
    fn validate_get_leave_statistics_request(
        &self,
        request: &GetLeaveStatisticsRequest,
    ) -> SDKResult<()> {
        if request.year < 2000 || request.year > 2100 {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "年份必须在2000-2100之间".to_string(),
            ));
        }

        if let Some(month) = request.month {
            if month < 1 || month > 12 {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "月份必须在1-12之间".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// 验证创建请假规则请求参数
    fn validate_create_leave_rule_request(
        &self,
        request: &CreateLeaveRuleRequest,
    ) -> SDKResult<()> {
        if request.name.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "规则名称不能为空".to_string(),
            ));
        }

        if let Some(max_days) = request.max_leave_days {
            if max_days <= 0.0 {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "最大请假天数必须大于0".to_string(),
                ));
            }
        }

        if let Some(min_hours) = request.min_duration_hours {
            if min_hours < 0.0 {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "最小请假时长不能为负数".to_string(),
                ));
            }
        }

        if let Some(advance_days) = request.advance_notice_days {
            if advance_days < 0 {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "提前申请天数不能为负数".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// 验证更新请假规则请求参数
    fn validate_update_leave_rule_request(
        &self,
        request: &UpdateLeaveRuleRequest,
    ) -> SDKResult<()> {
        if request.rule_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::IllegalParamError(
                "规则ID不能为空".to_string(),
            ));
        }

        // 验证各更新字段的值
        if let Some(ref name) = request.update_fields.name {
            if name.is_empty() {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "规则名称不能为空".to_string(),
                ));
            }
        }

        if let Some(max_days) = request.update_fields.max_leave_days {
            if max_days <= 0.0 {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "最大请假天数必须大于0".to_string(),
                ));
            }
        }

        if let Some(min_hours) = request.update_fields.min_duration_hours {
            if min_hours < 0.0 {
                return Err(crate::core::error::LarkAPIError::IllegalParamError(
                    "最小请假时长不能为负数".to_string(),
                ));
            }
        }

        Ok(())
    }
}

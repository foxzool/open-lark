//! 审计日志API v1
//!
//! 提供审计日志的创建、查询和管理功能。

use crate::compliance::models::*;
use crate::compliance::service::ComplianceService;
use crate::error::{SecurityError, SecurityResult};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// 审计日志API v1
pub struct AuditLogV1API<'a> {
    service: &'a dyn ComplianceService,
}

impl<'a> AuditLogV1API<'a> {
    /// 创建新的审计日志API实例
    pub fn new(service: &'a dyn ComplianceService) -> Self {
        Self { service }
    }

    /// 创建审计日志
    ///
    /// # security_audit_log_create_v1
    ///
    /// 创建一条新的审计日志记录。
    ///
    /// ## 请求参数
    ///
    /// - `level`: 日志级别
    /// - `audit_type`: 审计类型
    /// - `user_id`: 用户ID（可选）
    /// - `action`: 操作描述
    /// - `resource`: 资源标识（可选）
    /// - `result`: 操作结果
    ///
    /// ## 使用方法
    ///
    /// ```rust
    /// use openlark_security::compliance::v1::*;
    ///
    /// let api = AuditLogV1API::new(&compliance_service);
    ///
    /// let request = AuditLogCreateRequestBuilder::new()
    ///     .level(AuditLevel::Info)
    ///     .audit_type(AuditType::UserAction)
    ///     .user_id("user_123")
    ///     .action("用户登录")
    ///     .result(AuditResult {
    ///         success: true,
    ///         error_code: None,
    ///         error_message: None,
    ///         response_time_ms: Some(150),
    ///     })
    ///     .build();
    ///
    /// let response = api.create_audit_log(request).await?;
    /// ```
    ///
    /// ## 返回值
    ///
    /// 返回 `AuditLogCreateResponse`，包含创建结果信息。
    pub async fn create_audit_log(
        &self,
        request: AuditLogCreateRequest,
    ) -> SecurityResult<AuditLogCreateResponse> {
        info!(
            "创建审计日志: action={}, user_id={:?}",
            request.action, request.user_id
        );

        // 验证请求参数
        validate_audit_log_request(&request)?;

        // 调用服务层创建审计日志
        let response = self.service.create_audit_log(request).await?;

        info!("审计日志创建成功: log_id={}", response.log_id);
        Ok(response)
    }

    /// 查询审计日志
    ///
    /// # security_audit_log_query_v1
    ///
    /// 根据条件查询审计日志记录。
    ///
    /// ## 请求参数
    ///
    /// - `start_time`: 开始时间
    /// - `end_time`: 结束时间
    /// - `user_ids`: 用户ID列表（可选）
    /// - `audit_types`: 审计类型列表（可选）
    /// - `levels`: 日志级别列表（可选）
    /// - `page`: 页码（可选，默认1）
    /// - `page_size`: 页面大小（可选，默认20）
    ///
    /// ## 使用方法
    ///
    /// ```rust
    /// use openlark_security::compliance::v1::*;
    ///
    /// let api = AuditLogV1API::new(&compliance_service);
    ///
    /// let request = AuditLogQueryRequestBuilder::new()
    ///     .start_time(Utc::now() - chrono::Duration::hours(24))
    ///     .end_time(Utc::now())
    ///     .user_ids(vec!["user_123".to_string()])
    ///     .audit_types(vec![AuditType::UserAction])
    ///     .page(1)
    ///     .page_size(20)
    ///     .build();
    ///
    /// let response = api.query_audit_logs(request).await?;
    /// ```
    ///
    /// ## 返回值
    ///
    /// 返回 `AuditLogQueryResponse`，包含查询结果和分页信息。
    pub async fn query_audit_logs(
        &self,
        request: AuditLogQueryRequest,
    ) -> SecurityResult<AuditLogQueryResponse> {
        info!(
            "查询审计日志: start_time={:?}, end_time={:?}",
            request.start_time, request.end_time
        );

        // 验证请求参数
        validate_audit_log_query_request(&request)?;

        // 调用服务层查询审计日志
        let response = self.service.query_audit_logs(request).await?;

        info!("审计日志查询成功: total_count={}", response.total_count);
        Ok(response)
    }
}

/// 审计日志创建请求构建器
#[derive(Debug, Clone)]
pub struct AuditLogCreateRequestBuilder {
    request: AuditLogCreateRequest,
}

impl AuditLogCreateRequestBuilder {
    /// 创建新的审计日志创建请求构建器
    pub fn new() -> Self {
        Self {
            request: AuditLogCreateRequest {
                level: AuditLevel::Info,
                audit_type: AuditType::UserAction,
                user_id: None,
                action: String::new(),
                resource: None,
                resource_type: None,
                ip_address: None,
                user_agent: None,
                result: AuditResult {
                    success: true,
                    error_code: None,
                    error_message: None,
                    response_time_ms: None,
                },
                details: None,
                session_id: None,
                trace_id: None,
            },
        }
    }

    /// 设置日志级别
    pub fn level(mut self, level: AuditLevel) -> Self {
        self.request.level = level;
        self
    }

    /// 设置审计类型
    pub fn audit_type(mut self, audit_type: AuditType) -> Self {
        self.request.audit_type = audit_type;
        self
    }

    /// 设置用户ID
    pub fn user_id<S: Into<String>>(mut self, user_id: S) -> Self {
        self.request.user_id = Some(user_id.into());
        self
    }

    /// 设置操作描述
    pub fn action<S: Into<String>>(mut self, action: S) -> Self {
        self.request.action = action.into();
        self
    }

    /// 设置资源标识
    pub fn resource<S: Into<String>>(mut self, resource: S) -> Self {
        self.request.resource = Some(resource.into());
        self
    }

    /// 设置资源类型
    pub fn resource_type<S: Into<String>>(mut self, resource_type: S) -> Self {
        self.request.resource_type = Some(resource_type.into());
        self
    }

    /// 设置IP地址
    pub fn ip_address<S: Into<String>>(mut self, ip_address: S) -> Self {
        self.request.ip_address = Some(ip_address.into());
        self
    }

    /// 设置用户代理
    pub fn user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.request.user_agent = Some(user_agent.into());
        self
    }

    /// 设置操作结果
    pub fn result(mut self, result: AuditResult) -> Self {
        self.request.result = result;
        self
    }

    /// 设置详细信息
    pub fn details(mut self, details: serde_json::Value) -> Self {
        self.request.details = Some(details);
        self
    }

    /// 设置会话ID
    pub fn session_id<S: Into<String>>(mut self, session_id: S) -> Self {
        self.request.session_id = Some(session_id.into());
        self
    }

    /// 设置追踪ID
    pub fn trace_id<S: Into<String>>(mut self, trace_id: S) -> Self {
        self.request.trace_id = Some(trace_id.into());
        self
    }

    /// 构建审计日志创建请求
    pub fn build(self) -> AuditLogCreateRequest {
        self.request
    }
}

impl Default for AuditLogCreateRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 审计日志查询请求构建器
#[derive(Debug, Clone)]
pub struct AuditLogQueryRequestBuilder {
    request: AuditLogQueryRequest,
}

impl AuditLogQueryRequestBuilder {
    /// 创建新的审计日志查询请求构建器
    pub fn new() -> Self {
        Self {
            request: AuditLogQueryRequest {
                start_time: Utc::now() - chrono::Duration::hours(24),
                end_time: Utc::now(),
                user_ids: None,
                audit_types: None,
                levels: None,
                actions: None,
                resource_types: None,
                ip_addresses: None,
                page: Some(1),
                page_size: Some(20),
                sort_field: Some("timestamp".to_string()),
                sort_direction: Some(SortDirection::Desc),
            },
        }
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.request.start_time = start_time;
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.request.end_time = end_time;
        self
    }

    /// 设置用户ID列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.request.user_ids = Some(user_ids);
        self
    }

    /// 设置审计类型列表
    pub fn audit_types(mut self, audit_types: Vec<AuditType>) -> Self {
        self.request.audit_types = Some(audit_types);
        self
    }

    /// 设置日志级别列表
    pub fn levels(mut self, levels: Vec<AuditLevel>) -> Self {
        self.request.levels = Some(levels);
        self
    }

    /// 设置操作列表
    pub fn actions(mut self, actions: Vec<String>) -> Self {
        self.request.actions = Some(actions);
        self
    }

    /// 设置资源类型列表
    pub fn resource_types(mut self, resource_types: Vec<String>) -> Self {
        self.request.resource_types = Some(resource_types);
        self
    }

    /// 设置IP地址列表
    pub fn ip_addresses(mut self, ip_addresses: Vec<String>) -> Self {
        self.request.ip_addresses = Some(ip_addresses);
        self
    }

    /// 设置分页信息
    pub fn pagination(mut self, page: u32, page_size: u32) -> Self {
        self.request.page = Some(page);
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置排序字段
    pub fn sort_field<S: Into<String>>(mut self, sort_field: S) -> Self {
        self.request.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方向
    pub fn sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.request.sort_direction = Some(sort_direction);
        self
    }

    /// 构建审计日志查询请求
    pub fn build(self) -> AuditLogQueryRequest {
        self.request
    }
}

impl Default for AuditLogQueryRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============ 验证函数 ============

/// 验证审计日志创建请求
fn validate_audit_log_request(request: &AuditLogCreateRequest) -> SecurityResult<()> {
    if request.action.is_empty() {
        return Err(SecurityError::InvalidParameter {
            parameter: "action".to_string(),
            reason: "操作描述不能为空".to_string(),
        });
    }

    // 验证时间范围（如果有时间相关字段）
    // 这里可以添加更多的验证逻辑

    Ok(())
}

/// 验证审计日志查询请求
fn validate_audit_log_query_request(request: &AuditLogQueryRequest) -> SecurityResult<()> {
    if request.start_time > request.end_time {
        return Err(SecurityError::InvalidParameter {
            parameter: "time_range".to_string(),
            reason: "开始时间不能晚于结束时间".to_string(),
        });
    }

    // 验证时间范围不能超过30天
    let max_duration = chrono::Duration::days(30);
    if request.end_time - request.start_time > max_duration {
        return Err(SecurityError::InvalidParameter {
            parameter: "time_range".to_string(),
            reason: "查询时间范围不能超过30天".to_string(),
        });
    }

    // 验证分页参数
    if let Some(page) = request.page {
        if page == 0 {
            return Err(SecurityError::InvalidParameter {
                parameter: "page".to_string(),
                reason: "页码必须大于0".to_string(),
            });
        }
    }

    if let Some(page_size) = request.page_size {
        if page_size == 0 || page_size > 1000 {
            return Err(SecurityError::InvalidParameter {
                parameter: "page_size".to_string(),
                reason: "页面大小必须在1-1000之间".to_string(),
            });
        }
    }

    Ok(())
}

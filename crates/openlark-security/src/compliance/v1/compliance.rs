//! 合规检查API v1
//!
//! 提供合规检查、报告生成等功能。

use crate::compliance::models::*;
use crate::compliance::service::ComplianceService;
use crate::error::{SecurityError, SecurityResult};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// 合规检查API v1
pub struct ComplianceCheckV1API<'a> {
    service: &'a dyn ComplianceService,
}

impl<'a> ComplianceCheckV1API<'a> {
    /// 创建新的合规检查API实例
    pub fn new(service: &'a dyn ComplianceService) -> Self {
        Self { service }
    }

    /// 执行合规检查
    ///
    /// # security_compliance_check_v1
    ///
    /// 执行指定类型的合规性检查。
    pub async fn perform_compliance_check(
        &self,
        request: ComplianceCheckRequest,
    ) -> SecurityResult<ComplianceCheckResponse> {
        info!("执行合规检查: types={:?}", request.compliance_types);

        // 验证请求参数
        if request.compliance_types.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "compliance_types".to_string(),
                reason: "至少需要指定一种合规类型".to_string(),
            });
        }

        // 调用服务层执行合规检查
        let response = self.service.perform_compliance_check(request).await?;

        info!(
            "合规检查完成: check_id={}, overall_score={}",
            response.check_id, response.overall_score
        );
        Ok(response)
    }

    /// 生成合规报告
    ///
    /// # security_compliance_report_v1
    ///
    /// 生成指定时间段的合规性报告。
    pub async fn generate_compliance_report(
        &self,
        request: ComplianceReportRequest,
    ) -> SecurityResult<ComplianceReportResponse> {
        info!(
            "生成合规报告: start_time={:?}, end_time={:?}",
            request.start_time, request.end_time
        );

        // 验证请求参数
        if request.start_time >= request.end_time {
            return Err(SecurityError::InvalidParameter {
                parameter: "time_range".to_string(),
                reason: "开始时间必须早于结束时间".to_string(),
            });
        }

        // 调用服务层生成合规报告
        let response = self.service.generate_compliance_report(request).await?;

        info!("合规报告生成完成: report_id={}", response.report_id);
        Ok(response)
    }
}

/// 合规检查请求构建器
#[derive(Debug, Clone)]
pub struct ComplianceCheckRequestBuilder {
    request: ComplianceCheckRequest,
}

impl ComplianceCheckRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: ComplianceCheckRequest {
                compliance_types: vec![],
                resource_ids: None,
                force_recheck: Some(false),
                include_details: Some(true),
            },
        }
    }

    pub fn compliance_types(mut self, types: Vec<ComplianceType>) -> Self {
        self.request.compliance_types = types;
        self
    }

    pub fn resource_ids(mut self, ids: Vec<String>) -> Self {
        self.request.resource_ids = Some(ids);
        self
    }

    pub fn force_recheck(mut self, force: bool) -> Self {
        self.request.force_recheck = Some(force);
        self
    }

    pub fn include_details(mut self, include: bool) -> Self {
        self.request.include_details = Some(include);
        self
    }

    pub fn build(self) -> ComplianceCheckRequest {
        self.request
    }
}

impl Default for ComplianceCheckRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 合规报告请求构建器
#[derive(Debug, Clone)]
pub struct ComplianceReportRequestBuilder {
    request: ComplianceReportRequest,
}

impl ComplianceReportRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: ComplianceReportRequest {
                start_time: Utc::now() - chrono::Duration::days(30),
                end_time: Utc::now(),
                compliance_types: None,
                include_charts: Some(true),
                report_format: Some(ReportFormat::Json),
            },
        }
    }

    pub fn time_range(
        mut self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.request.start_time = start;
        self.request.end_time = end;
        self
    }

    pub fn compliance_types(mut self, types: Vec<ComplianceType>) -> Self {
        self.request.compliance_types = Some(types);
        self
    }

    pub fn include_charts(mut self, include: bool) -> Self {
        self.request.include_charts = Some(include);
        self
    }

    pub fn report_format(mut self, format: ReportFormat) -> Self {
        self.request.report_format = Some(format);
        self
    }

    pub fn build(self) -> ComplianceReportRequest {
        self.request
    }
}

impl Default for ComplianceReportRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

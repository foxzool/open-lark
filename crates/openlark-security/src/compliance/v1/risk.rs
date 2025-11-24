//! 风险评估API v1
//!
//! 提供风险评估、查询和管理功能。

use crate::compliance::models::*;
use crate::compliance::service::ComplianceService;
use crate::error::{SecurityError, SecurityResult};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// 风险评估API v1
pub struct RiskAssessmentV1API<'a> {
    service: &'a dyn ComplianceService,
}

impl<'a> RiskAssessmentV1API<'a> {
    /// 创建新的风险评估API实例
    pub fn new(service: &'a dyn ComplianceService) -> Self {
        Self { service }
    }

    /// 执行风险评估
    ///
    /// # security_risk_assess_v1
    ///
    /// 对指定资源执行风险评估。
    pub async fn perform_risk_assessment(
        &self,
        request: RiskAssessmentRequest,
    ) -> SecurityResult<RiskAssessmentResponse> {
        info!("执行风险评估: resources={:?}", request.resource_ids);

        // 验证请求参数
        if request.resource_ids.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "resource_ids".to_string(),
                reason: "至少需要指定一个资源ID".to_string(),
            });
        }

        // 调用服务层执行风险评估
        let response = self.service.perform_risk_assessment(request).await?;

        info!(
            "风险评估完成: assessment_id={}, overall_level={:?}",
            response.assessment_id, response.overall_risk_level
        );
        Ok(response)
    }

    /// 查询风险
    ///
    /// # security_risk_query_v1
    ///
    /// 根据条件查询风险记录。
    pub async fn query_risks(
        &self,
        request: RiskQueryRequest,
    ) -> SecurityResult<RiskQueryResponse> {
        info!("查询风险: levels={:?}", request.risk_levels);

        // 验证请求参数
        if let (Some(start), Some(end)) = (request.start_time, request.end_time) {
            if start >= end {
                return Err(SecurityError::InvalidParameter {
                    parameter: "time_range".to_string(),
                    reason: "开始时间必须早于结束时间".to_string(),
                });
            }
        }

        // 调用服务层查询风险
        let response = self.service.query_risks(request).await?;

        info!("风险查询完成: total_count={}", response.total_count);
        Ok(response)
    }

    /// 更新风险状态
    ///
    /// # security_risk_update_status_v1
    ///
    /// 更新指定风险的状态。
    pub async fn update_risk_status(
        &self,
        risk_id: String,
        status: RiskStatus,
    ) -> SecurityResult<bool> {
        info!("更新风险状态: risk_id={}, status={:?}", risk_id, status);

        // 验证风险ID
        if risk_id.is_empty() {
            return Err(SecurityError::InvalidParameter {
                parameter: "risk_id".to_string(),
                reason: "风险ID不能为空".to_string(),
            });
        }

        // 调用服务层更新风险状态
        let result = self.service.update_risk_status(risk_id, status).await?;

        info!("风险状态更新完成: result={}", result);
        Ok(result)
    }

    /// 获取安全指标
    ///
    /// # security_metrics_v1
    ///
    /// 获取指定时间范围内的安全指标。
    pub async fn get_security_metrics(
        &self,
        time_range: crate::compliance::service::TimeRange,
    ) -> SecurityResult<crate::compliance::service::SecurityMetrics> {
        info!("获取安全指标: time_range={:?}", time_range);

        // 验证时间范围
        if time_range.start_time >= time_range.end_time {
            return Err(SecurityError::InvalidParameter {
                parameter: "time_range".to_string(),
                reason: "开始时间必须早于结束时间".to_string(),
            });
        }

        // 调用服务层获取安全指标
        let metrics = self.service.get_security_metrics(time_range).await?;

        info!(
            "安全指标获取完成: total_audit_logs={}, compliance_score={}",
            metrics.total_audit_logs, metrics.compliance_score
        );
        Ok(metrics)
    }
}

/// 风险评估请求构建器
#[derive(Debug, Clone)]
pub struct RiskAssessmentRequestBuilder {
    request: RiskAssessmentRequest,
}

impl RiskAssessmentRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: RiskAssessmentRequest {
                resource_ids: vec![],
                risk_types: None,
                assessment_depth: Some(AssessmentDepth::Standard),
                include_recommendations: Some(true),
            },
        }
    }

    pub fn resource_ids(mut self, ids: Vec<String>) -> Self {
        self.request.resource_ids = ids;
        self
    }

    pub fn risk_types(mut self, types: Vec<RiskType>) -> Self {
        self.request.risk_types = Some(types);
        self
    }

    pub fn assessment_depth(mut self, depth: AssessmentDepth) -> Self {
        self.request.assessment_depth = Some(depth);
        self
    }

    pub fn include_recommendations(mut self, include: bool) -> Self {
        self.request.include_recommendations = Some(include);
        self
    }

    pub fn build(self) -> RiskAssessmentRequest {
        self.request
    }
}

impl Default for RiskAssessmentRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 风险查询请求构建器
#[derive(Debug, Clone)]
pub struct RiskQueryRequestBuilder {
    request: RiskQueryRequest,
}

impl RiskQueryRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: RiskQueryRequest {
                start_time: None,
                end_time: None,
                risk_levels: None,
                risk_types: None,
                risk_statuses: None,
                resource_ids: None,
                page: Some(1),
                page_size: Some(20),
                sort_field: Some("detected_at".to_string()),
                sort_direction: Some(SortDirection::Desc),
            },
        }
    }

    pub fn time_range(
        mut self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.request.start_time = Some(start);
        self.request.end_time = Some(end);
        self
    }

    pub fn risk_levels(mut self, levels: Vec<RiskLevel>) -> Self {
        self.request.risk_levels = Some(levels);
        self
    }

    pub fn risk_types(mut self, types: Vec<RiskType>) -> Self {
        self.request.risk_types = Some(types);
        self
    }

    pub fn risk_statuses(mut self, statuses: Vec<RiskStatus>) -> Self {
        self.request.risk_statuses = Some(statuses);
        self
    }

    pub fn resource_ids(mut self, ids: Vec<String>) -> Self {
        self.request.resource_ids = Some(ids);
        self
    }

    pub fn pagination(mut self, page: u32, page_size: u32) -> Self {
        self.request.page = Some(page);
        self.request.page_size = Some(page_size);
        self
    }

    pub fn sort_field<S: Into<String>>(mut self, field: S) -> Self {
        self.request.sort_field = Some(field.into());
        self
    }

    pub fn sort_direction(mut self, direction: SortDirection) -> Self {
        self.request.sort_direction = Some(direction);
        self
    }

    pub fn build(self) -> RiskQueryRequest {
        self.request
    }
}

impl Default for RiskQueryRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

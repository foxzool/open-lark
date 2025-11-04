//! 安全合规数据模型
//!
//! 定义安全合规相关的数据结构，包括审计日志、OpenAPI日志、
//! 合规检查等请求和响应模型。

use serde::{Deserialize, Serialize};

/// 审计日志查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogQueryRequest {
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 事件类型
    pub event_type: Option<String>,
    /// 用户ID
    pub user_id: Option<String>,
    /// 应用ID
    pub app_id: Option<String>,
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// OpenAPI日志查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenapiLogQueryRequest {
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 应用ID列表
    pub app_ids: Option<Vec<String>>,
    /// API接口列表
    pub apis: Option<Vec<String>>,
    /// 响应码列表
    pub response_codes: Option<Vec<String>>,
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 安全统计请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatsRequest {
    /// 日期范围
    pub date_range: Option<String>,
    /// 指标类型列表
    pub metric_types: Option<Vec<String>>,
    /// 应用ID
    pub app_id: Option<String>,
}

/// 合规检查请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckRequest {
    /// 检查类型
    pub check_type: Option<String>,
    /// 检查范围
    pub scope: Option<String>,
    /// 应用ID
    pub app_id: Option<String>,
    /// 时间范围
    pub time_range: Option<String>,
}

/// 审计日志响应项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogItem {
    /// 日志ID
    pub log_id: String,
    /// 时间戳
    pub timestamp: String,
    /// 事件类型
    pub event_type: String,
    /// 用户ID
    pub user_id: String,
    /// 应用ID
    pub app_id: String,
    /// 操作描述
    pub description: String,
    /// IP地址
    pub ip_address: Option<String>,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 结果状态
    pub status: String,
}

/// OpenAPI日志响应项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenapiLogItem {
    /// 日志ID
    pub log_id: String,
    /// 时间戳
    pub timestamp: String,
    /// 应用ID
    pub app_id: String,
    /// API接口
    pub api_path: String,
    /// HTTP方法
    pub method: String,
    /// 响应码
    pub response_code: i32,
    /// 响应时间（毫秒）
    pub response_time: i32,
    /// 用户ID
    pub user_id: Option<String>,
    /// IP地址
    pub ip_address: Option<String>,
    /// 请求大小
    pub request_size: Option<i64>,
    /// 响应大小
    pub response_size: Option<i64>,
    /// 错误信息
    pub error_message: Option<String>,
}

/// 安全统计数据项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatsItem {
    /// 指标名称
    pub metric_name: String,
    /// 指标值
    pub metric_value: i64,
    /// 统计时间
    pub timestamp: String,
    /// 维度信息
    pub dimensions: Option<serde_json::Value>,
}

/// 合规检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckResult {
    /// 检查类型
    pub check_type: String,
    /// 检查范围
    pub scope: String,
    /// 合规状态
    pub compliance_status: String,
    /// 检查结果
    pub result: String,
    /// 建议措施
    pub recommendations: Option<Vec<String>>,
    /// 检查时间
    pub check_time: String,
    /// 下次检查时间
    pub next_check_time: Option<String>,
}

/// 分页响应信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    /// 是否有下一页
    pub has_more: bool,
    /// 页面标记
    pub page_token: Option<String>,
    /// 页面大小
    pub page_size: i32,
    /// 总数
    pub total: Option<i32>,
}

/// 审计日志查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogQueryResponse {
    /// 审计日志列表
    pub items: Vec<AuditLogItem>,
    /// 分页信息
    pub pagination: Option<PaginationInfo>,
}

/// OpenAPI日志查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenapiLogQueryResponse {
    /// OpenAPI日志列表
    pub items: Vec<OpenapiLogItem>,
    /// 分页信息
    pub pagination: Option<PaginationInfo>,
}

/// 安全统计响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatsResponse {
    /// 统计数据列表
    pub items: Vec<SecurityStatsItem>,
    /// 统计时间范围
    pub time_range: Option<String>,
    /// 总计信息
    pub summary: Option<serde_json::Value>,
}

/// 异常检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectionResult {
    /// 异常类型
    pub anomaly_type: String,
    /// 异常描述
    pub description: String,
    /// 严重程度
    pub severity: String,
    /// 影响范围
    pub impact_scope: Option<Vec<String>>,
    /// 检测时间
    pub detection_time: String,
    /// 建议处理措施
    pub recommended_actions: Option<Vec<String>>,
}

/// API访问模式分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPatternAnalysisResult {
    /// 模式类型
    pub pattern_type: String,
    /// 模式描述
    pub description: String,
    /// 出现频率
    pub frequency: i32,
    /// 时间分布
    pub time_distribution: Option<Vec<String>>,
    /// 用户分布
    pub user_distribution: Option<Vec<String>>,
    /// 趋势分析
    pub trend_analysis: Option<serde_json::Value>,
}

/// 使用构建器模式的审计日志查询请求构建器
pub struct AuditLogQueryBuilder {
    request: AuditLogQueryRequest,
}

impl AuditLogQueryBuilder {
    /// 创建新的查询构建器
    pub fn new() -> Self {
        Self {
            request: AuditLogQueryRequest {
                start_time: None,
                end_time: None,
                event_type: None,
                user_id: None,
                app_id: None,
                page_size: None,
                page_token: None,
            },
        }
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.request.start_time = Some(start_time.into());
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.request.end_time = Some(end_time.into());
        self
    }

    /// 设置事件类型
    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.request.event_type = Some(event_type.into());
        self
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.request.user_id = Some(user_id.into());
        self
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = Some(app_id.into());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置页面标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> AuditLogQueryRequest {
        self.request
    }
}

impl Default for AuditLogQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 使用构建器模式的OpenAPI日志查询请求构建器
pub struct OpenapiLogQueryBuilder {
    request: OpenapiLogQueryRequest,
}

impl OpenapiLogQueryBuilder {
    /// 创建新的查询构建器
    pub fn new() -> Self {
        Self {
            request: OpenapiLogQueryRequest {
                start_time: None,
                end_time: None,
                app_ids: None,
                apis: None,
                response_codes: None,
                page_size: None,
                page_token: None,
            },
        }
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.request.start_time = Some(start_time.into());
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.request.end_time = Some(end_time.into());
        self
    }

    /// 设置应用ID列表
    pub fn app_ids(mut self, app_ids: Vec<String>) -> Self {
        self.request.app_ids = Some(app_ids);
        self
    }

    /// 设置API接口列表
    pub fn apis(mut self, apis: Vec<String>) -> Self {
        self.request.apis = Some(apis);
        self
    }

    /// 设置响应码列表
    pub fn response_codes(mut self, response_codes: Vec<String>) -> Self {
        self.request.response_codes = Some(response_codes);
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置页面标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> OpenapiLogQueryRequest {
        self.request
    }
}

impl Default for OpenapiLogQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 使用构建器模式的安全统计请求构建器
pub struct SecurityStatsBuilder {
    request: SecurityStatsRequest,
}

impl SecurityStatsBuilder {
    /// 创建新的统计请求构建器
    pub fn new() -> Self {
        Self {
            request: SecurityStatsRequest {
                date_range: None,
                metric_types: None,
                app_id: None,
            },
        }
    }

    /// 设置日期范围
    pub fn date_range(mut self, date_range: impl Into<String>) -> Self {
        self.request.date_range = Some(date_range.into());
        self
    }

    /// 设置指标类型列表
    pub fn metric_types(mut self, metric_types: Vec<String>) -> Self {
        self.request.metric_types = Some(metric_types);
        self
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = Some(app_id.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> SecurityStatsRequest {
        self.request
    }
}

impl Default for SecurityStatsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 使用构建器模式的合规检查请求构建器
pub struct ComplianceCheckBuilder {
    request: ComplianceCheckRequest,
}

impl ComplianceCheckBuilder {
    /// 创建新的合规检查构建器
    pub fn new() -> Self {
        Self {
            request: ComplianceCheckRequest {
                check_type: None,
                scope: None,
                app_id: None,
                time_range: None,
            },
        }
    }

    /// 设置检查类型
    pub fn check_type(mut self, check_type: impl Into<String>) -> Self {
        self.request.check_type = Some(check_type.into());
        self
    }

    /// 设置检查范围
    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.request.scope = Some(scope.into());
        self
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = Some(app_id.into());
        self
    }

    /// 设置时间范围
    pub fn time_range(mut self, time_range: impl Into<String>) -> Self {
        self.request.time_range = Some(time_range.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> ComplianceCheckRequest {
        self.request
    }
}

impl Default for ComplianceCheckBuilder {
    fn default() -> Self {
        Self::new()
    }
}
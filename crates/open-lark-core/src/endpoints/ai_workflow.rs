//! ai_workflow 服务端点常量定义
//!
//! AI智能工作流相关 API 端点常量，包括：
//! - 智能工作流创建和管理
//! - 自动化决策流程
//! - 智能业务流程
//! - 工作流模板管理

/// 创建智能工作流
pub const CREATE_AI_WORKFLOW: &str = "/open-apis/ai_workflow/v1/workflow/create";

/// 执行智能工作流
pub const EXECUTE_AI_WORKFLOW: &str = "/open-apis/ai_workflow/v1/workflow/execute";

/// 获取智能工作流详情
pub const GET_AI_WORKFLOW: &str = "/open-apis/ai_workflow/v1/workflow/{workflow_id}";

/// 更新智能工作流
pub const UPDATE_AI_WORKFLOW: &str = "/open-apis/ai_workflow/v1/workflow/{workflow_id}";

/// 删除智能工作流
pub const DELETE_AI_WORKFLOW: &str = "/open-apis/ai_workflow/v1/workflow/{workflow_id}";

/// 列出智能工作流
pub const LIST_AI_WORKFLOWS: &str = "/open-apis/ai_workflow/v1/workflows";

/// 智能工作流模板创建
pub const CREATE_WORKFLOW_TEMPLATE: &str = "/open-apis/ai_workflow/v1/template/create";

/// 获取工作流模板
pub const GET_WORKFLOW_TEMPLATE: &str = "/open-apis/ai_workflow/v1/template/{template_id}";

/// 工作流模板市场
pub const WORKFLOW_TEMPLATE_MARKET: &str = "/open-apis/ai_workflow/v1/template/market";

/// 智能决策支持
pub const INTELLIGENT_DECISION: &str = "/open-apis/ai_workflow/v1/decision/support";

/// 业务流程自动化
pub const BUSINESS_PROCESS_AUTOMATION: &str = "/open-apis/ai_workflow/v1/process/automation";

/// 智能任务分配
pub const SMART_TASK_ASSIGNMENT: &str = "/open-apis/ai_workflow/v1/task/assignment";

/// 智能优先级排序
pub const INTELLIGENT_PRIORITY: &str = "/open-apis/ai_workflow/v1/priority/sort";

/// 工作流执行监控
pub const WORKFLOW_EXECUTION_MONITOR: &str = "/open-apis/ai_workflow/v1/execution/monitor";

/// 工作流执行历史
pub const WORKFLOW_EXECUTION_HISTORY: &str = "/open-apis/ai_workflow/v1/execution/history";

/// 工作流性能分析
pub const WORKFLOW_PERFORMANCE_ANALYSIS: &str = "/open-apis/ai_workflow/v1/performance/analysis";

/// 智能异常处理
pub const INTELLIGENT_EXCEPTION_HANDLING: &str = "/open-apis/ai_workflow/v1/exception/handle";

/// 工作流优化建议
pub const WORKFLOW_OPTIMIZATION_SUGGESTION: &str = "/open-apis/ai_workflow/v1/optimization/suggest";
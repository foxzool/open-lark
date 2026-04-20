/// 自动化流程模块
pub mod list;
/// update 子模块。
pub mod update;

/// 重新导出相关类型。
pub use list::{ListWorkflowRequest, ListWorkflowResponse, Workflow};
/// 重新导出相关类型。
pub use update::{
    UpdateWorkflowBody, UpdateWorkflowRequest, UpdateWorkflowResponse, WorkflowStatus,
};

// Type alias for compatibility
/// 兼容旧版接口的服务类型占位符。
pub type ServiceType = ();

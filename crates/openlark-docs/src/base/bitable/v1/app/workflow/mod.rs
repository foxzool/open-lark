/// 自动化流程模块
pub mod list;
pub mod update;

pub use list::{ListWorkflowRequest, ListWorkflowResponse, Workflow};
pub use update::{UpdateWorkflowRequest, UpdateWorkflowResponse, WorkflowStatus};

// Type alias for compatibility
pub type ServiceType = ();

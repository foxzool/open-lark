pub use app_dashboard::AppDashboardService;
pub use app_role::AppRoleService;
pub use app_role_member::AppRoleMemberService;
pub use app_table_field::AppTableFieldService;
pub use app_workflow::AppWorkflowService;
pub use form::FormService;
// 为了避免名称冲突，使用模块路径而不是全局导入
pub mod app;
pub mod app_table;
pub mod app_table_record;
pub mod app_table_view;
use config::Config;
mod app_dashboard;
mod app_role;
mod app_role_member;
mod app_table_field;
mod app_workflow;
mod form;
mod share;
pub use share::*;
pub struct V1 {
}

impl V1 {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
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

use crate::core::config::Config;

mod app_dashboard;
mod app_role;
mod app_role_member;
mod app_table_field;
mod app_workflow;
mod form;

mod share;
pub use share::*;

pub struct V1 {
    /// 多维表格
    pub app: app::AppService,
    /// 数据表
    pub app_table: app_table::AppTableService,
    /// 视图
    pub app_table_view: app_table_view::AppTableViewService,
    /// 字段
    pub app_table_field: AppTableFieldService,
    /// 记录
    pub app_table_record: app_table_record::AppTableRecordService,
    /// 仪表盘
    pub app_dashboard: AppDashboardService,
    /// 表单
    pub form: FormService,
    /// 自定义角色
    pub app_role: AppRoleService,
    /// 协作者
    pub app_role_member: AppRoleMemberService,
    /// 自动化流程
    pub app_workflow: AppWorkflowService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            app: app::AppService::new(config.clone()),
            app_table: app_table::AppTableService::new(config.clone()),
            app_table_view: app_table_view::AppTableViewService::new(config.clone()),
            app_table_field: AppTableFieldService::new(config.clone()),
            app_table_record: app_table_record::AppTableRecordService::new(config.clone()),
            app_dashboard: AppDashboardService::new(config.clone()),
            form: FormService::new(config.clone()),
            app_role: AppRoleService::new(config.clone()),
            app_role_member: AppRoleMemberService::new(config.clone()),
            app_workflow: AppWorkflowService::new(config),
        }
    }
}

//! 多维表格服务模块
//!
//! 提供完整的多维表格操作功能，包括：
//! - 应用管理（创建、复制、删除、元数据）
//! - 数据表管理（CRUD操作）
//! - 记录操作（增删改查）
//! - 字段管理
//! - 视图管理
//! - 权限管理
//! - 仪表板管理
//! - 表单管理
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::{BitableService, app::AppService};
//!
//! let service = BitableService::new(config);
//!
//! // 通过服务访问应用管理功能
//! let app_service = service.app();
//!
//! // 创建新的多维表格
//! let response = app_service
//!     .create_app_builder()
//!     .name("项目管理表")
//!     .execute(&app_service)
//!     .await?;
//! ```

use openlark_core::{
    config::Config,
};

// 导入子模块
pub mod app;
pub mod app_table;
pub mod app_table_record;
pub mod app_table_field;
pub mod app_table_view;
pub mod app_role;
pub mod app_role_member;
pub mod app_dashboard;
pub mod app_form;
pub mod app_table_form_field;
pub mod app_workflow;
pub mod advanced_permission;
pub mod v2;

// 重新导出主要类型
pub use app::{AppService, AppInfo, CreateAppRequestBuilder, UpdateAppRequestBuilder};
pub use app_table::{TableInfo, CreateTableRequestBuilder, UpdateTableRequestBuilder};
pub use app_table_record::{RecordInfo, CreateRecordRequestBuilder, UpdateRecordRequestBuilder};
pub use app_table_field::{FieldInfo, CreateFieldRequestBuilder, UpdateFieldRequestBuilder};
pub use app_table_view::{ViewInfo, CreateViewRequestBuilder, UpdateViewRequestBuilder};
pub use app_role::{RoleInfo, CreateRoleRequestBuilder, UpdateRoleRequestBuilder};
pub use app_role_member::{RoleMemberInfo, CreateRoleMemberRequestBuilder};
pub use app_dashboard::{DashboardInfo, CreateDashboardRequestBuilder};
pub use app_form::{FormInfo, GetFormRequestBuilder, PatchFormMetaRequestBuilder, PatchFormQuestionRequestBuilder};
pub use app_table_form_field::{AppTableFormFieldService, ListFormFieldRequestBuilder, PatchFormFieldRequestBuilder};
pub use app_workflow::{AppWorkflowService, ListWorkflowRequestBuilder, UpdateWorkflowRequestBuilder};
pub use advanced_permission::{AdvancedPermissionService, ListRolesV2RequestBuilder, CreateRoleV2RequestBuilder, UpdateRoleV2RequestBuilder};
pub use v2::{BitableV2Service, AdvancedSearchRequestBuilder, BulkOperationRequestBuilder, SmartQueryRequestBuilder};

/// 多维表格服务
#[derive(Debug, Clone)]
pub struct BitableService {
    config: Config,
}

impl BitableService {
    /// 创建多维表格服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取应用管理服务
    pub fn app(&self) -> AppService {
        AppService::new(self.config.clone())
    }

    /// 获取数据表管理服务
    pub fn app_table(&self) -> app_table::AppTableService {
        app_table::AppTableService::new(self.config.clone())
    }

    /// 获取数据记录管理服务
    pub fn app_table_record(&self) -> app_table_record::AppTableRecordService {
        app_table_record::AppTableRecordService::new(self.config.clone())
    }

    /// 获取字段管理服务
    pub fn app_table_field(&self) -> app_table_field::AppTableFieldService {
        app_table_field::AppTableFieldService::new(self.config.clone())
    }

    /// 获取视图管理服务
    pub fn app_table_view(&self) -> app_table_view::AppTableViewService {
        app_table_view::AppTableViewService::new(self.config.clone())
    }

    /// 获取角色权限管理服务
    pub fn app_role(&self) -> app_role::AppRoleService {
        app_role::AppRoleService::new(self.config.clone())
    }

    /// 获取角色成员管理服务
    pub fn app_role_member(&self) -> app_role_member::AppRoleMemberService {
        app_role_member::AppRoleMemberService::new(self.config.clone())
    }

    /// 获取仪表板管理服务
    pub fn app_dashboard(&self) -> app_dashboard::AppDashboardService {
        app_dashboard::AppDashboardService::new(self.config.clone())
    }

    /// 获取 V2 服务（高级功能）
    pub fn v2(&self) -> v2::BitableV2Service {
        v2::BitableV2Service::new(self.config.clone())
    }

    /// 获取表单管理服务
    pub fn app_form(&self) -> app_form::AppFormService {
        app_form::AppFormService::new(self.config.clone())
    }

    /// 获取表单字段管理服务
    pub fn app_table_form_field(&self) -> app_table_form_field::AppTableFormFieldService {
        app_table_form_field::AppTableFormFieldService::new(self.config.clone())
    }

    /// 获取工作流管理服务
    pub fn app_workflow(&self) -> app_workflow::AppWorkflowService {
        app_workflow::AppWorkflowService::new(self.config.clone())
    }

    /// 获取高级权限管理服务
    pub fn advanced_permission(&self) -> advanced_permission::AdvancedPermissionService {
        advanced_permission::AdvancedPermissionService::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitable_service_creation() {
        let config = Config::default();
        let service = BitableService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_submodules() {
        let config = Config::default();
        let service = BitableService::new(config);

        // 测试子模块访问
        let _app_service = service.app();
        let _table_service = service.app_table();
        let _record_service = service.app_table_record();
        let _field_service = service.app_table_field();
        let _view_service = service.app_table_view();
        let _role_service = service.app_role();
        let _role_member_service = service.app_role_member();
        let _dashboard_service = service.app_dashboard();
        let _form_service = service.app_form();
        let _form_field_service = service.app_table_form_field();
        let _workflow_service = service.app_workflow();
        let _advanced_permission_service = service.advanced_permission();
        let _v2_service = service.v2();
    }
}
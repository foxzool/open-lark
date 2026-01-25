use openlark_core::config::Config;

// 导入Base模块的RoleService
// #[cfg(feature = "base")]
// use crate::base::v2::app::role::RoleService; // Disabled: base::v2 module is disabled

// 应用级别API
pub mod copy;
pub mod create;
pub mod get;
pub mod models;
pub mod update;

// 子模块API
pub mod dashboard;
pub mod role;
pub mod table;
pub mod workflow;

// 仪表盘相关 - 选择性导入以避免命名冲突
pub use dashboard::AppDashboardService;

// 数据表相关 - 选择性导入以避免命名冲突
pub use table::TableService;

// 角色相关 - 选择性导入以避免命名冲突
pub use role::{
    AppRoleService, BatchCreateRoleMemberRequest, BatchCreateRoleMemberResponse,
    BatchDeleteRoleMemberRequest, BatchDeleteRoleMemberResponse, CreateAppRoleRequest,
    CreateAppRoleRequestBody, CreateAppRoleResponse, CreateRoleMemberRequest,
    CreateRoleMemberResponse, DeleteAppRoleRequest, DeleteAppRoleResponse, ListAppRoleRequest,
    ListAppRoleResponse, RoleMemberInfo, RoleMemberService, UpdateAppRoleRequest,
    UpdateAppRoleRequestBody, UpdateAppRoleResponse,
};

// 工作流相关 - 选择性导入以避免命名冲突
pub use workflow::{
    AppWorkflowService, ListWorkflowRequest, ListWorkflowResponse, UpdateWorkflowRequest,
    UpdateWorkflowResponse,
};

// table 模块显式导出
pub use table::{
    BatchCreateRecordRequest,
    BatchCreateRecordResponse,
    BatchCreateTableRequest,
    BatchCreateTableResponse,
    BatchDeleteRecordRequest,
    BatchDeleteRecordResponse,
    BatchDeleteTableRequest,
    BatchDeleteTableResponse,
    BatchGetRecordRequest,
    BatchGetRecordResponse,
    BatchUpdateRecordRequest,
    BatchUpdateRecordResponse,
    CreateFieldRequest,
    CreateFieldResponse,
    CreateRecordItem,
    CreateRecordRequest,
    CreateRecordResponse,
    CreateTableRequest,
    CreateTableResponse,
    CreateViewData,
    CreateViewRequest,
    CreateViewResponse,
    DeleteFieldRequest,
    DeleteFieldResponse,
    DeleteRecordRequest,
    DeleteRecordRequestBuilder,
    DeleteRecordResponse,
    DeleteTableRequest,
    DeleteTableResponse,
    DeleteViewRequest,
    DeleteViewResponse,
    DeletedRecord,
    Field,
    FieldDescription,
    FieldProperty,
    FieldType,
    FilterCondition,
    FilterInfo,
    Form,
    FormFieldQuestion,
    GetFormRequest,
    GetFormResponse,
    GetRecordRequest,
    GetRecordResponse,
    GetViewRequest,
    GetViewResponse,
    ListFieldRequest,
    ListFieldResponse,
    ListFormFieldQuestionRequest,
    ListFormFieldQuestionResponse,
    ListRecordRequest,
    ListRecordResponse,
    ListTablesRequest,
    ListTablesRequestBuilder,
    ListTablesResponse,
    ListViewsRequest,
    ListViewsRequestBuilder,
    ListViewsResponse,
    PatchFormFieldQuestionBuilder,
    PatchFormFieldQuestionRequest,
    PatchFormFieldQuestionResponse,
    PatchFormFieldRequest,
    PatchFormRequest,
    PatchFormResponse,
    PatchTableRequest,
    PatchTableResponse,
    PatchViewData,
    PatchViewRequest,
    PatchViewResponse,
    PatchedFormFieldQuestion,
    Person,
    Record,
    SearchRecordRequest,
    SearchRecordRequestBody,
    SearchRecordRequestBuilder,
    SearchRecordResponse,
    SortCondition,
    TableData,
    TableField,
    TableInfo,
    UpdateFieldRequest,
    UpdateFieldResponse,
    UpdateRecordItem,
    UpdateRecordRequest,
    UpdateRecordRequestBuilder,
    UpdateRecordResponse,
    View,
    app_token,
    automatic_fields,
    build,
    client_token,
    date,
    description,
    display_formula_ref,
    execute,
    execute_with_options,
    fetch_all,
    field_id,
    field_name,
    field_names,
    field_type,
    fields,
    filter,
    form_id,
    gallery_view,
    gantt_view,
    grid_view,
    ignore_consistency_check,
    kanban_view,
    multi_select,
    name,
    new,
    number,
    page_size,
    page_token,
    payload,
    pre_field_id,
    property,
    record_id,
    record_ids,
    records,
    required,
    shared,
    shared_limit,
    single_select,
    sort,
    submit_limit_once,
    table,
    table_id,
    table_ids,
    tables,
    text,
    text_field_as_array,
    title,
    ui_type,
    user_id_type,
    validate,
    view,
    view_id,
    visible,
    with_default_view_name,
    with_fields,
    with_property,
    with_shared_url,
    with_view_type,
};

/// 多维表格应用服务
pub struct AppService {
    pub config: Config,
}

impl AppService {
    /// 创建应用服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取数据表服务
    pub fn table_service(&self) -> table::TableService {
        table::TableService::new(self.config.clone())
    }

    // /// 获取角色服务
    // #[cfg(feature = "base")]
    // pub fn role_service(&self) -> RoleService {
    //     RoleService::new(self.config.clone())
    // } // Disabled: RoleService from base::v2 module is disabled

    /// 获取仪表盘服务
    pub fn dashboard_service(&self) -> dashboard::AppDashboardService {
        dashboard::AppDashboardService::new(self.config.clone())
    }

    /// 获取工作流服务
    pub fn workflow_service(&self) -> workflow::AppWorkflowService {
        workflow::AppWorkflowService::new(self.config.clone())
    }
}

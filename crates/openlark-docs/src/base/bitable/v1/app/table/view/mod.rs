/// 视图管理服务模块
///
/// 提供多维表格视图的创建、更新、删除和查询功能。
use openlark_core::config::Config;

// 导入子模块
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod patch;

// 重新导出所有模块内容
// create 模块显式导出
pub use create::{
    CreateViewData,
    CreateViewRequest,
    CreateViewResponse,
    DeleteViewRequest,
    DeleteViewResponse,
    GetViewRequest,
    GetViewResponse,
    ListViewsRequest,
    ListViewsRequestBuilder,
    ListViewsResponse,
    PatchViewData,
    PatchViewRequest,
    PatchViewResponse,
    View,
    app_token,
    build,
    execute,
    execute_with_options,
    gallery_view,
    gantt_view,
    grid_view,
    kanban_view,
    new,
    page_size,
    page_token,
    payload,
    table_id,
    user_id_type,
    view,
    view_id,
    with_property,
    with_view_type,
};
// delete 模块显式导出
pub use delete::{
    CreateViewData,
    CreateViewRequest,
    CreateViewResponse,
    DeleteViewRequest,
    DeleteViewResponse,
    GetViewRequest,
    GetViewResponse,
    ListViewsRequest,
    ListViewsRequestBuilder,
    ListViewsResponse,
    PatchViewData,
    PatchViewRequest,
    PatchViewResponse,
    View,
    app_token,
    build,
    execute,
    execute_with_options,
    gallery_view,
    gantt_view,
    grid_view,
    kanban_view,
    new,
    page_size,
    page_token,
    payload,
    table_id,
    user_id_type,
    view,
    view_id,
    with_property,
    with_view_type,
};
// get 模块显式导出
pub use get::{
    CreateViewData,
    CreateViewRequest,
    CreateViewResponse,
    DeleteViewRequest,
    DeleteViewResponse,
    GetViewRequest,
    GetViewResponse,
    ListViewsRequest,
    ListViewsRequestBuilder,
    ListViewsResponse,
    PatchViewData,
    PatchViewRequest,
    PatchViewResponse,
    View,
    app_token,
    build,
    execute,
    execute_with_options,
    gallery_view,
    gantt_view,
    grid_view,
    kanban_view,
    new,
    page_size,
    page_token,
    payload,
    table_id,
    user_id_type,
    view,
    view_id,
    with_property,
    with_view_type,
};
// list 模块显式导出
pub use list::{
    CreateViewData,
    CreateViewRequest,
    CreateViewResponse,
    DeleteViewRequest,
    DeleteViewResponse,
    GetViewRequest,
    GetViewResponse,
    ListViewsRequest,
    ListViewsRequestBuilder,
    ListViewsResponse,
    PatchViewData,
    PatchViewRequest,
    PatchViewResponse,
    View,
    app_token,
    build,
    execute,
    execute_with_options,
    gallery_view,
    gantt_view,
    grid_view,
    kanban_view,
    new,
    page_size,
    page_token,
    payload,
    table_id,
    user_id_type,
    view,
    view_id,
    with_property,
    with_view_type,
};
// patch 模块显式导出
pub use patch::{
    CreateViewData,
    CreateViewRequest,
    CreateViewResponse,
    DeleteViewRequest,
    DeleteViewResponse,
    GetViewRequest,
    GetViewResponse,
    ListViewsRequest,
    ListViewsRequestBuilder,
    ListViewsResponse,
    PatchViewData,
    PatchViewRequest,
    PatchViewResponse,
    View,
    app_token,
    build,
    execute,
    execute_with_options,
    gallery_view,
    gantt_view,
    grid_view,
    kanban_view,
    new,
    page_size,
    page_token,
    payload,
    table_id,
    user_id_type,
    view,
    view_id,
    with_property,
    with_view_type,
};

/// 视图服务
pub struct AppTableViewService {
    config: Config,
}

impl AppTableViewService {
    /// 创建视图服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// Type alias for compatibility
pub type ServiceType = AppTableViewService;

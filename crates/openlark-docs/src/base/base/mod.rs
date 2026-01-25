/// base 项目模块
///
/// base 项目当前仅包含 `base/v2`（自定义角色）相关接口。
pub mod v2;

// v2 模块显式导出
pub use v2::{
    AppRole,
    Create,
    CreateReq,
    CreateResp,
    List,
    ListReq,
    ListResp,
    Update,
    UpdateReq,
    UpdateResp,
    app_token,
    base_rule,
    block_roles,
    execute,
    execute_with_options,
    new,
    page_size,
    page_token,
    role_id,
    role_name,
    table_roles,
};

/// Wiki V2 空间设置模块
pub mod update;

// 导出所有子模块内容，避免命名冲突
// update 模块显式导出
pub use update::{
    UpdateWikiSpaceSettingRequest,
    UpdateWikiSpaceSettingResponse,
    execute,
    execute_with_options,
    new,
    space_id,
};

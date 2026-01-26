/// 文件评论管理模块
use openlark_core::{validate_required, SDKResult};

pub mod batch_query;
pub mod create;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;
pub mod reply;

// 验证评论文件类型的辅助函数
pub fn validate_comment_file_type_for_list_like(file_type: &str) -> SDKResult<()> {
    validate_required!(file_type.trim(), "file_type不能为空");
    Ok(())
}

pub fn validate_comment_file_type_for_create(file_type: &str) -> SDKResult<()> {
    validate_required!(file_type.trim(), "file_type不能为空");
    Ok(())
}

pub fn validate_comment_file_type_for_get(file_type: &str) -> SDKResult<()> {
    validate_required!(file_type.trim(), "file_type不能为空");
    Ok(())
}

// 使用通配符导出所有子模块,避免维护大量重复的导出列表
pub use batch_query::*;
pub use create::*;
pub use get::*;
pub use list::*;
pub use models::*;
pub use patch::*;
pub use reply::*;

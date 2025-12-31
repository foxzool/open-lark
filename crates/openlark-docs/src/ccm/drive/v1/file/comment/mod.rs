/// 文件评论管理模块
pub mod batch_query;
pub mod create;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;
pub mod reply;

// 重新导出所有API函数
pub use batch_query::*;
pub use create::*;
pub use get::*;
pub use list::*;
pub use patch::*;
pub use reply::*;

// 仅导出高层模型，避免与 reply/models.rs 的类型名冲突（TextRun/DocsLink/Person 等）
pub use models::{Comment, CreateCommentReply, CreateCommentReplyList};

pub(crate) fn validate_comment_file_type_for_list_like(file_type: &str) -> openlark_core::SDKResult<()> {
    match file_type {
        "doc" | "docx" | "sheet" | "file" | "slides" => Ok(()),
        _ => Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 仅支持 doc/docx/sheet/file/slides",
        )),
    }
}

pub(crate) fn validate_comment_file_type_for_get(file_type: &str) -> openlark_core::SDKResult<()> {
    match file_type {
        "doc" | "docx" | "sheet" | "file" => Ok(()),
        _ => Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 仅支持 doc/docx/sheet/file",
        )),
    }
}

pub(crate) fn validate_comment_file_type_for_create(file_type: &str) -> openlark_core::SDKResult<()> {
    match file_type {
        "doc" | "docx" => Ok(()),
        _ => Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 仅支持 doc/docx",
        )),
    }
}

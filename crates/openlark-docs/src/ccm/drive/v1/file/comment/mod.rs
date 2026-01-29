/// 文件评论管理模块
///
/// 提供云文档评论的增删改查功能。
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

// 显式导出 - 避免使用 glob reexport
pub use batch_query::{BatchQueryCommentRequest, BatchQueryCommentResponse};

pub use create::CreateCommentRequest;

pub use get::{GetCommentRequest, GetCommentResponse};

pub use list::{ListCommentsRequest, ListCommentsResponse};

pub use models::{
    Comment, CommentContent, CommentElement, CreateCommentReply, CreateCommentReplyList, DocsLink,
    Person, ReplyExtra, TextRun,
};

pub use patch::{PatchCommentRequest, PatchCommentResponse};

pub use reply::{
    DeleteCommentReplyRequest, DeleteCommentReplyResponse, ListCommentReplyRequest,
    ListCommentReplyResponse,
};

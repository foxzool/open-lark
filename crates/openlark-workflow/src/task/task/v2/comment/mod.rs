//! 评论 API v2（strict 路径）

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod patch;

pub use create::CreateCommentRequest;
pub use delete::DeleteCommentRequest;
pub use get::GetCommentRequest;
pub use list::ListCommentsRequest;
pub use models::{CommentItem, CommentMember, GetCommentResponse, ListCommentsResponse};
pub use patch::PatchCommentRequest;

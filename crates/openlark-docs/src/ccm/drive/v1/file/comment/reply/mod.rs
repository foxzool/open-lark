/// 评论回复管理模块
use openlark_core::{validate_required, SDKResult};

pub mod delete;
pub mod list;
pub mod models;
pub mod update;

// 验证评论文件类型的辅助函数
pub fn validate_comment_file_type_for_list_like(file_type: &str) -> SDKResult<()> {
    validate_required!(file_type.trim(), "file_type不能为空");
    Ok(())
}

// 使用通配符导出所有子模块
// delete 模块显式导出
pub use delete::{DeleteCommentReplyRequest, DeleteCommentReplyResponse};
// list 模块显式导出
pub use list::{ListCommentReplyRequest, ListCommentReplyResponse};
// models 模块显式导出
pub use models::{DocsLink, Person, ReplyContent, ReplyElement, ReplyExtra, ReplyInfo, TextRun};
// update 模块显式导出
pub use update::{UpdateReplyRequest, UpdateReplyResponse};

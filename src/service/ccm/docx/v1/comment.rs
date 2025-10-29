// docx v1 comment - 评论操作API,
//,
// 实现文档评论的操作,
use crate::prelude::*;
/// 评论操作服务
#[derive(.*?)]
pub struct CommentService {
    client: std::sync::Arc<LarkClient>,
}
impl CommentService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client },
}
}
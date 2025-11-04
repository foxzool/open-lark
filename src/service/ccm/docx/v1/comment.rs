#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// docx v1 comment - 评论操作API,
//,
// 实现文档评论的操作,
use crate::prelude::*;
/// 评论操作服务
#[derive(Debug, Clone)]
pub struct CommentService {
    client: std::sync::Arc<LarkClient>,
}
impl CommentService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
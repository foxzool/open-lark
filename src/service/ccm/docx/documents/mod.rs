#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// docx documents - 文档操作API
//,
// 提供文档操作的功能
use crate::prelude::*;
/// 文档操作服务
#[derive(Debug, Clone)]
pub struct DocumentsService {
    client: std::sync::Arc<LarkClient>,
}
impl DocumentsService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}
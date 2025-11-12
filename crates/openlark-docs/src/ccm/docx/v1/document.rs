#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// docx v1 document - 文档操作API,
//,
// 实现文档级别的操作,
use openlark_core::prelude::*;
/// 文档操作服务
#[derive(Debug, Clone)]
pub struct DocumentService {
    client: std::sync::Arc<LarkClient>,
}
impl DocumentService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
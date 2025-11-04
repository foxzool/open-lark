#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// drive v1 meta - 元数据服务,
//,
// 提供文件元数据相关的功能,
use crate::prelude::*;
/// 元数据服务
#[derive(Debug, Clone)]
pub struct MetaService {
    client: std::sync::Arc<LarkClient>,
}
impl MetaService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
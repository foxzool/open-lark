#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// sessions - 会话管理服务
//,
// 提供会话管理相关的功能
use crate::prelude::*;
/// 会话管理服务
#[derive(Debug, Clone)]
pub struct SessionsService {
    client: std::sync::Arc<crate::client::LarkClient>,
}
impl SessionsService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}
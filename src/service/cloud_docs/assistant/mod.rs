#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::config::Config;
pub use v1::V1;
pub mod v1;
/// 云文档助手服务
pub struct AssistantService {
}

impl AssistantService {
}
/// 使用共享配置（实验性）
    pub fn new_from_shared() -> Self {
Self {
            v1: V1::new(shared.as_ref().clone())}
}
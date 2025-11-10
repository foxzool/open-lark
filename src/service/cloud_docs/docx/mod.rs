#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::config::Config;
pub mod v1;
pub struct DocxService {
}    pub v1: v1::V1}
impl DocxService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 使用共享配置（实验性）
    pub fn new_from_shared() -> Self {
DocxService {,
            v1: v1::V1::new(shared.as_ref().clone())}
}
use crate::core::config::Config;
pub use v2::V2;
pub mod v2;
pub struct WikiService {
}

impl WikiService {
}
/// 使用共享配置创建服务（实验性）
    pub fn new_from_shared() -> Self {
Self {
            v2: V2::new(shared.as_ref().clone())}
}
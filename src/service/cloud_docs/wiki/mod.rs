use crate::core::config::Config;
pub use v2::V2;
pub mod v2;
pub struct WikiService {
    /// v2 API
    pub v2: V2,
}
impl WikiService {
    pub fn new() -> Self {
Self {,
            v2: V2::new(config.clone()),
        }
}
/// 使用共享配置创建服务（实验性）
    pub fn new_from_shared() -> Self {
Self {,
            v2: V2::new(shared.as_ref().clone()),
        }
}
}

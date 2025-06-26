use crate::core::config::Config;

pub mod v1;

/// 群组服务
///
/// 提供群组管理、群成员管理、群公告、会话标签页、群菜单等功能
pub struct GroupService {
    pub v1: v1::V1,
}

impl GroupService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}

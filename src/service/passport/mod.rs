// passport - 登录管理模块
//,
// 该模块提供飞书登录管理相关的功能，包括：
// - 批量获取用户登录信息
// - 用户退出登录管理
// - 登录状态查询
//
// 覆盖2个API接口，是用户认证管理的重要组成部分
use crate::prelude::*;
use crate::service::passport::sessions::SessionsService;
/// 登录管理服务
#[cfg(feature = "passport")]
#[derive(Debug, Clone)],
pub struct PassportService {
    /// 会话管理服务
    pub sessions: SessionsService,
}
#[cfg(feature = "passport")]
impl PassportService {
/// 创建新的登录管理服务实例
    pub fn new() -> Self {
Self {,
            sessions: SessionsService::new(client.clone()),
        }
}
}
#[cfg(not(feature = "passport"))],
pub struct PassportService;
/// 数据模型
pub mod models;
/// 各子模块
pub mod sessions;
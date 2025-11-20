//! Drive V2 API模块
//!
//! 提供Drive V2版本的API功能框架

use crate::prelude::*;

/// Drive V2 API服务
#[derive(Clone, Debug)]
pub struct DriveV2Service {
    client: std::sync::Arc<LarkClient>,
}

impl DriveV2Service {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for DriveV2Service {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

// 注意：子模块暂时被禁用，因为需要进一步开发
// - explorer: 文件浏览器功能

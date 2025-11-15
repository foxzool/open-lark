//! 资源浏览器API模块
//!
//! 提供云空间资源浏览器相关的功能

use crate::prelude::*;

/// 资源浏览器服务
#[derive(Clone)]
pub struct ExplorerService {
    client: std::sync::Arc<LarkClient>,
}

impl ExplorerService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for ExplorerService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
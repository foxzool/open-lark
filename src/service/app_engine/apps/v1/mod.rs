// apps v1 - 应用管理v1版本API
//
// 包含应用管理的完整功能

use crate::prelude::*;

/// 应用管理v1版本服务
#[derive(Debug, Clone)]
pub struct AppsV1Service {
    client: std::sync::Arc<crate::client::LarkClient>,
}

impl AppsV1Service {
    pub fn new(client: std::sync::Arc<crate::client::LarkClient>) -> Self {
        Self { client }
    }
}
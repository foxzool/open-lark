// sessions - 会话管理服务
//
// 提供会话管理相关的功能

use crate::prelude::*;

/// 会话管理服务
#[derive(Debug, Clone)]
pub struct SessionsService {
    client: std::sync::Arc<crate::client::LarkClient>,
}

impl SessionsService {
    pub fn new(client: std::sync::Arc<crate::client::LarkClient>) -> Self {
        Self { client }
    }
}
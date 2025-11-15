//! 权限管理API模块
//!
//! 提供云空间文件权限管理相关的功能

use crate::prelude::*;

/// 权限管理服务
#[derive(Clone)]
pub struct PermissionService {
    client: std::sync::Arc<LarkClient>,
}

impl PermissionService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for PermissionService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
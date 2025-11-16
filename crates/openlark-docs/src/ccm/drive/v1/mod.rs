//! Drive V1 API模块
//!
//! 提供Drive V1版本的API功能框架

use crate::prelude::*;

/// Drive V1 API服务
#[derive(Clone)]
pub struct DriveV1Service {
    client: std::sync::Arc<LarkClient>,
}

impl DriveV1Service {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}

impl std::ops::Deref for DriveV1Service {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

// 注意：子模块暂时被禁用，因为存在语法错误
// 需要后续修复以下模块：
// - file: 文件操作
// - file_version: 文件版本管理
// - files: 批量文件操作
// - folder: 文件夹管理
// - meta: 元数据管理
// - statistics: 统计信息
// - view_record: 查看记录
// - event: 事件处理
// - permissions: 权限管理
// 等等...

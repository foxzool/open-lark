// core v1 departments - 部门管理API
//
// 实现部门管理的功能

use crate::prelude::*;

/// 部门管理服务
#[derive(Debug, Clone)]
pub struct DepartmentsService {
    client: std::sync::Arc<LarkClient>,
}

impl DepartmentsService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}
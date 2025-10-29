// core v1 positions - 职位管理API
//
// 实现职位管理的功能

use crate::prelude::*;

/// 职位管理服务
#[derive(Debug, Clone)]
pub struct PositionsService {
    client: std::sync::Arc<LarkClient>,
}

impl PositionsService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}
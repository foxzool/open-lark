// drive v1 view_record - 访问记录服务
//
// 提供文件访问记录相关的功能

use crate::prelude::*;

/// 访问记录服务
#[derive(Debug, Clone)]
pub struct ViewRecordService {
    client: std::sync::Arc<LarkClient>,
}

impl ViewRecordService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }
}
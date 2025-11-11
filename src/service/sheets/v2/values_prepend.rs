//! 表格数据前置插入服务
//!
//! 提供飞书电子表格v2版本的数据前置插入功能。

use serde::{Deserialize, Serialize};

/// 数据前置插入请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesPrependRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 数据
    pub values: ValuesData,
}

/// 数据格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesData {
    /// 数据内容
    pub data: Vec<Vec<String>>,
}

/// 数据前置插入服务
#[derive(Debug, Clone)]
pub struct ValuesPrependService {
    // 占位符实现
}

impl ValuesPrependService {
    /// 创建新的服务实例
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        let service = ValuesPrependService::new();
        assert!(true); // 占位符测试
    }
}
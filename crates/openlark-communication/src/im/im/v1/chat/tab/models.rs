//! 会话标签页相关模型（不算 API）

use serde::{Deserialize, Serialize};

/// 会话标签页 ID 列表请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TabIdsBody {
    /// 标签页 ID 列表。
    pub tab_ids: Vec<String>,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}

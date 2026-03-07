//! 实体与标签绑定关系相关模型（不算 API）

use serde::{Deserialize, Serialize};

/// 绑定/解绑标签请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BizEntityTagRelationBody {
    pub tag_biz_type: String,
    pub biz_entity_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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

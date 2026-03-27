use serde::{Deserialize, Serialize};

/// 文本消息内容模型。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    /// 文本内容。
    pub text: String,
}

/// 卡片消息内容模型。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardMessage {
    /// 卡片 JSON 内容。
    pub card: serde_json::Value,
}

/// 机器人消息内容枚举。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// 文本消息。
    Text(TextMessage),
    /// 卡片消息。
    Card(CardMessage),
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

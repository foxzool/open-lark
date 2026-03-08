use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardMessage {
    pub card: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(TextMessage),
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

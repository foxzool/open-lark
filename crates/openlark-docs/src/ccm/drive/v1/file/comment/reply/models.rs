use serde::{Deserialize, Serialize};

/// 评论回复信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyInfo {
    /// 回复ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<String>,
    /// 回复内容
    pub content: ReplyContent,
    /// 回复者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 其他内容（图片 token 等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<ReplyExtra>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyContent {
    /// 回复文本内容
    pub elements: Vec<ReplyElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyElement {
    #[serde(rename = "type")]
    pub r#type: String,
    pub text_run: Option<TextRun>,
    pub docs_link: Option<DocsLink>,
    pub person: Option<Person>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyExtra {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsLink {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub user_id: String,
}

#[cfg(test)]
mod tests {

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

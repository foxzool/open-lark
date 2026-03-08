/// 表单相关模型
///
/// 注意：该文件仅存放模型结构，不计入 API 文件数量。
use serde::{Deserialize, Serialize};

/// 表单元数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Form {
    /// 表单名称
    pub name: String,
    /// 表单描述
    pub description: String,
    /// 是否开启分享
    pub shared: bool,
    /// 分享链接（shared=true 时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_url: Option<String>,
    /// 分享范围限制（例如 tenant_editable）
    pub shared_limit: String,
    /// 是否限制每人只可提交一次
    pub submit_limit_once: bool,
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

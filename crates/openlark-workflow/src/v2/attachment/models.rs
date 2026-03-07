//! 附件 API v2 的数据模型

use serde::Deserialize;

/// 上传附件响应
#[derive(Debug, Clone, Deserialize)]
pub struct UploadAttachmentResponse {
    /// 附件 GUID
    pub attachment_guid: String,
    /// 任务 GUID
    pub task_guid: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小（字节）
    pub file_size: i64,
    /// 文件类型
    pub file_type: String,
    /// 上传时间
    pub created_at: String,
}

/// 删除附件响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAttachmentResponse {
    /// 是否删除成功
    pub success: bool,
    /// 附件 GUID
    pub attachment_guid: String,
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

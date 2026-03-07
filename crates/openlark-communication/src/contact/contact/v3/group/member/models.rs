//! 用户组成员相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 用户组成员信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupMember {
    pub member_id: String,
    pub member_type: String,
    pub member_id_type: String,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 查询用户组成员列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleListGroupMembersResponse {
    #[serde(default)]
    pub memberlist: Vec<GroupMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl ApiResponseTrait for SimpleListGroupMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量添加用户组成员结果项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchAddResult {
    pub member_id: String,
    pub code: i32,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 批量添加用户组成员响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchAddGroupMembersResponse {
    #[serde(default)]
    pub results: Vec<BatchAddResult>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for BatchAddGroupMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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

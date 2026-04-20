//! 用户组相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 用户组信息（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Group {
    /// 用户组 ID。
    pub id: String,
    /// 用户组名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户组描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 用户成员数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_user_count: Option<i32>,
    /// 部门成员数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_department_count: Option<i32>,
    /// 用户组类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 查询指定用户组响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetGroupResponse {
    /// 用户组详情。
    pub group: Group,
}

impl ApiResponseTrait for GetGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建用户组响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateGroupResponse {
    /// 新创建的用户组 ID。
    pub group_id: String,
}

impl ApiResponseTrait for CreateGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询用户组列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SimpleListGroupsResponse {
    /// 用户组列表。
    #[serde(default)]
    pub grouplist: Vec<Group>,
    /// 分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for SimpleListGroupsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询用户所属用户组响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemberBelongGroupsResponse {
    /// 所属用户组 ID 列表。
    #[serde(default)]
    pub group_list: Vec<String>,
    /// 分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    #[serde(default)]
    pub has_more: bool,
}

impl ApiResponseTrait for MemberBelongGroupsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    fn test_roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(
        original: &T,
    ) {
        let json = serde_json::to_string(original).expect("序列化失败");
        let deserialized: T = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(original, &deserialized, "roundtrip 后数据不一致");
    }

    #[test]
    fn test_group_serialization() {
        let group = Group {
            id: "group123".to_string(),
            name: Some("测试组".to_string()),
            description: Some("这是一个测试组".to_string()),
            member_user_count: Some(10),
            member_department_count: Some(2),
            r#type: Some(1),
            extra: HashMap::new(),
        };
        test_roundtrip(&group);
    }

    #[test]
    fn test_get_group_response_serialization() {
        let response = GetGroupResponse {
            group: Group {
                id: "group123".to_string(),
                name: Some("测试组".to_string()),
                description: None,
                member_user_count: None,
                member_department_count: None,
                r#type: None,
                extra: HashMap::new(),
            },
        };
        test_roundtrip(&response);
    }

    #[test]
    fn test_create_group_response_serialization() {
        let response = CreateGroupResponse {
            group_id: "group123".to_string(),
        };
        test_roundtrip(&response);
    }

    #[test]
    fn test_simple_list_groups_response_serialization() {
        let response = SimpleListGroupsResponse {
            grouplist: vec![Group {
                id: "group123".to_string(),
                name: Some("测试组".to_string()),
                description: None,
                member_user_count: None,
                member_department_count: None,
                r#type: None,
                extra: HashMap::new(),
            }],
            page_token: Some("next_page".to_string()),
            has_more: Some(true),
            extra: HashMap::new(),
        };
        test_roundtrip(&response);
    }

    #[test]
    fn test_member_belong_groups_response_serialization() {
        let response = MemberBelongGroupsResponse {
            group_list: vec!["group1".to_string(), "group2".to_string()],
            page_token: Some("next_page".to_string()),
            has_more: true,
        };
        test_roundtrip(&response);
    }
}

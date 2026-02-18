//! 用户相关模型（不算 API）

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// 用户信息（字段随文档演进，未显式建模字段使用 `extra` 透传）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<UserAvatar>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 用户头像
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAvatar {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 用户 ID 类型（查询参数）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    OpenId,
    UnionId,
    UserId,
}

impl UserIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenId => "open_id",
            Self::UnionId => "union_id",
            Self::UserId => "user_id",
        }
    }
}

/// 部门 ID 类型（查询参数）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepartmentIdType {
    DepartmentId,
    OpenDepartmentId,
}

impl DepartmentIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DepartmentId => "department_id",
            Self::OpenDepartmentId => "open_department_id",
        }
    }
}

#[cfg(test)]
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
    fn test_user_serialization() {
        let user = User {
            union_id: Some("union123".to_string()),
            user_id: Some("user123".to_string()),
            open_id: Some("open123".to_string()),
            name: Some("张三".to_string()),
            en_name: Some("Zhang San".to_string()),
            nickname: None,
            email: Some("zhangsan@example.com".to_string()),
            mobile: Some("13800138000".to_string()),
            mobile_visible: Some(true),
            gender: Some(1),
            avatar_key: Some("avatar_key_123".to_string()),
            avatar: Some(UserAvatar {
                avatar_72: Some("https://avatar72.jpg".to_string()),
                avatar_240: Some("https://avatar240.jpg".to_string()),
                avatar_640: Some("https://avatar640.jpg".to_string()),
                extra: HashMap::new(),
            }),
            extra: HashMap::new(),
        };
        test_roundtrip(&user);
    }

    #[test]
    fn test_user_avatar_serialization() {
        let avatar = UserAvatar {
            avatar_72: Some("https://avatar72.jpg".to_string()),
            avatar_240: None,
            avatar_640: None,
            extra: HashMap::new(),
        };
        test_roundtrip(&avatar);
    }

    #[test]
    fn test_user_id_type_serialization() {
        test_roundtrip(&UserIdType::OpenId);
        test_roundtrip(&UserIdType::UnionId);
        test_roundtrip(&UserIdType::UserId);
    }

    #[test]
    fn test_department_id_type_serialization() {
        test_roundtrip(&DepartmentIdType::DepartmentId);
        test_roundtrip(&DepartmentIdType::OpenDepartmentId);
    }

    #[test]
    fn test_user_id_type_as_str() {
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
    }

    #[test]
    fn test_department_id_type_as_str() {
        assert_eq!(DepartmentIdType::DepartmentId.as_str(), "department_id");
        assert_eq!(
            DepartmentIdType::OpenDepartmentId.as_str(),
            "open_department_id"
        );
    }
}

//! 群成员相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 群成员 ID 类型（member_id_type）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberIdType {
    OpenId,
    UnionId,
    UserId,
    AppId,
}

impl MemberIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenId => "open_id",
            Self::UnionId => "union_id",
            Self::UserId => "user_id",
            Self::AppId => "app_id",
        }
    }
}

/// 出现不可用 ID 后的处理方式（succeed_type）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SucceedType {
    Type0,
    Type1,
    Type2,
}

impl SucceedType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Type0 => "0",
            Self::Type1 => "1",
            Self::Type2 => "2",
        }
    }
}

/// 拉群成员请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateChatMembersBody {
    pub id_list: Vec<String>,
}

/// 拉群成员响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateChatMembersResponse {
    #[serde(default)]
    pub invalid_id_list: Option<Vec<String>>,
    #[serde(default)]
    pub not_existed_id_list: Option<Vec<String>>,
    #[serde(default)]
    pub pending_approval_id_list: Option<Vec<String>>,
}

impl ApiResponseTrait for CreateChatMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移除群成员请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteChatMembersBody {
    pub id_list: Vec<String>,
}

/// 移除群成员响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteChatMembersResponse {
    #[serde(default)]
    pub invalid_id_list: Option<Vec<String>>,
}

impl ApiResponseTrait for DeleteChatMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 群成员信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMemberItem {
    pub member_id_type: String,
    pub member_id: String,
    pub name: String,
    pub tenant_key: String,
}

/// 获取群成员列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListChatMembersResponse {
    #[serde(default)]
    pub items: Option<Vec<ChatMemberItem>>,
    #[serde(default)]
    pub page_token: Option<String>,
    #[serde(default)]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub member_total: Option<i64>,
}

impl ApiResponseTrait for ListChatMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 判断用户或机器人是否在群里响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IsInChatResponse {
    pub is_in_chat: bool,
}

impl ApiResponseTrait for IsInChatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
    fn test_member_id_type_serialization() {
        test_roundtrip(&MemberIdType::OpenId);
        test_roundtrip(&MemberIdType::UnionId);
        test_roundtrip(&MemberIdType::UserId);
        test_roundtrip(&MemberIdType::AppId);
    }

    #[test]
    fn test_member_id_type_as_str() {
        assert_eq!(MemberIdType::OpenId.as_str(), "open_id");
        assert_eq!(MemberIdType::UnionId.as_str(), "union_id");
        assert_eq!(MemberIdType::UserId.as_str(), "user_id");
        assert_eq!(MemberIdType::AppId.as_str(), "app_id");
    }

    #[test]
    fn test_succeed_type_as_str() {
        assert_eq!(SucceedType::Type0.as_str(), "0");
        assert_eq!(SucceedType::Type1.as_str(), "1");
        assert_eq!(SucceedType::Type2.as_str(), "2");
    }

    #[test]
    fn test_create_chat_members_body_serialization() {
        let body = CreateChatMembersBody {
            id_list: vec!["user1".to_string(), "user2".to_string()],
        };
        test_roundtrip(&body);
    }

    #[test]
    fn test_create_chat_members_response_serialization() {
        let resp = CreateChatMembersResponse {
            invalid_id_list: Some(vec!["invalid1".to_string()]),
            not_existed_id_list: None,
            pending_approval_id_list: Some(vec!["pending1".to_string()]),
        };
        test_roundtrip(&resp);
    }

    #[test]
    fn test_delete_chat_members_body_serialization() {
        let body = DeleteChatMembersBody {
            id_list: vec!["user1".to_string()],
        };
        test_roundtrip(&body);
    }

    #[test]
    fn test_delete_chat_members_response_serialization() {
        let resp = DeleteChatMembersResponse {
            invalid_id_list: Some(vec!["invalid1".to_string()]),
        };
        test_roundtrip(&resp);
    }

    #[test]
    fn test_chat_member_item_serialization() {
        let item = ChatMemberItem {
            member_id_type: "open_id".to_string(),
            member_id: "user123".to_string(),
            name: "张三".to_string(),
            tenant_key: "tenant123".to_string(),
        };
        test_roundtrip(&item);
    }

    #[test]
    fn test_list_chat_members_response_serialization() {
        let resp = ListChatMembersResponse {
            items: Some(vec![ChatMemberItem {
                member_id_type: "open_id".to_string(),
                member_id: "user123".to_string(),
                name: "张三".to_string(),
                tenant_key: "tenant123".to_string(),
            }]),
            page_token: Some("next_page".to_string()),
            has_more: Some(true),
            member_total: Some(100),
        };
        test_roundtrip(&resp);
    }

    #[test]
    fn test_is_in_chat_response_serialization() {
        let resp_true = IsInChatResponse { is_in_chat: true };
        let resp_false = IsInChatResponse { is_in_chat: false };
        test_roundtrip(&resp_true);
        test_roundtrip(&resp_false);
    }
}

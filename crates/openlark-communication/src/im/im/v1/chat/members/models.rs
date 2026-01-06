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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatMembersBody {
    pub id_list: Vec<String>,
}

/// 拉群成员响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteChatMembersBody {
    pub id_list: Vec<String>,
}

/// 移除群成员响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberItem {
    pub member_id_type: String,
    pub member_id: String,
    pub name: String,
    pub tenant_key: String,
}

/// 获取群成员列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsInChatResponse {
    pub is_in_chat: bool,
}

impl ApiResponseTrait for IsInChatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

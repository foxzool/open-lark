//! 群发言权限相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 群发言模式（moderation_setting）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModerationSetting {
    AllMembers,
    OnlyOwner,
    ModeratorList,
}

impl ModerationSetting {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllMembers => "all_members",
            Self::OnlyOwner => "only_owner",
            Self::ModeratorList => "moderator_list",
        }
    }
}

/// 更新群发言权限请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateChatModerationBody {
    /// 群发言模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_setting: Option<ModerationSetting>,
    /// 添加可发言的用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator_added_list: Option<Vec<String>>,
    /// 移除可发言的用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderator_removed_list: Option<Vec<String>>,
}

/// 群发言权限列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatModerationItem {
    pub user_id_type: String,
    pub user_id: String,
    pub tenant_key: String,
}

/// 获取群成员发言权限响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatModerationResponse {
    pub moderation_setting: String,
    #[serde(default)]
    pub page_token: Option<String>,
    #[serde(default)]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub items: Option<Vec<ChatModerationItem>>,
}

impl ApiResponseTrait for GetChatModerationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

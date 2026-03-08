//! 公共邮箱模块数据模型

use serde::{Deserialize, Serialize};

/// 创建公共邮箱请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreatePublicMailboxBody {
    /// 公共邮箱名称
    pub name: String,

    /// 公共邮箱描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 公共邮箱所有者的用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
}

/// 创建公共邮箱响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePublicMailboxResponse {
    /// 公共邮箱 ID
    pub public_mailbox_id: String,

    /// 公共邮箱名称
    pub name: String,

    /// 公共邮箱描述
    #[serde(default)]
    pub description: Option<String>,

    /// 公共邮箱所有者的用户 ID
    #[serde(default)]
    pub owner_user_id: Option<String>,

    /// 是否已删除
    pub is_deleted: bool,

    /// 创建时间
    pub created_time: String,

    /// 更新时间
    #[serde(default)]
    pub update_time: Option<String>,
}

/// 获取公共邮箱详情响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetPublicMailboxResponse {
    /// 公共邮箱 ID
    pub public_mailbox_id: String,

    /// 公共邮箱名称
    pub name: String,

    /// 公共邮箱描述
    #[serde(default)]
    pub description: Option<String>,

    /// 公共邮箱所有者的用户 ID
    #[serde(default)]
    pub owner_user_id: Option<String>,

    /// 是否已删除
    pub is_deleted: bool,

    /// 创建时间
    pub created_time: String,

    /// 更新时间
    #[serde(default)]
    pub update_time: Option<String>,
}

/// 更新公共邮箱请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdatePublicMailboxBody {
    /// 公共邮箱名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 公共邮箱描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 公共邮箱所有者的用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
}

/// 更新公共邮箱响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePublicMailboxResponse {
    /// 公共邮箱 ID
    pub public_mailbox_id: String,

    /// 更新时间
    pub update_time: String,
}

/// 部分更新公共邮箱请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct PatchPublicMailboxBody {
    /// 公共邮箱名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 公共邮箱描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 部分更新公共邮箱响应
#[derive(Debug, Clone, Deserialize)]
pub struct PatchPublicMailboxResponse {
    /// 公共邮箱 ID
    pub public_mailbox_id: String,

    /// 更新时间
    pub update_time: String,
}

/// 删除公共邮箱响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeletePublicMailboxResponse {
    /// 是否删除成功
    pub success: bool,
}

/// 公共邮箱列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxListResponse {
    /// 公共邮箱列表
    pub items: Vec<PublicMailboxItem>,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 公共邮箱项目
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxItem {
    /// 公共邮箱 ID
    pub public_mailbox_id: String,

    /// 公共邮箱名称
    pub name: String,

    /// 公共邮箱描述
    #[serde(default)]
    pub description: Option<String>,

    /// 公共邮箱所有者的用户 ID
    #[serde(default)]
    pub owner_user_id: Option<String>,

    /// 是否已删除
    pub is_deleted: bool,

    /// 创建时间
    pub created_time: String,
}

/// 移至回收站响应
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveToRecycleBinResponse {
    /// 是否成功
    pub success: bool,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_public_mailbox_body_serialize() {
        let body = CreatePublicMailboxBody {
            name: "support@example.com".to_string(),
            description: Some("技术支持邮箱".to_string()),
            owner_user_id: Some("ou_xxx".to_string()),
        };

        let value = serde_json::to_value(body).expect("serialize body");
        assert_eq!(
            value,
            json!({
                "name": "support@example.com",
                "description": "技术支持邮箱",
                "owner_user_id": "ou_xxx"
            })
        );
    }

    #[test]
    fn test_public_mailbox_list_response_deserialize() {
        let value = json!({
            "items": [
                {
                    "public_mailbox_id": "pm_1",
                    "name": "邮箱一",
                    "description": "描述",
                    "owner_user_id": "ou_1",
                    "is_deleted": false,
                    "created_time": "1700000000"
                }
            ],
            "page_token": "next",
            "has_more": true
        });

        let resp: PublicMailboxListResponse =
            serde_json::from_value(value).expect("deserialize list response");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.page_token.as_deref(), Some("next"));
        assert_eq!(resp.has_more, Some(true));
    }
}

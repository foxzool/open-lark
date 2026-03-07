//! 公共邮箱别名数据模型

use serde::{Deserialize, Serialize};

/// 创建公共邮箱别名请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreatePublicMailboxAliasBody {
    /// 别名邮箱地址
    pub email: String,
}

/// 创建公共邮箱别名响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePublicMailboxAliasResponse {
    /// 别名 ID
    pub alias_id: String,

    /// 别名邮箱地址
    pub email: String,

    /// 创建时间
    pub create_time: String,
}

/// 公共邮箱别名列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxAliasListResponse {
    /// 别名列表
    pub items: Vec<PublicMailboxAliasItem>,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 公共邮箱别名项目
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxAliasItem {
    /// 别名 ID
    pub alias_id: String,

    /// 别名邮箱地址
    pub email: String,

    /// 创建时间
    pub create_time: String,
}

/// 删除公共邮箱别名响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeletePublicMailboxAliasResponse {
    /// 是否删除成功
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_public_mailbox_alias_body_serialize() {
        let body = CreatePublicMailboxAliasBody {
            email: "alias@example.com".to_string(),
        };

        let value = serde_json::to_value(body).expect("serialize body");
        assert_eq!(value, json!({"email": "alias@example.com"}));
    }

    #[test]
    fn test_public_mailbox_alias_list_response_deserialize() {
        let value = json!({
            "items": [
                {
                    "alias_id": "alias_1",
                    "email": "alias@example.com",
                    "create_time": "1700000000"
                }
            ],
            "page_token": "next",
            "has_more": false
        });

        let resp: PublicMailboxAliasListResponse =
            serde_json::from_value(value).expect("deserialize alias list");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.has_more, Some(false));
    }
}

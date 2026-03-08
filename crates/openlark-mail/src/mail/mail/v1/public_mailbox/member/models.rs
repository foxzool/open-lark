//! 公共邮箱成员数据模型

use serde::{Deserialize, Serialize};

/// 创建公共邮箱成员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreatePublicMailboxMemberBody {
    /// 成员邮箱地址
    pub email: String,

    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<i32>,
}

/// 创建公共邮箱成员响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePublicMailboxMemberResponse {
    /// 成员 ID
    pub member_id: String,

    /// 成员邮箱地址
    pub email: String,

    /// 成员类型
    pub member_type: i32,

    /// 创建时间
    pub create_time: String,
}

/// 获取公共邮箱成员详情响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetPublicMailboxMemberResponse {
    /// 成员 ID
    pub member_id: String,

    /// 成员邮箱地址
    pub email: String,

    /// 成员类型
    pub member_type: i32,

    /// 创建时间
    pub create_time: String,
}

/// 公共邮箱成员列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxMemberListResponse {
    /// 成员列表
    pub items: Vec<PublicMailboxMemberItem>,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 公共邮箱成员项目
#[derive(Debug, Clone, Deserialize)]
pub struct PublicMailboxMemberItem {
    /// 成员 ID
    pub member_id: String,

    /// 成员邮箱地址
    pub email: String,

    /// 成员类型
    pub member_type: i32,

    /// 创建时间
    pub create_time: String,
}

/// 删除公共邮箱成员响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeletePublicMailboxMemberResponse {
    /// 是否删除成功
    pub success: bool,
}

/// 批量创建公共邮箱成员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchCreatePublicMailboxMemberBody {
    /// 成员邮箱地址列表
    pub emails: Vec<String>,
}

/// 批量删除公共邮箱成员请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchDeletePublicMailboxMemberBody {
    /// 成员 ID 列表
    pub member_ids: Vec<String>,
}

/// 清空公共邮箱成员响应
#[derive(Debug, Clone, Deserialize)]
pub struct ClearPublicMailboxMemberResponse {
    /// 是否清空成功
    pub success: bool,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_public_mailbox_member_body_serialize() {
        let body = CreatePublicMailboxMemberBody {
            email: "member@example.com".to_string(),
            member_type: Some(1),
        };

        let value = serde_json::to_value(body).expect("serialize body");
        assert_eq!(
            value,
            json!({
                "email": "member@example.com",
                "member_type": 1
            })
        );
    }

    #[test]
    fn test_public_mailbox_member_list_response_deserialize() {
        let value = json!({
            "items": [
                {
                    "member_id": "mem_1",
                    "email": "member@example.com",
                    "member_type": 1,
                    "create_time": "1700000000"
                }
            ],
            "page_token": "next",
            "has_more": true
        });

        let resp: PublicMailboxMemberListResponse =
            serde_json::from_value(value).expect("deserialize member list");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].member_id, "mem_1");
    }
}

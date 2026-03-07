//! 邮件 API v1 的数据模型

use serde::{Deserialize, Serialize};

/// 创建邮件组请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateMailGroupBody {
    /// 邮件组名称（邮箱地址）
    pub mail_group_id: String,

    /// 邮件组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 邮件组所有者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    /// 邮件组成员邮箱列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,

    /// 仅管理员发送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_admins_send: Option<bool>,
}

/// 创建邮件组响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateMailGroupResponse {
    /// 邮件组 ID
    pub mail_group_id: String,

    /// 邮件组描述
    #[serde(default)]
    pub description: Option<String>,

    /// 创建时间
    pub created_at: String,
}

/// 获取邮件组详情响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetMailGroupResponse {
    /// 邮件组 ID
    pub mail_group_id: String,

    /// 邮件组描述
    #[serde(default)]
    pub description: Option<String>,

    /// 邮件组所有者
    #[serde(default)]
    pub owner: Option<String>,

    /// 仅管理员发送
    #[serde(default)]
    pub only_admins_send: Option<bool>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// 更新邮件组请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateMailGroupBody {
    /// 邮件组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 仅管理员发送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_admins_send: Option<bool>,
}

/// 更新邮件组响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMailGroupResponse {
    /// 邮件组 ID
    pub mail_group_id: String,

    /// 更新时间
    pub updated_at: String,
}

/// 删除邮件组响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMailGroupResponse {
    /// 是否删除成功
    pub success: bool,
}

/// 邮件组列表响应
#[derive(Debug, Clone, Deserialize)]
pub struct MailGroupListResponse {
    /// 邮件组列表
    pub mail_groups: Vec<MailGroupItem>,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 是否有更多
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 邮件组项目
#[derive(Debug, Clone, Deserialize)]
pub struct MailGroupItem {
    /// 邮件组 ID
    pub mail_group_id: String,

    /// 邮件组描述
    #[serde(default)]
    pub description: Option<String>,

    /// 创建时间
    pub created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_mailgroup_body_serialize() {
        let body = CreateMailGroupBody {
            mail_group_id: "eng@example.com".to_string(),
            description: Some("工程组".to_string()),
            owner: Some("ou_owner".to_string()),
            members: Some(vec![
                "a@example.com".to_string(),
                "b@example.com".to_string(),
            ]),
            only_admins_send: Some(true),
        };

        let value = serde_json::to_value(body).expect("serialize body");
        assert_eq!(
            value,
            json!({
                "mail_group_id": "eng@example.com",
                "description": "工程组",
                "owner": "ou_owner",
                "members": ["a@example.com", "b@example.com"],
                "only_admins_send": true
            })
        );
    }

    #[test]
    fn test_mailgroup_list_response_deserialize() {
        let value = json!({
            "mail_groups": [
                {
                    "mail_group_id": "eng@example.com",
                    "description": "工程组",
                    "created_at": "1700000000"
                }
            ],
            "page_token": "next",
            "has_more": true
        });

        let resp: MailGroupListResponse =
            serde_json::from_value(value).expect("deserialize list response");
        assert_eq!(resp.mail_groups.len(), 1);
        assert_eq!(resp.mail_groups[0].mail_group_id, "eng@example.com");
    }
}

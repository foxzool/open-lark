//! 邮件 API 端点定义（类型安全枚举系统）

/// 邮件 API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum MailApiV1 {
    /// 创建邮件组
    MailGroupCreate,
    /// 获取邮件组详情
    MailGroupGet(String),
    /// 更新邮件组
    MailGroupUpdate(String),
    /// 删除邮件组
    MailGroupDelete(String),
    /// 获取邮件组列表
    MailGroupList,
    /// 添加邮件组成员
    MailGroupMemberAdd(String),
    /// 删除邮件组成员
    MailGroupMemberRemove(String),
    /// 获取邮件组成员列表
    MailGroupMemberList(String),
}

impl MailApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            MailApiV1::MailGroupCreate => "/open-apis/mail/v1/mail_groups".to_string(),
            MailApiV1::MailGroupGet(group_id) => {
                format!("/open-apis/mail/v1/mail_groups/{}", group_id)
            }
            MailApiV1::MailGroupUpdate(group_id) => {
                format!("/open-apis/mail/v1/mail_groups/{}", group_id)
            }
            MailApiV1::MailGroupDelete(group_id) => {
                format!("/open-apis/mail/v1/mail_groups/{}", group_id)
            }
            MailApiV1::MailGroupList => "/open-apis/mail/v1/mail_groups".to_string(),
            MailApiV1::MailGroupMemberAdd(group_id) => {
                format!("/open-apis/mail/v1/mail_groups/{}/members", group_id)
            }
            MailApiV1::MailGroupMemberRemove(group_id) => {
                format!("/open-apis/mail/v1/mail_groups/{}/members", group_id)
            }
            MailApiV1::MailGroupMemberList(group_id) => {
                format!("/open-apis/mail/v1/mail_groups/{}/members", group_id)
            }
        }
    }
}

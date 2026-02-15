#!/usr/bin/env python3
"""批量创建剩余 API 文件"""

import os

# 模板
TEMPLATE_SIMPLE = '''//! {description}

use openlark_core::{{
    api::{{ApiRequest, ApiResponseTrait, ResponseFormat}},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
}};
use serde::{{Deserialize, Serialize}};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct {api_name}Request {{
    config: Arc<Config>,
    {fields}
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {api_name}Response {{
    pub data: Option<serde_json::Value>,
}}

impl ApiResponseTrait for {api_name}Response {{
    fn data_format() -> ResponseFormat {{
        ResponseFormat::Data
    }}
}}

impl {api_name}Request {{
    pub fn new(config: Arc<Config>{new_params}) -> Self {{
        Self {{
            config,
            {init_fields}
        }}
    }}

    pub async fn execute(self) -> SDKResult<{api_name}Response> {{
        self.execute_with_options(RequestOption::default()).await
    }}

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<{api_name}Response> {{
        let path = format!("{url}"{format_args});
        let req: ApiRequest<{api_name}Response> = ApiRequest::{method}(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {{
            openlark_core::error::validation_error("{description}", "响应数据为空")
        }})
    }}
}}
'''

# openlark-mail 剩余 API
mail_apis = [
    # mailgroup/manager
    ("crates/openlark-mail/src/mail/mail/v1/mailgroup/manager/batch_create.rs", 
     "BatchCreateMailGroupManager", "post", 
     "/open-apis/mail/v1/mailgroups/{{}}/managers/batch_create",
     "批量创建邮件组管理员", "mailgroup_id", "mailgroup_id: String,"),
    
    ("crates/openlark-mail/src/mail/mail/v1/mailgroup/manager/batch_delete.rs",
     "BatchDeleteMailGroupManager", "post", 
     "/open-apis/mail/v1/mailgroups/{{}}/managers/batch_delete",
     "批量删除邮件组管理员", "mailgroup_id", "mailgroup_id: String,"),
    
    ("crates/openlark-mail/src/mail/mail/v1/mailgroup/manager/list.rs",
     "ListMailGroupManager", "get",
     "/open-apis/mail/v1/mailgroups/{{}}/managers",
     "批量获取邮件组管理员", "mailgroup_id", "mailgroup_id: String,"),
    
    # mailgroup/member
    ("crates/openlark-mail/src/mail/mail/v1/mailgroup/member/batch_delete.rs",
     "BatchDeleteMailGroupMember", "post",
     "/open-apis/mail/v1/mailgroups/{{}}/members/batch_delete",
     "批量删除邮件组成员", "mailgroup_id", "mailgroup_id: String,"),
    
    ("crates/openlark-mail/src/mail/mail/v1/mailgroup/member/get.rs",
     "GetMailGroupMember", "get",
     "/open-apis/mail/v1/mailgroups/{{}}/members/{{}}",
     "查询指定邮件组成员", "mailgroup_id, member_id", "mailgroup_id: String,\n    member_id: String,"),
    
    ("crates/openlark-mail/src/mail/mail/v1/mailgroup/member/list.rs",
     "ListMailGroupMember", "get",
     "/open-apis/mail/v1/mailgroups/{{}}/members",
     "获取所有邮件组成员", "mailgroup_id", "mailgroup_id: String,"),
    
    # mailgroup/permission_member
    ("crates/openlark-mail/src/mail/mail/v1/mailgroup/permission_member/batch_delete.rs",
     "BatchDeleteMailGroupPermissionMember", "post",
     "/open-apis/mail/v1/mailgroups/{{}}/permission_members/batch_delete",
     "批量删除邮件组权限成员", "mailgroup_id", "mailgroup_id: String,"),
    
    ("crates/openlark-mail/src/mail/mail/v1/mailgroup/permission_member/list.rs",
     "ListMailGroupPermissionMember", "get",
     "/open-apis/mail/v1/mailgroups/{{}}/permission_members",
     "批量获取邮件组权限成员", "mailgroup_id", "mailgroup_id: String,"),
    
    # public_mailbox/member
    ("crates/openlark-mail/src/mail/mail/v1/public_mailbox/member/batch_create.rs",
     "BatchCreatePublicMailboxMember", "post",
     "/open-apis/mail/v1/public_mailboxes/{{}}/members/batch_create",
     "批量添加公共邮箱成员", "public_mailbox_id", "public_mailbox_id: String,"),
    
    ("crates/openlark-mail/src/mail/mail/v1/public_mailbox/member/batch_delete.rs",
     "BatchDeletePublicMailboxMember", "delete",
     "/open-apis/mail/v1/public_mailboxes/{{}}/members/batch_delete",
     "批量删除公共邮箱成员", "public_mailbox_id", "public_mailbox_id: String,"),
    
    # user_mailbox/rule
    ("crates/openlark-mail/src/mail/mail/v1/user_mailbox/rule/update.rs",
     "UpdateMailboxRule", "put",
     "/open-apis/mail/v1/user_mailboxes/{{}}/rules/{{}}",
     "更新收信规则", "user_mailbox_id, rule_id", "user_mailbox_id: String,\n    rule_id: String,"),
    
    # user_mailbox/event
    ("crates/openlark-mail/src/mail/mail/v1/user_mailbox/event/subscription.rs",
     "GetMailboxEventSubscription", "get",
     "/open-apis/mail/v1/user_mailboxes/{{}}/event/subscription",
     "获取订阅状态", "user_mailbox_id", "user_mailbox_id: String,"),
    
    # user_mailbox/message
    ("crates/openlark-mail/src/mail/mail/v1/user_mailbox/message/get_by_card.rs",
     "GetMailboxMessageByCard", "get",
     "/open-apis/mail/v1/user_mailboxes/{{}}/messages/get_by_card",
     "获取邮件卡片的邮件列表", "user_mailbox_id", "user_mailbox_id: String,"),
    
    # user
    ("crates/openlark-mail/src/mail/mail/v1/user/query.rs",
     "QueryMailUser", "post",
     "/open-apis/mail/v1/users/query",
     "查询邮箱地址状态", "", ""),
]

def generate_api(filepath, api_name, method, url, description, new_params, fields):
    # 计算 format 参数
    format_args = ""
    if new_params:
        params = new_params.split(", ")
        format_args = ", self." + ", self.".join(params)
    
    # 计算 new 参数
    new_args = ""
    init_fields = ""
    if new_params:
        for param in new_params.split(", "):
            new_args += f", {param}: impl Into<String>"
            init_fields += f"{param}: {param}.into(),\n            "
    
    content = TEMPLATE_SIMPLE.format(
        api_name=api_name,
        description=description,
        method=method,
        url=url,
        format_args=format_args,
        new_params=new_args,
        fields=fields,
        init_fields=init_fields
    )
    
    with open(filepath, 'w') as f:
        f.write(content)
    print(f"✅ {filepath}")

# 创建目录并生成文件
for filepath, api_name, method, url, description, new_params, fields in mail_apis:
    os.makedirs(os.path.dirname(filepath), exist_ok=True)
    generate_api(filepath, api_name, method, url, description, new_params, fields)

print("\n✅ openlark-mail 剩余 14 个 API 已创建")

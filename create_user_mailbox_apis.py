#!/usr/bin/env python3
"""批量创建 mail API 文件"""

import os

BASE = "crates/openlark-mail/src/mail/mail/v1"

# API 定义: (文件路径, API名称, HTTP方法, URL模板, 描述)
apis = [
    # user_mailbox/folder
    ("user_mailbox/folder/create.rs", "CreateMailboxFolder", "post", "/open-apis/mail/v1/user_mailboxes/{}/folders", "创建邮箱文件夹"),
    ("user_mailbox/folder/delete.rs", "DeleteMailboxFolder", "delete", "/open-apis/mail/v1/user_mailboxes/{}/folders/{}", "删除邮箱文件夹"),
    ("user_mailbox/folder/list.rs", "ListMailboxFolder", "get", "/open-apis/mail/v1/user_mailboxes/{}/folders", "列出邮箱文件夹"),
    ("user_mailbox/folder/patch.rs", "PatchMailboxFolder", "patch", "/open-apis/mail/v1/user_mailboxes/{}/folders/{}", "修改邮箱文件夹"),
    
    # user_mailbox/mail_contact
    ("user_mailbox/mail_contact/create.rs", "CreateMailContact", "post", "/open-apis/mail/v1/user_mailboxes/{}/mail_contacts", "创建邮箱联系人"),
    ("user_mailbox/mail_contact/delete.rs", "DeleteMailContact", "delete", "/open-apis/mail/v1/user_mailboxes/{}/mail_contacts/{}", "删除邮箱联系人"),
    ("user_mailbox/mail_contact/list.rs", "ListMailContact", "get", "/open-apis/mail/v1/user_mailboxes/{}/mail_contacts", "列出邮箱联系人"),
    ("user_mailbox/mail_contact/patch.rs", "PatchMailContact", "patch", "/open-apis/mail/v1/user_mailboxes/{}/mail_contacts/{}", "修改邮箱联系人"),
    
    # user_mailbox/rule
    ("user_mailbox/rule/create.rs", "CreateMailboxRule", "post", "/open-apis/mail/v1/user_mailboxes/{}/rules", "创建收信规则"),
    ("user_mailbox/rule/delete.rs", "DeleteMailboxRule", "delete", "/open-apis/mail/v1/user_mailboxes/{}/rules/{}", "删除收信规则"),
    ("user_mailbox/rule/list.rs", "ListMailboxRule", "get", "/open-apis/mail/v1/user_mailboxes/{}/rules", "列出收信规则"),
    ("user_mailbox/rule/reorder.rs", "ReorderMailboxRule", "post", "/open-apis/mail/v1/user_mailboxes/{}/rules/reorder", "对收信规则进行排序"),
    
    # user_mailbox/message
    ("user_mailbox/message/list.rs", "ListMailboxMessage", "get", "/open-apis/mail/v1/user_mailboxes/{}/messages", "列出邮件"),
    ("user_mailbox/message/send.rs", "SendMailboxMessage", "post", "/open-apis/mail/v1/user_mailboxes/{}/messages/send", "发送邮件"),
    
    # user_mailbox/alias
    ("user_mailbox/alias/create.rs", "CreateMailboxAlias", "post", "/open-apis/mail/v1/user_mailboxes/{}/aliases", "创建用户邮箱别名"),
    ("user_mailbox/alias/delete.rs", "DeleteMailboxAlias", "delete", "/open-apis/mail/v1/user_mailboxes/{}/aliases/{}", "删除用户邮箱别名"),
    ("user_mailbox/alias/list.rs", "ListMailboxAlias", "get", "/open-apis/mail/v1/user_mailboxes/{}/aliases", "列出用户邮箱别名"),
    
    # user_mailbox/event
    ("user_mailbox/event/subscribe.rs", "SubscribeMailboxEvent", "post", "/open-apis/mail/v1/user_mailboxes/{}/event/subscribe", "订阅邮箱事件"),
    ("user_mailbox/event/unsubscribe.rs", "UnsubscribeMailboxEvent", "post", "/open-apis/mail/v1/user_mailboxes/{}/event/unsubscribe", "取消订阅"),
    
    # user_mailbox
    ("user_mailbox/delete.rs", "DeleteUserMailbox", "delete", "/open-apis/mail/v1/user_mailboxes/{}", "从回收站删除用户邮箱地址"),
]

template = '''//! {description}

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
    user_mailbox_id: String,
    {extra_fields}
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
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {{
        Self {{
            config,
            user_mailbox_id: user_mailbox_id.into(),
            {extra_init}
        }}
    }}

    pub async fn execute(self) -> SDKResult<{api_name}Response> {{
        self.execute_with_options(RequestOption::default()).await
    }}

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<{api_name}Response> {{
        let path = format!("{url}", self.user_mailbox_id);
        let req: ApiRequest<{api_name}Response> = ApiRequest::{method}(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {{
            openlark_core::error::validation_error("{description}", "响应数据为空")
        }})
    }}
}}
'''

for filepath, api_name, method, url, description in apis:
    full_path = os.path.join(BASE, filepath)
    
    # 检查是否有额外参数（如 rule_id, folder_id 等）
    extra_fields = ""
    extra_init = ""
    if "{}" in url:
        count = url.count("{}")
        if count == 2:  # 有额外ID参数
            param_name = filepath.split("/")[-1].replace(".rs", "_id")
            extra_fields = f'{param_name}: String,'
            extra_init = f'{param_name}: String::new(),'
    
    content = template.format(
        api_name=api_name,
        description=description,
        method=method,
        url=url.replace("{", "{{").replace("}", "}}").replace("{{}}", "{}"),
        extra_fields=extra_fields,
        extra_init=extra_init
    )
    
    with open(full_path, 'w') as f:
        f.write(content)
    print(f"✅ Created: {filepath}")

print("\\n✅ 所有 user_mailbox API 文件已创建")

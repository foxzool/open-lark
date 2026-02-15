#!/usr/bin/env python3
"""批量创建 security 剩余 API"""

import os

BASE = "crates/openlark-security/src/security/acs/v1"

apis = [
    # user/face
    ("user/face/update", "UpdateUserFace", "put", "/open-apis/acs/v1/users/{}/face", "上传人脸图片", "user_id"),
    ("user/face/get", "GetUserFace", "get", "/open-apis/acs/v1/users/{}/face", "下载人脸图片", "user_id"),
    # user
    ("user/patch", "PatchUser", "patch", "/open-apis/acs/v1/users/{}", "修改用户部分信息", "user_id"),
    ("user/get", "GetUser", "get", "/open-apis/acs/v1/users/{}", "获取单个用户信息", "user_id"),
    ("user/list", "ListUsers", "get", "/open-apis/acs/v1/users", "获取用户列表", ""),
    # access_record/access_photo
    ("access_record/access_photo/get", "GetAccessPhoto", "get", "/open-apis/acs/v1/access_records/{}/access_photo", "下载开门时的人脸识别图片", "access_record_id"),
    # rule_external
    ("rule_external/create", "CreateRuleExternal", "post", "/open-apis/acs/v1/rule_external", "创建或更新权限组", ""),
    ("rule_external/delete", "DeleteRuleExternal", "delete", "/open-apis/acs/v1/rule_external", "删除权限组", ""),
    ("rule_external/get", "GetRuleExternal", "get", "/open-apis/acs/v1/rule_external", "获取权限组信息", ""),
    # visitor
    ("visitor/create", "CreateVisitor", "post", "/open-apis/acs/v1/visitors", "添加访客", ""),
    ("visitor/delete", "DeleteVisitor", "delete", "/open-apis/acs/v1/visitors/{}", "删除访客", "visitor_id"),
    # device
    ("device/list", "ListDevices", "get", "/open-apis/acs/v1/devices", "获取设备列表", ""),
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

        let _resp: openlark_core::api::Response<{api_name}Response> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok({api_name}Response {{ data: None }})
    }}
}}
'''

for filepath, api_name, method, url, description, param in apis:
    full_path = os.path.join(BASE, f"{filepath}.rs")
    os.makedirs(os.path.dirname(full_path), exist_ok=True)
    
    if param:
        fields = f"{param}: String,"
        new_params = f", {param}: impl Into<String>"
        init_fields = f"{param}: {param}.into(),"
        format_args = f", self.{param}"
    else:
        fields = ""
        new_params = ""
        init_fields = ""
        format_args = ""
    
    content = template.format(
        api_name=api_name,
        description=description,
        method=method,
        url=url,
        fields=fields,
        new_params=new_params,
        init_fields=init_fields,
        format_args=format_args
    )
    
    with open(full_path, 'w') as f:
        f.write(content)
    print(f"✅ {filepath}.rs")

print(f"\n✅ security {len(apis)} 个 API 已创建")

#!/usr/bin/env python3
"""批量创建所有剩余 API"""

import os

# openlark-application APIs
application_apis = [
    # app
    "application/application/v1/app/create",
    "application/application/v1/app/delete", 
    "application/application/v1/app/get",
    "application/application/v1/app/list",
    "application/application/v1/app/patch",
    # app_visibility
    "application/application/v1/app_visibility/get",
    "application/application/v1/app_visibility/patch",
    # app_version
    "application/application/v1/app_version/create",
    "application/application/v1/app_version/delete",
    "application/application/v1/app_version/get",
    "application/application/v1/app_version/list",
    "application/application/v1/app_version/patch",
    # collaborator
    "application/application/v1/collaborator/create",
    "application/application/v1/collaborator/delete",
    "application/application/v1/collaborator/list",
    # feedback
    "application/application/v1/feedback/create",
    "application/application/v1/feedback/delete",
    "application/application/v1/feedback/get",
    "application/application/v1/feedback/list",
    # management
    "application/application/v1/management/get",
    "application/application/v1/management/patch",
    # usage
    "application/application/v1/usage/get",
    # visibility
    "application/application/v1/visibility/check",
]

# openlark-security APIs
security_apis = [
    # acs
    "security/acs/v1/user/create",
    "security/acs/v1/user/delete",
    "security/acs/v1/user/get",
    "security/acs/v1/user/list",
    "security/acs/v1/user/patch",
    # acs/face
    "security/acs/v1/face/create",
    "security/acs/v1/face/delete",
    "security/acs/v1/face/get",
    # acs/rule
    "security/acs/v1/rule/create",
    "security/acs/v1/rule/delete",
    "security/acs/v1/rule/get",
    "security/acs/v1/rule/list",
    # acs/device
    "security/acs/v1/device/create",
    "security/acs/v1/device/delete",
    "security/acs/v1/device/get",
    "security/acs/v1/device/list",
    # acs/visitor
    "security/acs/v1/visitor/create",
    "security/acs/v1/visitor/delete",
]

# openlark-user APIs
user_apis = [
    "personal_settings/personal_settings/v1/system_status/create",
    "personal_settings/personal_settings/v1/system_status/delete",
    "personal_settings/personal_settings/v1/system_status/get",
    "personal_settings/personal_settings/v1/system_status/list",
    "personal_settings/personal_settings/v1/system_status/patch",
    "personal_settings/personal_settings/v1/system_status/batch_open",
]

def create_api_file(crate, api_path):
    """创建单个 API 文件"""
    parts = api_path.split("/")
    if len(parts) < 4:
        return
    
    # 构建完整路径
    full_path = f"crates/{crate}/src/{api_path}.rs"
    os.makedirs(os.path.dirname(full_path), exist_ok=True)
    
    # 提取 API 名称
    resource = parts[-2]
    action = parts[-1]
    api_name = "".join([p.capitalize() for p in resource.split("_")]) + action.capitalize()
    
    # 写入文件内容
    content = f"""//! {resource} {action}

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
    pub fn new(config: Arc<Config>) -> Self {{
        Self {{ config }}
    }}

    pub async fn execute(self) -> SDKResult<{api_name}Response> {{
        self.execute_with_options(RequestOption::default()).await
    }}

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<{api_name}Response> {{
        let path = "/open-apis/{api_path.replace('_', '-')}"
            .replace("application", "application")
            .replace("security", "acs")
            .replace("personal_settings", "personal_settings");
        let req: ApiRequest<{api_name}Response> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {{
            openlark_core::error::validation_error("{resource} {action}", "响应数据为空")
        }})
    }}
}}
"""
    
    with open(full_path, 'w') as f:
        f.write(content)
    
    print(f"✅ {full_path}")

# 创建所有 API
print("=== Creating openlark-application APIs ===")
for api in application_apis:
    create_api_file("openlark-application", api)

print("\n=== Creating openlark-security APIs ===")
for api in security_apis:
    create_api_file("openlark-security", api)

print("\n=== Creating openlark-user APIs ===")
for api in user_apis:
    create_api_file("openlark-user", api)

print(f"\n✅ 总共创建 {len(application_apis) + len(security_apis) + len(user_apis)} 个 API 文件")

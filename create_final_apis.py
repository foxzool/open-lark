#!/usr/bin/env python3
"""批量创建所有剩余 API"""

import os

# application 剩余 API
application_apis = [
    ("application/collaborators/list", "ListApplicationCollaborators", "get", "/open-apis/application/v6/applications/{}/collaborators", "获取应用协作者列表", "app_id"),
    ("application/feedback/list", "ListApplicationFeedback", "get", "/open-apis/application/v6/applications/{}/feedbacks", "获取应用反馈列表", "app_id"),
    ("application/app_version/contacts_range", "GetApplicationVersionContactsRange", "get", "/open-apis/application/v6/applications/{}/app_versions/{}/contacts_range", "获取应用版本中开发者申请的通讯录权限范围", "app_id, version_id"),
    ("application/app_version/get", "GetApplicationVersion", "get", "/open-apis/application/v6/applications/{}/app_versions/{}", "获取应用版本信息", "app_id, version_id"),
    ("application/app_version/list", "ListApplicationVersions", "get", "/open-apis/application/v6/applications/{}/app_versions", "获取应用版本列表", "app_id"),
    ("app_usage/message_push_overview", "GetMessagePushOverview", "get", "/open-apis/application/v6/app_usage/message_push_overview", "获取消息推送概览", ""),
    ("frequently_used/get", "GetFrequentlyUsedApps", "get", "/open-apis/application/v6/applications/frequently_used", "获取用户自定义常用的应用", ""),
    ("owner/recommended", "GetRecommendedApps", "get", "/open-apis/application/v6/applications/recommended", "获取管理员推荐的应用", ""),
    ("owner/transfer", "TransferAppOwner", "post", "/open-apis/application/v6/applications/{}/owner/transfer", "转移应用所有者", "app_id"),
]

# security 剩余 API
security_apis = [
    ("device/list", "ListDevices", "get", "/open-apis/acs/v1/devices", "获取门禁设备列表", ""),
    ("rule_external/device_bind", "BindDeviceToRule", "post", "/open-apis/acs/v1/rule_external/device_bind", "设备绑定权限组", ""),
    ("device/delete", "DeleteDevice", "delete", "/open-apis/acs/v1/devices/{}", "删除设备", "device_id"),
    ("device/approve", "ApproveDevice", "post", "/open-apis/acs/v1/devices/{}/approve", "审批设备申报", "device_id"),
    ("device/create", "CreateDevice", "post", "/open-apis/acs/v1/devices", "新增设备", ""),
    ("device/update", "UpdateDevice", "put", "/open-apis/acs/v1/devices/{}", "更新设备", "device_id"),
    ("device/get", "GetDevice", "get", "/open-apis/acs/v1/devices/{}", "查询设备信息", "device_id"),
    ("openapi_audit/get", "GetOpenapiAudit", "get", "/open-apis/acs/v1/openapi_audit", "获取OpenAPI审计日志数据", ""),
    ("client_device/get", "GetClientDevice", "get", "/open-apis/acs/v1/client_devices/{}", "获取客户端设备认证信息", "device_id"),
    ("device/query", "QueryDevice", "get", "/open-apis/acs/v1/devices/{}/query", "获取设备信息", "device_id"),
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

def create_apis(base_path, apis):
    for filepath, api_name, method, url, description, params in apis:
        full_path = os.path.join(base_path, f"{filepath}.rs")
        os.makedirs(os.path.dirname(full_path), exist_ok=True)
        
        if params:
            param_list = params.split(", ")
            if len(param_list) == 1:
                fields = f"{param_list[0]}: String,"
                new_params = f", {param_list[0]}: impl Into<String>"
                init_fields = f"{param_list[0]}: {param_list[0]}.into(),"
                format_args = f", self.{param_list[0]}"
            else:
                fields = "\n    ".join([f"{p}: String," for p in param_list])
                new_params = "".join([f", {p}: impl Into<String>" for p in param_list])
                init_fields = "\n            ".join([f"{p}: {p}.into()," for p in param_list])
                format_args = "".join([f", self.{p}" for p in param_list])
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
        print(f"✅ {filepath}")

print("=== Creating application APIs ===")
create_apis("crates/openlark-application/src/application/application/v6", application_apis)

print("\n=== Creating security APIs ===")
create_apis("crates/openlark-security/src/security/acs/v1", security_apis)

print(f"\n✅ 总共创建 {len(application_apis) + len(security_apis)} 个 API")

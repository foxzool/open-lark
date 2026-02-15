#!/usr/bin/env python3
"""批量创建 application 剩余 API"""

import os

BASE = "crates/openlark-application/src/application/application/v6"

apis = [
    # application management
    ("application/management/update", "UpdateApplicationManagement", "put", "/open-apis/application/v6/applications/{}/management", "启停用应用"),
    ("application/patch", "PatchApplication", "patch", "/open-apis/application/v6/applications/{}", "更新应用分组信息"),
    ("application/collaborators/update", "UpdateApplicationCollaborators", "put", "/open-apis/application/v6/applications/{}/collaborators", "更新应用协作者"),
    ("application/feedback/patch", "PatchApplicationFeedback", "patch", "/open-apis/application/v6/applications/{}/feedbacks/{}", "更新应用反馈"),
    ("application/visibility/patch", "PatchApplicationVisibility", "patch", "/open-apis/application/v6/applications/{}/visibility", "更新应用可用范围"),
    ("application/app_version/patch", "PatchApplicationAppVersion", "patch", "/open-apis/application/v6/applications/{}/app_versions/{}", "更新应用审核状态"),
    ("application/contacts_range/patch", "PatchApplicationContactsRange", "patch", "/open-apis/application/v6/applications/{}/contacts_range", "更新应用通讯录权限范围配置"),
    ("application/underauditlist", "GetApplicationUnderauditlist", "get", "/open-apis/application/v6/applications/underauditlist", "查看待审核的应用列表"),
    ("application/visibility/check_white_black_list", "CheckApplicationVisibilityWhiteBlackList", "post", "/open-apis/application/v6/applications/{}/visibility/check_white_black_list", "查询用户或部门是否在应用的可用或禁用名单"),
    ("scope/list", "ListApplicationScope", "get", "/open-apis/application/v6/scopes", "查询租户授权状态"),
    ("application/list", "ListApplications", "get", "/open-apis/application/v6/applications", "获取企业安装的应用"),
    ("application/app_usage/department_overview", "GetApplicationUsageDepartmentOverview", "post", "/open-apis/application/v6/applications/{}/app_usage/department_overview", "获取多部门应用使用概览"),
    ("application/app_usage/overview", "GetApplicationUsageOverview", "get", "/open-apis/application/v6/applications/{}/app_usage/overview", "获取应用使用概览"),
    ("application/get", "GetApplication", "get", "/open-apis/application/v6/applications/{}", "获取应用信息"),
    ("application/collaborators/list", "ListApplicationCollaborators", "get", "/open-apis/application/v6/applications/{}/collaborators", "获取应用协作者列表"),
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

for filepath, api_name, method, url, description in apis:
    full_path = os.path.join(BASE, f"{filepath}.rs")
    os.makedirs(os.path.dirname(full_path), exist_ok=True)
    
    # 检查是否需要路径参数
    if "{}" in url:
        params = url.count("{}")
        if params == 1:
            fields = "app_id: String,"
            new_params = ", app_id: impl Into<String>"
            init_fields = "app_id: app_id.into(),"
            format_args = ", self.app_id"
        elif params == 2:
            fields = "app_id: String,\n    resource_id: String,"
            new_params = ", app_id: impl Into<String>, resource_id: impl Into<String>"
            init_fields = "app_id: app_id.into(),\n            resource_id: resource_id.into(),"
            format_args = ", self.app_id, self.resource_id"
        else:
            fields = ""
            new_params = ""
            init_fields = ""
            format_args = ""
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

print(f"\n✅ application {len(apis)} 个 API 已创建")

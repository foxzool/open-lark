#!/usr/bin/env python3
"""批量创建 openlark-analytics API"""

import os

BASE = "crates/openlark-analytics/src"

# API 定义
apis = [
    # REPORT APIs
    ("report/report/v1/task/query.rs", "QueryReportTask", "post", "/open-apis/report/v1/tasks/query", "查询任务"),
    ("report/report/v1/rule/query.rs", "QueryReportRule", "get", "/open-apis/report/v1/rules/query", "查询规则"),
    ("report/report/v1/rule/view/remove.rs", "RemoveReportRuleView", "post", "/open-apis/report/v1/rules/{}/views/remove", "移除规则看板"),
    
    # SEARCH - data_source
    ("search/search/v2/data_source/create.rs", "CreateDataSource", "post", "/open-apis/search/v2/data_sources", "创建数据源"),
    ("search/search/v2/data_source/delete.rs", "DeleteDataSource", "delete", "/open-apis/search/v2/data_sources/{}", "删除数据源"),
    ("search/search/v2/data_source/list.rs", "ListDataSource", "get", "/open-apis/search/v2/data_sources", "批量获取数据源"),
    ("search/search/v2/data_source/patch.rs", "PatchDataSource", "patch", "/open-apis/search/v2/data_sources/{}", "修改数据源"),
    
    # SEARCH - data_source/item
    ("search/search/v2/data_source/item/create.rs", "CreateDataSourceItem", "post", "/open-apis/search/v2/data_sources/{}/items", "为指定数据项创建索引"),
    ("search/search/v2/data_source/item/delete.rs", "DeleteDataSourceItem", "delete", "/open-apis/search/v2/data_sources/{}/items/{}", "删除数据项"),
    
    # SEARCH - schema
    ("search/search/v2/schema/create.rs", "CreateSchema", "post", "/open-apis/search/v2/schemas", "创建数据范式"),
    ("search/search/v2/schema/delete.rs", "DeleteSchema", "delete", "/open-apis/search/v2/schemas/{}", "删除数据范式"),
    ("search/search/v2/schema/patch.rs", "PatchSchema", "patch", "/open-apis/search/v2/schemas/{}", "修改数据范式"),
    
    # SEARCH - app
    ("search/search/v2/app/create.rs", "SearchApp", "post", "/open-apis/search/v2/app", "搜索应用"),
    
    # SEARCH - doc_wiki
    ("search/search/v2/doc_wiki/search.rs", "SearchDocWiki", "post", "/open-apis/search/v2/doc_wiki/search", "搜索文档"),
    
    # SEARCH - message
    ("search/search/v2/message/create.rs", "SearchMessage", "post", "/open-apis/search/v2/message/create", "搜索消息"),
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
        let path = "{url}".to_string();
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
    os.makedirs(os.path.dirname(full_path), exist_ok=True)
    
    content = template.format(
        api_name=api_name,
        description=description,
        method=method,
        url=url
    )
    
    with open(full_path, 'w') as f:
        f.write(content)
    print(f"✅ {filepath}")

print(f"\n✅ openlark-analytics {len(apis)} 个 API 已创建")

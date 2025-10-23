#!/bin/bash

# 精确修复 open-lark-communication 中的 ApiRequest 私有字段访问错误

echo "正在修复 open-lark-communication 中的 ApiRequest 私有字段访问错误..."

# 修复 contact/v3/department.rs
cat > /tmp/department_fix.rs << 'EOF'
use open_lark_core::core::{
    api_req::ApiRequest, api_resp::BaseResponse, constants::AccessTokenType,
    endpoints::EndpointBuilder, http::Transport, req_option::RequestOption, SDKResult,
};

impl DepartmentService {
    pub async fn get(
        &self,
        department_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: Option<RequestOption>,
    ) -> SDKResult<serde_json::Value> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENTS,
            "department_id",
            department_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        if let Some(user_id_type) = user_id_type {
            api_req.query_params.insert("user_id_type".to_string(), user_id_type.to_string());
        }
        if let Some(department_id_type) = department_id_type {
            api_req.query_params.insert("department_id_type".to_string(), department_id_type.to_string());
        }

        let api_resp: BaseResponse<serde_json::Value> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result().map(|resp| resp.data)
    }

    pub async fn list(
        &self,
        department_id: Option<&str>,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        parent_department_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: Option<RequestOption>,
    ) -> SDKResult<serde_json::Value> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::GET);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        if let Some(department_id) = department_id {
            api_req.query_params.insert("department_id".to_string(), department_id.to_string());
        }
        if let Some(user_id_type) = user_id_type {
            api_req.query_params.insert("user_id_type".to_string(), user_id_type.to_string());
        }
        if let Some(department_id_type) = department_id_type {
            api_req.query_params.insert("department_id_type".to_string(), department_id_type.to_string());
        }
        if let Some(parent_department_id) = parent_department_id {
            api_req.query_params.insert("parent_department_id".to_string(), parent_department_id.to_string());
        }
        if let Some(page_size) = page_size {
            api_req.query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = page_token {
            api_req.query_params.insert("page_token".to_string(), page_token.to_string());
        }

        let api_resp: BaseResponse<serde_json::Value> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result().map(|resp| resp.data)
    }

    pub async fn create(
        &self,
        req: serde_json::Value,
        option: Option<RequestOption>,
    ) -> SDKResult<serde_json::Value> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::POST);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&req)?;

        let api_resp: BaseResponse<serde_json::Value> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result().map(|resp| resp.data)
    }

    pub async fn update(
        &self,
        department_id: &str,
        req: serde_json::Value,
        option: Option<RequestOption>,
    ) -> SDKResult<serde_json::Value> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::PATCH);
        api_req.set_api_path(EndpointBuilder::replace_param(
            open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENT,
            "department_id",
            department_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&req)?;

        let api_resp: BaseResponse<serde_json::Value> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result().map(|resp| resp.data)
    }

    pub async fn delete(
        &self,
        department_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<serde_json::Value> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(
            open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENT,
            "department_id",
            department_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = Vec::new();

        let api_resp: BaseResponse<serde_json::Value> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result().map(|resp| resp.data)
    }
}
EOF

echo "创建了修复模板文件"
echo "需要手动修复每个文件中的 ApiRequest 构造问题"

# 列出需要修复的文件
echo "需要修复的文件列表："
find crates/open-lark-communication/src -name "*.rs" -exec grep -l "ApiRequest {" {} \;

echo "修复提示："
echo "1. 将 ApiRequest { ... } 替换为 builder 模式"
echo "2. 使用 api_req.set_http_method() 设置 HTTP 方法"
echo "3. 使用 api_req.set_api_path() 设置 API 路径"
echo "4. 使用 api_req.set_supported_access_token_types() 设置访问令牌类型"
echo "5. 直接设置 api_req.body 和 api_req.query_params"
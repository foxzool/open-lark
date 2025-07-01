use crate::core::{
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config, constants::AccessTokenType,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 角色管理服务
pub struct FunctionalRoleService {
    config: Config,
}

impl FunctionalRoleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建角色
    pub async fn create(
        &self,
        req: &CreateFunctionalRoleRequest,
    ) -> crate::core::SDKResult<CreateFunctionalRoleResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/functional_roles".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 修改角色名称
    pub async fn update(
        &self,
        role_id: &str,
        req: &UpdateFunctionalRoleRequest,
    ) -> crate::core::SDKResult<UpdateFunctionalRoleResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path: format!("/open-apis/contact/v3/functional_roles/{}", role_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个角色信息
    pub async fn get(&self, role_id: &str) -> crate::core::SDKResult<GetFunctionalRoleResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/contact/v3/functional_roles/{}", role_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<GetFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取角色列表
    pub async fn list(
        &self,
        req: &ListFunctionalRolesRequest,
    ) -> crate::core::SDKResult<ListFunctionalRolesResponse> {
        let mut api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/contact/v3/functional_roles".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_size) = req.page_size {
            api_req
                .query_params
                .insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = &req.page_token {
            api_req
                .query_params
                .insert("page_token".to_string(), page_token.clone());
        }

        let resp =
            Transport::<ListFunctionalRolesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除角色
    pub async fn delete(
        &self,
        role_id: &str,
    ) -> crate::core::SDKResult<DeleteFunctionalRoleResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: format!("/open-apis/contact/v3/functional_roles/{}", role_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<DeleteFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFunctionalRoleRequest {
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFunctionalRoleResponse {
    pub role_id: String,
}

impl ApiResponseTrait for CreateFunctionalRoleResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionalRoleRequest {
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFunctionalRoleResponse {}

impl ApiResponseTrait for UpdateFunctionalRoleResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetFunctionalRoleResponse {
    pub role: FunctionalRole,
}

impl ApiResponseTrait for GetFunctionalRoleResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFunctionalRolesRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListFunctionalRolesResponse {
    pub roles: Vec<FunctionalRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListFunctionalRolesResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionalRole {
    /// 角色ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFunctionalRoleResponse {}

impl ApiResponseTrait for DeleteFunctionalRoleResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

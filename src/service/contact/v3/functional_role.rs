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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFunctionalRoleResponse {
    pub role_id: String,
}

impl ApiResponseTrait for CreateFunctionalRoleResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for CreateFunctionalRoleResponse {
    fn default() -> Self {
        Self {
            role_id: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionalRoleRequest {
    pub role_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionalRoleResponse {}

impl ApiResponseTrait for UpdateFunctionalRoleResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for UpdateFunctionalRoleResponse {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFunctionalRoleResponse {}

impl ApiResponseTrait for DeleteFunctionalRoleResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for DeleteFunctionalRoleResponse {
    fn default() -> Self {
        Self {}
    }
}

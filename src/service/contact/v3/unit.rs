use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, http::Transport,
    },
    service::contact::models::*,
};
use serde::{Deserialize, Serialize};

/// 单位管理服务
pub struct UnitService {
    config: Config,
}

impl UnitService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建单位
    pub async fn create(
        &self,
        req: &CreateUnitRequest,
    ) -> crate::core::SDKResult<CreateUnitResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/units".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateUnitResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 修改单位信息
    pub async fn patch(
        &self,
        unit_id: &str,
        req: &PatchUnitRequest,
    ) -> crate::core::SDKResult<PatchUnitResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path: format!("/open-apis/contact/v3/units/{}", unit_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<PatchUnitResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 建立部门与单位的绑定关系
    pub async fn bind_department(
        &self,
        unit_id: &str,
        req: &BindDepartmentRequest,
    ) -> crate::core::SDKResult<BindDepartmentResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!("/open-apis/contact/v3/units/{}/bind_department", unit_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BindDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 解除部门与单位的绑定关系
    pub async fn unbind_department(
        &self,
        unit_id: &str,
        req: &UnbindDepartmentRequest,
    ) -> crate::core::SDKResult<UnbindDepartmentResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: format!("/open-apis/contact/v3/units/{}/unbind_department", unit_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UnbindDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单位绑定的部门列表
    pub async fn list_department(
        &self,
        unit_id: &str,
        _req: &ListUnitDepartmentsRequest,
    ) -> crate::core::SDKResult<ListUnitDepartmentsResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/contact/v3/units/{}/list_department", unit_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp =
            Transport::<ListUnitDepartmentsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单位信息
    pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/contact/v3/units/{}", unit_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetUnitResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单位列表
    pub async fn list(&self, _req: &ListUnitsRequest) -> crate::core::SDKResult<ListUnitsResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/contact/v3/units".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<ListUnitsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除单位
    pub async fn delete(&self, unit_id: &str) -> crate::core::SDKResult<DeleteUnitResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: format!("/open-apis/contact/v3/units/{}", unit_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<DeleteUnitResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUnitRequest {
    pub unit: Unit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUnitResponse {
    pub unit: Unit,
}

impl ApiResponseTrait for CreateUnitResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for CreateUnitResponse {
    fn default() -> Self {
        Self {
            unit: Unit::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchUnitRequest {
    pub unit: Unit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchUnitResponse {
    pub unit: Unit,
}

impl ApiResponseTrait for PatchUnitResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for PatchUnitResponse {
    fn default() -> Self {
        Self {
            unit: Unit::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindDepartmentRequest {
    pub department_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindDepartmentResponse {}

impl ApiResponseTrait for BindDepartmentResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for BindDepartmentResponse {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindDepartmentRequest {
    pub department_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindDepartmentResponse {}

impl ApiResponseTrait for UnbindDepartmentResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for UnbindDepartmentResponse {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUnitDepartmentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUnitDepartmentsResponse {
    pub department_list: Vec<Department>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListUnitDepartmentsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for ListUnitDepartmentsResponse {
    fn default() -> Self {
        Self {
            department_list: Vec::new(),
            has_more: None,
            page_token: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUnitResponse {
    pub unit: Unit,
}

impl ApiResponseTrait for GetUnitResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for GetUnitResponse {
    fn default() -> Self {
        Self {
            unit: Unit::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUnitsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUnitsResponse {
    pub items: Vec<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListUnitsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for ListUnitsResponse {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            has_more: None,
            page_token: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUnitResponse {}

impl ApiResponseTrait for DeleteUnitResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for DeleteUnitResponse {
    fn default() -> Self {
        Self {}
    }
}

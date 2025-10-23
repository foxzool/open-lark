use open_lark_core::core::{
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
    constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
};
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 人员类型服务
pub struct EmployeeTypeEnumService {
    config: Config,
}

impl EmployeeTypeEnumService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 新增人员类型
    pub async fn create(
        &self,
        req: &CreateEmployeeTypeRequest,
    ) -> open_lark_core::core::SDKResult<CreateEmployeeTypeResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::POST);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_EMPLOYEE_TYPE_ENUMS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(req)?;

        let resp =
            Transport::<CreateEmployeeTypeResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新人员类型
    pub async fn update(
        &self,
        enum_id: &str,
        req: &UpdateEmployeeTypeRequest,
    ) -> open_lark_core::core::SDKResult<UpdateEmployeeTypeResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::PUT);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_EMPLOYEE_TYPE_ENUMS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(req)?;

        let resp =
            Transport::<UpdateEmployeeTypeResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询人员类型
    pub async fn list(
        &self,
        _req: &ListEmployeeTypesRequest,
    ) -> open_lark_core::core::SDKResult<ListEmployeeTypesResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::GET);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_EMPLOYEE_TYPE_ENUMS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = Vec::new();
        api_req.query_params = std::collections::HashMap::new();

        let resp =
            Transport::<ListEmployeeTypesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除人员类型
    pub async fn delete(
        &self,
        enum_id: &str,
    ) -> open_lark_core::core::SDKResult<DeleteEmployeeTypeResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::DELETE);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_EMPLOYEE_TYPE_ENUMS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = Vec::new();

        let resp =
            Transport::<DeleteEmployeeTypeResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployeeTypeRequest {
    pub employee_type_enum: EmployeeTypeEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateEmployeeTypeResponse {
    pub employee_type_enum: EmployeeTypeEnum,
}

impl ApiResponseTrait for CreateEmployeeTypeResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmployeeTypeRequest {
    pub employee_type_enum: EmployeeTypeEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateEmployeeTypeResponse {
    pub employee_type_enum: EmployeeTypeEnum,
}

impl ApiResponseTrait for UpdateEmployeeTypeResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEmployeeTypesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListEmployeeTypesResponse {
    pub items: Vec<EmployeeTypeEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListEmployeeTypesResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteEmployeeTypeResponse {}

impl ApiResponseTrait for DeleteEmployeeTypeResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}
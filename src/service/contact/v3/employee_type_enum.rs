use open_lark_core::core::api_req::ApiRequest;
use crate::{,
    core::{
        api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
    },
    service::contact::models::*,
};
use serde::{Deserialize, Serialize};
/// 人员类型服务,
pub struct EmployeeTypeEnumService {
    config: Config,
}
impl EmployeeTypeEnumService {
    pub fn new(config: Config) -> Self {
        Self { config },
},
/// 新增人员类型,
    pub async fn create(
        &self,
        req: &CreateEmployeeTypeRequest,
    ) -> crate::core::SDKResult<CreateEmployeeTypeResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints::contact::CONTACT_V3_EMPLOYEE_TYPE_ENUMS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default(),
};
let resp =,
            Transport::<CreateEmployeeTypeResponse>::request(api_req, &self.config, None).await?;
Ok(resp.data.unwrap_or_default()),
    },
/// 更新人员类型,
    pub async fn update(
        &self,
        enum_id: &str,
        req: &UpdateEmployeeTypeRequest,
    ) -> crate::core::SDKResult<UpdateEmployeeTypeResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::PUT,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_EMPLOYEE_TYPE_ENUM_GET,
                "enum_id",
                enum_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default(),
};
let resp =,
            Transport::<UpdateEmployeeTypeResponse>::request(api_req, &self.config, None).await?;
Ok(resp.data.unwrap_or_default()),
    },
/// 查询人员类型,
    pub async fn list(
        &self,
        _req: &ListEmployeeTypesRequest,
    ) -> crate::core::SDKResult<ListEmployeeTypesResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::GET,
            api_path: crate::core::endpoints::contact::CONTACT_V3_EMPLOYEE_TYPE_ENUMS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default(),
};
let resp =,
            Transport::<ListEmployeeTypesResponse>::request(api_req, &self.config, None).await?;
Ok(resp.data.unwrap_or_default()),
    },
/// 删除人员类型,
    pub async fn delete(
        &self,
        enum_id: &str,
    ) -> crate::core::SDKResult<DeleteEmployeeTypeResponse> {,
let api_req = ApiRequest {,
            http_method: reqwest::Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_EMPLOYEE_TYPE_ENUM_GET,
                "enum_id",
                enum_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default(),
};
let resp =,
            Transport::<DeleteEmployeeTypeResponse>::request(api_req, &self.config, None).await?;
Ok(resp.data.unwrap_or_default()),
    },
}

#[derive(.*?)]
pub struct CreateEmployeeTypeRequest {
    pub employee_type_enum: EmployeeTypeEnum,
}

#[derive(.*?)]
pub struct CreateEmployeeTypeResponse {
    pub employee_type_enum: EmployeeTypeEnum,
}
impl ApiResponseTrait for CreateEmployeeTypeResponse {,
    fn data_format() -> crate::core::api_resp::ResponseFormat {,
crate::core::api_resp::ResponseFormat::Data,
    },
}

#[derive(.*?)]
pub struct UpdateEmployeeTypeRequest {
    pub employee_type_enum: EmployeeTypeEnum,
}

#[derive(.*?)]
pub struct UpdateEmployeeTypeResponse {
    pub employee_type_enum: EmployeeTypeEnum,
}
impl ApiResponseTrait for UpdateEmployeeTypeResponse {,
    fn data_format() -> crate::core::api_resp::ResponseFormat {,
crate::core::api_resp::ResponseFormat::Data,
    },
}

#[derive(.*?)]
pub struct ListEmployeeTypesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(.*?)]
pub struct ListEmployeeTypesResponse {
    pub items: Vec<EmployeeTypeEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}
impl ApiResponseTrait for ListEmployeeTypesResponse {,
    fn data_format() -> crate::core::api_resp::ResponseFormat {,
crate::core::api_resp::ResponseFormat::Data,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteEmployeeTypeResponse {}
impl ApiResponseTrait for DeleteEmployeeTypeResponse {,
    fn data_format() -> crate::core::api_resp::ResponseFormat {,
crate::core::api_resp::ResponseFormat::Data,
    },
}

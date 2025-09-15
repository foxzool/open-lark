use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::task::models::{CustomField, UserIdType},
};

/// 自定义字段服务
pub struct CustomFieldService {
    pub config: Config,
}

/// 创建自定义字段请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomFieldRequest {
    /// 字段名称
    pub name: String,
    /// 字段类型
    #[serde(rename = "type")]
    pub type_: String,
    /// 资源类型
    pub resource_type: String,
    /// 资源ID
    pub resource_id: String,
}

/// 创建自定义字段响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomFieldResponse {
    /// 创建的自定义字段
    pub custom_field: CustomField,
}

impl ApiResponseTrait for CreateCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新自定义字段请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCustomFieldRequest {
    /// 字段名称
    pub name: String,
}

/// 更新自定义字段响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCustomFieldResponse {
    /// 更新后的自定义字段
    pub custom_field: CustomField,
}

impl ApiResponseTrait for UpdateCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取自定义字段响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCustomFieldResponse {
    /// 自定义字段详情
    pub custom_field: CustomField,
}

impl ApiResponseTrait for GetCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 自定义字段列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListCustomFieldsResponse {
    /// 自定义字段列表
    pub items: Vec<CustomField>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListCustomFieldsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加自定义字段到资源请求
#[derive(Debug, Serialize, Deserialize)]
pub struct AddCustomFieldRequest {
    /// 资源类型
    pub resource_type: String,
    /// 资源ID
    pub resource_id: String,
}

/// 移除自定义字段请求
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveCustomFieldRequest {
    /// 资源类型
    pub resource_type: String,
    /// 资源ID
    pub resource_id: String,
}

impl CustomFieldService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建自定义字段
    pub async fn create(
        &self,
        request: CreateCustomFieldRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateCustomFieldResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::TASK_V2_CUSTOM_FIELDS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取自定义字段
    pub async fn get(
        &self,
        custom_field_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetCustomFieldResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_CUSTOM_FIELD_GET,
                "custom_field_guid",
                custom_field_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新自定义字段
    pub async fn patch(
        &self,
        custom_field_guid: &str,
        request: UpdateCustomFieldRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateCustomFieldResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_CUSTOM_FIELD_GET,
                "custom_field_guid",
                custom_field_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 列取自定义字段
    pub async fn list(
        &self,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListCustomFieldsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(resource_type) = resource_type {
            query_params.insert("resource_type", resource_type.to_string());
        }
        if let Some(resource_id) = resource_id {
            query_params.insert("resource_id", resource_id.to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::TASK_V2_CUSTOM_FIELDS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 将自定义字段加入资源
    pub async fn add(
        &self,
        custom_field_guid: &str,
        request: AddCustomFieldRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_CUSTOM_FIELD_ADD,
                "custom_field_guid",
                custom_field_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 将自定义字段移出资源
    pub async fn remove(
        &self,
        custom_field_guid: &str,
        request: RemoveCustomFieldRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_CUSTOM_FIELD_REMOVE,
                "custom_field_guid",
                custom_field_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::task::models::{CustomFieldOption, UserIdType},
};

/// 自定义字段选项服务
pub struct CustomFieldOptionService {
    pub config: Config,
}

/// 创建自定义字段选项请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomFieldOptionRequest {
    /// 选项名称
    pub name: String,
    /// 颜色索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
}

/// 创建自定义字段选项响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomFieldOptionResponse {
    /// 创建的选项
    pub option: CustomFieldOption,
}

impl ApiResponseTrait for CreateCustomFieldOptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新自定义字段选项请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCustomFieldOptionRequest {
    /// 选项名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 颜色索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
}

/// 更新自定义字段选项响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCustomFieldOptionResponse {
    /// 更新后的选项
    pub option: CustomFieldOption,
}

impl ApiResponseTrait for UpdateCustomFieldOptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CustomFieldOptionService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建自定义字段选项
    pub async fn create(
        &self,
        custom_field_guid: &str,
        request: CreateCustomFieldOptionRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateCustomFieldOptionResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_CUSTOM_FIELD_OPTIONS,
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

    /// 更新自定义字段选项
    pub async fn patch(
        &self,
        custom_field_guid: &str,
        option_guid: &str,
        request: UpdateCustomFieldOptionRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateCustomFieldOptionResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let temp_path = EndpointBuilder::replace_param(
            Endpoints::TASK_V2_CUSTOM_FIELD_OPTION_GET,
            "custom_field_guid",
            custom_field_guid,
        );
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(&temp_path, "option_guid", option_guid),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

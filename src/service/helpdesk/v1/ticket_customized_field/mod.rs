use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{helpdesk::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::helpdesk::models::{CustomizedField, UserIdType},
};

/// 工单自定义字段服务
pub struct TicketCustomizedFieldService {
    pub config: Config,
}

/// 创建工单自定义字段请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTicketCustomizedFieldRequest {
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    pub field_type: String,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 默认值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// 选项列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

/// 创建工单自定义字段响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTicketCustomizedFieldResponse {
    /// 创建的自定义字段
    pub field: CustomizedField,
}

impl ApiResponseTrait for CreateTicketCustomizedFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新工单自定义字段请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTicketCustomizedFieldRequest {
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 默认值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// 选项列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

/// 更新工单自定义字段响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTicketCustomizedFieldResponse {
    /// 更新后的自定义字段
    pub field: CustomizedField,
}

impl ApiResponseTrait for UpdateTicketCustomizedFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取工单自定义字段响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTicketCustomizedFieldResponse {
    /// 自定义字段
    pub field: CustomizedField,
}

impl ApiResponseTrait for GetTicketCustomizedFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取全部工单自定义字段响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListTicketCustomizedFieldsResponse {
    /// 自定义字段列表
    pub fields: Vec<CustomizedField>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListTicketCustomizedFieldsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TicketCustomizedFieldService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建工单自定义字段
    ///
    /// 该接口用于创建工单自定义字段。
    ///
    /// # 参数
    ///
    /// - `request`: 创建请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 字段名称已存在
    pub async fn create(
        &self,
        request: CreateTicketCustomizedFieldRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateTicketCustomizedFieldResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: HELPDESK_V1_TICKET_CUSTOMIZED_FIELDS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除工单自定义字段
    ///
    /// 该接口用于删除指定的工单自定义字段。
    ///
    /// # 参数
    ///
    /// - `field_id`: 字段ID
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 字段不存在
    pub async fn delete(
        &self,
        field_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_DELETE,
                "field_id",
                field_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新工单自定义字段
    ///
    /// 该接口用于更新指定的工单自定义字段。
    ///
    /// # 参数
    ///
    /// - `field_id`: 字段ID
    /// - `request`: 更新请求参数
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 字段不存在
    pub async fn update(
        &self,
        field_id: &str,
        request: UpdateTicketCustomizedFieldRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateTicketCustomizedFieldResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_UPDATE,
                "field_id",
                field_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取指定工单自定义字段
    ///
    /// 该接口用于获取指定的工单自定义字段详情。
    ///
    /// # 参数
    ///
    /// - `field_id`: 字段ID
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 字段不存在
    pub async fn get(
        &self,
        field_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetTicketCustomizedFieldResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_GET,
                "field_id",
                field_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取全部工单自定义字段
    ///
    /// 该接口用于获取全部工单自定义字段列表。
    ///
    /// # 参数
    ///
    /// - `user_id_type`: 用户ID类型
    /// - `page_token`: 分页标记
    /// - `page_size`: 分页大小
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    pub async fn list(
        &self,
        user_id_type: Option<UserIdType>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListTicketCustomizedFieldsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token.to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: HELPDESK_V1_TICKET_CUSTOMIZED_FIELDS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

use reqwest::Method;
use serde::{Deserialize, Serialize};

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
    service::search::v2::models::{CreateSchemaRequest, Schema, UpdateSchemaRequest},
};

/// 数据范式服务
pub struct SchemaService {
    pub config: Config,
}

/// 创建数据范式响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSchemaResponse {
    /// 数据范式信息
    pub schema: Schema,
}

impl ApiResponseTrait for CreateSchemaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取数据范式响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSchemaResponse {
    /// 数据范式信息
    pub schema: Schema,
}

impl ApiResponseTrait for GetSchemaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新数据范式响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSchemaResponse {
    /// 数据范式信息
    pub schema: Schema,
}

impl ApiResponseTrait for UpdateSchemaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 空响应（用于删除等操作）
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptySchemaResponse {}

impl ApiResponseTrait for EmptySchemaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SchemaService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建数据范式
    ///
    /// 该接口用于创建搜索连接器的数据范式。
    ///
    /// 注意事项：
    /// - 需要申请相应权限
    /// - 数据范式定义需要符合JSON Schema规范
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `request`: 创建数据范式请求参数
    /// - `option`: 可选的请求配置
    pub async fn create(
        &self,
        data_source_id: &str,
        request: CreateSchemaRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateSchemaResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::SEARCH_V2_SCHEMA_CREATE,
                "data_source_id",
                data_source_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除数据范式
    ///
    /// 该接口用于删除指定的数据范式。
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `schema_id`: 数据范式ID
    /// - `option`: 可选的请求配置
    pub async fn delete(
        &self,
        data_source_id: &str,
        schema_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptySchemaResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::SEARCH_V2_SCHEMA_OPERATION,
                &[("data_source_id", data_source_id), ("schema_id", schema_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改数据范式
    ///
    /// 该接口用于修改指定数据范式的信息。
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `schema_id`: 数据范式ID
    /// - `request`: 更新数据范式请求参数
    /// - `option`: 可选的请求配置
    pub async fn patch(
        &self,
        data_source_id: &str,
        schema_id: &str,
        request: UpdateSchemaRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateSchemaResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::SEARCH_V2_SCHEMA_OPERATION,
                &[("data_source_id", data_source_id), ("schema_id", schema_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取数据范式
    ///
    /// 该接口用于获取指定数据范式的详细信息。
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `schema_id`: 数据范式ID
    /// - `option`: 可选的请求配置
    pub async fn get(
        &self,
        data_source_id: &str,
        schema_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetSchemaResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::SEARCH_V2_SCHEMA_OPERATION,
                &[("data_source_id", data_source_id), ("schema_id", schema_id)],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

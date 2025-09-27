use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::search::v2::models::{
        CreateDataSourceRequest, DataSource, ListDataSourceRequest, ListDataSourceResponse,
        UpdateDataSourceRequest,
    },
};

/// 数据源服务
pub struct DataSourceService {
    pub config: Config,
}

/// 创建数据源响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDataSourceResponse {
    /// 数据源信息
    pub data_source: DataSource,
}

impl ApiResponseTrait for CreateDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取数据源响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDataSourceResponse {
    /// 数据源信息
    pub data_source: DataSource,
}

impl ApiResponseTrait for GetDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新数据源响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDataSourceResponse {
    /// 数据源信息
    pub data_source: DataSource,
}

impl ApiResponseTrait for UpdateDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 空响应（用于删除等操作）
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyDataSourceResponse {}

impl ApiResponseTrait for EmptyDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ListDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DataSourceService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建数据源
    ///
    /// 该接口用于创建搜索连接器的数据源。
    ///
    /// 注意事项：
    /// - 需要申请相应权限
    /// - 数据源名称需要唯一
    ///
    /// # 参数
    ///
    /// - `request`: 创建数据源请求参数
    /// - `option`: 可选的请求配置
    pub async fn create(
        &self,
        request: CreateDataSourceRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateDataSourceResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: crate::core::endpoints::search::SEARCH_V2_DATA_SOURCES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除数据源
    ///
    /// 该接口用于删除指定的数据源。
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `option`: 可选的请求配置
    pub async fn delete(
        &self,
        data_source_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyDataSourceResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::search::SEARCH_V2_DATA_SOURCE_OPERATION,
                "data_source_id",
                data_source_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改数据源
    ///
    /// 该接口用于修改指定数据源的信息。
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `request`: 更新数据源请求参数
    /// - `option`: 可选的请求配置
    pub async fn patch(
        &self,
        data_source_id: &str,
        request: UpdateDataSourceRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateDataSourceResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::search::SEARCH_V2_DATA_SOURCE_OPERATION,
                "data_source_id",
                data_source_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取数据源
    ///
    /// 该接口用于获取指定数据源的详细信息。
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `option`: 可选的请求配置
    pub async fn get(
        &self,
        data_source_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetDataSourceResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::search::SEARCH_V2_DATA_SOURCE_OPERATION,
                "data_source_id",
                data_source_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量获取数据源
    ///
    /// 该接口用于批量获取数据源列表。
    ///
    /// # 参数
    ///
    /// - `request`: 查询参数（可选）
    /// - `option`: 可选的请求配置
    pub async fn list(
        &self,
        request: Option<ListDataSourceRequest>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListDataSourceResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: crate::core::endpoints::search::SEARCH_V2_DATA_SOURCES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        // 如果有查询参数，添加到URL中
        if let Some(req) = request {
            if let Some(page_size) = req.page_size {
                api_req
                    .query_params
                    .insert("page_size", page_size.to_string());
            }
            if let Some(page_token) = req.page_token {
                api_req.query_params.insert("page_token", page_token);
            }
        }

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for DataSourceService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "data_source"
    }

    fn service_version() -> &'static str {
        "v2"
    }
}

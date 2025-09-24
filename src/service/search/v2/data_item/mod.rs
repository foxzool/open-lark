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
        SDKResult,
    },
    service::search::v2::models::{BatchCreateDataItemRequest, CreateDataItemRequest, DataItem},
};

/// 数据项服务
pub struct DataItemService {
    pub config: Config,
}

/// 创建数据项响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDataItemResponse {
    /// 数据项信息
    pub data_item: DataItem,
}

impl ApiResponseTrait for CreateDataItemResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量创建数据项响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateDataItemResponse {
    /// 成功创建的数据项列表
    pub success_items: Vec<DataItem>,
    /// 失败的数据项信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for BatchCreateDataItemResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取数据项响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDataItemResponse {
    /// 数据项信息
    pub data_item: DataItem,
}

impl ApiResponseTrait for GetDataItemResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 空响应（用于删除等操作）
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyDataItemResponse {}

impl ApiResponseTrait for EmptyDataItemResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DataItemService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 为指定数据项创建索引
    ///
    /// 该接口用于为指定的数据项创建搜索索引。
    ///
    /// 注意事项：
    /// - 需要申请相应权限
    /// - 数据项ID需要在数据源内唯一
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `request`: 创建数据项请求参数
    /// - `option`: 可选的请求配置
    pub async fn create(
        &self,
        data_source_id: &str,
        request: CreateDataItemRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateDataItemResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::search::SEARCH_V2_DATA_ITEM_CREATE,
                "data_source_id",
                data_source_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 批量为数据项创建索引
    ///
    /// 该接口用于批量为数据项创建搜索索引。
    ///
    /// 注意事项：
    /// - 支持批量操作提升效率
    /// - 最大支持批量创建100个数据项
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `request`: 批量创建数据项请求参数
    /// - `option`: 可选的请求配置
    pub async fn batch_create(
        &self,
        data_source_id: &str,
        request: BatchCreateDataItemRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchCreateDataItemResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::search::SEARCH_V2_DATA_ITEM_BATCH_CREATE,
                "data_source_id",
                data_source_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除数据项
    ///
    /// 该接口用于删除指定的数据项及其索引。
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `data_item_id`: 数据项ID
    /// - `option`: 可选的请求配置
    pub async fn delete(
        &self,
        data_source_id: &str,
        data_item_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyDataItemResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_params_from_array(
                crate::core::endpoints::search::SEARCH_V2_DATA_ITEM_OPERATION,
                &[
                    ("data_source_id", data_source_id),
                    ("data_item_id", data_item_id),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询指定数据项
    ///
    /// 该接口用于查询指定数据项的详细信息。
    ///
    /// # 参数
    ///
    /// - `data_source_id`: 数据源ID
    /// - `data_item_id`: 数据项ID
    /// - `option`: 可选的请求配置
    pub async fn get(
        &self,
        data_source_id: &str,
        data_item_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetDataItemResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                crate::core::endpoints::search::SEARCH_V2_DATA_ITEM_OPERATION,
                &[
                    ("data_source_id", data_source_id),
                    ("data_item_id", data_item_id),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

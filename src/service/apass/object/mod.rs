use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpointshttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::apass::models::{
        BatchRecordQueryRequest, BatchRecordRequest, OqlQueryRequest, OqlQueryResult, PageResponse,
        Record, RecordCreateRequest, RecordDeleteRequest, RecordQueryRequest, RecordSearchRequest,
        RecordUpdateRequest,
};
/// 对象操作服务
pub struct ObjectService {
}
    pub config: Config,
/// OQL查询响应
#[derive(Debug, Clone)]
pub struct OqlQueryResponse {
}
    /// OQL查询结果
#[serde(flatten)]
    pub query_result: OqlQueryResult,
impl ApiResponseTrait for.* {
}
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 记录搜索响应
#[derive(Debug, Clone)]
pub struct RecordSearchResponse {
}
    /// 分页响应数据
#[serde(flatten)]
    pub page_response: PageResponse<Record>,
impl ApiResponseTrait for.* {
}
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 记录查询响应
#[derive(Debug, Clone)]
pub struct RecordQueryResponse {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 记录创建响应
#[derive(Debug, Clone)]
}
pub struct RecordCreateResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 记录更新响应
#[derive(Debug, Clone)]
}
pub struct RecordUpdateResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 记录删除响应
#[derive(Debug, Clone)]
}
pub struct RecordDeleteResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 批量记录查询响应
#[derive(Debug, Clone)]
}
pub struct BatchRecordQueryResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 批量记录操作响应
#[derive(Debug, Clone)]
}
pub struct BatchRecordResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl ObjectService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 执行OQL查询
    ///,
/// 该接口用于执行OQL（Object Query Language）查询。
    ///,
/// # 参数
    ///,
/// - `request`: OQL查询请求参数
    /// - `option`: 可选的请求配置
pub async fn oql_query(,
        &self,
        request: OqlQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<OqlQueryResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::APASS_V1_OBJECT_OQL,
                "app_id",
                &request.app_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&serde_json::json!({
                "oql": request.oql,
                "parameters": request.parameters,
}))?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 搜索记录
    ///,
/// 该接口用于在指定对象中搜索记录。
    ///,
/// # 参数
    ///,
/// - `request`: 记录搜索请求参数
    /// - `option`: 可选的请求配置
pub async fn search_records(,
        &self,
        request: RecordSearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RecordSearchResponse>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_OBJECT_RECORD_SEARCH,
                &[
                    ("app_id", &request.app_id),
                    ("object_api_name", &request.object_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&serde_json::json!({,
"keyword": request.keyword,
            }))?,
            ..Default::default()
};
// 添加查询参数
        if let Some(page_size) = request.page_size {,
api_req
                .query_params
                .insert("page_size", page_size.to_string());
if let Some(page_token) = request.page_token {,
            api_req.query_params.insert("page_token", page_token);
        Transport::request(api_req, &self.config, option).await,
/// 获取记录详情
    ///,
/// 该接口用于获取指定记录的详情信息。
    ///,
/// # 参数
    ///,
/// - `request`: 记录查询请求参数
    /// - `option`: 可选的请求配置
pub async fn get_record(,
        &self,
        request: RecordQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RecordQueryResponse>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_OBJECT_RECORD_GET,
                &[
                    ("app_id", &request.app_id),
                    ("object_api_name", &request.object_api_name),
                    ("record_id", &request.record_id),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
// 添加字段查询参数
        if let Some(fields) = request.fields {
            api_req.query_params.insert("fields", fields.join(","));
        Transport::request(api_req, &self.config, option).await,
/// 编辑记录
    ///,
/// 该接口用于编辑指定的记录。
    ///,
/// # 参数
    ///,
/// - `request`: 记录更新请求参数
    /// - `option`: 可选的请求配置
pub async fn update_record(,
        &self,
        request: RecordUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RecordUpdateResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_OBJECT_RECORD_UPDATE,
                &[
                    ("app_id", &request.app_id),
                    ("object_api_name", &request.object_api_name),
                    ("record_id", &request.record_id),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request.data)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 删除记录
    ///,
/// 该接口用于删除指定的记录。
    ///,
/// # 参数
    ///,
/// - `request`: 记录删除请求参数
    /// - `option`: 可选的请求配置
pub async fn delete_record(,
        &self,
        request: RecordDeleteRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RecordDeleteResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_OBJECT_RECORD_DELETE,
                &[
                    ("app_id", &request.app_id),
                    ("object_api_name", &request.object_api_name),
                    ("record_id", &request.record_id),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 新建记录
    ///,
/// 该接口用于创建新的记录。
    ///,
/// # 参数
    ///,
/// - `request`: 记录创建请求参数
    /// - `option`: 可选的请求配置
pub async fn create_record(,
        &self,
        request: RecordCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RecordCreateResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_OBJECT_RECORD_CREATE,
                &[
                    ("app_id", &request.app_id),
                    ("object_api_name", &request.object_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request.data)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 批量编辑记录
    ///,
/// 该接口用于批量编辑记录。
    ///,
/// # 参数
    ///,
/// - `request`: 批量记录请求参数
    /// - `option`: 可选的请求配置
pub async fn batch_update_records(,
        &self,
        request: BatchRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchRecordResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_OBJECT_RECORD_BATCH_UPDATE,
                &[
                    ("app_id", &request.app_id),
                    ("object_api_name", &request.object_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&serde_json::json!({,
"records": request.records,
            }))?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 查询记录列表
    ///,
/// 该接口用于查询记录列表。
    ///,
/// # 参数
    ///,
/// - `request`: 批量记录查询请求参数
    /// - `option`: 可选的请求配置
pub async fn batch_query_records(,
        &self,
        request: BatchRecordQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchRecordQueryResponse>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_OBJECT_RECORD_BATCH_QUERY,
                &[
                    ("app_id", &request.app_id),
                    ("object_api_name", &request.object_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&serde_json::json!({
                "filter": request.filter,
                "sort": request.sort,
}))?,
            ..Default::default()
};
// 添加查询参数
        if let Some(page_size) = request.page_size {,
api_req
                .query_params
                .insert("page_size", page_size.to_string());
if let Some(page_token) = request.page_token {,
            api_req.query_params.insert("page_token", page_token);
        Transport::request(api_req, &self.config, option).await,
/// 批量删除记录
    ///,
/// 该接口用于批量删除记录。
    ///,
/// # 参数
    ///,
/// - `request`: 批量记录请求参数
    /// - `option`: 可选的请求配置
pub async fn batch_delete_records(,
        &self,
        request: BatchRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchRecordResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_OBJECT_RECORD_BATCH_DELETE,
                &[
                    ("app_id", &request.app_id),
                    ("object_api_name", &request.object_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&serde_json::json!({,
"records": request.records,
            }))?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 批量新建记录
    ///,
/// 该接口用于批量新建记录。
    ///,
/// # 参数
    ///,
/// - `request`: 批量记录请求参数
    /// - `option`: 可选的请求配置
pub async fn batch_create_records(,
        &self,
        request: BatchRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchRecordResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::APASS_V1_OBJECT_RECORD_BATCH_CREATE,
                &[
                    ("app_id", &request.app_id),
                    ("object_api_name", &request.object_api_name),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&serde_json::json!({,
"records": request.records,
            }))?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}
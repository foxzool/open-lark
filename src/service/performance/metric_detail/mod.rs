use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::performance::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    service::performance::models::MetricDetail,
};
/// 指标数据管理服务
pub struct MetricDetailService {
}

impl MetricDetailService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取被评估人关键指标结果
    ///,
/// 查询指定被评估人在指定项目中的关键指标数据。
    ///,
/// # Arguments
    ///,
/// * `request` - 指标详情查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回指标详情列表
pub async fn query_metric_details(,
        &self,
        request: MetricDetailQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricDetailQueryResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_V1_METRIC_DETAIL_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 录入被评估人关键指标数据
    ///,
/// 批量录入或更新被评估人的关键指标数据。
    ///,
/// # Arguments
    ///,
/// * `request` - 指标数据导入请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回导入结果
pub async fn import_metric_details(,
        &self,
        request: MetricDetailImportRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MetricDetailImportResponse>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: PERFORMANCE_V1_METRIC_DETAIL_IMPORT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 指标详情查询请求
#[derive(Debug, Clone)]
pub struct MetricDetailQueryRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 指标数据导入请求
#[derive(Debug, Clone)]
}
pub struct MetricDetailImportRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}}}}}}}}
use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::Deserialize;
use serde_json::json;
use crate::{,
core::{,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::{cardkit::*, EndpointBuilder}
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    }
    impl_executable_builder_owned,
    service::cardkit::v1::models::{BatchUpdateOperation, Card, UserIdType}
};
use super::CardService;
/// 批量更新卡片实体请求
#[derive(Debug, Clone)]
pub struct BatchUpdateCardRequest {
    pub api_req: ApiRequest,
    /// 卡片ID
    pub card_id: String,
    /// 批量更新操作列表
    pub operations: Vec<BatchUpdateOperation>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>}
impl BatchUpdateCardRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 批量更新卡片实体请求构建器,
pub struct BatchUpdateCardRequestBuilder {
    request: BatchUpdateCardRequest}
impl BatchUpdateCardRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 构建请求体,
        let body = json!({,
"operations": self.request.operations});
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 批量更新卡片实体响应数据,
#[derive(Debug, Clone)]
pub struct BatchUpdateCardResponseData {
    /// 更新后的卡片信息
    pub card: Card,
/// 批量更新卡片实体响应,
#[derive(Debug, Clone)]
pub struct BatchUpdateCardResponse {
    /// 响应数据
    pub data: BatchUpdateCardResponseData,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl CardService {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder宏,
impl_executable_builder_owned!(
    BatchUpdateCardRequestBuilder,
    CardService,
    BatchUpdateCardRequest,
    BaseResponse<BatchUpdateCardResponse>,
    batch_update,
);

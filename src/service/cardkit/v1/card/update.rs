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
    service::cardkit::v1::models::{Card, UserIdType}
};
use super::CardService;
/// 全量更新卡片实体请求
#[derive(Debug, Clone)]
pub struct UpdateCardRequest {
    pub api_req: ApiRequest,
    /// 卡片ID
    pub card_id: String,
    /// 卡片标题
    pub title: Option<String>,
    /// 卡片描述
    pub description: Option<String>,
    /// 卡片JSON内容
    pub card_json: Option<serde_json::Value>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>}
impl UpdateCardRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 全量更新卡片实体请求构建器,
pub struct UpdateCardRequestBuilder {
    request: UpdateCardRequest}
impl UpdateCardRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 构建请求体,
        let mut body = json!({});
if let Some(ref title) = self.request.title {,
            body["title"] = json!(title);
if let Some(ref description) = self.request.description {,
            body["description"] = json!(description);
if let Some(ref card_json) = self.request.card_json {,
            body["card_json"] = card_json.clone();
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 全量更新卡片实体响应数据,
#[derive(Debug, Clone)]
pub struct UpdateCardResponseData {
    /// 更新后的卡片信息
    pub card: Card,
/// 全量更新卡片实体响应,
#[derive(Debug, Clone)]
pub struct UpdateCardResponse {
    /// 响应数据
    pub data: UpdateCardResponseData,
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
    UpdateCardRequestBuilder,
    CardService,
    UpdateCardRequest,
    BaseResponse<UpdateCardResponse>,
    update,
);

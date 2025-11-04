#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
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
    service::cardkit::v1::models::{CardElement, UserIdType}
};
use super::CardElementService;
/// 新增组件请求
#[derive(Debug, Clone)]
pub struct CreateElementRequest {
    pub api_req: ApiRequest,
    /// 卡片ID
    pub card_id: String,
    /// 组件类型
    pub element_type: Option<String>,
    /// 组件内容
    pub content: Option<serde_json::Value>,
    /// 组件属性
    pub properties: Option<serde_json::Value>,
    /// 父组件ID
    pub parent_id: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>}
impl CreateElementRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 新增组件请求构建器,
pub struct CreateElementRequestBuilder {
    request: CreateElementRequest}
impl CreateElementRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 构建请求体,
        let mut body = json!({});
if let Some(ref element_type) = self.request.element_type {,
            body["element_type"] = json!(element_type);
if let Some(ref content) = self.request.content {,
            body["content"] = content.clone();
if let Some(ref properties) = self.request.properties {,
            body["properties"] = properties.clone();
if let Some(ref parent_id) = self.request.parent_id {,
            body["parent_id"] = json!(parent_id);
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 新增组件响应数据,
#[derive(Debug, Clone)]
pub struct CreateElementResponseData {
    /// 创建的组件信息
    pub element: CardElement,
/// 新增组件响应,
#[derive(Debug, Clone)]
pub struct CreateElementResponse {
    /// 响应数据
    pub data: CreateElementResponseData,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl CardElementService {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder宏,
impl_executable_builder_owned!(
    CreateElementRequestBuilder,
    CardElementService,
    CreateElementRequest,
    BaseResponse<CreateElementResponse>,
    create,
);

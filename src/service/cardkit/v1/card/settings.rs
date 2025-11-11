#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api_req::ApiRequest;
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
    service::cardkit::v1::models::{CardSettings, UserIdType}
};
use super::CardService;
/// 更新卡片配置请求
#[derive(Debug, Clone)]
pub struct UpdateCardSettingsRequest {
    pub api_req: ApiRequest,
    /// 卡片ID
    pub card_id: String,
    /// 是否启用交互
    pub enable_interaction: Option<bool>,
    /// 卡片主题
    pub theme: Option<String>,
    /// 自定义配置
    pub custom_config: Option<serde_json::Value>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>}
impl UpdateCardSettingsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新卡片配置请求构建器,
pub struct UpdateCardSettingsRequestBuilder {
    request: UpdateCardSettingsRequest}
impl UpdateCardSettingsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 构建请求体,
        let mut body = json!({});
if let Some(enable_interaction) = self.request.enable_interaction {,
            body["enable_interaction"] = json!(enable_interaction);
if let Some(ref theme) = self.request.theme {,
            body["theme"] = json!(theme);
if let Some(ref custom_config) = self.request.custom_config {,
            body["custom_config"] = custom_config.clone();
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 更新卡片配置响应数据,
#[derive(Debug, Clone)]
pub struct UpdateCardSettingsResponseData {
    /// 卡片配置信息
    pub settings: CardSettings,
/// 更新卡片配置响应,
#[derive(Debug, Clone)]
pub struct UpdateCardSettingsResponse {
    /// 响应数据
    pub data: UpdateCardSettingsResponseData,
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
    UpdateCardSettingsRequestBuilder,
    CardService,
    UpdateCardSettingsRequest,
    BaseResponse<UpdateCardSettingsResponse>,
    settings,
);

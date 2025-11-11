#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use reqwest::Method;
use open_lark_core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, LINGO_DRAFT_CREATE, LINGO_DRAFT_UPDATEhttp::Transport,
        req_option::RequestOption,
        trait_system::AsyncServiceOperation,
        SDKResult,
    impl _basic_service { impl _service_constructor {
    service::lingo::models::{Draft, OuterInfo, RelatedMeta}
};
/// 草稿管理服务
#[derive(Debug, Clone)]
pub struct DraftService {
}
    pub config: Config,
impl DraftService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新草稿
    ///,
/// 更新指定草稿的内容。
    ///,
/// # Arguments
    ///,
/// * `draft_id` - 草稿ID
    /// * `request` - 草稿更新请求
/// * `option` - 请求选项，可选
    ///,
/// # Returns
    ///,
/// 返回更新后的草稿信息
    pub async fn update_draft(
        &self,
        draft_id: &str,
        request: DraftUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DraftUpdateResponse>> {,
let draft_id = draft_id.to_string(); // 移到闭包外部以避免借用检查问题
        <Self as AsyncServiceOperation<DraftUpdateRequest, BaseResponse<DraftUpdateResponse>>>::execute_with_observability(
            self, "update_draft", move || async move {,
let api_req = ApiRequest {,
                http_http_http_method: Method::PUT,
                api_path: EndpointBuilder::replace_param(LINGO_DRAFT_UPDATE, "{draft_id}", &draft_id),
                supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
                body: serde_json::to_vec(&request)?,
                ..Default::default()
};
            Transport::request(api_req, &self.config, option).await,
}).await,
/// 草稿创建请求
#[derive(Debug, Clone)]
pub struct DraftCreateRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 草稿更新请求
#[derive(Debug, Clone)]
}
pub struct DraftUpdateRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    // 使用宏实现基础 Service traits
impl_basic_service!(DraftService, "lingo.draft", "v1");
impl_service_constructor!(DraftService);
// 实现异步操作支持
impl AsyncServiceOperation<DraftCreateRequest, BaseResponse<DraftCreateResponse>> for DraftService {impl AsyncServiceOperation<DraftUpdateRequest, BaseResponse<DraftUpdateResponse>> for DraftService {}
}
}}}}}}}}}}}}}
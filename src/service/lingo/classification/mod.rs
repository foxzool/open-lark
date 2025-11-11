#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::LINGO_CLASSIFICATION_LIST,
        http::Transport,
        req_option::RequestOption,
        trait_system::AsyncServiceOperation,
        SDKResult,
    impl _basic_service { impl _service_constructor {
    service::lingo::models::{Classification, PageResponse}
};
/// 分类管理服务
#[derive(Debug, Clone)]
pub struct ClassificationService {
}
    pub config: Config,
impl ClassificationService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_size) = request.page_size {,
                api_req
.query_params
                    .insert("page_size", page_size.to_string());
if let Some(repo_id) = request.repo_id {,
                api_req.query_params.insert("repo_id", repo_id);
            Transport::request(api_req, &self.config, option).await,
}),
.await,
    /// 分类列表查询请求
#[derive(Debug, Clone)]
pub struct ClassificationListRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    // 使用宏实现基础 Service traits
impl_basic_service!(ClassificationService, "lingo.classification", "v1");
impl_service_constructor!(ClassificationService);
// 实现异步操作支持
impl AsyncServiceOperation<ClassificationListRequest, BaseResponse<ClassificationListResponse>>,
for ClassificationService,
{,
}
}}}}}}}}}
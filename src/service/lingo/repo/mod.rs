use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::LINGO_REPO_LIST,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    service::lingo::models::{PageResponse, Repo}
};
/// 词库管理服务
pub struct RepoService {
}

impl RepoService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取词库列表
    ///,
/// 获取当前用户可访问的词库列表，支持分页查询。
    ///,
/// # Arguments
    ///,
/// * `request` - 词库列表查询请求
    /// * `option` - 请求选项，可选
///,
    /// # Returns
///,
    /// 返回词库列表
pub async fn list_repos(,
        &self,
        request: RepoListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RepoListResponse>> {,
let mut api_req = ApiRequest {,
            http_method: Method::GET,
            api_path: LINGO_REPO_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default(),
};
// 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert("page_size", page_size.to_string());
        Transport::request(api_req, &self.config, option).await,
/// 词库列表查询请求
#[derive(Debug, Clone)]
pub struct RepoListRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
}}}}}}}}
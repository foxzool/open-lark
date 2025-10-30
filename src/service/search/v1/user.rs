use log::error;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse}
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    validation::{self, ValidationResult}
    SDKResult,
};
use crate::impl_full_service;
pub struct UserService {
    config: Config}
impl UserService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 搜索用户,
    ///,
/// # API文档,
    ///,
/// https://open.feishu.cn/document/uMTM4UjLzEDO14yMxgTN,
    pub async fn search_user(
        &self,
        search_user_request: SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<SearchUserResponse> {,
let mut api_req = search_user_request.api_request;
        api_req.set_http_method(Method::GET);
api_req.set_api_path(crate::core::endpoints::search::SEARCH_V1_USER.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::User]);
let api_resp: BaseResponse<SearchUserResponse> =,
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result()}
/// 搜索用户 (返回BaseResponse供迭代器使用),
    async fn search_user_with_base_response(
        &self,
        search_user_request: SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchUserResponse>> {,
let mut api_req = search_user_request.api_request;
        api_req.set_http_method(Method::GET);
api_req.set_api_path(crate::core::endpoints::search::SEARCH_V1_USER.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::User]);

        Transport::request(api_req, &self.config, option).await}
pub fn search_user_iter(,
        &self,
        search_user_request: SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SearchUserIterator<'_> {,
SearchUserIterator {,
            user_service: self,
            request: search_user_request,
            option,
            has_more: true}
/// 使用分页验证搜索用户,
    ///,
/// 提供一个更安全的方式来搜索用户，自动验证分页参数,
    pub async fn search_user_with_validated_pagination(
        &self,
        query: impl ToString,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<SearchUserResponse> {,
// 创建请求构建器,
        let builder = SearchUserRequest::builder(),
.query()
            .with_pagination(page_size, page_token)?;

        self.search_user(builder.build(), option).await}

impl_full_service!(UserService, "search.user", "v1");
/// 搜索用户请求,
#[derive(Debug, Clone)]
pub struct SearchUserRequest {
    api_request: ApiRequest}
impl SearchUserRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct SearchUserRequestBuilder {
    search_user_request: SearchUserRequest}
impl SearchUserRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}ValidationResult::Invalid(msg) => {,
                log::error!("Invalid page token: {}", msg);
        }
self.search_user_request,
            .api_request,
.query_params
            .insert("page_token", token);
self,
    }
pub fn w+.*{
        self.search_user_request}
/// 使用分页验证构建器设置分页参数,
    ///,
/// 这个方法提供了一个更安全的分页参数设置方式，会自动验证参数的有效性,
    /// 搜索服务的分页大小限制为 1-200,
pub fn with_pagination(,
        mut self,
        page_size: Option<u32>,
        page_token: Option<String>,
    ) -> SDKResult<Self> {,
let mut pagination_builder =,
            validation::pagination::PaginationRequestBuilder::<SearchUserResponse>::new();
if let Some(size) = page_size {,
            // 搜索服务有更严格的分页大小限制（1-200）,
if size > 200 {,
                return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                    "Page size {} exceeds maximum limit of 200 for search service",
                    size,
)));
            }
pagination_builder = pagination_builder.with_page_size(size);
        }
if let Some(token) = page_token {,
            pagination_builder = pagination_builder.with_page_token(token);
// 构建分页参数,
        let params = pagination_builder.build()?;
// 应用到请求中,
        for (key, value) in params {,
self.search_user_request,
                .api_request,
.query_params
                .insert(key, value);
Ok(self),
    }
crate::impl_executable_builder_owned!(,
    SearchUserRequestBuilder,
    UserService,
    SearchUserRequest,
    SearchUserResponse,
    search_user,
);
#[derive(Debug, Clone)]
pub struct SearchUserResponse {
    /// 搜索到的用户列表。
    pub users: Vec<UserInSearchResponse>,
    /// 是否还有更多用户，值为 true 表示存在下一页。
    pub has_more: bool,
    /// 分页标识，存在下一页的时候返回。下次请求带上此标识可以获取下一页的用户。,
#[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>}
/// 搜索到的用户信息。,
#[derive(Debug, Clone)]
pub struct UserInSearchResponse {
    /// 用户的头像 URL。
    pub avatar: UserAvatar,
    /// 用户的部门信息。
    pub department_ids: Vec<String>,
    /// 用户的姓名。
    pub name: String,
    /// 用户的 open_id。
    pub open_id: String,
    /// 用户的 open_id。,
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>}
/// 用户的头像信息。,
#[derive(Debug, Clone)]
pub struct UserAvatar {
    /// 用户的头像图片 URL，72×72px。
    pub avatar_72: String,
    /// 用户的头像图片 URL，240×240px。
    pub avatar_240: String,
    /// 用户的头像图片 URL，640×640px。
    pub avatar_640: String,
    /// 用户的头像图片 URL，原始大小。
    pub avatar_origin: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> crate::core::api_resp::ResponseFormat {,
crate::core::api_resp::ResponseFormat::Data
    }
pub struct SearchUserIterator<'a> {,
    user_service: &'a UserService,
    request: SearchUserRequest,
    option: Option<RequestOption>,
    has_more: bool}
/// # API文档,
///
/// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search,
impl SearchUserIterator<'_> {,
    pub async fn next(&mut self) -> Option<Vec<UserInSearchResponse>> {,
if !self.has_more {,
            return None;
match self,
            .user_service
            .search_user_with_base_response(self.request.clone(), self.option.clone()),
.await,
        {,
Ok(resp) => match resp.data {,
                Some(data) => {,
self.has_more = data.has_more;
                    if data.has_more {,
if let Some(token) = data.page_token {,
                            self.request,
.api_request,
                                .query_params
                                .insert("page_token", token);
Some(data.users)} else {,
// has_more is true but no page_token. Stop iterating to avoid panic.,
                            self.has_more = false;
Some(data.users)}
} else if data.users.is_empty() {,
None} else {,
Some(data.users)}
                None => None,
            }
            Err(e) => {
                error!("Error: {e:?}");
None,
}

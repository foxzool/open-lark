use log::error;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    validation::{self, ValidationResult},
    SDKResult,
};
use crate::impl_full_service;

pub struct UserService {
    config: Config,
}

impl UserService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 搜索用户
    ///
    /// <https://open.feishu.cn/document/server-docs/search-v1/user/search>
    pub async fn search_user(
        &self,
        search_user_request: SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<SearchUserResponse> {
        let mut api_req = search_user_request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = crate::core::endpoints::search::SEARCH_V1_USER.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User];

        let api_resp: BaseResponse<SearchUserResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 搜索用户 (返回BaseResponse供迭代器使用)
    async fn search_user_with_base_response(
        &self,
        search_user_request: SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchUserResponse>> {
        let mut api_req = search_user_request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = crate::core::endpoints::search::SEARCH_V1_USER.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User];

        Transport::request(api_req, &self.config, option).await
    }

    pub fn search_user_iter(
        &self,
        search_user_request: SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SearchUserIterator<'_> {
        SearchUserIterator {
            user_service: self,
            request: search_user_request,
            option,
            has_more: true,
        }
    }

    /// 使用分页验证搜索用户
    ///
    /// 提供一个更安全的方式来搜索用户，自动验证分页参数
    pub async fn search_user_with_validated_pagination(
        &self,
        query: impl ToString,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<SearchUserResponse> {
        // 创建请求构建器
        let builder = SearchUserRequest::builder()
            .query(query)
            .with_pagination(page_size, page_token)?;

        self.search_user(builder.build(), option).await
    }
}

impl_full_service!(UserService, "search.user", "v1");

/// 搜索用户请求
#[derive(Default, Clone)]
pub struct SearchUserRequest {
    api_request: ApiRequest,
}

impl SearchUserRequest {
    pub fn builder() -> SearchUserRequestBuilder {
        SearchUserRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct SearchUserRequestBuilder {
    search_user_request: SearchUserRequest,
}

impl SearchUserRequestBuilder {
    /// 要执行搜索的字符串，一般为用户名。
    pub fn query(mut self, query: impl ToString) -> Self {
        self.search_user_request
            .api_request
            .query_params
            .insert("query", query.to_string());
        self
    }

    /// 分页大小，最小为 1，最大为 200，默认为 20。
    ///
    /// # 验证规则
    ///
    /// 分页大小必须在 1-200 之间（搜索服务限制），推荐值为 20
    pub fn page_size(mut self, page_size: i32) -> Self {
        // 搜索服务的分页大小限制更严格（1-200）
        if !(1..=200).contains(&page_size) {
            log::warn!(
                "Page size {} is out of valid range (1-200) for search service",
                page_size
            );
        }

        self.search_user_request
            .api_request
            .query_params
            .insert("page_size", page_size.to_string());
        self
    }

    /// 分页标识，获取首页不需要填写，获取下一页时传入上一页返回的分页标识值。
    /// 请注意此字段的值并没有特殊含义，请使用每次请求所返回的标识值。
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        let token = page_token.to_string();

        // 验证分页标记格式
        match validation::validate_page_token(&token, "page_token") {
            ValidationResult::Valid => {}
            ValidationResult::Warning(msg) => {
                log::warn!("Page token validation warning: {}", msg);
            }
            ValidationResult::Invalid(msg) => {
                log::error!("Invalid page token: {}", msg);
            }
        }

        self.search_user_request
            .api_request
            .query_params
            .insert("page_token", token);
        self
    }

    pub fn build(self) -> SearchUserRequest {
        self.search_user_request
    }

    /// 使用分页验证构建器设置分页参数
    ///
    /// 这个方法提供了一个更安全的分页参数设置方式，会自动验证参数的有效性
    /// 搜索服务的分页大小限制为 1-200
    pub fn with_pagination(
        mut self,
        page_size: Option<u32>,
        page_token: Option<String>,
    ) -> SDKResult<Self> {
        let mut pagination_builder =
            validation::pagination::PaginationRequestBuilder::<SearchUserResponse>::new();

        if let Some(size) = page_size {
            // 搜索服务有更严格的分页大小限制（1-200）
            if size > 200 {
                return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                    "Page size {} exceeds maximum limit of 200 for search service",
                    size
                )));
            }
            pagination_builder = pagination_builder.with_page_size(size);
        }

        if let Some(token) = page_token {
            pagination_builder = pagination_builder.with_page_token(token);
        }

        // 构建分页参数
        let params = pagination_builder.build()?;

        // 应用到请求中
        for (key, value) in params {
            self.search_user_request
                .api_request
                .query_params
                .insert(key, value);
        }

        Ok(self)
    }
}

crate::impl_executable_builder_owned!(
    SearchUserRequestBuilder,
    UserService,
    SearchUserRequest,
    SearchUserResponse,
    search_user
);

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUserResponse {
    /// 搜索到的用户列表。
    pub users: Vec<UserInSearchResponse>,
    /// 是否还有更多用户，值为 true 表示存在下一页。
    pub has_more: bool,
    /// 分页标识，存在下一页的时候返回。下次请求带上此标识可以获取下一页的用户。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 搜索到的用户信息。
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInSearchResponse {
    /// 用户的头像 URL。
    pub avatar: UserAvatar,
    /// 用户的部门信息。
    pub department_ids: Vec<String>,
    /// 用户的姓名。
    pub name: String,
    /// 用户的 open_id。
    pub open_id: String,
    /// 用户的 open_id。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// 用户的头像信息。
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAvatar {
    /// 用户的头像图片 URL，72×72px。
    pub avatar_72: String,
    /// 用户的头像图片 URL，240×240px。
    pub avatar_240: String,
    /// 用户的头像图片 URL，640×640px。
    pub avatar_640: String,
    /// 用户的头像图片 URL，原始大小。
    pub avatar_origin: String,
}

impl ApiResponseTrait for SearchUserResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

pub struct SearchUserIterator<'a> {
    user_service: &'a UserService,
    request: SearchUserRequest,
    option: Option<RequestOption>,
    has_more: bool,
}

impl SearchUserIterator<'_> {
    pub async fn next(&mut self) -> Option<Vec<UserInSearchResponse>> {
        if !self.has_more {
            return None;
        }

        match self
            .user_service
            .search_user_with_base_response(self.request.clone(), self.option.clone())
            .await
        {
            Ok(resp) => match resp.data {
                Some(data) => {
                    self.has_more = data.has_more;
                    if data.has_more {
                        if let Some(token) = data.page_token {
                            self.request
                                .api_request
                                .query_params
                                .insert("page_token", token);
                            Some(data.users)
                        } else {
                            // has_more is true but no page_token. Stop iterating to avoid panic.
                            self.has_more = false;
                            Some(data.users)
                        }
                    } else if data.users.is_empty() {
                        None
                    } else {
                        Some(data.users)
                    }
                }
                None => None,
            },
            Err(e) => {
                error!("Error: {e:?}");
                None
            }
        }
    }
}

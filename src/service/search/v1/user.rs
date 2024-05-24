use log::error;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use crate::core::api_resp::BaseResp;

pub struct UserService {
    config: Config,
}

impl UserService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 搜索用户。
    pub async fn search_user(
        &self,
        search_user_request: SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResp<SearchUserResponse>> {
        let mut api_req = search_user_request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/search/v1/user".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    pub fn search_user_iter(
        &self,
        search_user_request: SearchUserRequest,
        option: Option<RequestOption>,
    ) -> SearchUserIterator {
        SearchUserIterator {
            user_service: self,
            request: search_user_request,
            option,
            has_more: true,
        }
    }
}

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
            .insert("query".to_string(), query.to_string());
        self
    }

    /// 分页大小，最小为 1，最大为 200，默认为 20。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.search_user_request
            .api_request
            .query_params
            .insert("page_size".to_string(), page_size.to_string());
        self
    }

    /// 分页标识，获取首页不需要填写，获取下一页时传入上一页返回的分页标识值。
    /// 请注意此字段的值并没有特殊含义，请使用每次请求所返回的标识值。
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.search_user_request
            .api_request
            .query_params
            .insert("page_token".to_string(), page_token.to_string());
        self
    }

    pub fn build(self) -> SearchUserRequest {
        self.search_user_request
    }
}

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

impl<'a> SearchUserIterator<'a> {
    pub async fn next(&mut self) -> Option<Vec<UserInSearchResponse>> {
        if !self.has_more {
            return None;
        }

        match self
            .user_service
            .search_user(self.request.clone(), self.option.clone())
            .await
        {
            Ok(resp) => match resp.data {
                Some(data) => {
                    self.has_more = data.has_more;
                    if data.has_more {
                        self.request
                            .api_request
                            .query_params
                            .insert("page_token".to_string(), data.page_token.unwrap());
                        Some(data.users)
                    } else if data.users.is_empty() {
                        None
                    } else {
                        Some(data.users)
                    }
                }
                None => None,
            },
            Err(e) => {
                error!("Error: {:?}", e);
                None
            }
        }
    }
}

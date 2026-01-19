//! 搜索用户
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/search/users

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::UserIdType;
use crate::common::api_endpoints::BaikeApiV1;

/// 用户搜索结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResultUser {
    /// 用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 搜索用户响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchUsersResp {
    /// 搜索结果
    #[serde(default)]
    pub items: Vec<SearchResultUser>,
}

impl ApiResponseTrait for SearchUsersResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索用户请求
pub struct SearchUsersRequest {
    config: Config,
    query: String,
    user_id_type: Option<UserIdType>,
}

impl SearchUsersRequest {
    pub fn new(config: Config, query: impl Into<String>) -> Self {
        Self {
            config,
            query: query.into(),
            user_id_type: None,
        }
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchUsersResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<SearchUsersResp> {
        let query_len = self.query.chars().count();
        if !(1..=100).contains(&query_len) {
            return Err(openlark_core::error::validation_error(
                "query",
                "query 长度必须在 1~100 字符之间",
            ));
        }

        let mut api_request: ApiRequest<SearchUsersResp> =
            ApiRequest::get(&BaikeApiV1::SearchUser.to_url()).query("query", &self.query);

        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<SearchUsersResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

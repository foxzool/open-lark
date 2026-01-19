//! 搜索空间成员
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/search/space_members

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::UserIdType;
use crate::common::api_endpoints::BaikeApiV1;

/// 空间成员搜索结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResultSpaceMember {
    /// 用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 搜索空间成员响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchSpaceMembersResp {
    /// 搜索结果
    #[serde(default)]
    pub items: Vec<SearchResultSpaceMember>,
}

impl ApiResponseTrait for SearchSpaceMembersResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索空间成员请求
pub struct SearchSpaceMembersRequest {
    config: Config,
    space_id: String,
    query: String,
    user_id_type: Option<UserIdType>,
}

impl SearchSpaceMembersRequest {
    pub fn new(config: Config, space_id: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            config,
            space_id: space_id.into(),
            query: query.into(),
            user_id_type: None,
        }
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchSpaceMembersResp> {
        let query_len = self.query.chars().count();
        if !(1..=100).contains(&query_len) {
            return Err(openlark_core::error::validation_error(
                "query",
                "query 长度必须在 1~100 字符之间",
            ));
        }

        let mut api_request: ApiRequest<SearchSpaceMembersResp> =
            ApiRequest::get(&BaikeApiV1::SearchSpaceMember.to_url())
                .query("space_id", &self.space_id)
                .query("query", &self.query);

        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<SearchSpaceMembersResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

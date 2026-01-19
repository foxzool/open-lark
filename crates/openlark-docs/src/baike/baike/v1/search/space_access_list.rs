//! 搜索空间访问权限列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/search/space_access_list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::UserIdType;
use crate::common::api_endpoints::BaikeApiV1;

/// 空间访问权限列表条目
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpaceAccessListItem {
    /// 用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 权限级别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

/// 搜索空间访问权限列表响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchSpaceAccessListResp {
    /// 搜索结果
    #[serde(default)]
    pub items: Vec<SpaceAccessListItem>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchSpaceAccessListResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索空间访问权限列表请求
pub struct SearchSpaceAccessListRequest {
    config: Config,
    space_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl SearchSpaceAccessListRequest {
    pub fn new(config: Config, space_id: impl Into<String>) -> Self {
        Self {
            config,
            space_id: space_id.into(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchSpaceAccessListResp> {
        validate_required!(self.space_id, "space_id 不能为空");

        if let Some(page_size) = self.page_size {
            if !(1..=100).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 取值范围必须为 1~100",
                ));
            }
        }

        let mut api_request: ApiRequest<SearchSpaceAccessListResp> =
            ApiRequest::get(&BaikeApiV1::SearchSpaceAccessList.to_url())
                .query("space_id", &self.space_id);

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<SearchSpaceAccessListResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

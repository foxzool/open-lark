//! Bitable 列出协作者（自定义角色）
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role-member/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::RoleMemberInfo;

/// 列出协作者请求
#[derive(Debug, Clone)]
pub struct ListRoleMembersRequest {
    config: Config,
    app_token: String,
    role_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListRoleMembersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
            page_size: None,
            page_token: None,
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    pub async fn execute(self) -> SDKResult<ListRoleMembersResponse> {
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.role_id.trim(), "role_id");
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 必须在 1~100 之间",
                ));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RoleMemberList(self.app_token.clone(), self.role_id.clone());

        let mut api_request: ApiRequest<ListRoleMembersResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 列出协作者 Builder
pub struct ListRoleMembersRequestBuilder {
    request: ListRoleMembersRequest,
}

impl ListRoleMembersRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: ListRoleMembersRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn role_id(mut self, role_id: String) -> Self {
        self.request = self.request.role_id(role_id);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    pub fn build(self) -> ListRoleMembersRequest {
        self.request
    }
}

/// 列出协作者响应（data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRoleMembersResponse {
    /// 协作者列表
    pub items: Vec<RoleMemberInfo>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记（has_more=true 时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
}

impl ApiResponseTrait for ListRoleMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

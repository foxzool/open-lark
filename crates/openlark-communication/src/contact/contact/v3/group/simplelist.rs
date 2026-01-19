//! 查询用户组列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/group/simplelist

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::group::models::SimpleListGroupsResponse,
    endpoints::CONTACT_V3_GROUP_SIMPLELIST,
};

/// 查询用户组列表请求
pub struct SimpleListGroupsRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    r#type: Option<i32>,
}

impl SimpleListGroupsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            r#type: None,
        }
    }

    /// 分页大小（查询参数，可选，默认 50，最大 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 用户组类型（查询参数，可选，默认 1）
    pub fn group_type(mut self, group_type: i32) -> Self {
        self.r#type = Some(group_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/group/simplelist
    pub async fn execute(self) -> SDKResult<SimpleListGroupsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SimpleListGroupsResponse> {
        let mut req: ApiRequest<SimpleListGroupsResponse> =
            ApiRequest::get(CONTACT_V3_GROUP_SIMPLELIST);

        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(group_type) = self.r#type {
            req = req.query("type", group_type.to_string());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询用户组列表")
    }
}

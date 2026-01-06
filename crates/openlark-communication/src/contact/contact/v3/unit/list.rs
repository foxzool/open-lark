//! 获取单位列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/unit/list

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::unit::models::ListUnitsResponse, endpoints::CONTACT_V3_UNIT,
};

/// 获取单位列表请求
pub struct ListUnitsRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListUnitsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 分页大小（查询参数，可选，默认 50，范围 1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/unit/list
    pub async fn execute(self) -> SDKResult<ListUnitsResponse> {
        // url: GET:/open-apis/contact/v3/unit
        let mut req: ApiRequest<ListUnitsResponse> = ApiRequest::get(CONTACT_V3_UNIT);

        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取单位列表")
    }
}

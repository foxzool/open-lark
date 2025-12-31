//! 搜索对用户或机器人可见的群列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat/search

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    endpoints::IM_V1_CHATS,
    im::im::v1::message::models::UserIdType,
};

/// 搜索对用户或机器人可见的群列表请求
pub struct SearchChatsRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    query: Option<String>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl SearchChatsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            query: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 用户 ID 类型（查询参数，可选，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 关键词（查询参数，可选）
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（查询参数，可选，默认 20，最大 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat/search
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/im/v1/chats/search
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(format!("{}/search", IM_V1_CHATS));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(query) = self.query {
            req = req.query("query", query);
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "搜索对用户或机器人可见的群列表")
    }
}

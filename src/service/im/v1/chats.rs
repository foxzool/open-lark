use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::api_req::ApiReq;
use crate::core::api_resp::{ApiResp, CodeError};
use crate::core::config::Config;
use crate::core::constants::AccessTokenType;
use crate::core::http::Transport;
use crate::core::SDKResult;
use crate::service::im::share::ListChat;

pub struct Chats {
    pub config: Config,
}

impl Chats {
    /// 获取用户或机器人所在的群列表
    pub fn list(&self, req: ListChatReq) -> SDKResult<ListChatResp> {
        let mut api_req = req.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/im/v1/chats".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, vec![])?;
        let mut resp: ListChatResp = serde_json::from_slice(&api_resp.raw_body)?;
        resp.api_resp = api_resp;

        Ok(resp)
    }
}

pub struct ListChatReqBuilder {
    api_req: ApiReq,
    limit: Option<i32>,
}

impl ListChatReqBuilder {
    pub fn new() -> ListChatReqBuilder {
        let builder = ListChatReqBuilder {
            api_req: ApiReq::default(),
            limit: None,
        };
        builder
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.to_string());
        self
    }

    /// 群组排序方式
    pub fn sort_type(mut self, sort_type: impl ToString) -> Self {
        self.api_req
            .query_params
            .insert("sort_type".to_string(), sort_type.to_string());
        self
    }

    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该page_token 获取查询结果
    ///
    /// 示例值：dmJCRHhpd3JRbGV1VEVNRFFyTitRWDY5ZFkybmYrMEUwMUFYT0VMMWdENEtuYUhsNUxGMDIwemtvdE5ORjBNQQ==
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.api_req
            .query_params
            .insert("page_token".to_string(), page_token.to_string());
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.api_req
            .query_params
            .insert("page_size".to_string(), page_size.to_string());
        self
    }

    pub fn build(self) -> ListChatReq {
        ListChatReq {
            api_req: self.api_req,
            limit: self.limit,
        }
    }
}

pub struct ListChatReq {
    pub api_req: ApiReq,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListChatResp {
    #[serde(skip)]
    pub api_resp: ApiResp,
    #[serde(flatten)]
    pub code_error: CodeError,
    pub data: ListChatRespData,
}

impl ListChatResp {
    pub fn success(&self) -> bool {
        self.code_error.code == 0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListChatRespData {
    /// chat 列表
    pub items: Vec<ListChat>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: String,
    /// 是否还有更多项
    pub has_more: bool,
}

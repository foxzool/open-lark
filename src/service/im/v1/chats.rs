use log::error;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::api_req::ApiReq;
use crate::core::api_resp::BaseResp;
use crate::core::config::Config;
use crate::core::constants::AccessTokenType;
use crate::core::http::Transport;
use crate::core::req_option::RequestOptionFunc;
use crate::core::SDKResult;

pub struct ChatsService {
    pub config: Config,
}

impl ChatsService {
    /// 获取用户或机器人所在的群列表
    pub fn list(
        &self,
        req: &mut ListChatReq,
        options: &[RequestOptionFunc],
    ) -> SDKResult<BaseResp<ListChatRespData>> {
        let mut api_req = &mut req.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/im/v1/chats".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(&mut api_req, &self.config, options)?;

        Ok(api_resp.try_into()?)
    }

    pub fn list_iter(&self, req: ListChatReq, options: Vec<RequestOptionFunc>) -> ListChatIterator {
        ListChatIterator {
            service: self,
            req,
            options,
            has_more: true,
        }
    }
}

pub struct ListChatIterator<'a> {
    service: &'a ChatsService,
    req: ListChatReq,
    options: Vec<RequestOptionFunc>,
    has_more: bool,
}

impl<'a> Iterator for ListChatIterator<'a> {
    type Item = Vec<ListChat>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_more {
            return None;
        }
        match self.service.list(&mut self.req, &self.options) {
            Ok(resp) => {
                if resp.success() {
                    self.has_more = resp.data.has_more;
                    if resp.data.has_more {
                        self.req
                            .api_req
                            .query_params
                            .insert("page_token".to_string(), resp.data.page_token.to_string());
                        Some(resp.data.items)
                    } else if resp.data.items.is_empty() {
                        None
                    } else {
                        Some(resp.data.items)
                    }
                } else {
                    error!("Error: {}", resp.error_msg());
                    None
                }
            }
            Err(e) => {
                error!("Error: {:?}", e);
                None
            }
        }
    }
}

pub struct ListChatReqBuilder {
    api_req: ApiReq,
}

impl ListChatReqBuilder {
    pub fn new() -> ListChatReqBuilder {
        let builder = ListChatReqBuilder {
            api_req: ApiReq::default(),
        };
        builder
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
        }
    }
}

pub struct ListChatReq {
    pub api_req: ApiReq,
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

/// chat 列表
#[derive(Debug, Serialize, Deserialize)]
pub struct ListChat {
    /// 群组 ID
    pub chat_id: String,
    /// 群头像 URL
    pub avatar: String,
    /// 群名称
    pub name: String,
    /// 群描述
    pub description: String,
    /// 群主 ID
    pub owner_id: String,
    /// 群主 ID 类型
    pub owner_id_type: String,
    /// 是否是外部群
    pub external: bool,
    /// 租户Key，为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用中的唯一标识
    pub tenant_key: String,
    /// 群状态
    pub chat_status: String,
}

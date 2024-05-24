use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{api_req::ApiRequest, api_resp::{ApiResponse, ApiResponseTrait, ResponseFormat}, AfitAsyncIter, config::Config, constants::AccessTokenType, http::Transport, req_option::RequestOption, SDKResult};

pub struct ChatsService {
    pub config: Config,
}

impl ChatsService {
    /// 获取用户或机器人所在的群列表
    pub async fn list(
        &self,
        list_chat_request: ListChatRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ApiResponse<ListChatRespData>> {
        let mut api_req = list_chat_request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/im/v1/chats".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    pub fn list_iter(&self, list_chat_request: ListChatRequest, option: Option<RequestOption>) -> ListChatIterator {
        ListChatIterator {
            service: self,
            request: list_chat_request,
            option,
            has_more: true,
        }
    }
}

pub struct ListChatIterator<'a> {
    service: &'a ChatsService,
    request: ListChatRequest,
    option: Option<RequestOption>,
    has_more: bool,
}

impl<'a> AfitAsyncIter for ListChatIterator<'a> {
    type Item = Vec<ListChat>;

    async fn next(&mut self) -> Option<Self::Item> {
        if !self.has_more {
            return None;
        }
        match self.service.list(self.request.clone(), self.option.clone()).await {
            Ok(resp) => match resp {
                ApiResponse::Success { data, .. } => {
                    self.has_more = data.has_more;
                    if data.has_more {
                        self.request
                            .api_req
                            .query_params
                            .insert("page_token".to_string(), data.page_token.to_string());
                        Some(data.items)
                    } else if data.items.is_empty() {
                        None
                    } else {
                        Some(data.items)
                    }
                }
                ApiResponse::Error(_) => None,
            },
            Err(_) => None,
        }
    }
}

#[derive(Default, Clone)]
pub struct ListChatRequest {
    api_req: ApiRequest,
}

impl ListChatRequest {
    pub fn builder() -> ListChatRequestBuilder {
        ListChatRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListChatRequestBuilder {
    request: ListChatRequest,
}

impl ListChatRequestBuilder {
    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.to_string());
        self
    }

    /// 群组排序方式
    pub fn sort_type(mut self, sort_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("sort_type".to_string(), sort_type.to_string());
        self
    }

    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的
    /// page_token，下次遍历可采用该page_token 获取查询结果
    ///
    /// 示例值：dmJCRHhpd3JRbGV1VEVNRFFyTitRWDY5ZFkybmYrMEUwMUFYT0VMMWdENEtuYUhsNUxGMDIwemtvdE5ORjBNQQ==
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("page_token".to_string(), page_token.to_string());
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request
            .api_req
            .query_params
            .insert("page_size".to_string(), page_size.to_string());
        self
    }

    pub fn build(self) -> ListChatRequest {
        self.request
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

impl ApiResponseTrait for ListChatRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
    /// 租户Key，为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，
    /// 也可以用作租户在应用中的唯一标识
    pub tenant_key: String,
    /// 群状态
    pub chat_status: String,
}

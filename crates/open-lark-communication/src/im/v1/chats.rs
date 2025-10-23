use reqwest::Method;
use serde::{Deserialize, Serialize};

use open_lark_core::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::im,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use crate::impl_full_service;

/// 群聊服务
pub struct ChatsService {
    pub config: Config,
}

impl ChatsService {
    /// 获取用户或机器人所在的群列表
    ///
    /// 获取当前用户或机器人参与的所有群聊列表，包括群的基本信息、群主、
    /// 群状态等。支持分页查询和多种排序方式。
    ///
    /// # API文档
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/list
    pub async fn list(
        &self,
        list_chat_request: ListChatRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ListChatRespData> {
        let mut api_req = list_chat_request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(im::IM_CHAT_LIST.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        let api_resp: BaseResponse<ListChatRespData> = Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 创建分页迭代器
    pub fn list_iter(&self, list_chat_request: ListChatRequest) -> ListChatIterator<'_> {
        ListChatIterator {
            service: self,
            request: list_chat_request,
            has_more: true,
        }
    }
}

// 接入统一 Service 抽象（IM v1 - ChatsService）
impl_full_service!(ChatsService, "im.chats", "v1");

/// 分页迭代器
pub struct ListChatIterator<'a> {
    service: &'a ChatsService,
    request: ListChatRequest,
    has_more: bool,
}

impl ListChatIterator<'_> {
    /// 获取下一页数据
    pub async fn next(&mut self) -> Option<Vec<ListChat>> {
        if !self.has_more {
            return None;
        }

        match self.service.list(self.request.clone(), None).await {
            Ok(data) => {
                self.has_more = data.has_more;
                if data.has_more {
                    self.request
                        .api_req
                        .query_params
                        .insert("page_token", data.page_token.to_string());
                    Some(data.items)
                } else if data.items.is_empty() {
                    None
                } else {
                    Some(data.items)
                }
            }
            Err(_) => None,
        }
    }
}

/// 获取群列表请求
#[derive(Debug, Clone, Default)]
pub struct ListChatRequest {
    #[serde(skip)]
    pub api_req: ApiRequest,
}

impl ListChatRequest {
    pub fn builder() -> ListChatRequestBuilder {
        ListChatRequestBuilder::default()
    }
}

/// 群列表请求构建器
#[derive(Default)]
pub struct ListChatRequestBuilder {
    request: ListChatRequest,
}

impl ListChatRequestBuilder {
    /// 群大小过滤
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.api_req.query_params.insert("page_size", page_size.to_string());
        self
    }

    /// 分页token
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.api_req.query_params.insert("page_token", page_token.to_string());
        self
    }

    /// 排序字段
    pub fn sort_field(mut self, sort_field: impl ToString) -> Self {
        self.request.api_req.query_params.insert("sort_field", sort_field.to_string());
        self
    }

    /// 排序方向
    pub fn sort_direction(mut self, sort_direction: impl ToString) -> Self {
        self.request.api_req.query_params.insert("sort_direction", sort_direction.to_string());
        self
    }

    pub fn build(self) -> ListChatRequest {
        self.request
    }
}

/// 群信息
#[derive(Debug, Clone, Deserialize)]
pub struct ListChat {
    /// 群名称
    pub name: String,
    /// 群描述
    pub description: Option<String>,
    /// 群头像
    pub avatar: Option<String>,
    /// 群主
    pub owner: Option<String>,
    /// 群状态
    pub status: Option<String>,
    /// 群类型
    pub chat_type: Option<String>,
    /// 外部群标志
    pub external: Option<bool>,
    /// 群创建时间
    pub create_time: Option<String>,
    /// 群更新时间
    pub update_time: Option<String>,
    /// 群成员数量
    pub member_count: Option<i32>,
}

/// 群列表响应数据
#[derive(Debug, Clone, Deserialize)]
pub struct ListChatRespData {
    /// 群列表
    pub items: Vec<ListChat>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页token
    pub page_token: String,
}

impl ApiResponseTrait for ListChatRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_chat_request_builder() {
        let request = ListChatRequest::builder()
            .page_size(20)
            .page_token("token123")
            .sort_field("name")
            .sort_direction("asc")
            .build();

        assert_eq!(request.api_req.query_params.get("page_size"), Some(&"20".to_string()));
        assert_eq!(request.api_req.query_params.get("page_token"), Some(&"token123".to_string()));
        assert_eq!(request.api_req.query_params.get("sort_field"), Some(&"name".to_string()));
        assert_eq!(request.api_req.query_params.get("sort_direction"), Some(&"asc".to_string()));
    }

    #[test]
    fn test_list_chat_resp_data_format() {
        assert_eq!(ListChatRespData::data_format(), ResponseFormat::Data);
    }
}
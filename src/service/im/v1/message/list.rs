use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse},
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
        validation::{self, ValidationResult},
        SDKResult,
    },
    service::im::v1::message::{ListMessageIterator, Message},
};

use super::MessageService;

/// 列表消息请求
#[derive(Default, Clone)]
pub struct ListMessageRequest {
    pub api_req: ApiRequest,
}

impl ListMessageRequest {
    pub fn builder() -> ListMessageRequestBuilder {
        ListMessageRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListMessageRequestBuilder {
    request: ListMessageRequest,
}

impl ListMessageRequestBuilder {
    /// 容器类型 ，目前可选值仅有"chat"，包含单聊（p2p）和群聊（group）
    ///
    /// 示例值：chat
    pub fn container_id_type(mut self, container_id_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("container_id_type", container_id_type.to_string());
        self
    }

    /// 容器的id，即chat的id，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// 示例值：oc_234jsi43d3ssi993d43545f
    pub fn container_id(mut self, container_id: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("container_id", container_id.to_string());
        self
    }

    /// 历史信息的起始时间（秒级时间戳）
    ///
    /// 示例值：1609296809
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.request
            .api_req
            .query_params
            .insert("start_time", start_time.to_string());
        self
    }

    /// 历史信息的结束时间（秒级时间戳）
    ///
    /// 示例值：1608594809
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.request
            .api_req
            .query_params
            .insert("end_time", end_time.to_string());
        self
    }

    /// 消息排序方式
    ///
    /// 示例值：ByCreateTimeAsc
    pub fn sort_type(mut self, sort_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("sort_type", sort_type.to_string());
        self
    }

    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的
    /// page_token，下次遍历可采用该 page_token 获取查询结果
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("page_token", page_token.to_string());
        self
    }

    /// 分页大小
    ///
    /// 示例值：20
    ///
    /// # 验证规则
    ///
    /// 分页大小必须在 1-500 之间，推荐值为 50
    pub fn page_size(mut self, page_size: i32) -> Self {
        // 验证分页大小
        match validation::validate_page_size(page_size as u32, "page_size") {
            ValidationResult::Valid => {}
            ValidationResult::Warning(msg) => {
                log::warn!("Page size validation warning: {}", msg);
            }
            ValidationResult::Invalid(msg) => {
                log::error!("Invalid page size: {}", msg);
                // 仍然设置值，但记录错误，让用户决定是否继续
            }
        }
        self.request
            .api_req
            .query_params
            .insert("page_size", page_size.to_string());
        self
    }

    pub fn build(self) -> ListMessageRequest {
        // 验证分页标记（如果存在）
        if let Some(page_token) = self.request.api_req.query_params.get("page_token") {
            match validation::validate_page_token(page_token, "page_token") {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Page token validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    log::error!("Invalid page token: {}", msg);
                    // 仍然设置值，但记录错误
                }
            }
        }

        self.request
    }

    /// 使用分页验证构建器设置分页参数
    ///
    /// 这个方法提供了一个更安全的分页参数设置方式，会自动验证参数的有效性
    pub fn with_pagination(
        mut self,
        page_size: Option<u32>,
        page_token: Option<String>,
    ) -> SDKResult<Self> {
        let mut pagination_builder =
            validation::pagination::PaginationRequestBuilder::<ListMessageRespData>::new();

        if let Some(size) = page_size {
            pagination_builder = pagination_builder.with_page_size(size);
        }

        if let Some(token) = page_token {
            pagination_builder = pagination_builder.with_page_token(token);
        }

        // 构建分页参数
        let params = pagination_builder.build()?;

        // 应用到请求中
        for (key, value) in params {
            self.request.api_req.query_params.insert(key, value);
        }

        Ok(self)
    }
}

crate::impl_executable_builder_owned!(
    ListMessageRequestBuilder,
    MessageService,
    ListMessageRequest,
    ListMessageRespData,
    list
);

/// Response data for message listing
#[derive(Debug, Serialize, Deserialize)]
pub struct ListMessageRespData {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: Option<String>,
    pub items: Vec<Message>,
}

impl ApiResponseTrait for ListMessageRespData {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl MessageService {
    /// 获取会话历史消息
    ///
    /// 获取会话（包括单聊、群组）的历史消息（聊天记录）
    ///
    /// <https://open.feishu.cn/document/server-docs/im-v1/message/list>
    pub async fn list(
        &self,
        list_message_request: ListMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ListMessageRespData> {
        let mut api_req = list_message_request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = crate::core::endpoints::im::IM_V1_LIST_MESSAGE.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<ListMessageRespData> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result()
    }

    /// 创建消息列表迭代器
    ///
    /// 提供便捷的方式遍历所有消息，自动处理分页
    pub fn list_iter(
        &self,
        list_message_request: ListMessageRequest,
        _option: Option<RequestOption>,
    ) -> ListMessageIterator<'_> {
        ListMessageIterator::new(self, list_message_request)
    }

    /// 使用分页验证创建消息列表请求
    ///
    /// 提供一个更安全的方式来创建消息列表请求，自动验证分页参数
    pub async fn list_with_validated_pagination(
        &self,
        container_id: impl ToString,
        container_id_type: impl ToString,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<ListMessageRespData> {
        // 创建请求构建器
        let builder = ListMessageRequest::builder()
            .container_id(container_id)
            .container_id_type(container_id_type)
            .with_pagination(page_size, page_token)?;

        self.list(builder.build(), option).await
    }
}

use log::error;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        validation::{
            message_limits, uuid_limits, validate_content_size, validate_required,
            validate_string_length,
        },
    },
    service::im::v1::message::{Message, SendMessageTrait},
};

use super::MessageService;

/// 请求体
#[derive(Debug, Clone, Default)]
pub struct CreateMessageRequest {
    pub api_req: ApiRequest,
}

impl CreateMessageRequest {
    pub fn builder() -> CreateMessageRequestBuilder {
        CreateMessageRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateMessageRequestBuilder {
    request: CreateMessageRequest,
}

impl CreateMessageRequestBuilder {
    /// 设置接收者ID
    ///
    /// # 参数
    /// - `receive_id`: 消息接收者的ID
    pub fn receive_id(mut self, receive_id: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("receive_id", receive_id.to_string());
        self
    }

    /// 设置消息类型
    ///
    /// # 参数
    /// - `msg_type`: 消息类型
    pub fn msg_type(mut self, msg_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("msg_type", msg_type.to_string());
        self
    }

    /// 设置消息内容
    ///
    /// # 参数
    /// - `content`: 消息内容
    pub fn content(mut self, content: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("content", content.to_string());
        self
    }

    /// 设置消息接收者ID类型
    ///
    /// # 参数
    /// - `receive_id_type`: 接收者ID类型，可选值：
    ///   - `"open_id"`: Open ID（推荐）
    ///   - `"user_id"`: User ID
    ///   - `"union_id"`: Union ID
    ///   - `"email"`: 邮箱地址
    ///   - `"chat_id"`: 群聊ID
    pub fn receive_id_type(mut self, receive_id_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("receive_id_type", receive_id_type.to_string());
        self
    }

    /// 设置消息请求体
    ///
    /// # 参数
    /// - `body`: 包含接收者ID、消息类型和内容的请求体
    pub fn request_body(mut self, body: CreateMessageRequestBody) -> Self {
        match serde_json::to_vec(&body) {
            Ok(bytes) => {
                self.request.api_req.body = bytes;
            }
            Err(e) => {
                error!("Failed to serialize request body: {}", e);
                // 在序列化失败时使用空 body，避免 panic
                // 这允许请求继续，但可能会被 API 拒绝
                self.request.api_req.body = Vec::new();
            }
        }
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> CreateMessageRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait到CreateMessageRequestBuilder
crate::impl_executable_builder_owned!(
    CreateMessageRequestBuilder,
    MessageService,
    CreateMessageRequest,
    Message,
    create
);

/// 发送消息 请求体
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateMessageRequestBody {
    /// 消息接收者的ID，ID类型应与查询参数receive_id_type 对应；
    /// 推荐使用 OpenID，获取方式可参考文档如何获取 Open ID？
    ///
    /// 示例值："ou_7d8a6e6df7621556ce0d21922b676706ccs"
    pub receive_id: String,
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、
    /// share_user等，类型定义请参考发送消息内容
    ///
    /// 示例值："text"
    pub msg_type: String,
    /// 消息内容，JSON结构序列化后的字符串。不同msg_type对应不同内容，具体格式说明参考：
    /// 发送消息内容
    ///
    /// 注意：
    /// JSON字符串需进行转义，如换行符转义后为\\n
    /// 文本消息请求体最大不能超过150KB
    /// 卡片及富文本消息请求体最大不能超过30KB
    /// 示例值："{\"text\":\"test content\"}"
    pub content: String,
    /// 由开发者生成的唯一字符串序列，用于发送消息请求去重；
    /// 持有相同uuid的请求1小时内至多成功发送一条消息
    ///
    /// 示例值："选填，每次调用前请更换，如a0d69e20-1dd1-458b-k525-dfeca4015204"
    ///
    /// 数据校验规则：
    ///
    /// 最大长度：50 字符
    pub uuid: Option<String>,
}

impl CreateMessageRequestBody {
    pub fn builder() -> CreateMessageRequestBodyBuilder {
        CreateMessageRequestBodyBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateMessageRequestBodyBuilder {
    request: CreateMessageRequestBody,
}

impl CreateMessageRequestBodyBuilder {
    /// 消息接收者的ID，ID类型应与查询参数receive_id_type 对应；
    /// 推荐使用 OpenID，获取方式可参考文档如何获取 Open ID？
    ///
    /// 示例值："ou_7d8a6e6df7621556ce0d21922b676706ccs"
    pub fn receive_id(mut self, receive_id: impl ToString) -> Self {
        self.request.receive_id = receive_id.to_string();
        self
    }

    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、
    /// share_user等，类型定义请参考发送消息内容
    ///
    /// 示例值："text"
    pub fn msg_type(mut self, msg_type: impl ToString) -> Self {
        self.request.msg_type = msg_type.to_string();
        self
    }

    /// 消息内容，JSON结构序列化后的字符串。不同msg_type对应不同内容，具体格式说明参考：
    /// 发送消息内容
    ///
    /// 注意：
    /// JSON字符串需进行转义，如换行符转义后为\\n
    /// 文本消息请求体最大不能超过150KB
    /// 卡片及富文本消息请求体最大不能超过30KB
    /// 示例值："{\"text\":\"test content\"}"
    pub fn content(mut self, content: impl ToString) -> Self {
        self.request.content = content.to_string();
        self
    }

    /// 由开发者生成的唯一字符串序列，用于发送消息请求去重；
    /// 持有相同uuid的请求1小时内至多成功发送一条消息
    ///
    /// 示例值："选填，每次调用前请更换，如a0d69e20-1dd1-458b-k525-dfeca4015204"
    ///
    /// 数据校验规则：
    ///
    /// 最大长度：50 字符
    pub fn uuid(mut self, uuid: impl ToString) -> Self {
        let uuid_str = uuid.to_string();
        // 使用验证工具函数
        let validated_uuid = validate_string_length(uuid_str, uuid_limits::MAX_LENGTH, "UUID");
        self.request.uuid = Some(validated_uuid);
        self
    }

    pub fn build(self) -> CreateMessageRequestBody {
        // 验证必填字段
        validate_required(&self.request.receive_id, "receive_id");
        validate_required(&self.request.msg_type, "msg_type");
        validate_required(&self.request.content, "content");

        // 验证内容长度（根据消息类型）
        match self.request.msg_type.as_str() {
            "text" => {
                validate_content_size(
                    &self.request.content,
                    message_limits::TEXT_MESSAGE_MAX_SIZE,
                    "Text message",
                );
            }
            "post" | "interactive" => {
                validate_content_size(
                    &self.request.content,
                    message_limits::RICH_MESSAGE_MAX_SIZE,
                    "Post/interactive message",
                );
            }
            _ => {
                // 其他消息类型不验证内容大小
            }
        }

        self.request
    }
}

/// 更新消息请求
#[derive(Debug, Clone, Default)]
pub struct UpdateMessageRequest {
    pub api_req: ApiRequest,
}

impl UpdateMessageRequest {
    pub fn builder() -> UpdateMessageRequestBuilder {
        UpdateMessageRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateMessageRequestBuilder {
    request: UpdateMessageRequest,
}

impl UpdateMessageRequestBuilder {
    /// 设置消息内容
    ///
    /// # 参数
    /// - `content`: 消息内容
    pub fn content(mut self, content: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("content", content.to_string());
        self
    }

    pub fn build(self) -> UpdateMessageRequest {
        self.request
    }
}

/// 便捷方法：使用消息内容类型构建发送消息请求
impl CreateMessageRequest {
    /// 使用SendMessageTrait类型创建消息请求
    pub fn with_msg<T: SendMessageTrait>(receive_id: &str, msg: T, receive_id_type: &str) -> Self {
        let mut api_req = ApiRequest::default();
        api_req
            .query_params
            .insert("receive_id", receive_id.to_string());
        api_req.query_params.insert("msg_type", msg.msg_type());
        api_req.query_params.insert("content", msg.content());
        api_req
            .query_params
            .insert("receive_id_type", receive_id_type.to_string());

        Self { api_req }
    }
}

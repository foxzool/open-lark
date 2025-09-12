use super::MessageService;
use crate::core::api_resp::ApiResponseTrait;
use serde::{Deserialize, Serialize};

/// 发送消息的通用trait
///
/// 所有可以作为消息内容发送的类型都应该实现此trait。
/// 它定义了获取消息类型和内容的标准接口。
///
/// # 实现
///
/// - `MessageText`: 文本消息
/// - `MessagePost`: 富文本消息  
/// - `MessageImage`: 图片消息
/// - `MessageCardTemplate`: 卡片模板消息
///
/// # 示例
///
/// ```rust
/// use open_lark::service::im::v1::message::{MessageText, SendMessageTrait};
///
/// let text_msg = MessageText::new("Hello, World!");
/// assert_eq!(text_msg.msg_type(), "text");
/// assert_eq!(text_msg.content(), "{\"text\":\"Hello, World!\"}");
/// ```
pub trait SendMessageTrait {
    /// 获取消息类型
    ///
    /// 返回消息的类型标识，如 "text"、"post"、"image" 等
    fn msg_type(&self) -> String;

    /// 获取消息内容
    ///
    /// 返回序列化后的消息内容JSON字符串
    fn content(&self) -> String;
}

/// Response data for message creation
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageResp {
    pub data: Message,
}

impl crate::core::api_resp::ApiResponseTrait for CreateMessageResp {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 飞书消息
///
/// 表示一条完整的飞书消息，包含消息ID、类型、内容、发送者等所有信息。
///
/// # 字段说明
///
/// - `message_id`: 消息的唯一标识符
/// - `msg_type`: 消息类型（text、post、image等）
/// - `sender`: 消息发送者信息
/// - `body`: 消息具体内容
/// - `chat_id`: 所属会话ID
/// - `create_time`/`update_time`: 创建和更新时间戳
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// 消息id
    pub message_id: String,
    /// 根消息id，用于回复消息场景
    pub root_id: Option<String>,
    /// 父消息的id，用于回复消息场景
    pub parent_id: Option<String>,
    /// 消息所属的话题 ID（不返回说明该消息非话题消息）
    pub thread_id: Option<String>,
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、
    /// share_user等
    pub msg_type: String,
    /// 消息生成的时间戳（毫秒）
    pub create_time: String,
    /// 消息更新的时间戳（毫秒）
    pub update_time: String,
    /// 消息是否被撤回
    pub deleted: bool,
    /// 消息是否被更新
    pub updated: bool,
    /// 所属的群
    pub chat_id: String,
    /// 发送者，可以是用户或应用
    pub sender: Sender,
    /// 消息内容
    pub body: MessageBody,
    /// 被@的用户或机器人的id列表
    pub mentions: Option<Vec<Mention>>,
}

impl ApiResponseTrait for Message {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 发送者，可以是用户或应用
#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    /// 该字段标识发送者的id
    id: String,
    /// 该字段标识发送者的id类型
    ///
    /// 可选值有：
    /// - open_id
    /// - app_id
    id_type: String,
    /// 该字段标识发送者的类型
    ///
    /// 可选值有：
    /// - user: 用户
    /// - app: 应用
    /// - anonymous: 匿名
    /// - unknown: 未知
    sender_type: String,
    /// 为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，
    /// 也可以用作租户在应用里面的唯一标识
    tenant_key: String,
}

/// 消息内容
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageBody {
    /// 消息内容，json结构序列化后的字符串。不同msg_type对应不同内容。
    ///
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、
    /// share_user等，
    pub content: String,
}

/// 被@的用户或机器人的id列表
#[derive(Debug, Serialize, Deserialize)]
pub struct Mention {
    /// 被@的用户或机器人的序号。例如，第3个被@到的成员，值为"@_user_3"
    pub key: String,
    /// 被@的用户或者机器人的open_id
    pub id: String,
    /// 被@的用户或机器人 id 类型，目前仅支持 open_id
    pub id_type: String,
    /// 被@的用户或机器人的姓名
    pub name: String,
    /// 为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，
    /// 也可以用作租户在应用里面的唯一标识
    pub tenant_key: String,
    /// 合并转发消息中，上一层级的消息id message_id
    pub upper_message_id: String,
}

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

/// Message iterator for listing messages with pagination
#[allow(dead_code)]
pub struct ListMessageIterator<'a> {
    service: &'a super::MessageService,
    request: crate::service::im::v1::message::list::ListMessageRequest,
    page_token: Option<String>,
    has_more: bool,
}

impl<'a> ListMessageIterator<'a> {
    pub fn new(
        service: &'a MessageService,
        request: crate::service::im::v1::message::list::ListMessageRequest,
    ) -> Self {
        Self {
            service,
            request,
            page_token: None,
            has_more: true,
        }
    }

    // FUTURE: 实现异步迭代器或流式处理分页结果
    // 标准 Iterator trait 不支持异步，可考虑：
    // 1. 使用 futures::Stream trait
    // 2. 使用 async-stream crate
    // 3. 实现自定义的 AsyncIterator trait
}

impl<'a> std::fmt::Debug for ListMessageIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListMessageIterator")
            .field("has_more", &self.has_more)
            .field("page_token", &self.page_token)
            .finish()
    }
}

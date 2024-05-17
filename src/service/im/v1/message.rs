use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::core::api_req::ApiReq;
use crate::core::api_resp::{ApiResp, BaseResp, CodeMsg};
use crate::core::config::Config;
use crate::core::constants::AccessTokenType;
use crate::core::error::LarkAPIError;
use crate::core::http::Transport;
use crate::core::req_option::RequestOption;

pub struct MessageService {
    pub config: Config,
}

impl MessageService {
    /// 发送消息
    ///
    /// 给指定用户或者会话发送消息，支持文本、富文本、可交互的消息卡片、群名片、个人名片、图片、视频、音频、文件、表情包。
    pub fn create(
        &self,
        req: CreateMessageReq,
        option: Option<RequestOption>,
    ) -> Result<BaseResp<Message>, LarkAPIError> {
        let mut api_req = req.api_req;
        api_req.http_method = "POST".to_string();
        api_req.api_path = "/open-apis/im/v1/messages".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option)?;

        Ok(api_resp.try_into()?)
    }

    /// 获取会话历史消息
    ///
    /// 获取会话（包括单聊、群组）的历史消息（聊天记录）
    /// https://open.feishu.cn/document/server-docs/im-v1/message/list
    pub fn list(
        &self,
        req: &ListMessageReq,
        option: Option<RequestOption>,
    ) -> Result<BaseResp<ListMessageRespData>, LarkAPIError> {
        let mut api_req = req.api_req.clone();
        api_req.http_method = "GET".to_string();
        api_req.api_path = "/open-apis/im/v1/messages".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option)?;

        Ok(api_resp.try_into()?)
    }

    pub fn list_iter(
        &self,
        req: ListMessageReq,
        option: Option<RequestOption>,
    ) -> ListMessageIterator {
        ListMessageIterator {
            service: self,
            req,
            option,
            has_more: true,
        }
    }
}

pub struct ListMessageIterator<'a> {
    service: &'a MessageService,
    req: ListMessageReq,
    option: Option<RequestOption>,
    has_more: bool,
}

impl<'a> Iterator for ListMessageIterator<'a> {
    type Item = Vec<Message>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_more {
            return None;
        }
        match self.service.list(&self.req, self.option.clone()) {
            Ok(resp) => {
                if let Some(data) = resp.data {
                    self.has_more = data.has_more;
                    if data.has_more {
                        self.req
                            .api_req
                            .query_params
                            .insert("page_token".to_string(), data.page_token.unwrap());
                        Some(data.items)
                    } else if data.items.is_empty() {
                        return None;
                    } else {
                        Some(data.items)
                    }
                } else {
                    error!("Error: {:?}", resp.error_msg());
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

pub struct CreateMessageReqBuilder {
    api_req: ApiReq,
    body: Option<CreateMessageReqBody>,
}

impl Default for CreateMessageReqBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateMessageReqBuilder {
    pub fn new() -> CreateMessageReqBuilder {
        CreateMessageReqBuilder {
            api_req: ApiReq::default(),
            body: None,
        }
    }

    pub fn receive_id_type(mut self, receive_id_type: impl ToString) -> Self {
        self.api_req
            .query_params
            .insert("receive_id_type".to_string(), receive_id_type.to_string());
        self
    }

    pub fn body(mut self, body: CreateMessageReqBody) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(mut self) -> CreateMessageReq {
        self.api_req.body = serde_json::to_vec(&self.body.clone().unwrap())
            .unwrap()
            .into();
        CreateMessageReq {
            api_req: self.api_req,
            body: self.body.unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct CreateMessageReq {
    pub api_req: ApiReq,
    pub body: CreateMessageReqBody,
}

/// 发送消息 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageReqBody {
    /// 消息接收者的ID，ID类型应与查询参数receive_id_type 对应；推荐使用 OpenID，获取方式可参考文档如何获取 Open ID？
    ///
    /// 示例值："ou_7d8a6e6df7621556ce0d21922b676706ccs"
    pub receive_id: String,
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等，类型定义请参考发送消息内容
    ///
    /// 示例值："text"
    pub msg_type: String,
    /// 消息内容，JSON结构序列化后的字符串。不同msg_type对应不同内容，具体格式说明参考：发送消息内容
    ///
    /// 注意：
    /// JSON字符串需进行转义，如换行符转义后为\\n
    /// 文本消息请求体最大不能超过150KB
    /// 卡片及富文本消息请求体最大不能超过30KB
    /// 示例值："{\"text\":\"test content\"}"
    pub content: String,
    /// 由开发者生成的唯一字符串序列，用于发送消息请求去重；持有相同uuid的请求1小时内至多成功发送一条消息
    ///
    /// 示例值："选填，每次调用前请更换，如a0d69e20-1dd1-458b-k525-dfeca4015204"
    ///
    /// 数据校验规则：
    ///
    /// 最大长度：50 字符
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageResp {
    #[serde(skip)]
    pub api_resp: ApiResp,
    #[serde(flatten)]
    pub code_error: CodeMsg,
    pub data: Message,
}

impl CreateMessageResp {
    pub fn success(&self) -> bool {
        self.code_error.code == 0
    }
}

/// 消息类型
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
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等
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
    /// 为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识
    tenant_key: String,
}

/// 消息内容
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageBody {
    /// 消息内容，json结构序列化后的字符串。不同msg_type对应不同内容。
    ///
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、share_user等，
    pub content: String,
}

/// 被@的用户或机器人的id列表
#[derive(Debug, Serialize, Deserialize)]
pub struct Mention {
    /// 被@的用户或机器人的序号。例如，第3个被@到的成员，值为“@_user_3”
    pub key: String,
    /// 被@的用户或者机器人的open_id
    pub id: String,
    /// 被@的用户或机器人 id 类型，目前仅支持 open_id
    pub id_type: String,
    /// 被@的用户或机器人的姓名
    pub name: String,
    /// 为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，也可以用作租户在应用里面的唯一标识
    pub tenant_key: String,
    /// 合并转发消息中，上一层级的消息id message_id
    pub upper_message_id: String,
}

pub struct ListMessageReqBuilder {
    api_req: ApiReq,
    limit: Option<i32>,
}

impl Default for ListMessageReqBuilder {
    fn default() -> Self {
        ListMessageReqBuilder::new()
    }
}

impl ListMessageReqBuilder {
    pub fn new() -> ListMessageReqBuilder {
        ListMessageReqBuilder {
            api_req: ApiReq::default(),
            limit: None,
        }
    }

    /// 最大返回多少记录
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// 容器类型 ，目前可选值仅有"chat"，包含单聊（p2p）和群聊（group）
    ///
    /// 示例值：chat
    pub fn container_id_type(mut self, container_id_type: impl ToString) -> Self {
        self.api_req.query_params.insert(
            "container_id_type".to_string(),
            container_id_type.to_string(),
        );
        self
    }

    /// 容器的id，即chat的id，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// 示例值：oc_234jsi43d3ssi993d43545f
    pub fn container_id(mut self, container_id: impl ToString) -> Self {
        self.api_req
            .query_params
            .insert("container_id".to_string(), container_id.to_string());
        self
    }

    /// 历史信息的起始时间（秒级时间戳）
    ///
    /// 示例值：1609296809
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.api_req
            .query_params
            .insert("start_time".to_string(), start_time.to_string());
        self
    }

    /// 历史信息的结束时间（秒级时间戳）
    ///
    /// 示例值：1608594809
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.api_req
            .query_params
            .insert("end_time".to_string(), end_time.to_string());
        self
    }

    /// 消息排序方式
    ///
    /// 示例值：ByCreateTimeAsc
    pub fn sort_type(mut self, sort_type: impl ToString) -> Self {
        self.api_req
            .query_params
            .insert("sort_type".to_string(), sort_type.to_string());
        self
    }

    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的 page_token，下次遍历可采用该 page_token 获取查询结果
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.api_req
            .query_params
            .insert("page_token".to_string(), page_token.to_string());
        self
    }

    /// 分页大小
    ///
    /// 示例值：20
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.api_req
            .query_params
            .insert("page_size".to_string(), page_size.to_string());
        self
    }

    pub fn build(self) -> ListMessageReq {
        ListMessageReq {
            api_req: self.api_req,
        }
    }
}

pub struct ListMessageReq {
    pub api_req: ApiReq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMessageRespData {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: Option<String>,
    pub items: Vec<Message>,
}

pub trait SendMessageTrait {
    fn msg_type(&self) -> String;
    fn content(&self) -> String;
}

/// 文本 text
pub struct MessageText {
    pub text: String,
}

pub struct MessageTextBuilder {
    text: String,
}

impl MessageTextBuilder {
    pub fn new() -> MessageTextBuilder {
        MessageTextBuilder {
            text: "".to_string(),
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text += text;
        self
    }

    pub fn text_line(mut self, text: &str) -> Self {
        self.text = self.text + text + "\n";
        self
    }

    pub fn line(mut self) -> Self {
        self.text += "\n";
        self
    }

    pub fn at_user(mut self, user_id: &str) -> Self {
        self.text = self.text + &format!("<at user_id=\"{}\"></at>", user_id);
        self
    }

    pub fn at_all(mut self) -> Self {
        self.text += "<at user_id=\"all\">name=\"全体成员\"</at>";
        self
    }

    pub fn build(self) -> MessageText {
        MessageText { text: self.text }
    }
}

impl SendMessageTrait for MessageText {
    fn msg_type(&self) -> String {
        "text".to_string()
    }

    fn content(&self) -> String {
        json!({"text": self.text}).to_string()
    }
}

/// 富文本参数
#[derive(Debug, Serialize, Deserialize)]
pub enum MessagePost {
    #[serde(rename = "zh_cn")]
    ZhCN(MessagePostContent),
    #[serde(rename = "zh_cn")]
    EnUS(MessagePostContent),
}

impl SendMessageTrait for MessagePost {
    fn msg_type(&self) -> String {
        "post".to_string()
    }

    fn content(&self) -> String {
        json!(self).to_string()
    }
}

impl MessagePost {
    pub fn zh_cn() -> Self {
        Self::ZhCN(MessagePostContent {
            title: "".to_string(),
            content: vec![],
        })
    }

    pub fn en_us() -> Self {
        Self::EnUS(MessagePostContent {
            title: "".to_string(),
            content: vec![],
        })
    }

    pub fn title(self, title: impl ToString) -> Self {
        match self {
            Self::ZhCN(mut content) => {
                content.title = title.to_string();
                Self::ZhCN(content)
            }
            Self::EnUS(mut content) => {
                content.title = title.to_string();
                Self::EnUS(content)
            }
        }
    }

    pub fn append_content(self, contents: Vec<MessagePostNode>) -> Self {
        match self {
            Self::ZhCN(mut content) => {
                content.content.push(contents);
                Self::ZhCN(content)
            }
            Self::EnUS(mut content) => {
                content.content.push(contents);
                Self::EnUS(content)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePostContent {
    /// 富文本消息的标题。
    pub title: String,
    /// 富文本消息内容，由多个段落组成，每个段落为一个 node 列表。支持的 node 标签类型及对应参数
    pub content: Vec<Vec<MessagePostNode>>,
}

/// 富文本消息内容
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "tag")]
pub enum MessagePostNode {
    /// 文本内容。
    #[serde(rename = "text")]
    Text {
        text: String,
        /// 表示是不是 unescape 解码，默认为 false ，不用可以不填。
        #[serde(skip_serializing_if = "Option::is_none")]
        un_escape: Option<bool>,
        /// 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为bold、underline、lineThrough与italic，非可选值将被忽略。
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<Vec<String>>,
    },
    #[serde(rename = "a")]
    A {
        /// 文本内容
        text: String,
        /// 默认的链接地址，请确保链接地址的合法性，否则消息会发送失败。
        href: String,
        /// 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为bold、underline、lineThrough与italic，非可选值将被忽略。
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<Vec<String>>,
    },
    #[serde(rename = "at")]
    At {
        /// 用户的open_id，union_id 或 user_id，请参考如何获取 User ID、Open ID 和 Union ID？
        /// 注意: @单个用户时，user_id字段必须是有效值；@所有人填"all"。
        user_id: String,
        /// 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为bold、underline、lineThrough与italic，非可选值将被忽略。
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<Vec<String>>,
    },
    #[serde(rename = "img")]
    Img {
        /// 图片的唯一标识，可通过 上传图片 接口获取image_key。
        image_key: String,
    },
    #[serde(rename = "media")]
    Media {
        /// 视频文件的唯一标识，可通过 上传文件 接口获取file_key
        file_key: String,
        /// 视频封面图片的唯一标识，可通过 上传图片 接口获取image_key。
        #[serde(skip_serializing_if = "Option::is_none")]
        image_key: Option<String>,
    },
    #[serde(rename = "emotion")]
    Emotion {
        /// 表情类型，部分可选值请参见表情文案。
        emoji_type: String,
    },
}

/// 图片消息
pub struct MessageImage {
    pub image_key: String,
}

impl SendMessageTrait for MessageImage {
    fn msg_type(&self) -> String {
        "image".to_string()
    }

    fn content(&self) -> String {
        json!({"image_key": self.image_key}).to_string()
    }
}

/// 消息卡片
pub enum MessageInteractive {
    Card,
    Template,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::service::im::v1::message::{MessageTextBuilder, SendMessageTrait};

    #[test]
    fn test_message_text() {
        let t1 = MessageTextBuilder::new().text(" test content").build();
        assert_eq!(t1.text, " test content");
        let t2 = MessageTextBuilder::new().text_line(" test content").build();
        assert_eq!(t2.text, " test content\n");
        let t3 = MessageTextBuilder::new()
            .text(" test content")
            .line()
            .build();
        assert_eq!(t3.text, " test content\n");
        let t4 = MessageTextBuilder::new()
            .text(" test content")
            .at_user("user_id")
            .build();
        assert_eq!(t4.text, " test content<at user_id=\"user_id\"></at>");
        let t5 = MessageTextBuilder::new().at_all().build();
        assert_eq!(t5.text, "<at user_id=\"all\">name=\"全体成员\"</at>");
    }

    #[test]
    fn test_message_post() {
        use crate::service::im::v1::message::{MessagePost, MessagePostNode};
        let post = MessagePost::zh_cn().title("title").append_content(vec![
            MessagePostNode::Text {
                text: "text".to_string(),
                un_escape: None,
                style: None,
            },
            MessagePostNode::A {
                text: "text".to_string(),
                href: "https://www.feishu.cn".to_string(),
                style: None,
            },
            MessagePostNode::At {
                user_id: "user_id".to_string(),
                style: None,
            },
            MessagePostNode::Img {
                image_key: "image_key".to_string(),
            },
            MessagePostNode::Media {
                file_key: "file_key".to_string(),
                image_key: Some("image_key".to_string()),
            },
            MessagePostNode::Emotion {
                emoji_type: "SMILE".to_string(),
            },
        ]);
        assert_eq!(post.msg_type(), "post");
        assert_eq!(
            post.content(),
            json!({
                "zh_cn": {
                    "title":"title",
                    "content": [[{"tag":"text","text":"text"},{"tag":"a","text":"text","href":"https://www.feishu.cn"},{"tag":"at","user_id":"user_id"},{"tag":"img","image_key":"image_key"},{"tag":"media","file_key":"file_key","image_key":"image_key"},{"tag":"emotion","emoji_type":"SMILE"}
                    ]]
                }}).to_string()
        );
    }

    #[test]
    fn test_message_image() {
        use crate::service::im::v1::message::MessageImage;
        let image = MessageImage {
            image_key: "image_key".to_string(),
        };
        assert_eq!(image.msg_type(), "image");
        assert_eq!(
            image.content(),
            json!({"image_key": "image_key"}).to_string()
        );
    }
}

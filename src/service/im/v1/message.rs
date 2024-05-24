use std::collections::HashMap;

use log::error;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ ApiResponseTrait, BaseResp, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

pub struct MessageService {
    pub config: Config,
}

impl MessageService {
    /// 发送消息
    ///
    /// 给指定用户或者会话发送消息，支持文本、富文本、可交互的消息卡片、群名片、个人名片、图片、
    /// 视频、音频、文件、表情包。
    pub async fn create(
        &self,
        create_message_request: CreateMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResp<Message>> {
        let mut api_req = create_message_request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/im/v1/messages".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 获取会话历史消息
    ///
    /// 获取会话（包括单聊、群组）的历史消息（聊天记录）
    /// https://open.feishu.cn/document/server-docs/im-v1/message/list
    pub async fn list(
        &self,
        list_message_request: ListMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResp<ListMessageRespData>> {
        let mut api_req = list_message_request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/im/v1/messages".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    pub fn list_iter(
        &self,
        list_message_request: ListMessageRequest,
        option: Option<RequestOption>,
    ) -> ListMessageIterator {
        ListMessageIterator {
            service: self,
            req: list_message_request,
            option,
            has_more: true,
        }
    }
}

pub struct ListMessageIterator<'a> {
    service: &'a MessageService,
    req: ListMessageRequest,
    option: Option<RequestOption>,
    has_more: bool,
}

impl<'a> ListMessageIterator<'a> {
    pub async fn next(&mut self) -> Option<Vec<Message>> {
        if !self.has_more {
            return None;
        }
        match self
            .service
            .list(self.req.clone(), self.option.clone())
            .await
        {
            Ok(resp) => match resp.data {
                Some(data) => {
                    self.has_more = data.has_more;
                    if data.has_more {
                        self.req
                            .api_req
                            .query_params
                            .insert("page_token".to_string(), data.page_token.unwrap());
                        Some(data.items)
                    } else if data.items.is_empty() {
                        None
                    } else {
                        Some(data.items)
                    }
                }
                None => None,
            },
            Err(e) => {
                error!("Error: {:?}", e);
                None
            }
        }
    }
}

#[derive(Default)]
pub struct CreateMessageRequest {
    api_req: ApiRequest,
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
    pub fn receive_id_type(mut self, receive_id_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("receive_id_type".to_string(), receive_id_type.to_string());
        self
    }

    pub fn request_body(mut self, body: CreateMessageRequestBody) -> Self {
        self.request.api_req.body = serde_json::to_vec(&body).unwrap();
        self
    }

    pub fn build(self) -> CreateMessageRequest {
        self.request
    }
}

/// 发送消息 请求体
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateMessageRequestBody {
    /// 消息接收者的ID，ID类型应与查询参数receive_id_type 对应；
    /// 推荐使用 OpenID，获取方式可参考文档如何获取 Open ID？
    ///
    /// 示例值："ou_7d8a6e6df7621556ce0d21922b676706ccs"
    receive_id: String,
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、
    /// share_user等，类型定义请参考发送消息内容
    ///
    /// 示例值："text"
    msg_type: String,
    /// 消息内容，JSON结构序列化后的字符串。不同msg_type对应不同内容，具体格式说明参考：
    /// 发送消息内容
    ///
    /// 注意：
    /// JSON字符串需进行转义，如换行符转义后为\\n
    /// 文本消息请求体最大不能超过150KB
    /// 卡片及富文本消息请求体最大不能超过30KB
    /// 示例值："{\"text\":\"test content\"}"
    content: String,
    /// 由开发者生成的唯一字符串序列，用于发送消息请求去重；
    /// 持有相同uuid的请求1小时内至多成功发送一条消息
    ///
    /// 示例值："选填，每次调用前请更换，如a0d69e20-1dd1-458b-k525-dfeca4015204"
    ///
    /// 数据校验规则：
    ///
    /// 最大长度：50 字符
    uuid: Option<String>,
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
        self.request.uuid = Some(uuid.to_string());
        self
    }

    pub fn build(self) -> CreateMessageRequestBody {
        self.request
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageResp {
    pub data: Message,
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
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
    /// 被@的用户或机器人的序号。例如，第3个被@到的成员，值为“@_user_3”
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

#[derive(Default, Clone)]
pub struct ListMessageRequest {
    api_req: ApiRequest,
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
        self.request.api_req.query_params.insert(
            "container_id_type".to_string(),
            container_id_type.to_string(),
        );
        self
    }

    /// 容器的id，即chat的id，详情参见[群ID 说明](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-id-description)
    ///
    /// 示例值：oc_234jsi43d3ssi993d43545f
    pub fn container_id(mut self, container_id: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("container_id".to_string(), container_id.to_string());
        self
    }

    /// 历史信息的起始时间（秒级时间戳）
    ///
    /// 示例值：1609296809
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.request
            .api_req
            .query_params
            .insert("start_time".to_string(), start_time.to_string());
        self
    }

    /// 历史信息的结束时间（秒级时间戳）
    ///
    /// 示例值：1608594809
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.request
            .api_req
            .query_params
            .insert("end_time".to_string(), end_time.to_string());
        self
    }

    /// 消息排序方式
    ///
    /// 示例值：ByCreateTimeAsc
    pub fn sort_type(mut self, sort_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("sort_type".to_string(), sort_type.to_string());
        self
    }

    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的
    /// page_token，下次遍历可采用该 page_token 获取查询结果
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("page_token".to_string(), page_token.to_string());
        self
    }

    /// 分页大小
    ///
    /// 示例值：20
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request
            .api_req
            .query_params
            .insert("page_size".to_string(), page_size.to_string());
        self
    }

    pub fn build(self) -> ListMessageRequest {
        self.request
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMessageRespData {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: Option<String>,
    pub items: Vec<Message>,
}

impl ApiResponseTrait for ListMessageRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub trait SendMessageTrait {
    fn msg_type(&self) -> String;
    fn content(&self) -> String;
}

/// 文本 text
pub struct MessageText {
    text: String,
}

impl MessageText {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }

    pub fn add_text(mut self, text: &str) -> Self {
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
pub struct MessagePost {
    /// 默认的语言
    #[serde(skip)]
    default_language: String,
    post: HashMap<String, MessagePostContent>,
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
    pub fn new(lng: &str) -> Self {
        let post = HashMap::new();
        Self {
            default_language: lng.to_string(),
            post,
        }
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        let post = self
            .post
            .entry(self.default_language.clone())
            .or_insert(MessagePostContent {
                title: title.to_string(),
                content: vec![],
            });
        post.title = title.to_string();
        self
    }

    /// 追加富文本内容
    pub fn append_content(mut self, contents: Vec<MessagePostNode>) -> Self {
        let post = self
            .post
            .entry(self.default_language.clone())
            .or_insert(MessagePostContent {
                title: "".to_string(),
                content: vec![],
            });
        post.content.push(contents);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
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
    Text(TextNode),
    #[serde(rename = "a")]
    A(ANode),
    #[serde(rename = "at")]
    At(AtNode),
    #[serde(rename = "img")]
    Img(ImgNode),
    #[serde(rename = "media")]
    Media(MediaNode),
    #[serde(rename = "emotion")]
    Emotion(EmotionNode),
}

/// 文本node
#[derive(Debug, Serialize, Deserialize)]
pub struct TextNode {
    text: String,
    /// 表示是不是 unescape 解码，默认为 false ，不用可以不填。
    #[serde(skip_serializing_if = "Option::is_none")]
    un_escape: Option<bool>,
    /// 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为bold、underline、
    /// lineThrough与italic，非可选值将被忽略。
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Vec<String>>,
}

impl TextNode {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            un_escape: None,
            style: None,
        }
    }

    pub fn un_escape(mut self, un_escape: bool) -> Self {
        self.un_escape = Some(un_escape);
        self
    }

    pub fn style(mut self, style: Vec<&str>) -> Self {
        self.style = Some(style.iter().map(|s| s.to_string()).collect());
        self
    }
}

/// a Node
#[derive(Debug, Serialize, Deserialize)]
pub struct ANode {
    /// 文本内容
    text: String,
    /// 默认的链接地址，请确保链接地址的合法性，否则消息会发送失败。
    href: String,
    /// 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为bold、underline、
    /// lineThrough与italic，非可选值将被忽略。
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Vec<String>>,
}

impl ANode {
    pub fn new(text: &str, href: &str) -> Self {
        Self {
            text: text.to_string(),
            href: href.to_string(),
            style: None,
        }
    }

    pub fn style(mut self, style: Vec<&str>) -> Self {
        self.style = Some(style.iter().map(|s| s.to_string()).collect());
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtNode {
    /// 用户的open_id，union_id 或 user_id，请参考如何获取 User ID、Open ID 和 Union ID？
    /// 注意: @单个用户时，user_id字段必须是有效值；@所有人填"all"。
    user_id: String,
    /// 用于配置文本内容加粗、下划线、删除线和斜体样式，可选值分别为bold、underline、
    /// lineThrough与italic，非可选值将被忽略。
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Vec<String>>,
}

impl AtNode {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            style: None,
        }
    }

    pub fn style(mut self, style: Vec<&str>) -> Self {
        self.style = Some(style.iter().map(|s| s.to_string()).collect());
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImgNode {
    /// 图片的唯一标识，可通过 上传图片 接口获取image_key。
    image_key: String,
}

impl ImgNode {
    pub fn new(image_key: &str) -> Self {
        Self {
            image_key: image_key.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaNode {
    /// 视频文件的唯一标识，可通过 上传文件 接口获取file_key
    file_key: String,
    /// 视频封面图片的唯一标识，可通过 上传图片 接口获取image_key。
    #[serde(skip_serializing_if = "Option::is_none")]
    image_key: Option<String>,
}

impl MediaNode {
    pub fn new(file_key: &str, image_key: Option<&str>) -> Self {
        Self {
            file_key: file_key.to_string(),
            image_key: image_key.map(|s| s.to_string()),
        }
    }
}

/// 表情类型
#[derive(Debug, Serialize, Deserialize)]
pub struct EmotionNode {
    /// 表情类型，部分可选值请参见表情文案。
    emoji_type: String,
}

impl EmotionNode {
    pub fn new(emoji_type: &str) -> Self {
        Self {
            emoji_type: emoji_type.to_string(),
        }
    }
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

/// 卡片模板
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCardTemplate {
    /// 固定值：template
    r#type: String,
    /// 卡片模板数据
    data: CardTemplate,
}

impl SendMessageTrait for MessageCardTemplate {
    fn msg_type(&self) -> String {
        "interactive".to_string()
    }

    fn content(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl MessageCardTemplate {
    pub fn new(template_id: impl ToString, template_variable: Value) -> Self {
        Self {
            r#type: "template".to_string(),
            data: CardTemplate {
                template_id: template_id.to_string(),
                template_variable,
            },
        }
    }
}

/// 卡片模板数据
#[derive(Debug, Serialize, Deserialize)]
struct CardTemplate {
    /// 卡片模板 ID，可在消息卡片搭建工具，我的卡片中，通过复制卡片 ID 获取
    template_id: String,
    /// 卡片中的变量数据，值为{key:value}形式，其中 key 表示变量名称。value 值表示变量的值
    template_variable: Value,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::service::im::v1::message::{
        ANode, AtNode, EmotionNode, ImgNode, MediaNode, MessageText, SendMessageTrait, TextNode,
    };

    #[test]
    fn test_message_text() {
        let t1 = MessageText::new("").add_text(" test content").build();
        assert_eq!(t1.text, " test content");
        let t2 = MessageText::new("").text_line(" test content").build();
        assert_eq!(t2.text, " test content\n");
        let t3 = MessageText::new("")
            .add_text(" test content")
            .line()
            .build();
        assert_eq!(t3.text, " test content\n");
        let t4 = MessageText::new("")
            .add_text(" test content")
            .at_user("user_id")
            .build();
        assert_eq!(t4.text, " test content<at user_id=\"user_id\"></at>");
        let t5 = MessageText::new("").at_all().build();
        assert_eq!(t5.text, "<at user_id=\"all\">name=\"全体成员\"</at>");
    }

    #[test]
    fn test_message_post() {
        use crate::service::im::v1::message::{MessagePost, MessagePostNode};
        let post = MessagePost::new("zh_cn")
            .title("title")
            .append_content(vec![
                MessagePostNode::Text(TextNode::new("text")),
                MessagePostNode::A(ANode::new("text", "https://www.feishu.cn")),
                MessagePostNode::At(AtNode::new("user_id")),
                MessagePostNode::Img(ImgNode::new("image_key")),
                MessagePostNode::Media(MediaNode::new("file_key", Some("image_key"))),
                MessagePostNode::Emotion(EmotionNode::new("SMILE")),
            ]);
        assert_eq!(post.msg_type(), "post");
        assert_eq!(
            json!(post),
            json!({
            "post": {
            "zh_cn": {
                "title":"title",
                "content": [[{"tag":"text","text":"text"},{"tag":"a","text":"text","href":"https://www.feishu.cn"},{"tag":"at","user_id":"user_id"},{"tag":"img","image_key":"image_key"},{"tag":"media","file_key":"file_key","image_key":"image_key"},{"tag":"emotion","emoji_type":"SMILE"}
                ]]
            }}})
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

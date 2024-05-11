use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::api_req::ApiReq;
use crate::core::api_resp::{ApiResp, BaseResp, CodeMsg};
use crate::core::config::Config;
use crate::core::constants::AccessTokenType;
use crate::core::error::LarkAPIError;
use crate::core::http::Transport;

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
    ) -> Result<BaseResp<Message>, LarkAPIError> {
        let mut api_req = req.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/im/v1/messages".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, vec![])?;

        Ok(api_resp.try_into()?)
    }

    pub fn list(
        &self,
        req: ListMessageReq,
    ) -> Result<BaseResp<ListMessageRespData>, LarkAPIError> {
        let mut api_req = req.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/im/v1/messages".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, vec![])?;

        Ok(api_resp.try_into()?)
    }
}

pub struct CreateMessageReqBuilder {
    api_req: ApiReq,
    body: Option<CreateMessageReqBody>,
}

impl CreateMessageReqBuilder {
    pub fn new() -> CreateMessageReqBuilder {
        let builder = CreateMessageReqBuilder {
            api_req: ApiReq::default(),
            body: None,
        };
        builder
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
        // let json_string = self.body.clone().unwrap().content.clone();
        // let json_value: Value = serde_json::from_str(&json_string).unwrap();
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
    pub content: Value,
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

impl ListMessageReqBuilder {
    pub fn new() -> ListMessageReqBuilder {
        let builder = ListMessageReqBuilder {
            api_req: ApiReq::default(),
            limit: None,
        };
        builder
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
    pub items: Vec<Message>
}


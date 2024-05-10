use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::LarkClient;
use crate::core::api_req::ApiReq;
use crate::core::api_resp::ApiResp;
use crate::core::constants::AccessTokenType;
use crate::core::error::LarkAPIError;
use crate::core::http::Transport;

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

/// 请求体
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

/// 发送消息
///
/// 给指定用户或者会话发送消息，支持文本、富文本、可交互的消息卡片、群名片、个人名片、图片、视频、音频、文件、表情包。
pub fn create(client: &LarkClient, req: CreateMessageReq) -> Result<ApiResp, LarkAPIError> {
    let mut api_req = req.api_req;
    api_req.http_method = Method::POST;
    api_req.api_path = "/open-apis/im/v1/messages".to_string();
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let resp = Transport::request(api_req, &client.config.clone(), vec![])?;

    println!("{:?}", resp);

    Ok(resp)
}

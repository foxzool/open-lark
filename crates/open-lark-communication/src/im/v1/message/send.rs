use reqwest::Method;

use open_lark_core::core::{
    constants::AccessTokenType, http::Transport,
    api_resp::BaseResponse, constants::AccessTokenType, endpoints::EndpointBuilder,
    http::Transport, req_option::RequestOption, standard_response::StandardResponse, SDKResult,
},
use super::{CreateMessageResp, Message, MessageService },
use crate::im::v1::message::builders::{CreateMessageRequest, UpdateMessageRequest },
impl MessageService {
    /// 发送消息
    ///
    /// 给指定用户或者会话发送消息，支持文本、富文本、可交互的消息卡片、群名片、个人名片、图片、
    /// 视频、音频、文件、表情包。
    /// # API文档
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder()
    ///         .app_id("your_app_id")
    ///         .app_secret("your_app_secret")
    ///         .build()?;
    ///     // 发送文本消息
    ///     let message = client.im.v1.message.create_message_builder()
    ///         .receive_id("user_open_id")
    ///         .receive_id_type("open_id")
    ///         .content(r#"{"text":"Hello World"}"#)
    ///         .msg_type("text")
    ///         .execute(&client.im.v1.message)
    ///         .await?;
    ///     println!("发送成功，消息ID: {}", message.message_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(
        &self,
        create_message_request: CreateMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Message> {
        let mut api_req = create_message_request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V1_SEND_MESSAGE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        let api_resp: BaseResponse<CreateMessageResp> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result().map(|resp| resp.data)
    }
    /// 撤回消息
    /// 撤回已经发送成功的消息。支持撤回应用自身发送的消息、应用管理员撤回群成员的消息、
    /// 撤回指定用户在指定会话的消息等不同场景。
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/delete
    ///     // 撤回消息
    ///     client.im.v1.message.delete("om_1234567890", None).await?;
    ///     println!("消息撤回成功");
    pub async fn delete(&self, message_id: &str, option: Option<RequestOption>) -> SDKResult<()> {
        let mut api_req = open_lark_core::core::api_req::ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(
            open_lark_core::core::endpoints::im::IM_V1_DELETE_MESSAGE,
            "message_id",
            message_id,
        ));
        let api_resp: BaseResponse<serde_json::Value> =
        api_resp.into_result().map(|_| ())
    /// 更新消息
    /// 更新已发送的消息。仅支持更新应用自身发送的文本消息、图片消息和文件消息。
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/update
    pub async fn update(
        message_id: &str,
        update_message_request: UpdateMessageRequest,
        let mut api_req = update_message_request.api_req;
        api_req.set_http_method(Method::PATCH);
            open_lark_core::core::endpoints::im::IM_V1_UPDATE_MESSAGE,
    /// 回复消息
    /// 在指定消息下进行回复。支持回复文本、图片、文件等类型的消息。
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/reply
    pub async fn reply(
        reply_message_request: CreateMessageRequest,
        let mut api_req = reply_message_request.api_req;
            open_lark_core::core::endpoints::im::IM_V1_REPLY_MESSAGE,

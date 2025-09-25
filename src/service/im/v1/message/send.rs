use reqwest::Method;

use crate::{
    core::{
        api_resp::BaseResponse, constants::AccessTokenType, endpoints::EndpointBuilder,
        http::Transport, req_option::RequestOption, standard_response::StandardResponse, SDKResult,
    },
    service::im::v1::message::{CreateMessageResp, Message},
};

// MessageService is defined in the parent module (mod.rs)
use crate::service::im::v1::message::MessageService;

impl MessageService {
    /// 发送消息
    ///
    /// 给指定用户或者会话发送消息，支持文本、富文本、可交互的消息卡片、群名片、个人名片、图片、
    /// 视频、音频、文件、表情包。
    ///
    /// <https://open.feishu.cn/document/server-docs/im-v1/message/create>
    pub async fn create(
        &self,
        create_message_request: crate::service::im::v1::message::builders::CreateMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Message> {
        let mut api_req = create_message_request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = crate::core::endpoints::im::IM_V1_SEND_MESSAGE.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<CreateMessageResp> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result().map(|resp| resp.data)
    }

    /// 撤回消息
    ///
    /// 撤回已经发送成功的消息。支持撤回应用自身发送的消息、应用管理员撤回群成员的消息、
    /// 撤回指定用户在指定会话的消息等不同场景。
    ///
    /// <https://open.feishu.cn/document/server-docs/im-v1/message/delete>
    pub async fn delete(&self, message_id: &str, option: Option<RequestOption>) -> SDKResult<()> {
        let api_req = crate::core::api_req::ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V1_DELETE_MESSAGE,
                "message_id",
                message_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        let api_resp: BaseResponse<serde_json::Value> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result().map(|_| ())
    }

    /// 更新消息
    ///
    /// 更新已发送的消息。仅支持更新应用自身发送的文本消息、图片消息和文件消息。
    ///
    /// <https://open.feishu.cn/document/server-docs/im-v1/message/update>
    pub async fn update(
        &self,
        message_id: &str,
        update_message_request: crate::service::im::v1::message::builders::UpdateMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Message> {
        let mut api_req = update_message_request.api_req;
        api_req.http_method = Method::PATCH;
        api_req.api_path = EndpointBuilder::replace_param(
            crate::core::endpoints::im::IM_V1_UPDATE_MESSAGE,
            "message_id",
            message_id,
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<CreateMessageResp> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result().map(|resp| resp.data)
    }

    /// 回复消息
    ///
    /// 在指定消息下进行回复。支持回复文本、图片、文件等类型的消息。
    ///
    /// <https://open.feishu.cn/document/server-docs/im-v1/message/reply>
    pub async fn reply(
        &self,
        message_id: &str,
        reply_message_request: crate::service::im::v1::message::builders::CreateMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Message> {
        let mut api_req = reply_message_request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = EndpointBuilder::replace_param(
            crate::core::endpoints::im::IM_V1_REPLY_MESSAGE,
            "message_id",
            message_id,
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<CreateMessageResp> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result().map(|resp| resp.data)
    }
}

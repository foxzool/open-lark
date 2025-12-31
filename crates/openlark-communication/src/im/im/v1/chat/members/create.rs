//! 将用户或机器人拉入群聊
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat-member/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
    im::im::v1::chat::members::models::{CreateChatMembersBody, CreateChatMembersResponse, MemberIdType, SucceedType},
};

/// 将用户或机器人拉入群聊请求
pub struct CreateChatMembersRequest {
    config: Config,
    chat_id: String,
    member_id_type: Option<MemberIdType>,
    succeed_type: Option<SucceedType>,
}

impl CreateChatMembersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            member_id_type: None,
            succeed_type: None,
        }
    }

    /// 群 ID（路径参数）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    /// 群成员 ID 类型（查询参数，可选，默认 open_id）
    pub fn member_id_type(mut self, member_id_type: MemberIdType) -> Self {
        self.member_id_type = Some(member_id_type);
        self
    }

    /// 出现不可用 ID 后的处理方式（查询参数，可选，默认 0）
    pub fn succeed_type(mut self, succeed_type: SucceedType) -> Self {
        self.succeed_type = Some(succeed_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat-member/create
    pub async fn execute(self, body: CreateChatMembersBody) -> SDKResult<CreateChatMembersResponse> {
        validate_required!(self.chat_id, "chat_id 不能为空");
        if body.id_list.is_empty() {
            return Err(openlark_core::error::validation_error(
                "id_list 不能为空".to_string(),
                "成员列表不可为空".to_string(),
            ));
        }

        // url: POST:/open-apis/im/v1/chats/:chat_id/members
        let mut req: ApiRequest<CreateChatMembersResponse> = ApiRequest::post(format!(
            "{}/{}/members",
            IM_V1_CHATS, self.chat_id
        ))
        .body(serialize_params(&body, "将用户或机器人拉入群聊")?);

        if let Some(member_id_type) = self.member_id_type {
            req = req.query("member_id_type", member_id_type.as_str());
        }
        if let Some(succeed_type) = self.succeed_type {
            req = req.query("succeed_type", succeed_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "将用户或机器人拉入群聊")
    }
}


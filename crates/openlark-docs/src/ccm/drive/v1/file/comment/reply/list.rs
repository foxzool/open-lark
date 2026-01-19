//! 获取回复信息

//!

//! docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/list-2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::ReplyInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ListCommentReplyRequest {
    pub file_token: String,

    pub comment_id: String,

    /// 云文档类型（必填）
    pub file_type: String,

    pub page_token: Option<String>,

    pub page_size: Option<i32>,

    pub user_id_type: Option<String>,
}

impl ListCommentReplyRequest {
    pub fn new(
        file_token: impl Into<String>,

        comment_id: impl Into<String>,

        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            comment_id: comment_id.into(),

            file_type: file_type.into(),

            page_token: None,

            page_size: None,

            user_id_type: None,
        }
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());

        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);

        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());

        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ListCommentReplyResponse {
    #[serde(default)]
    pub items: Vec<ReplyInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    pub has_more: bool,
}

impl ApiResponseTrait for ListCommentReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn list_comment_reply(
    request: ListCommentReplyRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<ListCommentReplyResponse> {
    if request.file_token.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_token",
            "file_token 不能为空",
        ));
    }

    if request.comment_id.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "comment_id",
            "comment_id 不能为空",
        ));
    }

    if request.file_type.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 不能为空",
        ));
    }

    super::super::validate_comment_file_type_for_list_like(&request.file_type)?;

    if let Some(page_size) = request.page_size {
        if !(1..=100).contains(&page_size) {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 必须在 1~100 之间",
            ));
        }
    }

    let api_endpoint =
        DriveApi::ListCommentReplies(request.file_token.clone(), request.comment_id.clone());

    let mut api_request: ApiRequest<ListCommentReplyResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    api_request = api_request.query("file_type", &request.file_type);

    if let Some(token) = &request.page_token {
        api_request = api_request.query("page_token", token);
    }

    if let Some(size) = request.page_size {
        api_request = api_request.query("page_size", &size.to_string());
    }

    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, Some(option)).await?;

    extract_response_data(response, "获取回复信息")
}

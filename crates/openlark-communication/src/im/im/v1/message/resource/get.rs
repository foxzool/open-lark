//! 获取消息中的资源文件
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message/get-2

use openlark_core::{api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::IM_V1_MESSAGES};

/// 消息资源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageResourceType {
    Image,
    File,
}

impl MessageResourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::File => "file",
        }
    }
}

/// 获取消息中的资源文件请求
pub struct GetMessageResourceRequest {
    config: Config,
    message_id: String,
    file_key: String,
    resource_type: Option<MessageResourceType>,
}

impl GetMessageResourceRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
            file_key: String::new(),
            resource_type: None,
        }
    }

    /// 消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 资源 Key（路径参数）
    pub fn file_key(mut self, file_key: impl Into<String>) -> Self {
        self.file_key = file_key.into();
        self
    }

    /// 资源类型（查询参数，必填）
    pub fn resource_type(mut self, resource_type: MessageResourceType) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// 执行请求（返回二进制内容）
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message/get-2
    pub async fn execute(self) -> SDKResult<Vec<u8>> {
        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(self.file_key, "file_key 不能为空");
        let resource_type = self.resource_type.ok_or_else(|| {
            error::validation_error(
                "type 不能为空".to_string(),
                "获取消息资源需要指定资源类型（type=image/file）".to_string(),
            )
        })?;

        // url: GET:/open-apis/im/v1/messages/:message_id/resources/:file_key
        let req: ApiRequest<Vec<u8>> = ApiRequest::get(format!(
            "{}/{}/resources/{}",
            IM_V1_MESSAGES, self.message_id, self.file_key
        ))
        .query("type", resource_type.as_str());

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取消息中的资源文件")
    }
}


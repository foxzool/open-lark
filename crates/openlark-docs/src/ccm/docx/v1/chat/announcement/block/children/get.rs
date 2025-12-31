/// 获取所有子块
///
/// 获取群公告中指定块的所有子块的富文本内容并分页返回。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/get
/// doc: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::ccm::docx::common_types::DocxBlock;
use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 获取所有子块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlockChildrenParams {
    /// 群聊ID
    pub chat_id: String,
    /// 父块ID
    pub block_id: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 获取所有子块响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlockChildrenResponse {
    #[serde(default)]
    pub items: Vec<DocxBlock>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetChatAnnouncementBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取所有子块请求
pub struct GetChatAnnouncementBlockChildrenRequest {
    config: Config,
}

impl GetChatAnnouncementBlockChildrenRequest {
    /// 创建获取所有子块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/get
    pub async fn execute(
        self,
        params: GetChatAnnouncementBlockChildrenParams,
    ) -> SDKResult<GetChatAnnouncementBlockChildrenResponse> {
        validate_required!(params.chat_id, "群聊ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");

        let api_endpoint = DocxApiV1::ChatAnnouncementBlockChildrenGet(
            params.chat_id.clone(),
            params.block_id.clone(),
        );

        let mut api_request: ApiRequest<GetChatAnnouncementBlockChildrenResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(page_size) = params.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = params.page_token {
            api_request = api_request.query("page_token", &page_token);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取所有子块")
    }
}

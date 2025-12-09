//! 获取群公告所有块
//!
//! 获取群公告所有块的富文本内容并分页返回。
//! API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DocxApiV1;
use crate::ccm::docx::{BlockContent, BlockItem, PageData};

/// 获取群公告所有块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlocksParams {
    /// 群聊ID
    pub chat_id: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 获取群公告所有块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlocksResponse {
    /// 块列表
    pub data: Option<PageData<BlockItem>>,
}

impl ApiResponseTrait for GetChatAnnouncementBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群公告所有块请求
pub struct GetChatAnnouncementBlocksRequest {
    config: Config,
}

impl GetChatAnnouncementBlocksRequest {
    /// 创建获取群公告所有块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement/get
    pub async fn execute(self, params: GetChatAnnouncementBlocksParams) -> SDKResult<GetChatAnnouncementBlocksResponse> {
        // 验证必填字段
        validate_required!(params.chat_id, "群聊ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::ChatAnnouncementBlockList(params.chat_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetChatAnnouncementBlocksResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 设置查询参数
        if let Some(page_size) = params.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &params.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
//! 批量更新群公告块的内容
//!
//! 批量更新块的富文本内容。
//! API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/batch_update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::docx::v1::models::AnnouncementBlockInfo;
use crate::common::api_endpoints::DocxApiV1;

/// 批量更新群公告块的内容请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchUpdateChatAnnouncementBlocksParams {
    /// 群ID
    pub chat_id: String,
    /// 块更新列表
    pub blocks: Vec<AnnouncementBlockUpdate>,
}

/// 块更新信息
#[derive(Debug, Clone, Serialize, Default)]
pub struct AnnouncementBlockUpdate {
    /// 块ID
    pub block_id: String,
    /// 块内容
    pub content: serde_json::Value,
}

/// 批量更新群公告块的内容响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct BatchUpdateChatAnnouncementBlocksResponse {
    /// 更新结果
    pub data: Vec<AnnouncementBlockInfo>,
}

impl ApiResponseTrait for BatchUpdateChatAnnouncementBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新群公告块的内容请求
pub struct BatchUpdateChatAnnouncementBlocksRequest {
    config: Config,
}

impl BatchUpdateChatAnnouncementBlocksRequest {
    /// 创建新的批量更新群公告块的内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/batch_update
    pub async fn execute(self, params: BatchUpdateChatAnnouncementBlocksParams) -> SDKResult<BatchUpdateChatAnnouncementBlocksResponse> {
        // 验证必填字段
        validate_required!(params.chat_id, "群ID不能为空");
        validate_required!(params.blocks, "块更新列表不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::ChatAnnouncementBlockBatchUpdate(params.chat_id.clone());

        // 创建API请求
        let api_request: ApiRequest<BatchUpdateChatAnnouncementBlocksResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 设置请求体
        let body = serde_json::json!({
            "blocks": params.blocks
        });

        let request_with_body = api_request.body(body);

        // 发送请求
        let response = Transport::request(request_with_body, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
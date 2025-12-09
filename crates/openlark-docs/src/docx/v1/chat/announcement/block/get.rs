//! 获取群公告块的内容
//!
//! 获取指定块的富文本内容。
//! API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/get

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

/// 获取群公告块的内容请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetChatAnnouncementBlockParams {
    /// 群ID
    pub chat_id: String,
    /// 块ID
    pub block_id: String,
}

/// 获取群公告块的内容响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GetChatAnnouncementBlockResponse {
    /// 块信息
    pub data: AnnouncementBlockInfo,
}

impl ApiResponseTrait for GetChatAnnouncementBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群公告块的内容请求
pub struct GetChatAnnouncementBlockRequest {
    config: Config,
}

impl GetChatAnnouncementBlockRequest {
    /// 创建新的获取群公告块的内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/get
    pub async fn execute(self, params: GetChatAnnouncementBlockParams) -> SDKResult<GetChatAnnouncementBlockResponse> {
        // 验证必填字段
        validate_required!(params.chat_id, "群ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::ChatAnnouncementBlockGet(params.chat_id.clone(), params.block_id.clone());

        // 创建API请求
        let api_request: ApiRequest<GetChatAnnouncementBlockResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
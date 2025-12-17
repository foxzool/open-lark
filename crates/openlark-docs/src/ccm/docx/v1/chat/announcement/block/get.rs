/// 获取群公告块的内容
///
/// 获取指定块的富文本内容。
/// docPath: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::ccm::docx::BlockContent;
use crate::common::api_endpoints::DocxApiV1;

/// 获取群公告块内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlockParams {
    /// 群聊ID
    pub chat_id: String,
    /// 块ID
    pub block_id: String,
}

/// 获取群公告块内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlockResponse {
    /// 块信息
    pub data: Option<BlockData>,
}

/// 块数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockData {
    /// 块ID
    pub block_id: String,
    /// 块类型
    pub block_type: String,
    /// 块内容
    pub content: Option<BlockContent>,
    /// 子块数量
    pub children_count: Option<u32>,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
}

impl ApiResponseTrait for GetChatAnnouncementBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群公告块内容请求
pub struct GetChatAnnouncementBlockRequest {
    config: Config,
}

impl GetChatAnnouncementBlockRequest {
    /// 创建获取群公告块内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/get
    pub async fn execute(
        self,
        params: GetChatAnnouncementBlockParams,
    ) -> SDKResult<GetChatAnnouncementBlockResponse> {
        // 验证必填字段
        validate_required!(params.chat_id, "群聊ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint =
            DocxApiV1::ChatAnnouncementBlockGet(params.chat_id.clone(), params.block_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetChatAnnouncementBlockResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

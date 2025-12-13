/// 删除群公告中的块
///
/// 删除指定块的子块。
/// docPath: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/batch_delete

use crate::common::api_endpoints::DocxApiV1;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除群公告中的块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteChatAnnouncementBlockChildrenParams {
    /// 群聊ID
    pub chat_id: String,
    /// 父块ID
    pub block_id: String,
    /// 要删除的子块ID列表
    pub block_ids: Vec<String>,
}

/// 删除群公告中的块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteChatAnnouncementBlockChildrenResponse {
    /// 删除结果
    pub data: Option<BatchDeleteResult>,
}

/// 批量删除结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteResult {
    /// 删除成功的块数量
    pub deleted_count: Option<u32>,
    /// 删除失败的块数量
    pub failed_count: Option<u32>,
    /// 失败的块列表
    pub failed_blocks: Option<Vec<FailedBlock>>,
}

/// 失败的块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedBlock {
    /// 块ID
    pub block_id: String,
    /// 错误信息
    pub error_message: String,
}

impl ApiResponseTrait for BatchDeleteChatAnnouncementBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除群公告中的块请求
pub struct BatchDeleteChatAnnouncementBlockChildrenRequest {
    config: Config,
}

impl BatchDeleteChatAnnouncementBlockChildrenRequest {
    /// 创建删除群公告中的块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/batch_delete
    pub async fn execute(
        self,
        params: BatchDeleteChatAnnouncementBlockChildrenParams,
    ) -> SDKResult<BatchDeleteChatAnnouncementBlockChildrenResponse> {
        // 验证必填字段
        validate_required!(params.chat_id, "群聊ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");
        validate_required!(params.block_ids, "子块ID列表不能为空");

        // 构建API端点
        let api_endpoint = DocxApiV1::ChatAnnouncementBlockChildrenBatchDelete(
            params.chat_id.clone(),
            params.block_id.clone(),
        );

        // 创建API请求
        let mut api_request: ApiRequest<BatchDeleteChatAnnouncementBlockChildrenResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        // 设置请求体
        api_request = api_request.json_body(&params);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

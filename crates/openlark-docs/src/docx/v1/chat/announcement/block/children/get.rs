//! 获取所有子块
//!
//! 获取群公告中指定块的所有子块的富文本内容并分页返回。
//! API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/get-2

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

/// 获取所有子块请求参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetChatAnnouncementBlockChildrenParams {
    /// 群ID
    pub chat_id: String,
    /// 块ID
    pub block_id: String,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页令牌
    pub page_token: Option<String>,
}

/// 获取所有子块响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GetChatAnnouncementBlockChildrenResponse {
    /// 子块列表
    pub data: Vec<AnnouncementBlockInfo>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页令牌
    pub page_token: Option<String>,
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
    /// 创建新的获取所有子块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/get-2
    pub async fn execute(self, params: GetChatAnnouncementBlockChildrenParams) -> SDKResult<GetChatAnnouncementBlockChildrenResponse> {
        // 验证必填字段
        validate_required!(params.chat_id, "群ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::ChatAnnouncementBlockChildrenGet(params.chat_id.clone(), params.block_id.clone());

        // 创建API请求
        let mut api_request: ApiRequest<GetChatAnnouncementBlockChildrenResponse> =
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
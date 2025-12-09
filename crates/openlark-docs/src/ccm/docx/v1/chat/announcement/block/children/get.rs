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

use crate::common::api_endpoints::DocxApiV1;

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

/// 获取所有子块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlockChildrenResponse {
    /// 子块列表
    pub data: Option<ChildrenListData>,
}

/// 子块列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildrenListData {
    /// 子块列表
    pub items: Vec<ChildBlockItem>,
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

/// 子块项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildBlockItem {
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

/// 块内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockContent {
    /// 文本内容
    pub text: Option<String>,
    /// 富文本内容
    pub rich_text: Option<RichText>,
}

/// 富文本内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichText {
    /// 段落列表
    pub paragraphs: Vec<Paragraph>,
}

/// 段落
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paragraph {
    /// 文本元素列表
    pub elements: Vec<TextElement>,
}

/// 文本元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
    /// 文本
    pub text_run: Option<TextRun>,
}

/// 文本运行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    /// 内容
    pub content: String,
    /// 样式
    pub style: Option<TextStyle>,
}

/// 文本样式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    /// 是否加粗
    pub bold: Option<bool>,
    /// 是否斜体
    pub italic: Option<bool>,
    /// 是否删除线
    pub strikethrough: Option<bool>,
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
    /// API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/get
    pub async fn execute(self, params: GetChatAnnouncementBlockChildrenParams) -> SDKResult<GetChatAnnouncementBlockChildrenResponse> {
        // 验证必填字段
        validate_required!(params.chat_id, "群聊ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = DocxApiV1::ChatAnnouncementBlockChildrenGet(
            params.chat_id.clone(),
            params.block_id.clone(),
        );

        // 创建API请求 - 使用类型安全的URL生成
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
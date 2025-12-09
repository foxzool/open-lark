//! 在群公告中创建块
//!
//! 在指定块的子块列表中，新创建一批子块，并放置到指定位置。如果操作成功，接口将返回新创建子块的富文本内容。
//! API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 在群公告中创建块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatAnnouncementBlockChildrenParams {
    /// 群聊ID
    pub chat_id: String,
    /// 父块ID
    pub block_id: String,
    /// 新建的子块列表
    pub children: Vec<NewBlock>,
    /// 插入位置
    pub location: Option<BlockLocation>,
}

/// 新建的块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBlock {
    /// 块类型
    pub block_type: i32,
    /// 块内容
    pub content: Option<BlockContent>,
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

/// 块位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockLocation {
    /// 插入位置索引
    pub index: Option<i32>,
}

/// 在群公告中创建块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatAnnouncementBlockChildrenResponse {
    /// 创建结果
    pub data: Option<CreateResult>,
}

/// 创建结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResult {
    /// 创建成功的块列表
    pub blocks: Option<Vec<CreatedBlock>>,
}

/// 创建的块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedBlock {
    /// 块ID
    pub block_id: String,
    /// 块类型
    pub block_type: i32,
    /// 块内容
    pub content: Option<BlockContent>,
}

impl ApiResponseTrait for CreateChatAnnouncementBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 在群公告中创建块请求
pub struct CreateChatAnnouncementBlockChildrenRequest {
    config: Config,
}

impl CreateChatAnnouncementBlockChildrenRequest {
    /// 创建在群公告中创建块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/create
    pub async fn execute(self, params: CreateChatAnnouncementBlockChildrenParams) -> SDKResult<CreateChatAnnouncementBlockChildrenResponse> {
        // 验证必填字段
        validate_required!(params.chat_id, "群聊ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");
        validate_required!(params.children, "子块列表不能为空");

        // 构建API端点URL
        let url = format!("/open-apis/docx/v1/chats/{}/announcement/blocks/{}/children", params.chat_id, params.block_id);

        // 创建API请求
        let mut api_request: ApiRequest<CreateChatAnnouncementBlockChildrenResponse> = ApiRequest::post(&url);

        // 设置请求体
        api_request = api_request.body(&params)?;

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
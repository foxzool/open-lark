//! 获取文档所有块
//!
//! 获取文档所有块的富文本内容并分页返回。
//! API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取文档所有块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlocksParams {
    /// 文档ID
    pub document_id: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 版本号（可选）
    pub version: Option<u64>,
}

/// 获取文档所有块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlocksResponse {
    /// 块列表
    pub data: Option<BlockListData>,
}

/// 块列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockListData {
    /// 块列表
    pub items: Vec<BlockItem>,
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

/// 块项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockItem {
    /// 块ID
    pub block_id: String,
    /// 块类型
    pub block_type: String,
    /// 块内容
    pub content: Option<BlockContent>,
    /// 子块数量
    pub children_count: Option<u32>,
    /// 父块ID
    pub parent_block_id: Option<String>,
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

impl ApiResponseTrait for GetDocumentBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档所有块请求
pub struct GetDocumentBlocksRequest {
    config: Config,
}

impl GetDocumentBlocksRequest {
    /// 创建获取文档所有块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/list
    pub async fn execute(self, params: GetDocumentBlocksParams) -> SDKResult<GetDocumentBlocksResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");

        // 构建API端点URL
        let url = format!("/open-apis/docx/v1/documents/{}/blocks", params.document_id);

        // 创建API请求
        let mut api_request: ApiRequest<GetDocumentBlocksResponse> = ApiRequest::get(&url);

        // 设置查询参数
        if let Some(page_size) = params.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = params.page_token {
            api_request = api_request.query("page_token", &page_token);
        }
        if let Some(version) = params.version {
            api_request = api_request.query("version", &version.to_string());
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
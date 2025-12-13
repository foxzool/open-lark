/// Markdown/HTML 内容转换为文档块
///
/// 将 Markdown/HTML 格式的内容转换为文档块，以便于将 Markdown/HTML 格式的内容插入到文档中。目前支持转换为的块类型包含文本、一到九级标题、无序列表、有序列表、代码块、引用、待办事项、图片、表格、表格单元格。
/// API文档: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert

use crate::ccm::docx::common_types::RichText;
use crate::common::api_endpoints::DocxApiV1;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// Markdown/HTML 内容转换为文档块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertContentToBlocksParams {
    /// 源内容格式
    pub source_format: SourceFormat,
    /// 源内容
    pub content: String,
}

/// 源内容格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceFormat {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "html")]
    Html,
}

/// Markdown/HTML 内容转换为文档块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertContentToBlocksResponse {
    /// 转换结果
    pub data: Option<ConvertResult>,
}

/// 转换结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertResult {
    /// 转换的块列表
    pub blocks: Vec<ConvertedBlock>,
}

/// 转换的块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertedBlock {
    /// 块类型
    pub block_type: i32,
    /// 块内容
    pub content: Option<ExtendedBlockContent>,
    /// 子块
    pub children: Option<Vec<ConvertedBlock>>,
}

/// 块内容（扩展版本，包含代码和表格）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedBlockContent {
    /// 文本内容
    pub text: Option<String>,
    /// 富文本内容
    pub rich_text: Option<RichText>,
    /// 代码内容
    pub code: Option<CodeContent>,
    /// 表格内容
    pub table: Option<TableContent>,
    /// 其他类型内容
    #[serde(flatten)]
    pub other: Option<serde_json::Value>,
}

/// 代码内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeContent {
    /// 代码文本
    pub text: String,
    /// 编程语言
    pub language: Option<String>,
}

/// 表格内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableContent {
    /// 表格行
    pub rows: Vec<TableRow>,
    /// 表格列
    pub columns: Vec<TableColumn>,
}

/// 表格行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    /// 单元格列表
    pub cells: Vec<TableCell>,
}

/// 表格列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    /// 列宽（像素）
    pub width: Option<u32>,
}

/// 表格单元格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    /// 单元格内容
    pub content: Option<ExtendedBlockContent>,
}

impl ApiResponseTrait for ConvertContentToBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Markdown/HTML 内容转换为文档块请求
pub struct ConvertContentToBlocksRequest {
    config: Config,
}

impl ConvertContentToBlocksRequest {
    /// 创建Markdown/HTML 内容转换为文档块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert
    pub async fn execute(
        self,
        params: ConvertContentToBlocksParams,
    ) -> SDKResult<ConvertContentToBlocksResponse> {
        // 验证必填字段
        validate_required!(params.content, "源内容不能为空");

        // 构建API端点
        let api_endpoint = DocxApiV1::DocumentConvert;

        // 创建API请求
        let mut api_request: ApiRequest<ConvertContentToBlocksResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 设置请求体
        api_request = api_request.json_body(&params);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

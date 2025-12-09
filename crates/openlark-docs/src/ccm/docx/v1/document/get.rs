//! 获取文档基本信息
//!
//! 获取文档最新版本号、标题等
//! API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取文档基本信息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentParams {
    /// 文档ID
    pub document_id: String,
    /// 是否返回内容（可选）
    pub with_content: Option<bool>,
}

/// 获取文档基本信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentResponse {
    /// 文档信息
    pub data: Option<DocumentData>,
}

/// 文档数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentData {
    /// 文档ID
    pub document_id: String,
    /// 文档token
    pub document_token: String,
    /// 文档标题
    pub title: String,
    /// 文档URL
    pub url: String,
    /// 文档类型
    pub doc_type: String,
    /// 文档状态
    pub status: Option<i32>,
    /// 版本号
    pub version: Option<i64>,
    /// 字符数
    pub char_count: Option<i64>,
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
    /// 创建者信息
    pub creator: Option<UserInfo>,
    /// 更新者信息
    pub updater: Option<UserInfo>,
    /// 目录信息
    pub folder: Option<FolderInfo>,
    /// 文档内容（如果请求时with_content为true）
    pub content: Option<DocumentContent>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 目录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    /// 目录token
    pub folder_token: String,
    /// 目录名称
    pub name: String,
}

/// 文档内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContent {
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

impl ApiResponseTrait for GetDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档基本信息请求
pub struct GetDocumentRequest {
    config: Config,
}

impl GetDocumentRequest {
    /// 创建获取文档基本信息请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/get
    pub async fn execute(self, params: GetDocumentParams) -> SDKResult<GetDocumentResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");

        // 构建API端点URL
        let url = format!("/open-apis/docx/v1/documents/{}", params.document_id);

        // 创建API请求
        let mut api_request: ApiRequest<GetDocumentResponse> = ApiRequest::get(&url);

        // 设置查询参数
        if let Some(with_content) = params.with_content {
            api_request = api_request.query("with_content", &with_content.to_string());
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
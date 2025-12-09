//! 创建文档
//!
//! 创建新版文档，文档标题和目录可选。
//! API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建文档请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentParams {
    /// 文档标题（可选）
    pub title: Option<String>,
    /// 文档目录ID（可选）
    pub folder_token: Option<String>,
    /// 文档模板ID（可选）
    pub template_id: Option<String>,
    /// 文档类型（可选）
    pub doc_type: Option<String>,
}

/// 创建文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentResponse {
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
    /// 创建时间
    pub create_time: i64,
    /// 更新时间
    pub update_time: i64,
    /// 创建者信息
    pub creator: Option<UserInfo>,
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

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建文档请求
pub struct CreateDocumentRequest {
    config: Config,
}

impl CreateDocumentRequest {
    /// 创建创建文档请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/create
    pub async fn execute(self, params: CreateDocumentParams) -> SDKResult<CreateDocumentResponse> {
        // 构建API端点URL
        let url = "/open-apis/docx/v1/documents";

        // 创建API请求
        let mut api_request: ApiRequest<CreateDocumentResponse> = ApiRequest::post(&url);

        // 设置请求体
        api_request = api_request.body(&params)?;

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
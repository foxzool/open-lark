/// 创建文档
///
/// 创建新版文档，文档标题和目录可选。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create
/// doc: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 创建文档请求
pub struct CreateDocumentRequest {
    config: Config,
}

/// 创建文档请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDocumentParams {
    /// 文档标题（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文件夹 token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

/// 创建文档响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentResponse {
    pub document: CreatedDocument,
}

/// 创建文档返回的文档信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedDocument {
    pub document_id: String,
    pub revision_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateDocumentRequest {
    /// 创建创建文档请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create
    /// doc: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create
    pub async fn execute(self, params: CreateDocumentParams) -> SDKResult<CreateDocumentResponse> {
        let api_endpoint = DocxApiV1::DocumentCreate;
        let api_request: ApiRequest<CreateDocumentResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建文档")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建文档")
    }
}

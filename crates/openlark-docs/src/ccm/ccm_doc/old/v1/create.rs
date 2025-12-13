/// 创建旧版文档
///
/// 创建并初始化文档。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/apiRef/create-document
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};

/// 创建旧版文档请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentParams {
    /// 文档标题，长度限制：1-100字符
    pub title: String,
    /// 父文件夹token，不填则存在"我的空间"
    #[serde(rename = "folder_token")]
    pub folder_token: Option<String>,
    /// 文档类型，可选值：doc、sheet、bitable、mindnote、file
    #[serde(rename = "parent_type")]
    pub parent_type: Option<String>,
}

/// 创建旧版文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentResponse {
    /// 文档信息
    pub data: Option<DocumentData>,
}

/// 文档数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentData {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
}

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建旧版文档请求
pub struct CreateDocumentRequest {
    config: Config,
}

impl CreateDocumentRequest {
    /// 创建创建旧版文档请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/apiRef/create-document
    pub async fn execute(self, params: CreateDocumentParams) -> SDKResult<CreateDocumentResponse> {
        // 验证必填字段
        validate_required_field("文档标题", Some(&params.title), "文档标题不能为空")?;

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDocApiOld::Create;

        // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
        let mut api_request: ApiRequest<CreateDocumentResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建文档")?);

        // 发送请求并提取响应数据
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建文档")
    }
}

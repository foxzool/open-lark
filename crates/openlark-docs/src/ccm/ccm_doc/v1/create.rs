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

use super::models::*;
use super::requests::CreateDocumentRequest;
use super::responses::CreatedDocument;
use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建旧版文档
///
/// 创建并初始化文档。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/apiRef/create-document
pub async fn create_document(
    request: CreateDocumentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<CreatedDocument> {
    // 验证必填字段
    validate_required_field("文档标题", Some(&request.title), "文档标题不能为空")?;

    // 转换请求为参数
    let params = CreateDocumentParams {
        title: request.title,
        folder_token: request.folder_token.clone(),
        parent_type: request.parent_type,
    };

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDocApiOld::Create;

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<CreateDocumentResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建文档")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, option).await?;

    // 从响应中提取文档数据
    let resp: CreateDocumentResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;
    let doc_data = resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("document_data", "Document data is missing")
    })?;

    // 转换 DocumentData 为 CreatedDocument
    Ok(CreatedDocument {
        doc_token: doc_data.doc_token,
        title: doc_data.title,
        doc_type: doc_data.doc_type,
        folder_token: request.folder_token.clone(),
    })
}

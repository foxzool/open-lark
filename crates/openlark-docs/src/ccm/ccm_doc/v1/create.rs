//! 创建旧版文档
//!
//! 创建并初始化文档。
//! docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/apiRef/create-document

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};
use super::models::*;

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
    config: &Config,
    params: CreateDocumentParams,
) -> SDKResult<CreateDocumentResponse> {
    // 验证必填字段
    validate_required_field("文档标题", Some(&params.title), "文档标题不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDocApiOld::Create;

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<CreateDocumentResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&params, "创建文档")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建文档")
}
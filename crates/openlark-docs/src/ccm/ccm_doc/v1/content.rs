//! 获取旧版文档富文本内容
//!
//! 获取结构化的文档内容。
//! docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};
use super::models::*;

impl ApiResponseTrait for DocumentContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档富文本内容
///
/// 获取结构化的文档内容。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document
pub async fn get_document_content(
    config: &Config,
    doc_token: &str,
) -> SDKResult<DocumentContentResponse> {
    // 验证必填字段
    validate_required_field("文档Token", Some(doc_token), "文档Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDocApiOld::Content(doc_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DocumentContentResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取文档富文本内容")
}
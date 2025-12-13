/// 获取旧版文档富文本内容
///
/// 获取结构化的文档内容。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/get-document
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::*;
use super::requests::DocumentContentRequest;
use super::responses::DocumentContentData;
use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};

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
    request: DocumentContentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<super::responses::DocumentContentData> {
    // 验证必填字段
    validate_required_field("文档Token", Some(&request.document_token), "文档Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let document_token = request.document_token.clone();
    let api_endpoint = CcmDocApiOld::Content(document_token);

    // 创建API请求
    let api_request: ApiRequest<DocumentContentResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, option).await?;

    // 提取响应包装器
    let resp: DocumentContentResponse = extract_response_data(response, "获取文档富文本内容")?;

    // 提取内部数据并转换为responses::DocumentContentData格式
    let data = resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("document_content", "文档富文本内容数据为空")
    })?;

    // 计算元素数量（从content中推断）
    let element_count = if let Some(arr) = data.content.as_array() {
        arr.len() as i32
    } else {
        0
    };

    // 转换为responses::DocumentContentData格式（包含额外的字段）
    Ok(super::responses::DocumentContentData {
        content: data.content,
        doc_token: request.document_token,
        element_count,
        revision: 1,  // 默认版本号
    })
}

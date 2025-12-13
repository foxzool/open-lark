/// 获取旧版文档纯文本内容
///
/// 获取文档的纯文本内容，不包含富文本格式信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/obtain-document-content
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::*;
use super::requests::RawContentRequest;
use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};

impl ApiResponseTrait for RawContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档纯文本内容
///
/// 获取文档的纯文本内容，不包含富文本格式信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/obtain-document-content
pub async fn get_raw_content(
    request: RawContentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<super::responses::RawContentData> {
    // 验证必填字段
    validate_required_field("文档Token", Some(&request.document_token), "文档Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let document_token = request.document_token.clone();
    let api_endpoint = CcmDocApiOld::RawContent(document_token);

    // 创建API请求
    let api_request: ApiRequest<RawContentResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, option).await?;

    // 提取响应包装器
    let resp: RawContentResponse = extract_response_data(response, "获取文档纯文本内容")?;

    // 提取内部数据并转换为responses::RawContentData格式
    let data = resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("raw_content", "文档纯文本内容数据为空")
    })?;

    // 转换为responses::RawContentData格式（包含额外的字段）
    let content = data.content.clone();
    let length = content.len() as i32;
    Ok(super::responses::RawContentData {
        content,
        doc_token: request.document_token,
        length,
    })
}

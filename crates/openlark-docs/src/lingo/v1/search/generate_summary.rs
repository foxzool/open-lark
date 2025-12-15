/// 生成文本摘要
///
/// 使用Lingo语言服务生成文本摘要。
/// docPath: https://open.feishu.cn/document/lingo-v1/text/generateSummary
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};
use crate::lingo::v1::SummaryResult;

#[derive(Debug, serde::Deserialize)]
pub struct GenerateSummaryResponse {
    pub data: Option<SummaryResult>,
}

#[derive(Debug, serde::Serialize)]
pub struct GenerateSummaryParams {
    pub text: String,
    pub max_length: Option<usize>,
    pub language: Option<String>,
}

impl ApiResponseTrait for GenerateSummaryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 生成文本摘要
///
/// 使用Lingo语言服务生成文本摘要。
/// docPath: https://open.feishu.cn/document/lingo-v1/text/generateSummary
pub async fn generate_text_summary(
    config: &Config,
    params: GenerateSummaryParams,
) -> SDKResult<SummaryResult> {
    // 验证必填字段
    validate_required_field("文本内容", Some(&params.text), "文本内容不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::GenerateSummary;

    // 创建API请求
    let api_request: ApiRequest<GenerateSummaryResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "生成文本摘要")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: GenerateSummaryResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("summary_data", "Summary data is missing")
    })
}
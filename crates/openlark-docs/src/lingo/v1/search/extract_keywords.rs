/// 提取关键词
///
/// 使用Lingo语言服务提取文本关键词。
/// docPath: https://open.feishu.cn/document/lingo-v1/text/extractKeywords
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};
use crate::lingo::v1::KeywordsResult;

#[derive(Debug, serde::Deserialize)]
pub struct ExtractKeywordsResponse {
    pub data: Option<KeywordsResult>,
}

#[derive(Debug, serde::Serialize)]
pub struct ExtractKeywordsParams {
    pub text: String,
    pub max_keywords: Option<usize>,
    pub min_weight: Option<f64>,
}

impl ApiResponseTrait for ExtractKeywordsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 提取关键词
///
/// 使用Lingo语言服务提取文本关键词。
/// docPath: https://open.feishu.cn/document/lingo-v1/text/extractKeywords
pub async fn extract_text_keywords(
    config: &Config,
    params: ExtractKeywordsParams,
) -> SDKResult<KeywordsResult> {
    // 验证必填字段
    validate_required_field("文本内容", Some(&params.text), "文本内容不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::ExtractKeywords;

    // 创建API请求
    let api_request: ApiRequest<ExtractKeywordsResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "提取关键词")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: ExtractKeywordsResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("keywords_data", "Keywords data is missing")
    })
}
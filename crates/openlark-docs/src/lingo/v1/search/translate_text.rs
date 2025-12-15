/// 翻译文本
///
/// 使用Lingo语言服务翻译文本。
/// docPath: https://open.feishu.cn/document/lingo-v1/text/translate
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};
use crate::lingo::v1::TranslationResult;

#[derive(Debug, serde::Deserialize)]
pub struct TranslateTextResponse {
    pub data: Option<TranslationResult>,
}

#[derive(Debug, serde::Serialize)]
pub struct TranslateTextParams {
    pub text: String,
    pub source_language: String,
    pub target_language: String,
}

impl ApiResponseTrait for TranslateTextResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 翻译文本
///
/// 使用Lingo语言服务翻译文本。
/// docPath: https://open.feishu.cn/document/lingo-v1/text/translate
pub async fn translate_text(
    config: &Config,
    params: TranslateTextParams,
) -> SDKResult<TranslationResult> {
    // 验证必填字段
    validate_required_field("待翻译文本", Some(&params.text), "待翻译文本不能为空")?;
    validate_required_field("源语言", Some(&params.source_language), "源语言不能为空")?;
    validate_required_field("目标语言", Some(&params.target_language), "目标语言不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::TranslateText;

    // 创建API请求
    let api_request: ApiRequest<TranslateTextResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "翻译文本")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: TranslateTextResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("translation_data", "Translation data is missing")
    })
}
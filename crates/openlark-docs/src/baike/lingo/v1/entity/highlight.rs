/// 词条高亮
///
/// 传入一句话，智能识别句中对应的词条，并返回词条位置和 entity_id，可在外部系统中快速实现词条智能高亮。
/// docPath: https://open.feishu.cn/document/lingo-v1/entity/highlight
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct HighlightEntityResponse {
    pub data: Option<HighlightData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct HighlightData {
    pub highlights: Vec<EntityHighlight>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityHighlight {
    pub entity_id: String,
    pub title: String,
    pub start_pos: i32,
    pub end_pos: i32,
    pub matched_text: String,
}

#[derive(Debug, serde::Serialize)]
pub struct HighlightEntityParams {
    pub text: String,
    pub repo_id: Option<String>,
    pub max_highlights: Option<i32>,
}

impl ApiResponseTrait for HighlightEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条高亮
///
/// 传入一句话，智能识别句中对应的词条，并返回词条位置和 entity_id，可在外部系统中快速实现词条智能高亮。
/// docPath: https://open.feishu.cn/document/lingo-v1/entity/highlight
pub async fn highlight_entity(
    config: &Config,
    params: HighlightEntityParams,
) -> SDKResult<Vec<EntityHighlight>> {
    // 验证必填字段
    validate_required_field("文本", Some(&params.text), "文本不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::EntityHighlight;

    // 创建API请求
    let api_request: ApiRequest<HighlightEntityResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "词条高亮")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: HighlightEntityResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.map(|data| data.highlights).ok_or_else(|| {
        openlark_core::error::validation_error("highlight_data", "Highlight data is missing")
    })
}
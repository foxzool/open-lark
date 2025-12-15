/// 提取潜在的词条
///
/// 提取文本中可能成为词条的词语，且不会过滤已经成为词条的词语。同时返回推荐的别名。
/// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/extract
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::BaikeApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct ExtractEntityResponse {
    pub data: Option<ExtractData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ExtractData {
    pub entities: Vec<ExtractedEntity>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ExtractedEntity {
    pub text: String,
    pub start_pos: i32,
    pub end_pos: i32,
    pub confidence_score: f64,
    pub suggested_aliases: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct ExtractEntityParams {
    pub text: String,
    pub space_id: Option<String>,
    pub min_confidence: Option<f64>,
    pub max_entities: Option<i32>,
}

impl ApiResponseTrait for ExtractEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 提取潜在的词条
///
/// 提取文本中可能成为词条的词语，且不会过滤已经成为词条的词语。同时返回推荐的别名。
/// docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/extract
pub async fn extract_entity(
    config: &Config,
    params: ExtractEntityParams,
) -> SDKResult<Vec<ExtractedEntity>> {
    // 验证必填字段
    validate_required_field("文本", Some(&params.text), "文本不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = BaikeApiV1::EntityExtract;

    // 创建API请求
    let api_request: ApiRequest<ExtractEntityResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "提取潜在的词条")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: ExtractEntityResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.map(|data| data.entities).ok_or_else(|| {
        openlark_core::error::validation_error("extract_data", "Extract data is missing")
    })
}
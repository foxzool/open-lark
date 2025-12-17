/// 精准搜索词条
///
/// 将关键词与词条名、别名精准匹配，并返回对应的词条 ID。
/// docPath: https://open.feishu.cn/document/lingo-v1/entity/match
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct MatchEntityResponse {
    pub data: Option<MatchData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MatchData {
    pub matches: Vec<EntityMatch>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityMatch {
    pub entity_id: String,
    pub title: String,
    pub aliases: Vec<String>,
    pub match_score: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct MatchEntityParams {
    pub keyword: String,
    pub repo_id: Option<String>,
    pub page_size: Option<i32>,
    pub page_token: Option<String>,
}

impl ApiResponseTrait for MatchEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 精准搜索词条
///
/// 将关键词与词条名、别名精准匹配，并返回对应的词条 ID。
/// docPath: https://open.feishu.cn/document/lingo-v1/entity/match
pub async fn match_entity(
    config: &Config,
    params: MatchEntityParams,
) -> SDKResult<Vec<EntityMatch>> {
    // 验证必填字段
    validate_required_field("关键词", Some(&params.keyword), "关键词不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::EntityMatch;

    // 创建API请求
    let api_request: ApiRequest<MatchEntityResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "精准搜索词条")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: MatchEntityResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.map(|data| data.matches).ok_or_else(|| {
        openlark_core::error::validation_error("match_data", "Match data is missing")
    })
}
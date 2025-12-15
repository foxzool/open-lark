/// 创建草稿
///
/// 创建知识库草稿。
/// docPath: https://open.feishu.cn/document/baike-v1/draft/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::BaikeApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct CreateDraftResponse {
    pub data: Option<DraftData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DraftData {
    pub draft_id: String,
    pub title: String,
    pub content: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateDraftParams {
    pub title: String,
    pub content: Option<String>,
    pub space_id: String,
}

impl ApiResponseTrait for CreateDraftResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建草稿
///
/// 创建知识库草稿。
/// docPath: https://open.feishu.cn/document/baike-v1/draft/create
pub async fn create_draft(
    config: &Config,
    params: CreateDraftParams,
) -> SDKResult<DraftData> {
    // 验证必填字段
    validate_required_field("草稿标题", Some(&params.title), "草稿标题不能为空")?;
    validate_required_field("知识库ID", Some(&params.space_id), "知识库ID不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = BaikeApiV1::DraftCreate;

    // 创建API请求
    let api_request: ApiRequest<CreateDraftResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建草稿")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: CreateDraftResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("draft_data", "Draft data is missing")
    })
}
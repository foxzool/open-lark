/// 创建Lingo草稿
///
/// 创建Lingo语言服务草稿。
/// docPath: https://open.feishu.cn/document/lingo-v1/draft/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};
use crate::lingo::v1::{DraftStatus, LingoDraft};

#[derive(Debug, serde::Deserialize)]
pub struct CreateLingoDraftResponse {
    pub data: Option<LingoDraft>,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateLingoDraftParams {
    pub title: String,
    pub content: String,
    pub language: Option<String>,
}

impl ApiResponseTrait for CreateLingoDraftResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建Lingo草稿
///
/// 创建Lingo语言服务草稿。
/// docPath: https://open.feishu.cn/document/lingo-v1/draft/create
pub async fn create_lingo_draft(
    config: &Config,
    params: CreateLingoDraftParams,
) -> SDKResult<LingoDraft> {
    // 验证必填字段
    validate_required_field("草稿标题", Some(&params.title), "草稿标题不能为空")?;
    validate_required_field("草稿内容", Some(&params.content), "草稿内容不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::DraftCreate;

    // 创建API请求
    let api_request: ApiRequest<CreateLingoDraftResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建Lingo草稿")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: CreateLingoDraftResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("draft_data", "Draft data is missing")
    })
}

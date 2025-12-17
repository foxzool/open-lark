/// 更新Lingo草稿
///
/// 更新Lingo语言服务草稿内容。
/// docPath: https://open.feishu.cn/document/lingo-v1/draft/update
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};
use crate::lingo::v1::LingoDraft;

#[derive(Debug, serde::Deserialize)]
pub struct UpdateLingoDraftResponse {
    pub data: Option<LingoDraft>,
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateLingoDraftParams {
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: Option<String>,
}

impl ApiResponseTrait for UpdateLingoDraftResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新Lingo草稿
///
/// 更新Lingo语言服务草稿内容。
/// docPath: https://open.feishu.cn/document/lingo-v1/draft/update
pub async fn update_lingo_draft(
    draft_id: &str,
    config: &Config,
    params: UpdateLingoDraftParams,
) -> SDKResult<LingoDraft> {
    // 验证必填字段
    validate_required_field("草稿ID", Some(draft_id), "草稿ID不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::DraftUpdate(draft_id.to_string());

    // 创建API请求
    let api_request: ApiRequest<UpdateLingoDraftResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&params, "更新Lingo草稿")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: UpdateLingoDraftResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("draft_data", "Draft data is missing")
    })
}
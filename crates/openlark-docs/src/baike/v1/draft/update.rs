/// 更新草稿
///
/// 更新知识库草稿内容。
/// docPath: https://open.feishu.cn/document/baike-v1/draft/update
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::BaikeApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct UpdateDraftResponse {
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
pub struct UpdateDraftParams {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl ApiResponseTrait for UpdateDraftResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新草稿
///
/// 更新知识库草稿内容。
/// docPath: https://open.feishu.cn/document/baike-v1/draft/update
pub async fn update_draft(
    draft_id: &str,
    config: &Config,
    params: UpdateDraftParams,
) -> SDKResult<DraftData> {
    // 验证必填字段
    validate_required_field("草稿ID", Some(draft_id), "草稿ID不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = BaikeApiV1::DraftUpdate(draft_id.to_string());

    // 创建API请求
    let api_request: ApiRequest<UpdateDraftResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&params, "更新草稿")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: UpdateDraftResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("draft_data", "Draft data is missing")
    })
}
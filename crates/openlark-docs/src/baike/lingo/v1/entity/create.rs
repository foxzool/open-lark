/// 创建Lingo词条
///
/// 创建Lingo语言服务词条。
/// docPath: https://open.feishu.cn/document/lingo-v1/entity/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};
use crate::lingo::v1::LingoEntity;

#[derive(Debug, serde::Deserialize)]
pub struct CreateLingoEntityResponse {
    pub data: Option<LingoEntity>,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateLingoEntityParams {
    pub title: String,
    pub content: String,
    pub entity_type: String,
    pub tags: Option<Vec<String>>,
    pub language: Option<String>,
}

impl ApiResponseTrait for CreateLingoEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建Lingo词条
///
/// 创建Lingo语言服务词条。
/// docPath: https://open.feishu.cn/document/lingo-v1/entity/create
pub async fn create_lingo_entity(
    config: &Config,
    params: CreateLingoEntityParams,
) -> SDKResult<LingoEntity> {
    // 验证必填字段
    validate_required_field("词条标题", Some(&params.title), "词条标题不能为空")?;
    validate_required_field("词条内容", Some(&params.content), "词条内容不能为空")?;
    validate_required_field("词条类型", Some(&params.entity_type), "词条类型不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::EntityCreate;

    // 创建API请求
    let api_request: ApiRequest<CreateLingoEntityResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建Lingo词条")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: CreateLingoEntityResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("entity_data", "Entity data is missing")
    })
}

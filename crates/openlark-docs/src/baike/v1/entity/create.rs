/// 创建词条
///
/// 创建知识库词条。
/// docPath: https://open.feishu.cn/document/baike-v1/entity/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::BaikeApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct CreateEntityResponse {
    pub data: Option<EntityData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityData {
    pub entity_id: String,
    pub title: String,
    pub content: String,
    pub space_id: String,
    pub status: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateEntityParams {
    pub title: String,
    pub content: String,
    pub space_id: String,
    pub tags: Option<Vec<String>>,
}

impl ApiResponseTrait for CreateEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建词条
///
/// 创建知识库词条。
/// docPath: https://open.feishu.cn/document/baike-v1/entity/create
pub async fn create_entity(
    config: &Config,
    params: CreateEntityParams,
) -> SDKResult<EntityData> {
    // 验证必填字段
    validate_required_field("词条标题", Some(&params.title), "词条标题不能为空")?;
    validate_required_field("词条内容", Some(&params.content), "词条内容不能为空")?;
    validate_required_field("知识库ID", Some(&params.space_id), "知识库ID不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = BaikeApiV1::EntityCreate;

    // 创建API请求
    let api_request: ApiRequest<CreateEntityResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建词条")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: CreateEntityResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("entity_data", "Entity data is missing")
    })
}
/// 删除免审词条
///
/// 删除指定的词条。
/// docPath: https://open.feishu.cn/document/lingo-v1/entity/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct DeleteLingoEntityResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteLingoEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除Lingo词条
pub async fn delete_lingo_entity(
    config: &Config,
    entity_id: &str,
) -> SDKResult<()> {
    // 验证必填字段
    validate_required_field("词条ID", Some(entity_id), "词条ID不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::EntityDelete(entity_id.to_string());

    // 创建API请求
    let api_request: ApiRequest<DeleteLingoEntityResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    // 发送请求
    let _response: openlark_core::api::Response<DeleteLingoEntityResponse> =
        Transport::request(api_request, config, None).await?;

    Ok(())
}

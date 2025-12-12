//! 获取元数据
//!
//! 根据 token 获取各类文件的元数据。
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDocsApiOld, api_utils::*};
use super::super::models::*;

impl ApiResponseTrait for GetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取元数据
///
/// 根据 token 获取各类文件的元数据。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata
pub async fn get_meta(
    config: &Config,
    params: GetMetaParams,
) -> SDKResult<GetMetaResponse> {
    // 验证必填字段
    if params.obj_tokens.is_empty() {
        return Err(openlark_core::error::CoreError::validation(
            "obj_tokens",
            "文档token列表不能为空"
        ));
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDocsApiOld::Meta;

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let mut api_request: ApiRequest<GetMetaResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&params, "获取元数据")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取元数据")
}
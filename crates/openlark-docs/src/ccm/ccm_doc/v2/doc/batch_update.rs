/// 编辑旧版文档内容
///
/// 批量编辑更新文档内容，包括更新标题、范围删除、插入内容。
/// docPath: https://open.feishu.cn/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::super::models::*;
use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};

impl ApiResponseTrait for BatchUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 编辑旧版文档内容
///
/// 批量编辑更新文档内容，包括更新标题、范围删除、插入内容。
/// docPath: https://open.feishu.cn/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN
pub async fn batch_update_document(
    config: &Config,
    doc_token: &str,
    params: BatchUpdateParams,
) -> SDKResult<BatchUpdateResponse> {
    // 验证必填字段
    validate_required_field("文档Token", Some(doc_token), "文档Token不能为空")?;

    if params.operations.is_empty() {
        return Err(openlark_core::error::CoreError::validation(
            "operations",
            "操作列表不能为空",
        ));
    }

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDocApiOld::BatchUpdate(doc_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<BatchUpdateResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "批量更新文档")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "批量更新文档")
}

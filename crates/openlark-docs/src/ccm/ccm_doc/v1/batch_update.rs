/// 编辑旧版文档内容
///
/// 批量编辑更新文档内容，包括更新标题、范围删除、插入内容。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/batch-update-document
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::*;
use super::requests::{BatchUpdateRequest, DocumentOperation};
use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};

impl ApiResponseTrait for BatchUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 编辑旧版文档内容
///
/// 批量编辑更新文档内容，包括更新标题、范围删除、插入内容。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/batch-update-document
pub async fn batch_update_document(
    request: BatchUpdateRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<super::responses::BatchUpdateData> {
    // 验证必填字段
    validate_required_field("文档Token", Some(&request.document_token), "文档Token不能为空")?;

    if request.operations.is_empty() {
        return Err(openlark_core::error::CoreError::validation(
            "operations",
            "操作列表不能为空",
        ));
    }

    // 转换为参数
    let params = BatchUpdateParams {
        operations: request.operations,
    };

    // 使用enum+builder系统生成API端点
    let document_token = request.document_token.clone();
    let api_endpoint = CcmDocApiOld::BatchUpdate(document_token);

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let mut api_request: ApiRequest<BatchUpdateResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "批量更新文档")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, option).await?;

    // 提取响应包装器
    let resp: BatchUpdateResponse = extract_response_data(response, "批量更新文档")?;

    // 提取内部数据并转换为responses::BatchUpdateData格式
    let data = resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("batch_update", "批量更新响应数据为空")
    })?;

    // 转换为responses::BatchUpdateData格式（包含额外的字段）
    Ok(super::responses::BatchUpdateData {
        success: data.success,
        doc_token: Some(request.document_token),
        error: data.error,
        revision: Some(1),  // 默认版本号
    })
}

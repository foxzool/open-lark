//! 获取旧版文档中的电子表格元数据
//!
//! 根据 docToken 获取文档中的电子表格的元数据。
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};
use super::models::*;

impl ApiResponseTrait for SheetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档中的电子表格元数据
///
/// 根据 docToken 获取文档中的电子表格的元数据。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc
pub async fn get_sheet_meta(
    config: &Config,
    doc_token: &str,
) -> SDKResult<SheetMetaResponse> {
    // 验证必填字段
    validate_required_field("文档Token", Some(doc_token), "文档Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmDocApiOld::SheetMeta(doc_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<SheetMetaResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取电子表格元信息")
}
/// 获取旧版文档中的电子表格元数据
///
/// 根据 docToken 获取文档中的电子表格的元数据。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::*;
use super::requests::SheetMetaRequest;
use super::responses::SheetMetaData;
use crate::common::{api_endpoints::CcmDocApiOld, api_utils::*};

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
    request: SheetMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<super::responses::SheetMetaData> {
    // 验证必填字段
    validate_required_field("电子表格Token", Some(&request.spreadsheet_token), "电子表格Token不能为空")?;

    // 使用enum+builder系统生成API端点
    let spreadsheet_token = request.spreadsheet_token.clone();
    let api_endpoint = CcmDocApiOld::SheetMeta(spreadsheet_token);

    // 创建API请求
    let api_request: ApiRequest<SheetMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, option).await?;

    // 提取响应包装器
    let resp: SheetMetaResponse = extract_response_data(response, "获取电子表格元信息")?;

    // 提取内部数据并转换为responses::SheetMetaData格式
    let meta = resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("sheet_meta", "电子表格元信息数据为空")
    })?;

    // 转换为responses::SheetMetaData格式（包含额外的字段）
    let sheets: Vec<super::responses::SheetInfo> = meta.sheets.into_iter().map(|s| {
        super::responses::SheetInfo {
            sheet_id: s.sheet_id,
            title: s.title,
            index: s.index,
            sheet_type: "sheet".to_string(),  // 默认类型
            row_count: 0,  // SheetInfo中没有这些字段
            column_count: 0,
        }
    }).collect();

    let sheet_count = sheets.len() as i32;

    Ok(super::responses::SheetMetaData {
        sheets,
        spreadsheet_token: request.spreadsheet_token,
        title: "电子表格".to_string(),  // 默认标题
        sheet_count,
    })
}

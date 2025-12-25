//! 操作工作表
//!
//! docPath: /document/ukTMukTMukTM/uYTMzUjL2EzM14iNxMTN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/operate-sheets

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetsBatchUpdateReq {
    pub requests: Vec<RequestItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RequestItem {
    pub addSheet: Option<AddSheetProperty>,
    pub updateSheet: Option<UpdateSheetProperty>,
    pub deleteSheet: Option<DeleteSheetProperty>,
    pub copySheet: Option<CopySheetProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddSheetProperty {
    pub properties: Option<SheetProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetProperty {
    pub properties: Option<SheetProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteSheetProperty {
    pub sheetId: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopySheetProperty {
    pub sourceSheetId: String,
    pub destinationSheetId: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetProperty {
    pub sheetId: Option<String>,
    pub title: Option<String>,
    pub index: Option<i32>,
    pub hidden: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetsBatchUpdateResponse {
    pub replies: Vec<ReplyItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ReplyItem {
    pub addSheet: Option<SheetProperty>,
    pub updateSheet: Option<SheetProperty>,
    pub deleteSheet: Option<DeleteSheetResult>,
    pub copySheet: Option<SheetProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteSheetResult {
    pub result: bool,
    pub sheetId: String,
}

impl ApiResponseTrait for SheetsBatchUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 操作工作表（对应 CSV 的“操作工作表”）
pub async fn operate_sheets(
    request: SheetsBatchUpdateReq,
    spreadsheet_token: String,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<SheetsBatchUpdateResponse> {
    use crate::common::api_endpoints::CcmSheetApiOld;

    let api_endpoint = CcmSheetApiOld::OperateSheets(spreadsheet_token);
    let mut api_request: ApiRequest<SheetsBatchUpdateResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request, "操作工作表")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "操作工作表")
}

// “更新工作表属性” 单独放在 `update_sheet_properties.rs`，用于满足 API 数量统计与语义区分。

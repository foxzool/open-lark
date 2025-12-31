//! 操作工作表
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/operate-sheets

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
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
    pub deleteSheet: Option<DeleteSheetProperty>,
    pub copySheet: Option<CopySheetProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddSheetProperty {
    pub properties: AddSheetProperties,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteSheetProperty {
    pub sheetId: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopySheetProperty {
    pub source: CopySheetSource,
    pub destination: CopySheetDestination,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopySheetSource {
    pub sheetId: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopySheetDestination {
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddSheetProperties {
    pub title: String,
    pub index: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetProperty {
    pub sheetId: Option<String>,
    pub title: Option<String>,
    pub index: Option<i32>,
    pub hidden: Option<bool>,
    pub frozenRowCount: Option<i32>,
    pub frozenColCount: Option<i32>,
    pub protect: Option<SheetProtect>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetProtect {
    pub lock: String,
    pub lockInfo: Option<String>,
    pub userIDs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetsBatchUpdateResponse {
    #[serde(default)]
    pub replies: Vec<ReplyItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ReplyItem {
    pub addSheet: Option<SheetPropertiesReply>,
    pub deleteSheet: Option<DeleteSheetResult>,
    pub copySheet: Option<SheetPropertiesReply>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetPropertiesReply {
    pub properties: SheetProperty,
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
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.requests, "requests 不能为空");
    for (idx, item) in request.requests.iter().enumerate() {
        let op_cnt = [
            item.addSheet.is_some(),
            item.deleteSheet.is_some(),
            item.copySheet.is_some(),
        ]
        .iter()
        .filter(|v| **v)
        .count();
        if op_cnt != 1 {
            return Err(openlark_core::error::validation_error(
                &format!("requests[{}]", idx),
                "每个 request 必须且只能包含一种操作（addSheet/deleteSheet/copySheet）",
            ));
        }
        if let Some(add_sheet) = &item.addSheet {
            validate_required!(add_sheet.properties.title, "title 不能为空");
        }
        if let Some(delete_sheet) = &item.deleteSheet {
            validate_required!(delete_sheet.sheetId, "sheetId 不能为空");
        }
        if let Some(copy_sheet) = &item.copySheet {
            validate_required!(copy_sheet.source.sheetId, "source.sheetId 不能为空");
        }
    }

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

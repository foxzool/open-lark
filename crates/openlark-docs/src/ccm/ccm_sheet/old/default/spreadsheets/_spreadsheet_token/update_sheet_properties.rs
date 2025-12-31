//! 更新工作表属性
//!
//! docPath: /document/ukTMukTMukTM/ugjMzUjL4IzM14COyMTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;
use crate::common::api_utils::*;

use super::sheets_batch_update::{SheetProperty, SheetProtect};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetPropertiesReq {
    /// 用户 ID 类型（query 参数）。仅当使用 protect.userIDs 时需要传。
    pub user_id_type: Option<String>,
    /// 更新工作表属性请求列表
    pub requests: Vec<UpdateSheetRequestItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetRequestItem {
    pub updateSheet: UpdateSheetOperation,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetOperation {
    pub properties: UpdateSheetProperties,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetProperties {
    pub sheetId: String,
    pub title: Option<String>,
    pub index: Option<i32>,
    pub hidden: Option<bool>,
    pub frozenRowCount: Option<i32>,
    pub frozenColCount: Option<i32>,
    pub protect: Option<SheetProtect>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct UpdateSheetPropertiesBody {
    pub requests: Vec<UpdateSheetRequestItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetPropertiesResponse {
    #[serde(default)]
    pub replies: Vec<UpdateSheetReplyItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetReplyItem {
    pub updateSheet: Option<UpdateSheetReply>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSheetReply {
    pub properties: SheetProperty,
}

impl ApiResponseTrait for UpdateSheetPropertiesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新工作表属性
pub async fn update_sheet_properties(
    spreadsheet_token: String,
    request: UpdateSheetPropertiesReq,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<UpdateSheetPropertiesResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.requests, "requests 不能为空");

    let UpdateSheetPropertiesReq {
        user_id_type,
        requests,
    } = request;

    if let Some(user_id_type) = user_id_type.as_deref() {
        if user_id_type != "open_id" && user_id_type != "union_id" {
            return Err(openlark_core::error::validation_error(
                "user_id_type",
                "user_id_type 必须为 open_id 或 union_id",
            ));
        }
    }

    let mut need_user_id_type = false;
    for (idx, item) in requests.iter().enumerate() {
        validate_required!(
            item.updateSheet.properties.sheetId,
            &format!("requests[{}].updateSheet.properties.sheetId 不能为空", idx)
        );

        if let Some(title) = item.updateSheet.properties.title.as_deref() {
            if title.chars().count() > 100 {
                return Err(openlark_core::error::validation_error(
                    "title",
                    "title 长度不能超过 100 个字符",
                ));
            }
            // 文档限制：不包含 / \\ ? * [ ] :
            if title.contains('/')
                || title.contains('\\')
                || title.contains('?')
                || title.contains('*')
                || title.contains('[')
                || title.contains(']')
                || title.contains(':')
            {
                return Err(openlark_core::error::validation_error(
                    "title",
                    "title 不能包含特殊字符：/ \\\\ ? * [ ] :",
                ));
            }
        }

        if let Some(protect) = &item.updateSheet.properties.protect {
            validate_required!(protect.lock, "protect.lock 不能为空");
            if protect.lock != "LOCK" && protect.lock != "UNLOCK" {
                return Err(openlark_core::error::validation_error(
                    "protect.lock",
                    "protect.lock 必须为 LOCK 或 UNLOCK",
                ));
            }

            if protect
                .userIDs
                .as_ref()
                .is_some_and(|ids| !ids.is_empty())
            {
                need_user_id_type = true;
            }
        }
    }

    if need_user_id_type {
        validate_required!(
            user_id_type.clone().unwrap_or_default(),
            "当传 protect.userIDs 时，user_id_type 不能为空"
        );
    }

    let api_endpoint = CcmSheetApiOld::UpdateSheetProperties(spreadsheet_token);
    let body = UpdateSheetPropertiesBody { requests };
    let mut api_request: ApiRequest<UpdateSheetPropertiesResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&body, "更新工作表属性")?);

    if let Some(user_id_type) = user_id_type.as_deref() {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新工作表属性")
}

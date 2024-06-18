use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest, api_resp::BaseResponse, constants::AccessTokenType, req_option,
        SDKResult,
    },
    service::sheets::v2::spreadsheet_sheet::{OperateSheetResponse, OperateSheetsRequestElem},
};
use crate::service::sheets::v2::SpreadsheetSheetService;

#[derive(Serialize, Debug, Default)]
pub struct UpdateSheetPropertiesRequest {
    #[serde(skip)]
    api_request: ApiRequest,

    /// 用户 ID 类型。默认值为 open_id。可选值有：
    ///
    /// * open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
    ///   不同。了解更多：如何获取 Open ID
    ///
    /// * union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID
    ///   是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union
    ///   ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID
    #[serde(skip)]
    user_id_type: Option<String>,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 更新工作表属性的请求
    requests: Vec<OperateSheetsRequestElem>,
}

/// 工作表属性
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateSheetProperty {
    /// 要更新的工作表的 ID。调用获取工作表获取 ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 工作表的标题
    pub title: String,
    /// 工作表的位置
    pub index: Option<i32>,
    /// 是否要隐藏表格。默认值为 false
    pub hidden: Option<bool>,
    /// 要冻结的工作表的行数。小于或等于工作表的最大行数，0 表示取消冻结行
    #[serde(rename = "frozenColCount")]
    pub frozen_col_count: Option<i32>,
    /// 要冻结的工作表的列数。小于等于工作表的最大列数，0 表示取消冻结列
    #[serde(rename = "frozenRowCount")]
    pub frozen_row_count: Option<i32>,
    /// 是否要保护该工作表
    pub protect: Option<UpdateSheetPropertyProtect>,
}

/// 是否要保护该工作表
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSheetPropertyProtect {
    /// 是否要保护该工作表。可选值：
    ///
    /// LOCK：保护
    /// UNLOCK：取消保护
    pub lock: bool,
    /// 保护工作表的备注信息
    #[serde(rename = "lockInfo")]
    pub lock_info: Option<String>,
    /// 除了本人与所有者外，添加其他人员的 ID，为其它人员添加保护范围的编辑权限。ID 类型由查询参数
    /// user_id_type 决定。user_id_type 不为空时，该字段生效。
    #[serde(rename = "userIDs")]
    pub user_ids: Option<Vec<String>>,
}

impl UpdateSheetPropertiesRequest {
    pub fn builder() -> UpdateSheetPropertiesRequestBuilder {
        UpdateSheetPropertiesRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateSheetPropertiesRequestBuilder {
    request: UpdateSheetPropertiesRequest,
}

impl UpdateSheetPropertiesRequestBuilder {
    pub fn user_id_type(mut self, user_id_type: Option<String>) -> Self {
        self.request.user_id_type = user_id_type;
        self
    }

    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn add_request(mut self, update_property: UpdateSheetProperty) -> Self {
        self.request
            .requests
            .push(OperateSheetsRequestElem::UpdateSheet {
                properties: update_property,
            });
        self
    }

    pub fn build(mut self) -> UpdateSheetPropertiesRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl SpreadsheetSheetService {
    pub async fn update_sheet_properties(
        &self,
        request: UpdateSheetPropertiesRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<OperateSheetResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

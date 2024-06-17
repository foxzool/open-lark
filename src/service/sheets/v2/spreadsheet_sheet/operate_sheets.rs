use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        req_option, SDKResult,
    },
    service::sheets::v2::spreadsheet_sheet::SpreadsheetSheetService,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OperateSheetsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 支持增加、复制、和删除工作表。一次请求可以同时进行多个操作。
    requests: Vec<OperateSheetsRequestElem>,
}

#[derive(Serialize, Deserialize, Debug)]
enum OperateSheetsRequestElem {
    /// 增加工作表。
    #[serde(rename = "addSheet")]
    AddSheet {
        /// 工作表属性
        properties: AddSheetProperty,
    },
    /// 复制工作表。复制的新工作表位于源工作表索引位置之后。
    #[serde(rename = "copySheet")]
    CopySheet {
        /// 需要复制的工作表资源
        source: CopySheetSource,
        /// 新工作表的属性
        destination: CopySheetDestination,
    },
    /// 删除工作表。
    #[serde(rename = "deleteSheet")]
    DeleteSheet {
        /// 要删除的工作表的 ID。调用获取工作表获取 ID
        #[serde(rename = "sheetId")]
        sheet_id: String,
    },
}

/// 工作表属性
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AddSheetProperty {
    /// 新增工作表的标题
    pub title: String,
    /// 新增工作表的位置。不填默认在工作表的第 0 索引位置增加工作表。
    pub index: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CopySheet {
    /// 需要复制的工作表资源
    pub source: CopySheetSource,
    /// 新工作表的属性
    pub destination: CopySheetDestination,
}

/// 需要复制的工作表资源
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CopySheetSource {
    /// 源工作表的 ID。调用获取工作表获取 ID
    #[serde(rename = "sheetId")]
    sheet_id: String,
}

/// 新工作表的属性
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CopySheetDestination {
    /// 新工作表名称。不填默认为“源工作表名称”+“(副本_源工作表的 index 值)”，如 “Sheet1(副本_0)”。
    title: Option<String>,
}

/// 删除工作表。
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeleteSheet {
    /// 要删除的工作表的 ID。调用获取工作表获取 ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
}

impl OperateSheetsRequest {
    pub fn builder() -> OperateSheetsRequestBuilder {
        OperateSheetsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct OperateSheetsRequestBuilder {
    request: OperateSheetsRequest,
}

impl OperateSheetsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 增加工作表。
    pub fn add_sheet(mut self, title: impl ToString, index: Option<u32>) -> Self {
        self.request
            .requests
            .push(OperateSheetsRequestElem::AddSheet {
                properties: AddSheetProperty {
                    title: title.to_string(),
                    index,
                },
            });
        self
    }

    /// 复制工作表。复制的新工作表位于源工作表索引位置之后。
    pub fn copy_sheet(
        mut self,
        source: impl ToString,
        destination: Option<String>,
    ) -> Self {
        self.request
            .requests
            .push(OperateSheetsRequestElem::CopySheet {
                source: CopySheetSource {
                    sheet_id: source.to_string(),
                },
                destination: CopySheetDestination {
                    title: destination,
                },
            });
        self
    }

    /// 删除工作表。
    pub fn delete_sheet(mut self, sheet_id: impl ToString) -> Self {
        self.request
            .requests
            .push(OperateSheetsRequestElem::DeleteSheet {
                sheet_id: sheet_id.to_string(),
            });
        self
    }

    pub fn build(mut self) -> OperateSheetsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl SpreadsheetSheetService {
    /// 操作工作表
    /// 新增、复制、删除工作表。
    pub async fn operate(
        &self,
        request: OperateSheetsRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<OperateSheetResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

#[derive(Deserialize, Debug)]
pub struct OperateSheetResponse {
    pub replies: Vec<OperateSheetReply>,
}

impl ApiResponseTrait for OperateSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Deserialize, Debug)]
pub enum OperateSheetReply {
    /// 新增工作表的属性
    #[serde(rename = "addSheet")]
    AddSheet { properties: SheetResponse },
    /// 复制工作表的结果
    #[serde(rename = "copySheet")]
    CopySheet { properties: SheetResponse },
    /// 删除工作表的结果
    #[serde(rename = "deleteSheet")]
    DeleteSheet {
        /// 删除工作表是否成功
        result: bool,
        /// 被删除的工作表的 ID
        #[serde(rename = "sheetId")]
        sheet_id: String },
}

#[derive(Deserialize, Debug)]
/// 工作表的属性
pub struct SheetResponse {
    /// 工作表的 sheetId
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 工作表的标题
    pub title: String,
    /// 工作表的位置
    pub index: Option<u32>,
}

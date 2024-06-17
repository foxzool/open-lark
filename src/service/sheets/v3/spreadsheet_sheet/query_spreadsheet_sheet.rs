use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::spreadsheet_sheet::SpreadsheetSheetService,
};
use crate::core::constants::AccessTokenType;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct QuerySpreadsheetSheetRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
}

impl SpreadsheetSheetService {
    /// 获取工作表
    /// 根据电子表格 token 获取表格中所有工作表及其属性信息，包括工作表 ID、标题、索引位置、是否被隐藏等。
    pub async fn query(
        &self,
        request: QuerySpreadsheetSheetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QuerySpreadsheetSheetResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/query",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::GET;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

impl QuerySpreadsheetSheetRequest {
    pub fn builder() -> QuerySpreadsheetSheetRequestBuilder {
        QuerySpreadsheetSheetRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct QuerySpreadsheetSheetRequestBuilder {
    request: QuerySpreadsheetSheetRequest,
}

impl QuerySpreadsheetSheetRequestBuilder {
    /// 表格的token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn build(mut self) -> QuerySpreadsheetSheetRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct QuerySpreadsheetSheetResponse {
    pub sheets: Vec<Sheet>,
}

impl ApiResponseTrait for QuerySpreadsheetSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Deserialize, Debug)]
pub struct Sheet {
    /// 工作表 ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引位置，索引从 0 开始计数。
    pub index: u32,
    /// 工作表是否被隐藏
    ///
    /// * true：被隐藏
    /// * false：未被隐藏
    pub hidden: bool,
    /// 单元格属性，仅当 resource_type 为 sheet 即工作表类型为电子表格时返回
    pub grid_properties: Option<GridProperties>,
    /// 工作表类型
    ///
    /// * sheet：工作表
    /// * bitable：多维表格。详情参考多维表格概述
    /// * #UNSUPPORTED_TYPE：不支持的类型
    pub resource_type: String,
    /// 合并单元格的相关信息。没有合并单元格则不返回
    pub merges: Option<Vec<MergeRange>>,
}

/// 单元格属性，仅当 resource_type 为 sheet 即工作表类型为电子表格时返回
#[derive(Deserialize, Debug)]
pub struct GridProperties {
    /// 冻结的行数量
    pub frozen_row_count: i32,
    /// 冻结的列数量
    pub frozen_column_count: i32,
    /// 工作表的行数
    pub row_count: i32,
    /// 工作表的列数量
    pub column_count: i32,
}

/// 合并单元格的相关信息
#[derive(Deserialize, Debug)]
pub struct MergeRange {
    /// 起始行，从 0 开始计数
    pub start_row_index: i32,
    /// 结束行，从 0 开始计数
    pub end_row_index: i32,
    /// 起始列，从 0 开始计数
    pub start_column_index: i32,
    /// 结束列，从 0 开始计数
    pub end_column_index: i32,
}

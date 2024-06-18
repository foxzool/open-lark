use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        req_option, SDKResult,
    },
    service::sheets::v2::{
        data_operation::{CellStyle, SheetDataUpdates},
        SpreadsheetService,
    },
};

#[derive(Debug, Serialize, Default)]
pub struct BatchSetCellStyleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 设置单元格样式
    data: Vec<AppendStyle>,
}

#[derive(Debug, Serialize, Default)]
struct AppendStyle {
    ranges: String,
    style: CellStyle,
}

impl BatchSetCellStyleRequest {
    pub fn builder() -> BatchSetCellStyleRequestBuilder {
        BatchSetCellStyleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct BatchSetCellStyleRequestBuilder {
    request: BatchSetCellStyleRequest,
}

impl BatchSetCellStyleRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn add_style(mut self, ranges: impl ToString, style: CellStyle) -> Self {
        self.request.data.push(AppendStyle {
            ranges: ranges.to_string(),
            style,
        });
        self
    }

    pub fn build(mut self) -> BatchSetCellStyleRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct BatchSetCellStyleResponse {
    #[serde(rename = "spreadsheetToken")]
    pub spreed_sheet_token: String,
    #[serde(rename = "totalUpdatedRows")]
    pub total_updated_rows: i32,
    #[serde(rename = "totalUpdatedColumns")]
    pub total_updated_columns: i32,
    #[serde(rename = "totalUpdatedCells")]
    pub total_updated_cells: i32,
    /// sheet 的版本号
    pub revision: i32,
    /// 各个范围的设置单元格样式的范围、行列数等
    pub responses: Vec<SheetDataUpdates>,
}

impl ApiResponseTrait for BatchSetCellStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 该接口用于根据 spreadsheetToken 、range和样式信息
    /// 批量更新单元格样式；单次写入不超过5000行，100列。建议在设置边框样式时，
    /// 每次更新的单元格数量不要超过30000个。一个区域被多个range覆盖时，仅最后一个样式会被应用。
    pub async fn batch_set_cell_style(
        &self,
        request: BatchSetCellStyleRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<BatchSetCellStyleResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/styles_batch_update",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::PUT;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

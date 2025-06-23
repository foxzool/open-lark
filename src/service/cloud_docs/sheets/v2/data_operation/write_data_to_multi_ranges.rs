use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        req_option, SDKResult,
    },
    service::sheets::v2::{data_operation::ValueRangeRequest, SpreadsheetService},
};

/// 向多个范围写入数据请求
#[derive(Serialize, Debug, Default)]
pub struct WriteDataToMultiRangesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    #[serde(rename = "valueRanges")]
    value_ranges: Vec<ValueRangeRequest>,
}

impl WriteDataToMultiRangesRequest {
    pub fn builder() -> WriteDataToMultiRangesBuilder {
        WriteDataToMultiRangesBuilder::default()
    }
}

#[derive(Default)]
pub struct WriteDataToMultiRangesBuilder {
    request: WriteDataToMultiRangesRequest,
}

impl WriteDataToMultiRangesBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn add_value_range(mut self, range: impl ToString, values: serde_json::Value) -> Self {
        self.request.value_ranges.push(ValueRangeRequest {
            range: range.to_string(),
            values,
        });
        self
    }

    pub fn build(mut self) -> WriteDataToMultiRangesRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }

    /// 直接执行向多个范围写入数据操作
    pub async fn execute(
        self,
        service: &SpreadsheetService,
    ) -> SDKResult<BaseResponse<WriteDataToMultiRangesResponse>> {
        service.write_data_multi_ranges(self.build(), None).await
    }

    /// 直接执行向多个范围写入数据操作并提供自定义选项
    pub async fn execute_with_options(
        self,
        service: &SpreadsheetService,
        option: req_option::RequestOption,
    ) -> SDKResult<BaseResponse<WriteDataToMultiRangesResponse>> {
        service
            .write_data_multi_ranges(self.build(), Some(option))
            .await
    }
}

/// 向多个范围写入数据响应体
#[derive(Deserialize, Debug)]
pub struct WriteDataToMultiRangesResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// sheet 的版本号
    pub revision: i32,
    /// 响应
    pub responses: Vec<DataResponse>,
}

/// 追加数据的范围、行列数等
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DataResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreed_sheet_token: String,
    /// 写入的行数
    #[serde(rename = "updatedRows")]
    pub updated_rows: i32,
    /// 写入的列数
    #[serde(rename = "updatedColumns")]
    pub updated_columns: i32,
    /// 写入的单元格总数
    #[serde(rename = "updatedCells")]
    pub updated_cells: i32,
}

impl ApiResponseTrait for WriteDataToMultiRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 向多个范围写入数据
    pub async fn write_data_multi_ranges(
        &self,
        request: WriteDataToMultiRangesRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<WriteDataToMultiRangesResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_update",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

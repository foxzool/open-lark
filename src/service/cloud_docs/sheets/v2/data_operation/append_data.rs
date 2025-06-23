use serde::Serialize;
use serde_json::Value;

use crate::{
    core::{
        api_req::ApiRequest, api_resp::BaseResponse, constants::AccessTokenType, req_option,
        SDKResult,
    },
    service::sheets::v2::{
        data_operation::{UpdateSheetDataResponse, ValueRangeRequest},
        SpreadsheetSheetService,
    },
};

/// 追加数据请求
#[derive(Serialize, Debug, Default)]
pub struct AppendDataRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 遇到空行追加，默认 OVERWRITE，若空行的数量小于追加数据的行数，则会覆盖已有数据；可选
    /// INSERT_ROWS ，会在插入足够数量的行后再进行数据追加
    #[serde(rename = "insertDataOption")]
    insert_data_option: String,
    /// 值与范围
    #[serde(rename = "valueRange")]
    value_range: ValueRangeRequest,
}

impl AppendDataRequest {
    pub fn builder() -> AppendDataRequestBuilder {
        AppendDataRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct AppendDataRequestBuilder {
    request: AppendDataRequest,
}

impl AppendDataRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 插入范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见
    /// 在线表格开发指南，range所表示的范围需要大于等于values占用的范围。
    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.value_range.range = range.to_string();
        self
    }

    /// 需要写入的值，如要写入公式、超链接、email、@人等，可详看附录sheet 支持写入数据类型
    pub fn values(mut self, values: Value) -> Self {
        self.request.value_range.values = values;
        self
    }

    pub fn build(mut self) -> AppendDataRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }

    /// 直接执行追加数据操作
    pub async fn execute(
        self,
        service: &SpreadsheetSheetService,
    ) -> SDKResult<BaseResponse<AppendDataResponse>> {
        service.append_data(self.build(), None).await
    }

    /// 直接执行追加数据操作并提供自定义选项
    pub async fn execute_with_options(
        self,
        service: &SpreadsheetSheetService,
        option: req_option::RequestOption,
    ) -> SDKResult<BaseResponse<AppendDataResponse>> {
        service.append_data(self.build(), Some(option)).await
    }
}

pub type AppendDataResponse = UpdateSheetDataResponse;

impl SpreadsheetSheetService {
    /// 追加数据
    pub async fn append_data(
        &self,
        request: AppendDataRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<AppendDataResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_append",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

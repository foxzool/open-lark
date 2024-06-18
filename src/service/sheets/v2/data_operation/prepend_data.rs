use serde::Serialize;
use serde_json::Value;

use crate::{
    core::{
        api_req::ApiRequest, api_resp::BaseResponse, constants::AccessTokenType, req_option,
        SDKResult,
    },
    service::sheets::v2::{
        data_operation::{AppendDataResponse, ValueRangeRequest},
        SpreadsheetSheetService,
    },
};

/// 插入数据请求
#[derive(Serialize, Debug, Default)]
pub struct PrependDataRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 值与范围
    #[serde(rename = "valueRange")]
    value_range: ValueRangeRequest,
}

impl PrependDataRequest {
    pub fn builder() -> PrependDataRequestBuilder {
        PrependDataRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct PrependDataRequestBuilder {
    request: PrependDataRequest,
}

impl PrependDataRequestBuilder {
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

    pub fn build(mut self) -> PrependDataRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 插入数据响应体
pub type PrependDataResponse = AppendDataResponse;

impl SpreadsheetSheetService {
    /// 插入数据
    pub async fn prepend_data(
        &self,
        request: PrependDataRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<PrependDataResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_prepend",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

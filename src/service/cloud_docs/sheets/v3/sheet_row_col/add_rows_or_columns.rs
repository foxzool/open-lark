use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::SheetRowColService,
};

impl SheetRowColService {
    /// 增加行列
    pub async fn add_rows_or_columns(
        &self,
        request: AddRowsOrColumnsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AddRowsOrColumnsResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = SHEETS_V3_SPREADSHEET_DIMENSION_RANGE
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 增加行列请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AddRowsOrColumnsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 需要增加行列的维度
    dimension: String,
    /// 增加行列的长度
    length: i32,
}

impl AddRowsOrColumnsRequest {
    pub fn builder() -> AddRowsOrColumnsRequestBuilder {
        AddRowsOrColumnsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct AddRowsOrColumnsRequestBuilder {
    request: AddRowsOrColumnsRequest,
}

impl AddRowsOrColumnsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    /// 设置维度类型
    /// - ROWS: 行
    /// - COLUMNS: 列
    pub fn dimension(mut self, dimension: impl ToString) -> Self {
        self.request.dimension = dimension.to_string();
        self
    }

    /// 设置增加的长度
    pub fn length(mut self, length: i32) -> Self {
        self.request.length = length;
        self
    }

    pub fn build(mut self) -> AddRowsOrColumnsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 增加行列响应体最外层
#[derive(Deserialize, Debug)]
pub struct AddRowsOrColumnsResponseData {
    /// 增加行列后的信息
    pub add_range: AddRangeInfo,
}

impl ApiResponseTrait for AddRowsOrColumnsResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 增加范围信息
#[derive(Deserialize, Debug)]
pub struct AddRangeInfo {
    /// 增加的维度
    pub dimension: String,
    /// 增加的起始位置
    pub start_index: i32,
    /// 增加的结束位置
    pub end_index: i32,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use super::AddRowsOrColumnsResponseData;

    #[test]
    fn test_add_rows_or_columns_response() {
        let json = json!({
            "add_range": {
                "dimension": "ROWS",
                "start_index": 5,
                "end_index": 10
            }
        });

        let response: AddRowsOrColumnsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.add_range.dimension, "ROWS");
        assert_eq!(response.add_range.start_index, 5);
        assert_eq!(response.add_range.end_index, 10);
    }
}

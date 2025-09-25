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
        standard_response::StandardResponse,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};

impl DataOperationService {
    /// 追加数据
    ///
    /// <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-value/append>
    pub async fn append_data(
        &self,
        request: AppendDataRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<AppendDataResponseData> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = SHEETS_V3_SPREADSHEET_VALUES_APPEND
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.range);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<AppendDataResponseData> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result()
    }
}

/// 追加数据请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppendDataRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 查询范围，包含 sheetId 与单元格范围两部分
    range: String,
    /// 插入数据的方式
    #[serde(rename = "insertDataOption")]
    insert_data_option: Option<String>,
    /// 数据值
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

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
        self
    }

    pub fn insert_data_option(mut self, insert_data_option: impl ToString) -> Self {
        self.request.insert_data_option = Some(insert_data_option.to_string());
        self
    }

    pub fn values(mut self, values: Vec<Vec<serde_json::Value>>) -> Self {
        self.request.value_range = ValueRangeRequest {
            range: self.request.range.clone(),
            values,
        };
        self
    }

    pub fn build(mut self) -> AppendDataRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// Trait implementation
impl_executable_builder_owned!(
    AppendDataRequestBuilder,
    DataOperationService,
    AppendDataRequest,
    AppendDataResponseData,
    append_data
);

/// 值与范围请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ValueRangeRequest {
    /// 查询范围
    pub range: String,
    /// 范围内的值
    pub values: Vec<Vec<serde_json::Value>>,
}

/// 追加数据响应体最外层
#[derive(Deserialize, Debug)]
pub struct AppendDataResponseData {
    /// 表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 数据更新的位置
    #[serde(rename = "tableRange")]
    pub table_range: String,
    /// sheet 的版本号
    pub revision: i32,
    /// 更新的行数
    pub updates: UpdatesInfo,
}

impl ApiResponseTrait for AppendDataResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新信息
#[derive(Deserialize, Debug)]
pub struct UpdatesInfo {
    /// 受更新影响的表格范围
    #[serde(rename = "updatedRange")]
    pub updated_range: String,
    /// 更新的行数
    #[serde(rename = "updatedRows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updatedColumns")]
    pub updated_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "updatedCells")]
    pub updated_cells: i32,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use super::AppendDataResponseData;

    #[test]
    fn test_append_data_response() {
        let json = json!({
            "spreadsheetToken": "shtcnmBA*****yGehy8",
            "tableRange": "Sheet1!A1:B2",
            "revision": 123,
            "updates": {
                "updatedRange": "Sheet1!A3:B3",
                "updatedRows": 1,
                "updatedColumns": 2,
                "updatedCells": 2
            }
        });

        let response: AppendDataResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.spreadsheet_token, "shtcnmBA*****yGehy8");
        assert_eq!(response.updates.updated_rows, 1);
    }
}

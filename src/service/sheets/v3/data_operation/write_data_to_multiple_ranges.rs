use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::DataOperationService,
};

impl DataOperationService {
    /// 向多个范围写入数据
    pub async fn write_data_to_multiple_ranges(
        &self,
        request: WriteDataToMultipleRangesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<WriteDataToMultipleRangesResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/values/batch_update",
            request.spreadsheet_token
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 向多个范围写入数据请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WriteDataToMultipleRangesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 值范围数据列表
    #[serde(rename = "valueRanges")]
    value_ranges: Vec<MultiRangeValueData>,
}

impl WriteDataToMultipleRangesRequest {
    pub fn builder() -> WriteDataToMultipleRangesRequestBuilder {
        WriteDataToMultipleRangesRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct WriteDataToMultipleRangesRequestBuilder {
    request: WriteDataToMultipleRangesRequest,
}

impl WriteDataToMultipleRangesRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn add_range_data(
        mut self,
        range: impl ToString,
        values: Vec<Vec<serde_json::Value>>,
    ) -> Self {
        self.request.value_ranges.push(MultiRangeValueData {
            range: range.to_string(),
            values,
        });
        self
    }

    pub fn value_ranges(mut self, value_ranges: Vec<MultiRangeValueData>) -> Self {
        self.request.value_ranges = value_ranges;
        self
    }

    pub fn build(mut self) -> WriteDataToMultipleRangesRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 多范围值数据
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiRangeValueData {
    /// 范围
    pub range: String,
    /// 范围内的值
    pub values: Vec<Vec<serde_json::Value>>,
}

impl MultiRangeValueData {
    pub fn new(range: impl ToString, values: Vec<Vec<serde_json::Value>>) -> Self {
        Self {
            range: range.to_string(),
            values,
        }
    }
}

/// 向多个范围写入数据响应体最外层
#[derive(Deserialize, Debug)]
pub struct WriteDataToMultipleRangesResponseData {
    /// 表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 总更新的行数
    #[serde(rename = "totalUpdatedRows")]
    pub total_updated_rows: i32,
    /// 总更新的列数
    #[serde(rename = "totalUpdatedColumns")]
    pub total_updated_columns: i32,
    /// 总更新的单元格数
    #[serde(rename = "totalUpdatedCells")]
    pub total_updated_cells: i32,
    /// sheet 的版本号
    pub revision: i32,
    /// 每个范围的更新响应
    pub responses: Vec<RangeUpdateResponse>,
}

impl ApiResponseTrait for WriteDataToMultipleRangesResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 范围更新响应
#[derive(Deserialize, Debug)]
pub struct RangeUpdateResponse {
    /// 更新的范围
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
mod test {
    use serde_json::json;

    use super::{WriteDataToMultipleRangesResponseData, MultiRangeValueData};

    #[test]
    fn test_write_data_to_multiple_ranges_response() {
        let json = json!({
            "spreadsheetToken": "shtcnmBA*****yGehy8",
            "totalUpdatedRows": 10,
            "totalUpdatedColumns": 8,
            "totalUpdatedCells": 80,
            "revision": 130,
            "responses": [
                {
                    "updatedRange": "Sheet1!A1:B5",
                    "updatedRows": 5,
                    "updatedColumns": 2,
                    "updatedCells": 10
                },
                {
                    "updatedRange": "Sheet1!D1:G5",
                    "updatedRows": 5,
                    "updatedColumns": 4,
                    "updatedCells": 20
                }
            ]
        });

        let response: WriteDataToMultipleRangesResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.spreadsheet_token, "shtcnmBA*****yGehy8");
        assert_eq!(response.total_updated_cells, 80);
        assert_eq!(response.responses.len(), 2);
        assert_eq!(response.responses[0].updated_cells, 10);
        assert_eq!(response.responses[1].updated_cells, 20);
    }

    #[test]
    fn test_multi_range_value_data() {
        let values = vec![
            vec![json!("Name"), json!("Age")],
            vec![json!("Alice"), json!(25)],
            vec![json!("Bob"), json!(30)],
        ];

        let range_data = MultiRangeValueData::new("Sheet1!A1:B3", values.clone());
        assert_eq!(range_data.range, "Sheet1!A1:B3");
        assert_eq!(range_data.values.len(), 3);
        assert_eq!(range_data.values[0][0], json!("Name"));
    }

    #[test]
    fn test_multi_range_serialization() {
        let range_data = MultiRangeValueData {
            range: "A1:B2".to_string(),
            values: vec![
                vec![json!("test"), json!(123)],
                vec![json!("data"), json!(456)],
            ],
        };

        let json = serde_json::to_value(&range_data).unwrap();
        assert_eq!(json["range"], "A1:B2");
        assert_eq!(json["values"][0][0], "test");
        assert_eq!(json["values"][1][1], 456);
    }
}
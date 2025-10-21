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
    service::sheets::v3::SpreadsheetSheetService,
};

use super::create::DataValidationRule;

impl SpreadsheetSheetService {
    /// 查询下拉列表设置
    pub async fn query_data_validations(
        &self,
        request: QueryDataValidationsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<QueryDataValidationsResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = SHEETS_V3_SPREADSHEET_DATA_VALIDATION
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        // 添加查询参数
        if let Some(range) = &request.range {
            api_req.query_params.insert("range", range.clone());
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 查询下拉列表设置请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct QueryDataValidationsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 可选：查询范围，如果不提供则返回整个工作表的数据校验
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<String>,
}

impl QueryDataValidationsRequest {
    pub fn builder() -> QueryDataValidationsRequestBuilder {
        QueryDataValidationsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct QueryDataValidationsRequestBuilder {
    request: QueryDataValidationsRequest,
}

impl QueryDataValidationsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = Some(range.to_string());
        self
    }

    pub fn build(mut self) -> QueryDataValidationsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 数据校验信息
#[derive(Deserialize, Debug)]
pub struct DataValidationInfo {
    /// 数据校验 ID
    pub data_validation_id: String,
    /// 数据校验规则详细信息
    #[serde(flatten)]
    pub data_validation: DataValidationRule,
}

/// 查询下拉列表设置响应体最外层
#[derive(Deserialize, Debug)]
pub struct QueryDataValidationsResponseData {
    /// 数据校验列表
    pub items: Vec<DataValidationInfo>,
    /// 是否还有更多数据
    #[serde(default)]
    pub has_more: bool,
    /// 下次请求的页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for QueryDataValidationsResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_query_data_validations_response() {
        let json = json!({
            "items": [
                {
                    "data_validation_id": "dv_001",
                    "condition_type": "dropdown",
                    "range": "A1:A10",
                    "condition_values": ["选项1", "选项2"],
                    "strict": true,
                    "input_message": "请选择一个选项"
                },
                {
                    "data_validation_id": "dv_002",
                    "condition_type": "number_between",
                    "range": "B1:B10",
                    "condition_values": ["1", "100"],
                    "strict": true,
                    "error_message": "数字超出范围"
                }
            ],
            "has_more": false
        });

        let response: QueryDataValidationsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].data_validation_id, "dv_001");
        assert_eq!(
            response.items[1].data_validation.condition_type,
            "number_between"
        );
        assert!(!response.has_more);
    }
}

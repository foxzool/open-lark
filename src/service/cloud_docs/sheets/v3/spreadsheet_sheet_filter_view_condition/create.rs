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
    service::sheets::v3::SpreadsheetSheetFilterViewService,
};

impl SpreadsheetSheetFilterViewService {
    /// 创建筛选条件
    pub async fn create_condition(
        &self,
        request: CreateFilterViewConditionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateFilterViewConditionResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITIONS
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id)
            .replace("{}", &request.filter_view_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 创建筛选条件请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CreateFilterViewConditionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图 ID
    filter_view_id: String,
    /// 筛选条件
    condition: FilterCondition,
}

impl CreateFilterViewConditionRequest {
    pub fn builder() -> CreateFilterViewConditionRequestBuilder {
        CreateFilterViewConditionRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateFilterViewConditionRequestBuilder {
    request: CreateFilterViewConditionRequest,
}

impl CreateFilterViewConditionRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn filter_view_id(mut self, filter_view_id: impl ToString) -> Self {
        self.request.filter_view_id = filter_view_id.to_string();
        self
    }

    pub fn condition(mut self, condition: FilterCondition) -> Self {
        self.request.condition = condition;
        self
    }

    pub fn build(mut self) -> CreateFilterViewConditionRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 筛选条件
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FilterCondition {
    /// 筛选的列
    pub col_name: String,
    /// 筛选类型
    pub filter_type: String,
    /// 筛选值
    pub compare_values: Vec<String>,
}

impl FilterCondition {
    pub fn new(
        col_name: impl ToString,
        filter_type: impl ToString,
        compare_values: Vec<String>,
    ) -> Self {
        Self {
            col_name: col_name.to_string(),
            filter_type: filter_type.to_string(),
            compare_values,
        }
    }

    /// 等于条件
    pub fn equal(col_name: impl ToString, value: impl ToString) -> Self {
        Self::new(col_name, "equal", vec![value.to_string()])
    }

    /// 不等于条件
    pub fn not_equal(col_name: impl ToString, value: impl ToString) -> Self {
        Self::new(col_name, "notEqual", vec![value.to_string()])
    }

    /// 包含条件
    pub fn contains(col_name: impl ToString, value: impl ToString) -> Self {
        Self::new(col_name, "contains", vec![value.to_string()])
    }

    /// 大于条件
    pub fn greater_than(col_name: impl ToString, value: impl ToString) -> Self {
        Self::new(col_name, "greaterThan", vec![value.to_string()])
    }

    /// 小于条件
    pub fn less_than(col_name: impl ToString, value: impl ToString) -> Self {
        Self::new(col_name, "lessThan", vec![value.to_string()])
    }
}

/// 创建筛选条件响应体最外层
#[derive(Deserialize, Debug)]
pub struct CreateFilterViewConditionResponseData {
    /// 筛选条件 ID
    pub condition_id: String,
    /// 筛选条件信息
    pub condition: FilterCondition,
}

impl ApiResponseTrait for CreateFilterViewConditionResponseData {
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
    fn test_filter_condition_creation() {
        let condition = FilterCondition::equal("销售额", "1000");
        assert_eq!(condition.col_name, "销售额");
        assert_eq!(condition.filter_type, "equal");
        assert_eq!(condition.compare_values, vec!["1000"]);
    }

    #[test]
    fn test_create_filter_view_condition_response() {
        let json = json!({
            "condition_id": "cond_001",
            "condition": {
                "col_name": "销售额",
                "filter_type": "greaterThan",
                "compare_values": ["5000"]
            }
        });

        let response: CreateFilterViewConditionResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.condition_id, "cond_001");
        assert_eq!(response.condition.col_name, "销售额");
    }
}

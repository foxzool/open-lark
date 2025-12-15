use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 设置筛选请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetFilterRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 筛选范围
    pub range: String,
    /// 筛选条件
    pub criteria: Vec<FilterCriteria>,
}

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilterCriteria {
    /// 列索引
    pub column_index: i32,
    /// 筛选类型
    pub filter_type: String,
    /// 筛选值
    pub values: Vec<String>,
    /// 条件
    pub condition: Option<FilterCondition>,
}

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilterCondition {
    /// 条件类型
    pub r#type: String,
    /// 值
    pub value: String,
}

/// 设置筛选响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SetFilterResponse {
    /// 筛选信息
    pub filter: FilterInfo,
    /// 操作结果
    pub result: String,
}

/// 筛选信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct FilterInfo {
    /// 筛选ID
    pub filter_id: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 筛选范围
    pub range: String,
    /// 筛选条件
    pub criteria: Vec<FilterCriteriaInfo>,
}

/// 筛选条件信息
#[derive(Debug, Clone, Deserialize, Default)]
pub struct FilterCriteriaInfo {
    /// 列索引
    pub column_index: i32,
    /// 筛选类型
    pub filter_type: String,
    /// 筛选值
    pub values: Vec<String>,
}

impl ApiResponseTrait for SetFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SetFilterRequest {
    /// 创建新的设置筛选请求构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = token.into();
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, id: impl Into<String>) -> Self {
        self.sheet_id = id.into();
        self
    }

    /// 设置筛选范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = range.into();
        self
    }

    /// 添加筛选条件
    pub fn add_criteria(mut self, criteria: FilterCriteria) -> Self {
        self.criteria.push(criteria);
        self
    }

    /// 设置筛选条件
    pub fn criteria(mut self, criteria: Vec<FilterCriteria>) -> Self {
        self.criteria = criteria;
        self
    }
}

impl FilterCriteria {
    /// 创建新的筛选条件构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置列索引
    pub fn column_index(mut self, index: i32) -> Self {
        self.column_index = index;
        self
    }

    /// 设置筛选类型
    pub fn filter_type(mut self, filter_type: impl Into<String>) -> Self {
        self.filter_type = filter_type.into();
        self
    }

    /// 添加筛选值
    pub fn add_value(mut self, value: impl Into<String>) -> Self {
        self.values.push(value.into());
        self
    }

    /// 设置筛选值
    pub fn values(mut self, values: Vec<String>) -> Self {
        self.values = values;
        self
    }

    /// 设置条件
    pub fn condition(mut self, condition: FilterCondition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// 创建文本包含筛选
    pub fn text_contains(column_index: i32, text: impl Into<String>) -> Self {
        Self {
            column_index,
            filter_type: "TEXT_CONTAINS".to_string(),
            values: vec![text.into()],
            condition: None,
        }
    }

    /// 创建文本不包含筛选
    pub fn text_not_contains(column_index: i32, text: impl Into<String>) -> Self {
        Self {
            column_index,
            filter_type: "TEXT_NOT_CONTAINS".to_string(),
            values: vec![text.into()],
            condition: None,
        }
    }

    /// 创建数值大于筛选
    pub fn number_greater_than(column_index: i32, value: impl Into<String>) -> Self {
        Self {
            column_index,
            filter_type: "NUMBER_GREATER_THAN".to_string(),
            values: vec![],
            condition: Some(FilterCondition {
                r#type: "NUMBER_GREATER_THAN".to_string(),
                value: value.into(),
            }),
        }
    }

    /// 创建数值小于筛选
    pub fn number_less_than(column_index: i32, value: impl Into<String>) -> Self {
        Self {
            column_index,
            filter_type: "NUMBER_LESS_THAN".to_string(),
            values: vec![],
            condition: Some(FilterCondition {
                r#type: "NUMBER_LESS_THAN".to_string(),
                value: value.into(),
            }),
        }
    }

    /// 创建数值等于筛选
    pub fn number_equals(column_index: i32, value: impl Into<String>) -> Self {
        Self {
            column_index,
            filter_type: "NUMBER_EQUALS".to_string(),
            values: vec![],
            condition: Some(FilterCondition {
                r#type: "NUMBER_EQUALS".to_string(),
                value: value.into(),
            }),
        }
    }

    /// 创建值列表筛选
    pub fn values_in_list(column_index: i32, values: Vec<String>) -> Self {
        Self {
            column_index,
            filter_type: "VALUES_IN_LIST".to_string(),
            values,
            condition: None,
        }
    }
}

impl FilterCondition {
    /// 创建新的筛选条件构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置条件类型
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = r#type.into();
        self
    }

    /// 设置值
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }
}

/// 设置筛选
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/filter
pub async fn set_filter(
    request: SetFilterRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<SetFilterResponse>> {
    // 构建请求体
    let body = json!(request);

    // 创建API请求
    let mut api_request: ApiRequest<SetFilterResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/filter", SheetsApiV3, request.spreadsheet_token))
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_filter_request_builder() {
        let request = SetFilterRequest::new()
            .spreadsheet_token("test_token")
            .sheet_id("test_sheet")
            .range("A1:D10")
            .add_criteria(FilterCriteria::text_contains(0, "测试"))
            .add_criteria(FilterCriteria::number_greater_than(1, "100"));

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "test_sheet");
        assert_eq!(request.range, "A1:D10");
        assert_eq!(request.criteria.len(), 2);
        assert_eq!(request.criteria[0].column_index, 0);
        assert_eq!(request.criteria[0].filter_type, "TEXT_CONTAINS");
        assert_eq!(request.criteria[0].values, vec!["测试"]);
        assert_eq!(request.criteria[1].column_index, 1);
        assert_eq!(request.criteria[1].filter_type, "NUMBER_GREATER_THAN");
        assert_eq!(request.criteria[1].values, vec![]);
        assert!(request.criteria[1].condition.is_some());
    }

    #[test]
    fn test_filter_criteria_convenience_methods() {
        let criteria1 = FilterCriteria::text_contains(0, "重要");
        assert_eq!(criteria1.column_index, 0);
        assert_eq!(criteria1.filter_type, "TEXT_CONTAINS");
        assert_eq!(criteria1.values, vec!["重要"]);
        assert!(criteria1.condition.is_none());

        let criteria2 = FilterCriteria::number_greater_than(1, "100");
        assert_eq!(criteria2.column_index, 1);
        assert_eq!(criteria2.filter_type, "NUMBER_GREATER_THAN");
        assert_eq!(criteria2.values, vec![]);
        assert!(criteria2.condition.is_some());
        assert_eq!(criteria2.condition.unwrap().r#type, "NUMBER_GREATER_THAN");
        assert_eq!(criteria2.condition.unwrap().value, "100");

        let criteria3 = FilterCriteria::values_in_list(2, vec!["A".to_string(), "B".to_string()]);
        assert_eq!(criteria3.column_index, 2);
        assert_eq!(criteria3.filter_type, "VALUES_IN_LIST");
        assert_eq!(criteria3.values, vec!["A", "B"]);
        assert!(criteria3.condition.is_none());
    }

    #[test]
    fn test_filter_criteria_builder() {
        let criteria = FilterCriteria::new()
            .column_index(0)
            .filter_type("TEXT_CONTAINS")
            .add_value("测试")
            .condition(FilterCondition::new().r#type("TEXT_CONTAINS").value("测试"));

        assert_eq!(criteria.column_index, 0);
        assert_eq!(criteria.filter_type, "TEXT_CONTAINS");
        assert_eq!(criteria.values, vec!["测试"]);
        assert!(criteria.condition.is_some());
    }

    #[test]
    fn test_filter_condition_builder() {
        let condition = FilterCondition::new()
            .r#type("NUMBER_GREATER_THAN")
            .value("100");

        assert_eq!(condition.r#type, "NUMBER_GREATER_THAN");
        assert_eq!(condition.value, "100");
    }
}
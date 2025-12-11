//! 创建筛选条件
//!
//! 在筛选视图的筛选范围的某一列创建筛选条件
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/condition/create

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    error::LarkAPIError,
    http::Transport,
    SDKResult,
};

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ccm::sheets::v3::models::{
    FilterViewCondition, SpreadsheetToken, SheetId, FilterViewId, SheetsResponse
};

/// 创建筛选条件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterConditionResponse {
    /// 筛选条件信息
    pub condition: FilterViewCondition,
}

impl ApiResponseTrait for CreateFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选条件构建器
pub struct CreateFilterConditionBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    filter_view_id: Option<FilterViewId>,
    request: FilterViewCondition,
}

impl<'a> CreateFilterConditionBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            filter_view_id: None,
            request: FilterViewCondition {
                filter_view_id: String::new(),
                column_id: String::new(),
                operator: String::new(),
                value: None,
                ignore_case: Some(true),
            },
        }
    }

    /// 设置电子表格令牌
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, id: impl Into<String>) -> Self {
        self.sheet_id = Some(id.into());
        self
    }

    /// 设置筛选视图ID
    pub fn filter_view_id(mut self, id: impl Into<String>) -> Self {
        self.filter_view_id = Some(id.into());
        self.request.filter_view_id = id.into();
        self
    }

    /// 设置列标识符
    pub fn column_id(mut self, column_id: impl Into<String>) -> Self {
        self.request.column_id = column_id.into();
        self
    }

    /// 设置筛选操作符
    pub fn operator(mut self, operator: impl Into<String>) -> Self {
        self.request.operator = operator.into();
        self
    }

    /// 设置筛选值
    pub fn value(mut self, value: impl Into<Value>) -> Self {
        self.request.value = Some(value.into());
        self
    }

    /// 设置是否忽略大小写
    pub fn ignore_case(mut self, ignore_case: bool) -> Self {
        self.request.ignore_case = Some(ignore_case);
        self
    }

    /// 执行创建筛选条件请求
    pub async fn execute(self) -> SDKResult<CreateFilterConditionResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let filter_view_id = self.filter_view_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("筛选视图ID不能为空".to_string())
        })?;

        if self.request.column_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError("列标识符不能为空".to_string()));
        }

        if self.request.operator.is_empty() {
            return Err(LarkAPIError::IllegalParamError("筛选操作符不能为空".to_string()));
        }

        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions",
            self.config.base_url, spreadsheet_token, sheet_id, filter_view_id
        );

        let body = serde_json::to_value(&self.request).map_err(|e| {
            LarkAPIError::IllegalParamError(format!("请求体序列化失败: {}", e))
        })?;

        let mut api_request = ApiRequest::new(Method::POST, &url)
            .body(body)
            .bearer_auth(&config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<CreateFilterConditionResponse> =
            serde_json::from_str(&response).map_err(|e| {
                LarkAPIError::JsonParseError(format!("响应解析失败: {}", e))
            })?;

        if let Some(data) = api_response.data {
            Ok(data)
        } else if let Some(error) = api_response.error {
            Err(LarkAPIError::APIError(error.to_string()))
        } else {
            Err(LarkAPIError::UnknownError("未知错误".to_string()))
        }
    }
}

/// 创建筛选条件
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
/// * `filter_view_id` - 筛选视图ID
///
/// # 返回
/// 返回创建筛选条件的构建器
///
/// # 示例
/// ```rust
/// let response = create_filter_condition(&config, "sheet_token_123", "sheet_id_456", "fv_abc123")
///     .column_id("A")
///     .operator("contains")
///     .value(serde_json::json!("test"))
///     .ignore_case(true)
///     .execute()
///     .await?;
/// ```
pub fn create_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> CreateFilterConditionBuilder {
    CreateFilterConditionBuilder::new(config)
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
        .filter_view_id(filter_view_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_filter_condition_builder() {
        let config = Config::default();
        let builder = create_filter_condition(&config, "sheet_token_123", "sheet_id_456", "fv_abc123")
            .column_id("A")
            .operator("contains")
            .value(serde_json::json!("test"));

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.filter_view_id, Some("fv_abc123".to_string()));
        assert_eq!(builder.request.column_id, "A");
        assert_eq!(builder.request.operator, "contains");
        assert!(builder.request.value.is_some());
    }
}
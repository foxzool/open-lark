//! 创建筛选
//!
//! 在子表内创建筛选
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/create

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
    CreateFilterRequest, FilterInfo, SpreadsheetToken, SheetId, SheetsResponse
};

/// 创建筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterResponse {
    /// 筛选信息
    pub filter: FilterInfo,
}

impl ApiResponseTrait for CreateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选构建器
pub struct CreateFilterBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    request: CreateFilterRequest,
}

impl<'a> CreateFilterBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            request: CreateFilterRequest {
                range: String::new(),
                conditions: None,
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

    /// 设置筛选范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.request.range = range.into();
        self
    }

    /// 设置筛选条件
    pub fn conditions(mut self, conditions: Vec<crate::ccm::sheets::v3::models::FilterCondition>) -> Self {
        self.request.conditions = Some(conditions);
        self
    }

    /// 添加筛选条件
    pub fn add_condition(mut self, condition: crate::ccm::sheets::v3::models::FilterCondition) -> Self {
        match &mut self.request.conditions {
            Some(conditions) => conditions.push(condition),
            None => self.request.conditions = Some(vec![condition]),
        }
        self
    }

    /// 执行创建筛选请求
    pub async fn execute(self) -> SDKResult<CreateFilterResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        if self.request.range.is_empty() {
            return Err(LarkAPIError::IllegalParamError("筛选范围不能为空".to_string()));
        }

        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter",
            self.config.base_url, spreadsheet_token, sheet_id
        );

        let body = serde_json::to_value(&self.request).map_err(|e| {
            LarkAPIError::IllegalParamError(format!("请求体序列化失败: {}", e))
        })?;

        let mut api_request = ApiRequest::new(Method::POST, &url)
            .body(body)
            .bearer_auth(&self.config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<CreateFilterResponse> =
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

/// 创建筛选
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
///
/// # 返回
/// 返回创建筛选的构建器
///
/// # 示例
/// ```rust
/// let response = create_filter(&config, "sheet_token_123", "sheet_id_456")
///     .range("A1:D10")
///     .add_condition(FilterCondition {
///         column_id: "A".to_string(),
///         operator: "contains".to_string(),
///         value: Some(Value::String("test".to_string())),
///         ignore_case: Some(true),
///     })
///     .execute()
///     .await?;
/// ```
pub fn create_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> CreateFilterBuilder {
    CreateFilterBuilder::new(config)
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_filter_builder() {
        let config = Config::default();
        let builder = create_filter(&config, "sheet_token_123", "sheet_id_456")
            .range("A1:D10");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.request.range, "A1:D10");
    }
}
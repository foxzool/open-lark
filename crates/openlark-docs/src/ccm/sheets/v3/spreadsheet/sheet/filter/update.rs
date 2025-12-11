//! 更新筛选
//!
//! 更新子表筛选范围中的列筛选条件
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/update

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
    UpdateFilterRequest, FilterInfo, SpreadsheetToken, SheetId, SheetsResponse
};

/// 更新筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterResponse {
    /// 筛选信息
    pub filter: FilterInfo,
}

impl ApiResponseTrait for UpdateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选构建器
pub struct UpdateFilterBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    request: UpdateFilterRequest,
}

impl<'a> UpdateFilterBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            request: UpdateFilterRequest {
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

    /// 清空所有筛选条件
    pub fn clear_conditions(mut self) -> Self {
        self.request.conditions = Some(vec![]);
        self
    }

    /// 执行更新筛选请求
    pub async fn execute(self) -> SDKResult<UpdateFilterResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter",
            self.config.base_url, spreadsheet_token, sheet_id
        );

        let body = serde_json::to_value(&self.request).map_err(|e| {
            LarkAPIError::IllegalParamError(format!("请求体序列化失败: {}", e))
        })?;

        let mut api_request = ApiRequest::new(Method::PUT, &url)
            .body(body)
            .bearer_auth(&self.config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<UpdateFilterResponse> =
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

/// 更新筛选
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
///
/// # 返回
/// 返回更新筛选的构建器
///
/// # 示例
/// ```rust
/// let response = update_filter(&config, "sheet_token_123", "sheet_id_456")
///     .add_condition(FilterCondition {
///         column_id: "B".to_string(),
///         operator: "equals".to_string(),
///         value: Some(Value::String("value".to_string())),
///         ignore_case: Some(false),
///     })
///     .execute()
///     .await?;
/// ```
pub fn update_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> UpdateFilterBuilder {
    UpdateFilterBuilder::new(config)
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_filter_builder() {
        let config = Config::default();
        let builder = update_filter(&config, "sheet_token_123", "sheet_id_456");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
    }
}
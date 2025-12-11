//! 获取筛选条件
//!
//! 获取筛选视图某列的筛选条件信息
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/condition/get

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

use crate::ccm::sheets::v3::models::{
    FilterViewCondition, SpreadsheetToken, SheetId, FilterViewId, SheetsResponse
};

/// 获取筛选条件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterConditionResponse {
    /// 筛选条件信息
    pub condition: FilterViewCondition,
}

impl ApiResponseTrait for GetFilterConditionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选条件
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
/// * `filter_view_id` - 筛选视图ID
/// * `column_id` - 列标识符
///
/// # 返回
/// 返回筛选视图某列的筛选条件信息
///
/// # 示例
/// ```rust
/// let response = get_filter_condition(&config, "sheet_token_123", "sheet_id_456", "fv_abc123", "A")
///     .await?;
/// println!("筛选操作符: {}", response.condition.operator);
/// println!("筛选值: {:?}", response.condition.value);
/// ```
pub async fn get_filter_condition(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
    column_id: &str,
) -> SDKResult<GetFilterConditionResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions/{}",
        config.base_url, spreadsheet_token, sheet_id, filter_view_id, column_id
    );

    let mut api_request = ApiRequest::new(Method::GET, &url)
        .bearer_auth(&config.tenant_access_token);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<GetFilterConditionResponse> =
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

/// 获取筛选条件构建器
pub struct GetFilterConditionBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    filter_view_id: Option<FilterViewId>,
    column_id: Option<String>,
}

impl<'a> GetFilterConditionBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            filter_view_id: None,
            column_id: None,
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
        self
    }

    /// 设置列标识符
    pub fn column_id(mut self, id: impl Into<String>) -> Self {
        self.column_id = Some(id.into());
        self
    }

    /// 执行获取筛选条件请求
    pub async fn execute(self) -> SDKResult<GetFilterConditionResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let filter_view_id = self.filter_view_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("筛选视图ID不能为空".to_string())
        })?;

        let column_id = self.column_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("列标识符不能为空".to_string())
        })?;

        get_filter_condition(self.config, &spreadsheet_token, &sheet_id, &filter_view_id, &column_id).await
    }
}

/// 获取筛选条件的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回获取筛选条件的构建器
pub fn get_filter_condition_builder(config: &Config) -> GetFilterConditionBuilder {
    GetFilterConditionBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filter_condition_builder() {
        let config = Config::default();
        let builder = get_filter_condition_builder(&config)
            .spreadsheet_token("sheet_token_123")
            .sheet_id("sheet_id_456")
            .filter_view_id("fv_abc123")
            .column_id("A");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.filter_view_id, Some("fv_abc123".to_string()));
        assert_eq!(builder.column_id, Some("A".to_string()));
    }

    #[test]
    fn test_missing_parameters() {
        let config = Config::default();
        let builder = get_filter_condition_builder(&config);

        assert!(builder.spreadsheet_token.is_none());
        assert!(builder.sheet_id.is_none());
        assert!(builder.filter_view_id.is_none());
        assert!(builder.column_id.is_none());
    }
}
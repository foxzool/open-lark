//! 替换单元格
//!
//! 按照指定的条件查找子表的某个范围内的数据符合条件的单元格并替换值，返回替换成功的单元格位置。一次请求最多允许替换5000个单元格，如果超过请将range缩小范围再操作。请求体中的 range、find、replaccement 字段必填。
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/replace

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
    ReplaceCellsRequest, ReplaceResult, SheetId, SpreadsheetToken, SheetsResponse
};

/// 替换单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceCellsResponse {
    /// 电子表格令牌
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    pub sheet_id: SheetId,
    /// 替换结果
    pub replace_result: ReplaceResult,
}

impl ApiResponseTrait for ReplaceCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 替换单元格
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
/// * `request` - 替换单元格请求
///
/// # 返回
/// 返回替换单元格的操作结果
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::sheets::v3::spreadsheet::sheet::replace::*;
///
/// let request = ReplaceCellsRequest {
///     range: "A1:D100".to_string(),
///     find: FindCondition {
///         value: "旧值".to_string(),
///         match_type: "exact".to_string(),
///     },
///     replacement: ReplacementValue {
///         value: "新值".to_string(),
///     },
/// };
///
/// let response = replace_cells(&config, "sheet_token_123", "sheet_id_456", request)
///     .await?;
/// println!("替换成功: {} 个单元格", response.replace_result.total);
/// ```
pub async fn replace_cells(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    request: ReplaceCellsRequest,
) -> SDKResult<ReplaceCellsResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/replace",
        config.base_url, spreadsheet_token, sheet_id
    );

    let mut api_request = ApiRequest::new(Method::POST, &url)
        .bearer_auth(&config.tenant_access_token)
        .body(serde_json::to_value(&request)?);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<ReplaceCellsResponse> =
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

/// 替换单元格构建器
pub struct ReplaceCellsBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    range: Option<String>,
    find_condition: Option<serde_json::Value>,
    replacement_value: Option<serde_json::Value>,
}

impl<'a> ReplaceCellsBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            range: None,
            find_condition: None,
            replacement_value: None,
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

    /// 设置替换范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 设置查找条件
    pub fn find_condition(mut self, condition: impl Into<serde_json::Value>) -> Self {
        self.find_condition = Some(condition.into());
        self
    }

    /// 设置替换值
    pub fn replacement_value(mut self, value: impl Into<serde_json::Value>) -> Self {
        self.replacement_value = Some(value.into());
        self
    }

    /// 执行替换单元格请求
    pub async fn execute(self) -> SDKResult<ReplaceCellsResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let range = self.range.ok_or_else(|| {
            LarkAPIError::IllegalParamError("替换范围不能为空".to_string())
        })?;

        let find_condition = self.find_condition.ok_or_else(|| {
            LarkAPIError::IllegalParamError("查找条件不能为空".to_string())
        })?;

        let replacement_value = self.replacement_value.ok_or_else(|| {
            LarkAPIError::IllegalParamError("替换值不能为空".to_string())
        })?;

        let request = ReplaceCellsRequest {
            range,
            find: find_condition,
            replacement: replacement_value,
        };

        replace_cells(self.config, &spreadsheet_token, &sheet_id, request).await
    }
}

/// 替换单元格的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回替换单元格的构建器
pub fn replace_cells_builder(config: &Config) -> ReplaceCellsBuilder {
    ReplaceCellsBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_cells_builder() {
        let config = Config::default();
        let builder = replace_cells_builder(&config)
            .spreadsheet_token("sheet_token_123")
            .sheet_id("sheet_id_456")
            .range("A1:D100")
            .find_condition(serde_json::json!({
                "value": "旧值",
                "match_type": "exact"
            }))
            .replacement_value(serde_json::json!({
                "value": "新值"
            }));

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.range, Some("A1:D100".to_string()));
    }

    #[test]
    fn test_missing_parameters() {
        let config = Config::default();
        let builder = replace_cells_builder(&config);

        assert!(builder.spreadsheet_token.is_none());
        assert!(builder.sheet_id.is_none());
        assert!(builder.range.is_none());
        assert!(builder.find_condition.is_none());
        assert!(builder.replacement_value.is_none());
    }
}
//! 插入行
//!
//! 在工作表中插入空白行
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-data_operation/insert_rows

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

use crate::ccm::sheets::v3::models::{SpreadsheetToken, SheetId, SheetsResponse};

/// 插入行响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertRowsResponse {
    /// 操作结果
    pub success: bool,
    /// 插入的行数
    pub inserted_rows: i32,
}

impl ApiResponseTrait for InsertRowsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 插入行构建器
pub struct InsertRowsBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    request: Value,
}

impl<'a> InsertRowsBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            request: serde_json::json!({}),
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

    /// 设置插入位置（从0开始）
    pub fn row_index(mut self, index: i32) -> Self {
        if let Some(obj) = self.request.as_object_mut() {
            obj.insert("row_index".to_string(), Value::Number(serde_json::Number::from(index)));
        }
        self
    }

    /// 设置插入行数
    pub fn count(mut self, count: i32) -> Self {
        if let Some(obj) = self.request.as_object_mut() {
            obj.insert("count".to_string(), Value::Number(serde_json::Number::from(count)));
        }
        self
    }

    /// 设置插入方向
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        if let Some(obj) = self.request.as_object_mut() {
            obj.insert("direction".to_string(), Value::String(direction.into()));
        }
        self
    }

    /// 执行插入行请求
    pub async fn execute(self) -> SDKResult<InsertRowsResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/data_operation/insert_rows",
            self.config.base_url, spreadsheet_token, sheet_id
        );

        let mut api_request = ApiRequest::new(Method::POST, &url)
            .body(self.request)
            .bearer_auth(&self.config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<InsertRowsResponse> =
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

/// 插入行
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
///
/// # 返回
/// 返回插入行的构建器
///
/// # 示例
/// ```rust
/// let response = insert_rows(&config, "sheet_token_123", "sheet_id_456")
///     .row_index(2)
///     .count(3)
///     .direction("down")
///     .execute()
///     .await?;
/// println!("成功插入 {} 行", response.inserted_rows);
/// ```
pub fn insert_rows(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> InsertRowsBuilder {
    InsertRowsBuilder::new(config)
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_rows_builder() {
        let config = Config::default();
        let builder = insert_rows(&config, "sheet_token_123", "sheet_id_456")
            .row_index(2)
            .count(3)
            .direction("down");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
    }

    #[test]
    fn test_missing_parameters() {
        let config = Config::default();
        let builder = insert_rows(&config, "", "sheet_id_456");

        assert!(builder.spreadsheet_token.is_some());
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
    }
}
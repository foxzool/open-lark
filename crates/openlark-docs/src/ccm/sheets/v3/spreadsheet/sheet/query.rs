//! 获取工作表
//!
//! 该接口用于获取电子表格下所有工作表及其属性
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/query

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

use crate::ccm::sheets::v3::models::{SheetInfo, SpreadsheetToken, SheetsResponse};

/// 获取工作表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySheetsResponse {
    /// 电子表格令牌
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
}

impl ApiResponseTrait for QuerySheetsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取工作表
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
///
/// # 返回
/// 返回电子表格下所有工作表及其属性
///
/// # 示例
/// ```rust
/// let response = query_sheets(&config, "sheet_token_123")
///     .await?;
/// for sheet in response.sheets {
///     println!("工作表: {} ({})", sheet.title, sheet.sheet_id);
/// }
/// ```
pub async fn query_sheets(
    config: &Config,
    spreadsheet_token: &str,
) -> SDKResult<QuerySheetsResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/query",
        config.base_url, spreadsheet_token
    );

    let mut api_request = ApiRequest::new(Method::GET, &url)
        .bearer_auth(&config.tenant_access_token);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<QuerySheetsResponse> =
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

/// 获取工作表构建器
pub struct QuerySheetsBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
}

impl<'a> QuerySheetsBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
        }
    }

    /// 设置电子表格令牌
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    /// 执行获取工作表请求
    pub async fn execute(self) -> SDKResult<QuerySheetsResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        query_sheets(self.config, &spreadsheet_token).await
    }
}

/// 获取工作表的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回获取工作表的构建器
pub fn query_sheets_builder(config: &Config) -> QuerySheetsBuilder {
    QuerySheetsBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_sheets_builder() {
        let config = Config::default();
        let builder = query_sheets_builder(&config)
            .spreadsheet_token("sheet_token_123");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
    }

    #[test]
    fn test_missing_spreadsheet_token() {
        let config = Config::default();
        let builder = query_sheets_builder(&config);

        assert!(builder.spreadsheet_token.is_none());
    }
}
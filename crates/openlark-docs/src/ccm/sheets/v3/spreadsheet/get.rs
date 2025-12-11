//! 获取电子表格信息
//!
//! 该接口用于获取电子表格的基础信息
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/get

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

use crate::ccm::sheets::v3::models::{SpreadsheetInfo, SheetsResponse};

/// 获取电子表格信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetResponse {
    /// 电子表格令牌
    pub spreadsheet_token: String,
    /// 电子表格标题
    pub title: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: Option<String>,
    /// 时区
    pub time_zone: Option<String>,
    /// 语言
    pub locale: Option<String>,
    /// 工作表列表
    pub sheets: Option<Vec<SpreadsheetInfo>>,
}

impl ApiResponseTrait for GetSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取电子表格信息
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
///
/// # 返回
/// 返回电子表格的基础信息
///
/// # 示例
/// ```rust
/// let response = get_spreadsheet(&config, "sheet_token_123")
///     .await?;
/// println!("表格标题: {}", response.title);
/// ```
pub async fn get_spreadsheet(
    config: &Config,
    spreadsheet_token: &str,
) -> SDKResult<GetSpreadsheetResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}",
        config.base_url, spreadsheet_token
    );

    let mut api_request = ApiRequest::new(Method::GET, &url)
        .bearer_auth(&config.tenant_access_token);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<GetSpreadsheetResponse> =
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

/// 获取电子表格信息构建器
pub struct GetSpreadsheetBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<String>,
}

impl<'a> GetSpreadsheetBuilder<'a> {
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

    /// 执行获取电子表格信息请求
    pub async fn execute(self) -> SDKResult<GetSpreadsheetResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        get_spreadsheet(self.config, &spreadsheet_token).await
    }
}

/// 获取电子表格信息的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回获取电子表格信息的构建器
pub fn get_spreadsheet_builder(config: &Config) -> GetSpreadsheetBuilder {
    GetSpreadsheetBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_spreadsheet_builder() {
        let config = Config::default();
        let builder = get_spreadsheet_builder(&config)
            .spreadsheet_token("sheet_token_123");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
    }

    #[test]
    fn test_missing_spreadsheet_token() {
        let config = Config::default();
        let builder = get_spreadsheet_builder(&config);

        assert!(builder.spreadsheet_token.is_none());
    }
}
//! 获取筛选
//!
//! 获取子表的详细筛选信息
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/get

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

use crate::ccm::sheets::v3::models::{FilterInfo, SpreadsheetToken, SheetId, SheetsResponse};

/// 获取筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFilterResponse {
    /// 筛选信息
    pub filter: FilterInfo,
}

impl ApiResponseTrait for GetFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取筛选
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
///
/// # 返回
/// 返回子表的详细筛选信息
///
/// # 示例
/// ```rust
/// let response = get_filter(&config, "sheet_token_123", "sheet_id_456")
///     .await?;
/// println!("筛选范围: {}", response.filter.range);
/// ```
pub async fn get_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<GetFilterResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter",
        config.base_url, spreadsheet_token, sheet_id
    );

    let mut api_request = ApiRequest::new(Method::GET, &url)
        .bearer_auth(&config.tenant_access_token);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<GetFilterResponse> =
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

/// 获取筛选构建器
pub struct GetFilterBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
}

impl<'a> GetFilterBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
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

    /// 执行获取筛选请求
    pub async fn execute(self) -> SDKResult<GetFilterResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        get_filter(self.config, &spreadsheet_token, &sheet_id).await
    }
}

/// 获取筛选的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回获取筛选的构建器
pub fn get_filter_builder(config: &Config) -> GetFilterBuilder {
    GetFilterBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filter_builder() {
        let config = Config::default();
        let builder = get_filter_builder(&config)
            .spreadsheet_token("sheet_token_123")
            .sheet_id("sheet_id_456");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
    }
}
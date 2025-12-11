//! 删除筛选
//!
//! 删除子表的筛选
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/delete

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

use crate::ccm::sheets::v3::models::{SpreadsheetToken, SheetId, SheetsResponse};

/// 删除筛选响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterResponse {
    /// 操作结果
    pub success: bool,
    /// 删除的筛选ID
    pub filter_id: Option<String>,
}

impl ApiResponseTrait for DeleteFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除筛选
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
///
/// # 返回
/// 返回删除筛选的操作结果
///
/// # 示例
/// ```rust
/// let response = delete_filter(&config, "sheet_token_123", "sheet_id_456")
///     .await?;
/// if response.success {
///     println!("筛选删除成功");
/// }
/// ```
pub async fn delete_filter(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<DeleteFilterResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter",
        config.base_url, spreadsheet_token, sheet_id
    );

    let mut api_request = ApiRequest::new(Method::DELETE, &url)
        .bearer_auth(&self.config.tenant_access_token);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<DeleteFilterResponse> =
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

/// 删除筛选构建器
pub struct DeleteFilterBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
}

impl<'a> DeleteFilterBuilder<'a> {
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

    /// 执行删除筛选请求
    pub async fn execute(self) -> SDKResult<DeleteFilterResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        delete_filter(self.config, &spreadsheet_token, &sheet_id).await
    }
}

/// 删除筛选的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回删除筛选的构建器
pub fn delete_filter_builder(config: &Config) -> DeleteFilterBuilder {
    DeleteFilterBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_filter_builder() {
        let config = Config::default();
        let builder = delete_filter_builder(&config)
            .spreadsheet_token("sheet_token_123")
            .sheet_id("sheet_id_456");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
    }
}
//! 删除筛选视图
//!
//! 删除指定 id 对应的筛选视图
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/delete

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

use crate::ccm::sheets::v3::models::{SpreadsheetToken, SheetId, FilterViewId, SheetsResponse};

/// 删除筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilterViewResponse {
    /// 操作结果
    pub success: bool,
    /// 删除的筛选视图ID
    pub filter_view_id: FilterViewId,
}

impl ApiResponseTrait for DeleteFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除筛选视图
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
/// * `filter_view_id` - 筛选视图ID
///
/// # 返回
/// 返回删除筛选视图的操作结果
///
/// # 示例
/// ```rust
/// let response = delete_filter_view(&config, "sheet_token_123", "sheet_id_456", "fv_abc123")
///     .await?;
/// if response.success {
///     println!("筛选视图删除成功");
/// }
/// ```
pub async fn delete_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> SDKResult<DeleteFilterViewResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}",
        config.base_url, spreadsheet_token, sheet_id, filter_view_id
    );

    let mut api_request = ApiRequest::new(Method::DELETE, &url)
        .bearer_auth(&config.tenant_access_token);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<DeleteFilterViewResponse> =
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

/// 删除筛选视图构建器
pub struct DeleteFilterViewBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    filter_view_id: Option<FilterViewId>,
}

impl<'a> DeleteFilterViewBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            filter_view_id: None,
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

    /// 执行删除筛选视图请求
    pub async fn execute(self) -> SDKResult<DeleteFilterViewResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let filter_view_id = self.filter_view_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("筛选视图ID不能为空".to_string())
        })?;

        delete_filter_view(self.config, &spreadsheet_token, &sheet_id, &filter_view_id).await
    }
}

/// 删除筛选视图的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回删除筛选视图的构建器
pub fn delete_filter_view_builder(config: &Config) -> DeleteFilterViewBuilder {
    DeleteFilterViewBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_filter_view_builder() {
        let config = Config::default();
        let builder = delete_filter_view_builder(&config)
            .spreadsheet_token("sheet_token_123")
            .sheet_id("sheet_id_456")
            .filter_view_id("fv_abc123");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.filter_view_id, Some("fv_abc123".to_string()));
    }

    #[test]
    fn test_missing_parameters() {
        let config = Config::default();
        let builder = delete_filter_view_builder(&config);

        assert!(builder.spreadsheet_token.is_none());
        assert!(builder.sheet_id.is_none());
        assert!(builder.filter_view_id.is_none());
    }
}
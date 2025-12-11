//! 查询筛选视图
//!
//! 查询子表内所有的筛选视图基本信息，包括 id、name 和 range
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/query

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

use crate::ccm::sheets::v3::models::{FilterViewInfo, SpreadsheetToken, SheetId, SheetsResponse};

/// 查询筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilterViewsResponse {
    /// 筛选视图列表
    pub filter_views: Vec<FilterViewInfo>,
    /// 总数
    pub total: i32,
}

impl ApiResponseTrait for QueryFilterViewsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询筛选视图
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
///
/// # 返回
/// 返回子表内所有的筛选视图基本信息
///
/// # 示例
/// ```rust
/// let response = query_filter_views(&config, "sheet_token_123", "sheet_id_456")
///     .await?;
/// for filter_view in response.filter_views {
///     println!("筛选视图: {} ({})", filter_view.name, filter_view.filter_view_id);
/// }
/// ```
pub async fn query_filter_views(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<QueryFilterViewsResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/query",
        config.base_url, spreadsheet_token, sheet_id
    );

    let mut api_request = ApiRequest::new(Method::GET, &url)
        .bearer_auth(&config.tenant_access_token);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<QueryFilterViewsResponse> =
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

/// 查询筛选视图构建器
pub struct QueryFilterViewsBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
}

impl<'a> QueryFilterViewsBuilder<'a> {
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

    /// 执行查询筛选视图请求
    pub async fn execute(self) -> SDKResult<QueryFilterViewsResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        query_filter_views(self.config, &spreadsheet_token, &sheet_id).await
    }
}

/// 查询筛选视图的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回查询筛选视图的构建器
pub fn query_filter_views_builder(config: &Config) -> QueryFilterViewsBuilder {
    QueryFilterViewsBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_filter_views_builder() {
        let config = Config::default();
        let builder = query_filter_views_builder(&config)
            .spreadsheet_token("sheet_token_123")
            .sheet_id("sheet_id_456");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
    }

    #[test]
    fn test_missing_parameters() {
        let config = Config::default();
        let builder = query_filter_views_builder(&config);

        assert!(builder.spreadsheet_token.is_none());
        assert!(builder.sheet_id.is_none());
    }
}
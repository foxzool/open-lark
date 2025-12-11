//! 更新筛选视图
//!
//! 更新筛选视图的名字或者筛选范围。名字长度不超过100，不能重复即子表内唯一；筛选范围不超过子表的最大范围。
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/patch

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
    UpdateFilterViewRequest, FilterViewInfo, SpreadsheetToken, SheetId, FilterViewId, SheetsResponse
};

/// 更新筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFilterViewResponse {
    /// 筛选视图信息
    pub filter_view: FilterViewInfo,
}

impl ApiResponseTrait for UpdateFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新筛选视图构建器
pub struct UpdateFilterViewBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    filter_view_id: Option<FilterViewId>,
    request: UpdateFilterViewRequest,
}

impl<'a> UpdateFilterViewBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            filter_view_id: None,
            request: UpdateFilterViewRequest {
                name: None,
                range: None,
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

    /// 设置筛选视图ID
    pub fn filter_view_id(mut self, id: impl Into<String>) -> Self {
        self.filter_view_id = Some(id.into());
        self
    }

    /// 设置筛选视图名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let name_str = name.into();
        if name_str.len() > 100 {
            return Err(LarkAPIError::IllegalParamError("筛选视图名称长度不能超过100".to_string()));
        }
        self.request.name = Some(name_str);
        self
    }

    /// 设置筛选范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.request.range = Some(range.into());
        self
    }

    /// 执行更新筛选视图请求
    pub async fn execute(self) -> SDKResult<UpdateFilterViewResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let filter_view_id = self.filter_view_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("筛选视图ID不能为空".to_string())
        })?;

        // 验证至少有一个更新字段
        if self.request.name.is_none() && self.request.range.is_none() {
            return Err(LarkAPIError::IllegalParamError("至少需要更新筛选视图名称或范围".to_string()));
        }

        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}",
            self.config.base_url, spreadsheet_token, sheet_id, filter_view_id
        );

        let body = serde_json::to_value(&self.request).map_err(|e| {
            LarkAPIError::IllegalParamError(format!("请求体序列化失败: {}", e))
        })?;

        let mut api_request = ApiRequest::new(Method::PATCH, &url)
            .body(body)
            .bearer_auth(&config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<UpdateFilterViewResponse> =
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

/// 更新筛选视图
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
/// * `filter_view_id` - 筛选视图ID
///
/// # 返回
/// 返回更新筛选视图的构建器
///
/// # 示例
/// ```rust
/// let response = update_filter_view(&config, "sheet_token_123", "sheet_id_456", "fv_abc123")
///     .name("新的筛选视图名称")
///     .range("A1:F20")
///     .execute()
///     .await?;
/// ```
pub fn update_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    filter_view_id: &str,
) -> UpdateFilterViewBuilder {
    UpdateFilterViewBuilder::new(config)
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
        .filter_view_id(filter_view_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_filter_view_builder() {
        let config = Config::default();
        let builder = update_filter_view(&config, "sheet_token_123", "sheet_id_456", "fv_abc123")
            .name("新的名称")
            .range("A1:F20");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.filter_view_id, Some("fv_abc123".to_string()));
        assert_eq!(builder.request.name, Some("新的名称".to_string()));
        assert_eq!(builder.request.range, Some("A1:F20".to_string()));
    }

    #[test]
    fn test_name_length_validation() {
        let config = Config::default();
        let long_name = "a".repeat(101);
        let result = update_filter_view(&config, "sheet_token_123", "sheet_id_456", "fv_abc123")
            .name(long_name);

        // 长度验证会返回错误
        match result {
            Ok(_) => panic!("应该返回错误"),
            Err(_) => (), // 期望错误
        }
    }
}
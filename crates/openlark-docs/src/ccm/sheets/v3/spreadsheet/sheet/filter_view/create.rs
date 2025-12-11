//! 创建筛选视图
//!
//! 根据传入的参数创建一个筛选视图。Id 和 名字可选，不填的话会默认生成；range 必填。Id 长度为10，由 0-9、a-z、A-Z 组合生成。名字长度不超过100。单个子表内的筛选视图个数不超过 150。
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter_view/create

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
    CreateFilterViewRequest, FilterViewInfo, SpreadsheetToken, SheetId, SheetsResponse
};

/// 创建筛选视图响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterViewResponse {
    /// 筛选视图信息
    pub filter_view: FilterViewInfo,
}

impl ApiResponseTrait for CreateFilterViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建筛选视图构建器
pub struct CreateFilterViewBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    request: CreateFilterViewRequest,
}

impl<'a> CreateFilterViewBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            request: CreateFilterViewRequest {
                filter_view_id: None,
                name: None,
                range: String::new(),
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

    /// 设置筛选视图ID (可选，不填自动生成)
    pub fn filter_view_id(mut self, id: impl Into<String>) -> Self {
        self.request.filter_view_id = Some(id.into());
        self
    }

    /// 设置筛选视图名称 (可选，不填自动生成)
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    /// 设置筛选范围 (必填)
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.request.range = range.into();
        self
    }

    /// 执行创建筛选视图请求
    pub async fn execute(self) -> SDKResult<CreateFilterViewResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        if self.request.range.is_empty() {
            return Err(LarkAPIError::IllegalParamError("筛选范围不能为空".to_string()));
        }

        // 验证筛选视图ID长度
        if let Some(ref filter_view_id) = self.request.filter_view_id {
            if filter_view_id.len() != 10 {
                return Err(LarkAPIError::IllegalParamError("筛选视图ID长度必须为10位".to_string()));
            }
        }

        // 验证筛选视图名称长度
        if let Some(ref name) = self.request.name {
            if name.len() > 100 {
                return Err(LarkAPIError::IllegalParamError("筛选视图名称长度不能超过100".to_string()));
            }
        }

        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views",
            self.config.base_url, spreadsheet_token, sheet_id
        );

        let body = serde_json::to_value(&self.request).map_err(|e| {
            LarkAPIError::IllegalParamError(format!("请求体序列化失败: {}", e))
        })?;

        let mut api_request = ApiRequest::new(Method::POST, &url)
            .body(body)
            .bearer_auth(&config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<CreateFilterViewResponse> =
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

/// 创建筛选视图
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
///
/// # 返回
/// 返回创建筛选视图的构建器
///
/// # 示例
/// ```rust
/// let response = create_filter_view(&config, "sheet_token_123", "sheet_id_456")
///     .range("A1:D10")
///     .name("销售数据筛选")
///     .filter_view_id("fv12345678")
///     .execute()
///     .await?;
/// ```
pub fn create_filter_view(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> CreateFilterViewBuilder {
    CreateFilterViewBuilder::new(config)
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_filter_view_builder() {
        let config = Config::default();
        let builder = create_filter_view(&config, "sheet_token_123", "sheet_id_456")
            .range("A1:D10")
            .name("测试筛选视图");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.request.range, "A1:D10");
        assert_eq!(builder.request.name, Some("测试筛选视图".to_string()));
    }

    #[test]
    fn test_invalid_filter_view_id_length() {
        let config = Config::default();
        let builder = create_filter_view(&config, "sheet_token_123", "sheet_id_456")
            .range("A1:D10")
            .filter_view_id("short");

        assert_eq!(builder.request.filter_view_id, Some("short".to_string()));
        // 长度验证会在execute时进行
    }
}
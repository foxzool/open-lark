//! 修改电子表格属性
//!
//! 该接口用于修改电子表格的属性
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/patch

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
    UpdateSpreadsheetRequest, SpreadsheetInfo, SheetsResponse
};

/// 修改电子表格属性响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetResponse {
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

impl ApiResponseTrait for UpdateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改电子表格属性构建器
pub struct UpdateSpreadsheetBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<String>,
    request: UpdateSpreadsheetRequest,
}

impl<'a> UpdateSpreadsheetBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            request: UpdateSpreadsheetRequest {
                title: None,
                time_zone: None,
                locale: None,
                properties: None,
            },
        }
    }

    /// 设置电子表格令牌
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    /// 设置电子表格标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.request.time_zone = Some(crate::ccm::sheets::v3::models::TimeZone {
            time_zone: time_zone.into(),
        });
        self
    }

    /// 设置语言
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.request.locale = Some(crate::ccm::sheets::v3::models::Locale {
            locale: locale.into(),
        });
        self
    }

    /// 设置自定义属性
    pub fn properties(mut self, properties: std::collections::HashMap<String, Value>) -> Self {
        self.request.properties = Some(properties);
        self
    }

    /// 执行修改电子表格属性请求
    pub async fn execute(self) -> SDKResult<UpdateSpreadsheetResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}",
            self.config.base_url, spreadsheet_token
        );

        let body = serde_json::to_value(&self.request).map_err(|e| {
            LarkAPIError::IllegalParamError(format!("请求体序列化失败: {}", e))
        })?;

        let mut api_request = ApiRequest::new(Method::PATCH, &url)
            .body(body)
            .bearer_auth(&config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<UpdateSpreadsheetResponse> =
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

/// 修改电子表格属性
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
///
/// # 返回
/// 返回修改电子表格属性的构建器
///
/// # 示例
/// ```rust
/// let response = update_spreadsheet(&config, "sheet_token_123")
///     .title("新标题")
///     .time_zone("Asia/Shanghai")
///     .locale("zh_CN")
///     .execute()
///     .await?;
/// ```
pub fn update_spreadsheet(
    config: &Config,
    spreadsheet_token: &str,
) -> UpdateSpreadsheetBuilder {
    UpdateSpreadsheetBuilder::new(config)
        .spreadsheet_token(spreadsheet_token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_spreadsheet_builder() {
        let config = Config::default();
        let builder = update_spreadsheet(&config, "sheet_token_123")
            .title("新标题")
            .time_zone("Asia/Shanghai")
            .locale("zh_CN");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.request.title, Some("新标题".to_string()));
        assert!(builder.request.time_zone.is_some());
        assert!(builder.request.locale.is_some());
    }
}
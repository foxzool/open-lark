//! 创建电子表格
//!
//! 在指定目录下创建表格
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/create

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
    CreateSpreadsheetRequest, SpreadsheetInfo, SheetsResponse
};

/// 创建电子表格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpreadsheetResponse {
    /// 电子表格令牌
    pub spreadsheet_token: String,
    /// 电子表格标题
    pub title: String,
    /// 创建时间
    pub create_time: String,
    /// 时区
    pub time_zone: Option<String>,
    /// 语言
    pub locale: Option<String>,
    /// 工作表列表
    pub sheets: Option<Vec<SpreadsheetInfo>>,
}

impl ApiResponseTrait for CreateSpreadsheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建电子表格构建器
pub struct CreateSpreadsheetBuilder<'a> {
    config: &'a Config,
    request: CreateSpreadsheetRequest,
}

impl<'a> CreateSpreadsheetBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            request: CreateSpreadsheetRequest::default(),
        }
    }

    /// 设置电子表格标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = title.into();
        self
    }

    /// 设置工作表配置
    pub fn sheets(mut self, sheets: Vec<crate::ccm::sheets::v3::models::SheetProperty>) -> Self {
        self.request.sheets = Some(sheets);
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

    /// 设置文件夹路径
    pub fn folder_path(mut self, folder_path: impl Into<String>) -> Self {
        self.request.folder_path = Some(folder_path.into());
        self
    }

    /// 设置自定义属性
    pub fn properties(mut self, properties: std::collections::HashMap<String, Value>) -> Self {
        self.request.properties = Some(properties);
        self
    }

    /// 执行创建电子表格请求
    pub async fn execute(self) -> SDKResult<CreateSpreadsheetResponse> {
        let url = format!("{}/open-apis/sheets/v3/spreadsheets", self.config.base_url);

        let body = serde_json::to_value(&self.request).map_err(|e| {
            LarkAPIError::IllegalParamError(format!("请求体序列化失败: {}", e))
        })?;

        let mut api_request = ApiRequest::new(Method::POST, &url)
            .body(body)
            .bearer_auth(&self.config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<CreateSpreadsheetResponse> =
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

/// 创建电子表格
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回创建电子表格的构建器
///
/// # 示例
/// ```rust
/// let response = create_spreadsheet(&config)
///     .title("新表格")
///     .time_zone("Asia/Shanghai")
///     .locale("zh_CN")
///     .execute()
///     .await?;
/// ```
pub fn create_spreadsheet(config: &Config) -> CreateSpreadsheetBuilder {
    CreateSpreadsheetBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_spreadsheet_builder() {
        let config = Config::default();
        let builder = create_spreadsheet(&config)
            .title("测试表格")
            .time_zone("Asia/Shanghai")
            .locale("zh_CN");

        assert_eq!(builder.request.title, "测试表格");
        assert!(builder.request.time_zone.is_some());
        assert!(builder.request.locale.is_some());
    }
}
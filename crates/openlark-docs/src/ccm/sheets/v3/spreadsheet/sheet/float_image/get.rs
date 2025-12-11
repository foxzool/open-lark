//! 获取浮动图片
//!
//! 根据 float_image_id 获取对应浮动图片的信息
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/get

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

use crate::ccm::sheets::v3::models::{
    FloatImageInfo, SpreadsheetToken, SheetId, SheetsResponse
};

/// 获取浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFloatImageResponse {
    /// 浮动图片信息
    pub float_image: FloatImageInfo,
}

impl ApiResponseTrait for GetFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取浮动图片构建器
pub struct GetFloatImageBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    float_image_id: Option<String>,
}

impl<'a> GetFloatImageBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            float_image_id: None,
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

    /// 设置浮动图片ID
    pub fn float_image_id(mut self, id: impl Into<String>) -> Self {
        self.float_image_id = Some(id.into());
        self
    }

    /// 执行获取浮动图片请求
    pub async fn execute(self) -> SDKResult<GetFloatImageResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let float_image_id = self.float_image_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("浮动图片ID不能为空".to_string())
        })?;

        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}",
            self.config.base_url, spreadsheet_token, sheet_id, float_image_id
        );

        let mut api_request = ApiRequest::new(Method::GET, &url)
            .bearer_auth(&self.config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<GetFloatImageResponse> =
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

/// 获取浮动图片
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
/// * `float_image_id` - 浮动图片ID
///
/// # 返回
/// 返回浮动图片信息
///
/// # 示例
/// ```rust
/// let response = get_float_image(&config, "sheet_token_123", "sheet_id_456", "img_abc123")
///     .await?;
/// println!("图片位置: ({}, {})", response.float_image.row_index, response.float_image.column_index);
/// ```
pub async fn get_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
) -> SDKResult<GetFloatImageResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}",
        config.base_url, spreadsheet_token, sheet_id, float_image_id
    );

    let mut api_request = ApiRequest::new(Method::GET, &url)
        .bearer_auth(&self.config.tenant_access_token);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<GetFloatImageResponse> =
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

/// 获取浮动图片的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回获取浮动图片的构建器
pub fn get_float_image_builder(config: &Config) -> GetFloatImageBuilder {
    GetFloatImageBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_float_image_builder() {
        let config = Config::default();
        let builder = get_float_image_builder(&config)
            .spreadsheet_token("sheet_token_123")
            .sheet_id("sheet_id_456")
            .float_image_id("img_abc123");

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.float_image_id, Some("img_abc123".to_string()));
    }

    #[test]
    fn test_missing_parameters() {
        let config = Config::default();
        let builder = get_float_image_builder(&config);

        assert!(builder.spreadsheet_token.is_none());
        assert!(builder.sheet_id.is_none());
        assert!(builder.float_image_id.is_none());
    }
}
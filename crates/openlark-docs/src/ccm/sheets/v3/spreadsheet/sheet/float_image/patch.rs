//! 更新浮动图片
//!
//! 更新工作表中的浮动图片信息
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/patch

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
    FloatImageInfo, SpreadsheetToken, SheetId, SheetsResponse
};

/// 更新浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageResponse {
    /// 浮动图片信息
    pub float_image: FloatImageInfo,
}

impl ApiResponseTrait for UpdateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新浮动图片构建器
pub struct UpdateFloatImageBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    float_image_id: Option<String>,
    request: Value,
}

impl<'a> UpdateFloatImageBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            float_image_id: None,
            request: serde_json::json!({}),
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

    /// 设置浮动图片信息
    pub fn float_image(mut self, float_image: &FloatImageInfo) -> SDKResult<Self> {
        let image_json = serde_json::to_value(float_image).map_err(|e| {
            LarkAPIError::IllegalParamError(format!("浮动图片信息序列化失败: {}", e))
        })?;

        if let Some(obj) = self.request.as_object_mut() {
            obj.insert("float_image".to_string(), image_json);
        }

        Ok(self)
    }

    /// 设置图片位置
    pub fn position(mut self, row_index: i32, column_index: i32) -> Self {
        if let Some(obj) = self.request.as_object_mut() {
            obj.insert("row_index".to_string(), Value::Number(serde_json::Number::from(row_index)));
            obj.insert("column_index".to_string(), Value::Number(serde_json::Number::from(column_index)));
        }
        self
    }

    /// 设置图片大小
    pub fn size(mut self, width: i32, height: i32) -> Self {
        if let Some(obj) = self.request.as_object_mut() {
            obj.insert("width".to_string(), Value::Number(serde_json::Number::from(width)));
            obj.insert("height".to_string(), Value::Number(serde_json::Number::from(height)));
        }
        self
    }

    /// 设置图片层级
    pub fn z_index(mut self, z_index: i32) -> Self {
        if let Some(obj) = self.request.as_object_mut() {
            obj.insert("z_index".to_string(), Value::Number(serde_json::Number::from(z_index)));
        }
        self
    }

    /// 执行更新浮动图片请求
    pub async fn execute(self) -> SDKResult<UpdateFloatImageResponse> {
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

        let mut api_request = ApiRequest::new(Method::PATCH, &url)
            .body(self.request)
            .bearer_auth(&self.config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<UpdateFloatImageResponse> =
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

/// 更新浮动图片
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
/// * `float_image_id` - 浮动图片ID
///
/// # 返回
/// 返回更新浮动图片的构建器
///
/// # 示例
/// ```rust
/// let response = update_float_image(&config, "sheet_token_123", "sheet_id_456", "img_abc123")
///     .position(2, 3)
///     .size(250, 200)
///     .z_index(10)
///     .execute()
///     .await?;
/// println!("更新后的位置: ({}, {})", response.float_image.row_index, response.float_image.column_index);
/// ```
pub fn update_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
) -> UpdateFloatImageBuilder {
    UpdateFloatImageBuilder::new(config)
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
        .float_image_id(float_image_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_float_image_builder() {
        let config = Config::default();
        let builder = update_float_image(&config, "sheet_token_123", "sheet_id_456", "img_abc123")
            .position(2, 3)
            .size(250, 200)
            .z_index(10);

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.float_image_id, Some("img_abc123".to_string()));
    }

    #[test]
    fn test_missing_parameters() {
        let config = Config::default();
        let builder = update_float_image(&config, "", "sheet_id_456", "img_abc123");

        assert!(builder.spreadsheet_token.is_some());
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.float_image_id, Some("img_abc123".to_string()));
    }
}
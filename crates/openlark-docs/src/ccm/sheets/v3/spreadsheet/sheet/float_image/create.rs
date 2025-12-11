//! 创建浮动图片
//!
//! 根据传入的参数创建一张浮动图片。Float_image_token（上传图片至表格后得到）和range（只支持一个单元格）必填。
//! Float_image_id 可选，不填的话会默认生成，长度为10，由 0-9、a-z、A-Z 组合生成。表格内不重复的图片（浮动图片+单元格图片）总数不超过4000。
//! width 和 height 为图片展示的宽高，可选，不填的话会使用图片的真实宽高。offset_x 和 offset_y 为图片左上角距离所在单元格左上角的偏移，可选，默认为 0。
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/create

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

/// 创建浮动图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageResponse {
    /// 浮动图片信息
    pub float_image: FloatImageInfo,
}

impl ApiResponseTrait for CreateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建浮动图片构建器
pub struct CreateFloatImageBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    request: Value,
}

impl<'a> CreateFloatImageBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
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

    /// 设置图片ID
    pub fn image_id(mut self, image_id: impl Into<String>) -> Self {
        if let Some(obj) = self.request.as_object_mut() {
            obj.insert("image_id".to_string(), Value::String(image_id.into()));
        }
        self
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

    /// 执行创建浮动图片请求
    pub async fn execute(self) -> SDKResult<CreateFloatImageResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images",
            self.config.base_url, spreadsheet_token, sheet_id
        );

        let mut api_request = ApiRequest::new(Method::POST, &url)
            .body(self.request)
            .bearer_auth(&self.config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<CreateFloatImageResponse> =
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

/// 创建浮动图片
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
///
/// # 返回
/// 返回创建浮动图片的构建器
///
/// # 示例
/// ```rust
/// let response = create_float_image(&config, "sheet_token_123", "sheet_id_456")
///     .image_id("img_abc123")
///     .position(1, 1)
///     .size(200, 150)
///     .execute()
///     .await?;
/// println!("图片ID: {}", response.float_image.float_image_id);
/// ```
pub fn create_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> CreateFloatImageBuilder {
    CreateFloatImageBuilder::new(config)
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_float_image_builder() {
        let config = Config::default();
        let builder = create_float_image(&config, "sheet_token_123", "sheet_id_456")
            .image_id("img_abc123")
            .position(1, 1)
            .size(200, 150);

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
    }

    #[test]
    fn test_missing_parameters() {
        let config = Config::default();
        let builder = create_float_image(&config, "", "sheet_id_456");

        assert!(builder.spreadsheet_token.is_some());
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
    }
}
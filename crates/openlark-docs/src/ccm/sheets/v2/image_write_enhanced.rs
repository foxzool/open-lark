//! Sheets v2 增强图片写入服务
//!
//! 提供飞书电子表格v2版本的高级图片写入功能，包括：
//! - 向电子表格中插入多种格式的图片（PNG、JPEG、GIF等）
//! - 精确的图片位置控制（像素级定位和Excel风格定位）
//! - 灵活的图片尺寸设置（原始尺寸、自定义尺寸、比例缩放）
//! - 批量图片插入和管理功能
//! - 图片压缩和优化选项
//! - 企业级错误处理和数据验证
//! - 构建器模式支持，提供流畅API设计
use serde_json::Value;

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints_original::Endpoints,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use openlark_core::trait_system::Service;
// use openlark_core::SDKResult;

/// 图片位置类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageAnchorType {
    /// 锚定到单元格
    #[serde(rename = "CELL")]
    Cell,
    /// 锚定到工作表
    #[serde(rename = "SHEET")]
    Sheet,
}

/// 图片锚定位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnchor {
    /// 锚定类型
    pub r#type: ImageAnchorType,
    /// 锚定单元格（当type为CELL时必需）
    /// 支持Excel风格格式，如 "A1", "B2", "Sheet1!A1"
    pub cell: Option<String>,
    /// 工作表ID（当type为SHEET时必需）
    pub sheet_id: Option<String>,
    /// X轴偏移量（像素）
    /// 正数向右，负数向左
    pub offset_x: Option<i32>,
    /// Y轴偏移量（像素）
    /// 正数向下，负数向上
    pub offset_y: Option<i32>,
    /// 宽度（像素）
    pub width: Option<i32>,
    /// 高度（像素）
    pub height: Option<i32>,
}

impl ImageAnchor {
    /// 创建单元格锚定
    pub fn cell(cell: impl Into<String>) -> Self {
        Self {
            r#type: ImageAnchorType::Cell,
            cell: Some(cell.into()),
            sheet_id: None,
            offset_x: Some(0),
            offset_y: Some(0),
            width: None,
            height: None,
        }
    }

    /// 创建工作表锚定
    pub fn sheet(sheet_id: impl Into<String>) -> Self {
        Self {
            r#type: ImageAnchorType::Sheet,
            cell: None,
            sheet_id: Some(sheet_id.into()),
            offset_x: Some(0),
            offset_y: Some(0),
            width: None,
            height: None,
        }
    }

    /// 设置X偏移
    pub fn offset_x(mut self, offset_x: i32) -> Self {
        self.offset_x = Some(offset_x);
        self
    }

    /// 设置Y偏移
    pub fn offset_y(mut self, offset_y: i32) -> Self {
        self.offset_y = Some(offset_y);
        self
    }

    /// 设置尺寸
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// 验证锚定配置
    pub fn validate(&self) -> SDKResult<()> {
        match self.r#type {
            ImageAnchorType::Cell => {
                if self.cell.is_none() || self.cell.as_ref().unwrap().trim().is_empty() {
                    return Err(LarkAPIError::InvalidParameter(
                        "单元格锚定类型必须指定有效的单元格位置".to_string(),
                    ));
                }
            }
            ImageAnchorType::Sheet => {
                if self.sheet_id.is_none() || self.sheet_id.as_ref().unwrap().trim().is_empty() {
                    return Err(LarkAPIError::InvalidParameter(
                        "工作表锚定类型必须指定有效的工作表ID".to_string(),
                    ));
                }
            }
        }

        // 验证偏移量范围
        if let Some(offset_x) = self.offset_x {
            if offset_x < -10000 || offset_x > 10000 {
                return Err(LarkAPIError::InvalidParameter(
                    "X偏移量必须在-10000到10000像素之间".to_string(),
                ));
            }
        }

        if let Some(offset_y) = self.offset_y {
            if offset_y < -10000 || offset_y > 10000 {
                return Err(LarkAPIError::InvalidParameter(
                    "Y偏移量必须在-10000到10000像素之间".to_string(),
                ));
            }
        }

        // 验证尺寸
        if let Some(width) = self.width {
            if width <= 0 || width > 20000 {
                return Err(LarkAPIError::InvalidParameter(
                    "图片宽度必须在1到20000像素之间".to_string(),
                ));
            }
        }

        if let Some(height) = self.height {
            if height <= 0 || height > 20000 {
                return Err(LarkAPIError::InvalidParameter(
                    "图片高度必须在1到20000像素之间".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// 图片来源类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageSourceType {
    /// URL链接
    #[serde(rename = "URL")]
    Url,
    /// 文件ID
    #[serde(rename = "FILE_ID")]
    FileId,
    /// Base64编码数据
    #[serde(rename = "BASE64")]
    Base64,
}

/// 图片数据源
#[derive(Clone, Debug)]
pub struct ImageSource {
    /// 来源类型
    pub r#type: ImageSourceType,
    /// 图片数据
    pub data: String,
    /// MIME类型（当type为BASE64时需要）
    pub mime_type: Option<String>,
}

impl ImageSource {
    /// 从URL创建图片源
    pub fn url(url: impl Into<String>) -> Self {
        Self {
            r#type: ImageSourceType::Url,
            data: url.into(),
            mime_type: None,
        }
    }

    /// 从文件ID创建图片源
    pub fn file_id(file_id: impl Into<String>) -> Self {
        Self {
            r#type: ImageSourceType::FileId,
            data: file_id.into(),
            mime_type: None,
        }
    }

    /// 从Base64数据创建图片源
    pub fn base64(data: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Self {
            r#type: ImageSourceType::Base64,
            data: data.into(),
            mime_type: Some(mime_type.into()),
        }
    }

    /// 验证图片源
    pub fn validate(&self) -> SDKResult<()> {
        if self.data.trim().is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "图片数据不能为空".to_string(),
            ));
        }

        match self.r#type {
            ImageSourceType::Url => {
                // 基本URL格式验证
                if !self.data.starts_with("http://") && !self.data.starts_with("https://") {
                    return Err(LarkAPIError::InvalidParameter(
                        "无效的图片URL格式".to_string(),
                    ));
                }
            }
            ImageSourceType::FileId => {
                // 文件ID格式验证
                if self.data.len() < 10 || self.data.len() > 100 {
                    return Err(LarkAPIError::InvalidParameter("文件ID长度无效".to_string()));
                }
            }
            ImageSourceType::Base64 => {
                if self.mime_type.is_none() || self.mime_type.as_ref().unwrap().trim().is_empty() {
                    return Err(LarkAPIError::InvalidParameter(
                        "Base64图片必须指定有效的MIME类型".to_string(),
                    ));
                }

                // 验证MIME类型
                let valid_mime_types = vec![
                    "image/png",
                    "image/jpeg",
                    "image/jpg",
                    "image/gif",
                    "image/bmp",
                    "image/webp",
                ];
                let mime_type = self.mime_type.as_ref().unwrap();
                if !valid_mime_types.contains(&mime_type.as_str()) {
                    return Err(LarkAPIError::InvalidParameter(format!(
                        "不支持的图片格式: {}",
                        mime_type
                    )));
                }
            }
        }

        Ok(())
    }
}

/// 图片写入选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageWriteOptions {
    /// 是否保持宽高比
    pub maintain_aspect_ratio: Option<bool>,
    /// 图片质量（0.0-1.0，仅对JPEG有效）
    pub quality: Option<f32>,
    /// 是否压缩图片
    pub compress: Option<bool>,
    /// 图片透明度（0.0-1.0）
    pub opacity: Option<f32>,
}

impl Default for ImageWriteOptions {
    fn default() -> Self {
        Self {
            maintain_aspect_ratio: Some(true),
            quality: Some(0.8),
            compress: Some(true),
            opacity: Some(1.0),
        }
    }
}

impl ImageWriteOptions {
    /// 创建默认选项
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置是否保持宽高比
    pub fn maintain_aspect_ratio(mut self, maintain: bool) -> Self {
        self.maintain_aspect_ratio = Some(maintain);
        self
    }

    /// 设置图片质量
    pub fn quality(mut self, quality: f32) -> Self {
        if quality >= 0.0 && quality <= 1.0 {
            self.quality = Some(quality);
        }
        self
    }

    /// 设置是否压缩
    pub fn compress(mut self, compress: bool) -> Self {
        self.compress = Some(compress);
        self
    }

    /// 设置透明度
    pub fn opacity(mut self, opacity: f32) -> Self {
        if opacity >= 0.0 && opacity <= 1.0 {
            self.opacity = Some(opacity);
        }
        self
    }

    /// 验证选项
    pub fn validate(&self) -> SDKResult<()> {
        if let Some(quality) = self.quality {
            if quality < 0.0 || quality > 1.0 {
                return Err(LarkAPIError::InvalidParameter(
                    "图片质量必须在0.0到1.0之间".to_string(),
                ));
            }
        }

        if let Some(opacity) = self.opacity {
            if opacity < 0.0 || opacity > 1.0 {
                return Err(LarkAPIError::InvalidParameter(
                    "图片透明度必须在0.0到1.0之间".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// 图片写入请求
#[derive(Clone, Debug)]
pub struct ImageWriteEnhancedRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 图片锚定位置
    pub anchor: ImageAnchor,
    /// 图片数据源
    pub image_source: ImageSource,
    /// 写入选项
    pub options: Option<ImageWriteOptions>,
}

impl Default for ImageWriteEnhancedRequest {
    fn default() -> Self {
        Self {
            spreadsheet_token: String::new(),
            anchor: ImageAnchor::cell("A1"),
            image_source: ImageSource::url(""),
            options: Some(ImageWriteOptions::new()),
        }
    }
}

impl ImageWriteEnhancedRequest {
    /// 创建新的图片写入请求
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            
        }
    }

    /// 设置图片锚定位置
    pub fn anchor(mut self, anchor: ImageAnchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// 设置图片源
    pub fn image_source(mut self, source: ImageSource) -> Self {
        self.image_source = source;
        self
    }

    /// 设置写入选项
    pub fn options(mut self, options: ImageWriteOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证电子表格token
        if self.spreadsheet_token.trim().is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "电子表格token不能为空".to_string(),
            ));
        }

        // 验证锚定位置
        self.anchor.validate()?;

        // 验证图片源
        self.image_source.validate()?;

        // 验证选项
        if let Some(options) = &self.options {
            options.validate()?;
        }

        Ok(())
    }

    /// 转换为API请求体
    pub fn to_request_body(&self) -> SDKResult<Value> {
        let mut body = serde_json::Map::new();

        // 添加锚定位置
        let anchor_value = serde_json::to_value(&self.anchor)
            .map_err(|e| LarkAPIError::InvalidParameter(format!("锚定位置序列化失败: {}", e)))?;
        body.insert("anchor".to_string(), anchor_value);

        // 添加图片源
        let source_data = serde_json::Map::new();
        source_data.insert(
            "type".to_string(),
            Value::String(match self.image_source.r#type {
                ImageSourceType::Url => "URL",
                ImageSourceType::FileId => "FILE_ID",
                ImageSourceType::Base64 => "BASE64",
            }),
        );
        source_data.insert(
            "data".to_string(),
            Value::String(self.image_source.data.clone()),
        );

        if let Some(mime_type) = &self.image_source.mime_type {
            source_data.insert("mimeType".to_string(), Value::String(mime_type.clone()));
        }

        body.insert("imageSource".to_string(), Value::Object(source_data));

        // 添加选项
        if let Some(options) = &self.options {
            let options_value = serde_json::to_value(options).map_err(|e| {
                LarkAPIError::InvalidParameter(format!("图片选项序列化失败: {}", e))
            })?;
            body.insert("options".to_string(), options_value);
        }

        Ok(Value::Object(body))
    }
}

/// 图片写入响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageWriteEnhancedResponse {
    /// 图片ID
    pub image_id: Option<String>,
    /// 图片URL
    pub image_url: Option<String>,
    /// 图片宽度
    pub width: Option<i32>,
    /// 图片高度
    pub height: Option<i32>,
    /// 写入的位置范围
    pub range: Option<String>,
    /// 电子表格ID
    pub spreadsheet_id: Option<String>,
    /// 工作表ID
    pub sheet_id: Option<String>,
}

/// API响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageWriteEnhancedResponseBody {
    /// 写入结果
    pub data: ImageWriteEnhancedResponse,
}

// 移除重复的BaseResponse定义，使用openlark_core中的版本

/// 增强图片写入服务
#[derive(Clone, Debug)]
pub struct ImageWriteEnhancedService {
    config: Config,
}

impl ImageWriteEnhancedService {
    /// 创建增强图片写入服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 写入图片到电子表格
    ///
    /// # 参数
    /// - `request`: 图片写入请求
    ///
    /// # 返回
    /// 图片写入结果，包括图片ID、URL和位置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::image_write_enhanced::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = ImageWriteEnhancedService::new(config);
    ///
    /// // 使用URL图片
    /// let request = ImageWriteEnhancedRequest::new("spreadsheet_token")
    ///     .image_source(ImageSource::url("https://example.com/image.png"))
    ///     .anchor(ImageAnchor::cell("B2").offset_x(10).offset_y(5).size(200, 150))
    ///     .options(ImageWriteOptions::new().quality(0.9).maintain_aspect_ratio(true));
    ///
    /// // 使用Base64图片
    /// let base64_data = "iVBORw0KGgoAAAANSUhEUgAA...";
    /// let request = ImageWriteEnhancedRequest::new("spreadsheet_token")
    ///     .image_source(ImageSource::base64(base64_data, "image/png"))
    ///     .anchor(ImageAnchor::cell("C3"));
    /// ```
    pub async fn write_image(
        &self,
        request: ImageWriteEnhancedRequest,
    ) -> SDKResult<ImageWriteEnhancedResponse> {
        // 验证请求参数
        request.validate()?;

        // 构建请求体
        let body = request.to_request_body()?;

        // 构建URL
        let url = format!(
            "{}/open-apis/sheets/v2/spreadsheets/{}/values_image",
            self.config.base_url, request.spreadsheet_token
        );

        // 发送HTTP请求
        let response = self
            .config
            .transport
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LarkAPIError::NetworkError(format!("网络请求失败: {}", e)))?;

        // 处理响应
        if response.status().is_success() {
            let base_response: Response<ImageWriteEnhancedResponseBody> = response
                .json()
                .await
                .map_err(|e| LarkAPIError::JsonParseError(format!("响应解析失败: {}", e)))?;

            if base_response.code == 0 {
                Ok(base_response.data.data)
            } else {
                Err(LarkAPIError::APIError(
                    base_response.code,
                    base_response.msg,
                ))
            }
        } else {
            Err(LarkAPIError::HTTPError(
                response.status().as_u16(),
                "图片写入失败".to_string(),
            ))
        }
    }

    /// 创建图片写入构建器
    pub fn write_builder(&self) -> ImageWriteEnhancedBuilder {
        ImageWriteEnhancedBuilder::new(self.clone())
    }

    /// 快速写入URL图片
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `cell`: 锚定单元格，如 "A1", "B2"
    /// - `image_url`: 图片URL
    ///
    /// # 返回
    /// 图片写入结果
    pub async fn write_url_image(
        &self,
        spreadsheet_token: impl Into<String>,
        cell: impl Into<String>,
        image_url: impl Into<String>,
    ) -> SDKResult<ImageWriteEnhancedResponse> {
        let request = ImageWriteEnhancedRequest::new(spreadsheet_token)
            .image_source(ImageSource::url(image_url))
            .anchor(ImageAnchor::cell(cell));

        self.write_image(request).await
    }

    /// 快速写入Base64图片
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `cell`: 锚定单元格，如 "A1", "B2"
    /// - `base64_data`: Base64编码的图片数据
    /// - `mime_type`: 图片MIME类型，如 "image/png", "image/jpeg"
    ///
    /// # 返回
    /// 图片写入结果
    pub async fn write_base64_image(
        &self,
        spreadsheet_token: impl Into<String>,
        cell: impl Into<String>,
        base64_data: impl Into<String>,
        mime_type: impl Into<String>,
    ) -> SDKResult<ImageWriteEnhancedResponse> {
        let request = ImageWriteEnhancedRequest::new(spreadsheet_token)
            .image_source(ImageSource::base64(base64_data, mime_type))
            .anchor(ImageAnchor::cell(cell));

        self.write_image(request).await
    }

    /// 批量写入图片
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `images`: 图片列表，每个图片包含位置和源信息
    ///
    /// # 返回
    /// 所有图片的写入结果
    pub async fn batch_write_images(
        &self,
        spreadsheet_token: impl Into<String>,
        images: Vec<(ImageAnchor, ImageSource)>,
    ) -> SDKResult<Vec<ImageWriteEnhancedResponse>> {
        let mut results = vec![];

        for (anchor, image_source) in images {
            let request = ImageWriteEnhancedRequest::new(&spreadsheet_token)
                .anchor(anchor)
                .image_source(image_source);

            match self.write_image(request).await {
                Ok(response) => results.push(response),
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }
}

impl Service for ImageWriteEnhancedService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ImageWriteEnhancedService"
    }
}

/// 图片写入构建器
pub struct ImageWriteEnhancedBuilder {
    service: ImageWriteEnhancedService,
    spreadsheet_token: Option<String>,
    anchor: Option<ImageAnchor>,
    image_source: Option<ImageSource>,
    options: Option<ImageWriteOptions>,
}

impl ImageWriteEnhancedBuilder {
    /// 创建新的构建器
    pub fn new(service: ImageWriteEnhancedService) -> Self {
        Self {
            service,
            spreadsheet_token: None,
            anchor: None,
            image_source: None,
            options: Some(ImageWriteOptions::new()),
        }
    }

    /// 设置电子表格token
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    /// 设置图片锚定位置
    pub fn anchor(mut self, anchor: ImageAnchor) -> Self {
        self.anchor = Some(anchor);
        self
    }

    /// 设置图片源
    pub fn image_source(mut self, source: ImageSource) -> Self {
        self.image_source = Some(source);
        self
    }

    /// 设置图片选项
    pub fn options(mut self, options: ImageWriteOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// 从URL设置图片源
    pub fn image_url(mut self, url: impl Into<String>) -> Self {
        self.image_source = Some(ImageSource::url(url));
        self
    }

    /// 从文件ID设置图片源
    pub fn image_file_id(mut self, file_id: impl Into<String>) -> Self {
        self.image_source = Some(ImageSource::file_id(file_id));
        self
    }

    /// 从Base64设置图片源
    pub fn image_base64(mut self, data: impl Into<String>, mime_type: impl Into<String>) -> Self {
        self.image_source = Some(ImageSource::base64(data, mime_type));
        self
    }

    /// 设置单元格定位
    pub fn cell(mut self, cell: impl Into<String>) -> Self {
        self.anchor = Some(ImageAnchor::cell(cell));
        self
    }

    /// 设置工作表定位
    pub fn sheet(mut self, sheet_id: impl Into<String>) -> Self {
        self.anchor = Some(ImageAnchor::sheet(sheet_id));
        self
    }

    /// 设置偏移量
    pub fn offset(mut self, x: i32, y: i32) -> Self {
        if let Some(anchor) = &mut self.anchor {
            anchor.offset_x = Some(x);
            anchor.offset_y = Some(y);
        }
        self
    }

    /// 设置图片尺寸
    pub fn size(mut self, width: i32, height: i32) -> Self {
        if let Some(anchor) = &mut self.anchor {
            anchor.width = Some(width);
            anchor.height = Some(height);
        }
        self
    }

    /// 设置图片质量
    pub fn quality(mut self, quality: f32) -> Self {
        if let Some(options) = &mut self.options {
            options.quality = Some(quality);
        }
        self
    }

    /// 设置是否保持宽高比
    pub fn maintain_aspect_ratio(mut self, maintain: bool) -> Self {
        if let Some(options) = &mut self.options {
            options.maintain_aspect_ratio = Some(maintain);
        }
        self
    }

    /// 设置透明度
    pub fn opacity(mut self, opacity: f32) -> Self {
        if let Some(options) = &mut self.options {
            options.opacity = Some(opacity);
        }
        self
    }

    /// 执行图片写入操作
    pub async fn execute(self) -> SDKResult<ImageWriteEnhancedResponse> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::InvalidParameter("电子表格token不能为空".to_string()))?;

        let anchor = self
            .anchor
            .ok_or_else(|| LarkAPIError::InvalidParameter("必须指定图片锚定位置".to_string()))?;

        let image_source = self
            .image_source
            .ok_or_else(|| LarkAPIError::InvalidParameter("必须指定图片源".to_string()))?;

        let options = self.options.unwrap_or_else(ImageWriteOptions::new);

        let request = ImageWriteEnhancedRequest::new(spreadsheet_token)
            .anchor(anchor)
            .image_source(image_source)
            .options(options);

        self.service.write_image(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_anchor_validation() {
        // 测试单元格锚定
        let anchor = ImageAnchor::cell("A1");
        assert!(anchor.validate().is_ok());

        // 测试工作表锚定
        let anchor = ImageAnchor::sheet("sheet123");
        assert!(anchor.validate().is_ok());

        // 测试无效的单元格锚定
        let anchor = ImageAnchor {
            r#type: ImageAnchorType::Cell,
            cell: None,
            sheet_id: None,
            offset_x: None,
            offset_y: None,
            width: None,
            height: None,
        };
        assert!(anchor.validate().is_err());

        // 测试无效的工作表锚定
        let anchor = ImageAnchor {
            r#type: ImageAnchorType::Sheet,
            cell: None,
            sheet_id: None,
            offset_x: None,
            offset_y: None,
            width: None,
            height: None,
        };
        assert!(anchor.validate().is_err());

        // 测试偏移量范围
        let anchor = ImageAnchor::cell("A1").offset_x(-10001);
        assert!(anchor.validate().is_err());

        let anchor = ImageAnchor::cell("A1").offset_y(10001);
        assert!(anchor.validate().is_err());

        // 测试尺寸范围
        let anchor = ImageAnchor::cell("A1").size(0, 100);
        assert!(anchor.validate().is_err());

        let anchor = ImageAnchor::cell("A1").size(100, 20001);
        assert!(anchor.validate().is_err());
    }

    #[test]
    fn test_image_source_validation() {
        // 测试URL源
        let source = ImageSource::url("https://example.com/image.png");
        assert!(source.validate().is_ok());

        // 测试无效URL
        let source = ImageSource::url("invalid-url");
        assert!(source.validate().is_err());

        // 测试文件ID源
        let source = ImageSource::file_id("file_123456789");
        assert!(source.validate().is_ok());

        // 测试Base64源（有效MIME类型）
        let source = ImageSource::base64("data", "image/png");
        assert!(source.validate().is_ok());

        // 测试Base64源（无效MIME类型）
        let source = ImageSource::base64("data", "invalid/type");
        assert!(source.validate().is_err());

        // 测试空数据
        let source = ImageSource::url("");
        assert!(source.validate().is_err());
    }

    #[test]
    fn test_image_write_options_validation() {
        // 测试默认选项
        let options = ImageWriteOptions::new();
        assert!(options.validate().is_ok());

        // 测试无效质量
        let options = ImageWriteOptions::new().quality(-0.1);
        assert!(options.validate().is_err());

        let options = ImageWriteOptions::new().quality(1.1);
        assert!(options.validate().is_err());

        // 测试无效透明度
        let options = ImageWriteOptions::new().opacity(-0.1);
        assert!(options.validate().is_err());

        let options = ImageWriteOptions::new().opacity(1.1);
        assert!(options.validate().is_err());
    }

    #[test]
    fn test_image_write_enhanced_request_validation() {
        // 测试空token
        let request = ImageWriteEnhancedRequest::new("")
            .image_source(ImageSource::url("https://example.com/image.png"))
            .anchor(ImageAnchor::cell("A1"));
        assert!(request.validate().is_err());

        // 测试正常请求
        let request = ImageWriteEnhancedRequest::new("token")
            .image_source(ImageSource::url("https://example.com/image.png"))
            .anchor(ImageAnchor::cell("A1"))
            .options(ImageWriteOptions::new());
        assert!(request.validate().is_ok());

        // 测试无效的锚定位置
        let request = ImageWriteEnhancedRequest::new("token")
            .image_source(ImageSource::url("https://example.com/image.png"))
            .anchor(ImageAnchor {
                r#type: ImageAnchorType::Cell,
                cell: None,
                sheet_id: None,
                offset_x: Some(0),
                offset_y: Some(0),
                width: None,
                height: None,
            });
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_to_request_body() {
        let request = ImageWriteEnhancedRequest::new("token")
            .image_source(ImageSource::base64("data", "image/png"))
            .anchor(
                ImageAnchor::cell("A1")
                    .offset_x(10)
                    .offset_y(5)
                    .size(200, 150),
            )
            .options(
                ImageWriteOptions::new()
                    .quality(0.9)
                    .maintain_aspect_ratio(false),
            );

        let body = request.to_request_body().unwrap();

        assert!(body["anchor"].is_object());
        assert!(body["imageSource"].is_object());
        assert_eq!(body["anchor"]["type"], "CELL");
        assert_eq!(body["anchor"]["cell"], "A1");
        assert_eq!(body["anchor"]["offsetX"], 10);
        assert_eq!(body["anchor"]["offsetY"], 5);
        assert_eq!(body["anchor"]["width"], 200);
        assert_eq!(body["anchor"]["height"], 150);

        assert_eq!(body["imageSource"]["type"], "BASE64");
        assert_eq!(body["imageSource"]["data"], "data");
        assert_eq!(body["imageSource"]["mimeType"], "image/png");

        assert_eq!(body["options"]["maintainAspectRatio"], false);
        assert_eq!(body["options"]["quality"], 0.9);
    }

    #[test]
    fn test_builder_pattern() {
        let config = openlark_core::config::Config::default();
        let service = ImageWriteEnhancedService::new(config);

        let builder = service
            .write_builder()
            .spreadsheet_token("test_token")
            .image_url("https://example.com/image.png")
            .cell("B2")
            .offset(10, 5)
            .size(200, 150)
            .quality(0.9)
            .maintain_aspect_ratio(true)
            .opacity(0.8);

        // 验证构建器设置
        assert_eq!(builder.spreadsheet_token.as_ref().unwrap(), "test_token");
        assert!(matches!(builder.image_source, Some(ImageSource { .. })));
        assert!(matches!(builder.anchor, Some(ImageAnchor { .. })));
        assert_eq!(builder.options.as_ref().unwrap().quality, Some(0.9));
        assert_eq!(
            builder.options.as_ref().unwrap().maintain_aspect_ratio,
            Some(true)
        );
        assert_eq!(builder.options.as_ref().unwrap().opacity, Some(0.8));
    }

    #[test]
    fn test_image_source_types() {
        // 测试URL类型
        let source = ImageSource::url("https://example.com/image.png");
        assert!(matches!(source.r#type, ImageSourceType::Url));
        assert_eq!(source.data, "https://example.com/image.png");

        // 测试文件ID类型
        let source = ImageSource::file_id("file_123456789");
        assert!(matches!(source.r#type, ImageSourceType::FileId));
        assert_eq!(source.data, "file_123456789");

        // 测试Base64类型
        let source = ImageSource::base64("iVBORw0KGgoAAAANSUhEUgAA", "image/png");
        assert!(matches!(source.r#type, ImageSourceType::Base64));
        assert_eq!(source.data, "iVBORw0KGgoAAAANSUhEUgAA");
        assert_eq!(source.mime_type.as_ref().unwrap(), "image/png");
    }

    #[test]
    fn test_image_write_enhanced_service() {
        let config = openlark_core::config::Config::default();
        let service = ImageWriteEnhancedService::new(config);

        assert_eq!(service.service_name(), "ImageWriteEnhancedService");
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_complex_image_scenarios() {
        // 测试工作表级别的图片定位
        let anchor = ImageAnchor::sheet("sheet_123")
            .offset_x(100)
            .offset_y(50)
            .size(300, 200);

        let request = ImageWriteEnhancedRequest::new("token")
            .image_source(ImageSource::url("https://cdn.example.com/image.jpg"))
            .anchor(anchor)
            .options(
                ImageWriteOptions::new()
                    .quality(0.85)
                    .compress(true)
                    .opacity(0.9),
            );

        assert!(request.validate().is_ok());

        let body = request.to_request_body().unwrap();
        assert_eq!(body["anchor"]["type"], "SHEET");
        assert_eq!(body["anchor"]["sheetId"], "sheet_123");
        assert_eq!(body["anchor"]["offsetX"], 100);
        assert_eq!(body["anchor"]["offsetY"], 50);
        assert_eq!(body["anchor"]["width"], 300);
        assert_eq!(body["anchor"]["height"], 200);
        assert_eq!(body["options"]["quality"], 0.85);
        assert_eq!(body["options"]["compress"], true);
        assert_eq!(body["options"]["opacity"], 0.9);
    }

    #[test]
    fn test_batch_image_operations() {
        let config = openlark_core::config::Config::default();
        let service = ImageWriteEnhancedService::new(config);

        // 准备多个图片
        let images = vec![
            (
                ImageAnchor::cell("A1"),
                ImageSource::url("https://example.com/image1.png"),
            ),
            (ImageAnchor::cell("C3"), ImageSource::file_id("file_456")),
            (
                ImageAnchor::cell("E5").size(200, 150),
                ImageSource::base64("data1", "image/png"),
            ),
        ];

        // 验证数据准备
        assert_eq!(images.len(), 3);
        assert!(matches!(images[0].0.r#type, ImageAnchorType::Cell));
        assert!(matches!(images[1].1.r#type, ImageSourceType::FileId));
        assert!(matches!(images[2].1.r#type, ImageSourceType::Base64));

        // 这里不执行实际的批量写入，只验证数据结构
        // 实际测试需要mock HTTP响应
    }
}

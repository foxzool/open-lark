//! OCR 图片基础识别
//!
//! 专门针对图片的基础 OCR 识别服务，将图片中的文字转换为可编辑文本。
//!
//! docPath: https://open.feishu.cn/document/optical-char-recognition-v1/image_basic_recognize

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE;

/// OCR 图片基础识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageBasicRecognizeBody {
    /// 用户的 image_id，通过上传图片接口获取
    pub image_key: String,
    /// 是否需要文字检测框信息，默认值为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_rotation_correction: Option<bool>,
}

impl ImageBasicRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.image_key.trim().is_empty() {
            return Err("image_key 不能为空".to_string());
        }
        Ok(())
    }
}

/// OCR 图片基础识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageBasicRecognizeResponse {
    /// OCR 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ImageBasicRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for ImageBasicRecognizeResult {}

/// OCR 图片基础识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageBasicRecognizeResult {
    /// 识别到的文本片段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_blocks: Option<Vec<ImageTextBlock>>,
}

/// 图片文本块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTextBlock {
    /// 文本片段的起始坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_selection_start: Option<i32>,
    /// 文本片段的结束坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_selection_end: Option<i32>,
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 文本框坐标列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_boxes: Option<Vec<ImageTextBox>>,
}

/// 图片文本框坐标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTextBox {
    /// 左上角 x 坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i32>,
    /// 左上角 y 坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    /// 右下角 x 坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<i32>,
    /// 右下角 y 坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<i32>,
}

/// OCR 图片基础识别请求
#[derive(Debug, Clone)]
pub struct ImageBasicRecognizeRequest {
    config: Config,
}

impl ImageBasicRecognizeRequest {
    /// 创建新的 OCR 图片基础识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行 OCR 图片基础识别请求
    pub async fn execute(
        self,
        body: ImageBasicRecognizeBody,
    ) -> SDKResult<ImageBasicRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行 OCR 图片基础识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: ImageBasicRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<ImageBasicRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<ImageBasicRecognizeResponse> =
            ApiRequest::post(OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE)
                .body(serialize_params(&body, "OCR 图片基础识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "OCR 图片基础识别")
    }
}

/// OCR 图片基础识别请求构建器
#[derive(Debug, Clone)]
pub struct ImageBasicRecognizeRequestBuilder {
    request: ImageBasicRecognizeRequest,
    image_key: Option<String>,
    need_rotation_correction: Option<bool>,
}

impl ImageBasicRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: ImageBasicRecognizeRequest::new(config),
            image_key: None,
            need_rotation_correction: None,
        }
    }

    /// 设置图片 key
    pub fn image_key(mut self, image_key: impl Into<String>) -> Self {
        self.image_key = Some(image_key.into());
        self
    }

    /// 设置是否需要旋转校正
    pub fn need_rotation_correction(mut self, need: impl Into<bool>) -> Self {
        self.need_rotation_correction = Some(need.into());
        self
    }

    /// 构建请求体
    pub fn body(self) -> ImageBasicRecognizeBody {
        ImageBasicRecognizeBody {
            image_key: self.image_key.unwrap_or_default(),
            need_rotation_correction: self.need_rotation_correction,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ImageBasicRecognizeResponse> {
        let body = self.body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ImageBasicRecognizeResponse> {
        let body = self.body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行 OCR 图片基础识别
///
/// docPath: https://open.feishu.cn/document/optical-char-recognition-v1/image_basic_recognize
pub async fn image_basic_recognize(
    config: &Config,
    body: ImageBasicRecognizeBody,
) -> SDKResult<ImageBasicRecognizeResponse> {
    image_basic_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行 OCR 图片基础识别（支持自定义选项）
pub async fn image_basic_recognize_with_options(
    config: &Config,
    body: ImageBasicRecognizeBody,
    option: RequestOption,
) -> SDKResult<ImageBasicRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<ImageBasicRecognizeResponse> =
        ApiRequest::post(OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE)
            .body(serialize_params(&body, "OCR 图片基础识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "OCR 图片基础识别")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = ImageBasicRecognizeRequestBuilder::new(config.clone());

        assert!(builder.image_key.is_none());
        assert!(builder.need_rotation_correction.is_none());
    }

    #[test]
    fn test_builder_image_key() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = ImageBasicRecognizeRequestBuilder::new(config.clone())
            .image_key("image_key_123");

        assert_eq!(builder.image_key, Some("image_key_123".to_string()));
    }

    #[test]
    fn test_builder_need_rotation_correction() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = ImageBasicRecognizeRequestBuilder::new(config.clone())
            .need_rotation_correction(true);

        assert_eq!(builder.need_rotation_correction, Some(true));
    }

    #[test]
    fn test_body_validation_empty_image_key() {
        let body = ImageBasicRecognizeBody {
            image_key: "".to_string(),
            need_rotation_correction: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = ImageBasicRecognizeBody {
            image_key: "valid_key".to_string(),
            need_rotation_correction: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_from_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = ImageBasicRecognizeRequestBuilder::new(config.clone())
            .image_key("key_123")
            .need_rotation_correction(true)
            .body();

        assert_eq!(body.image_key, "key_123");
        assert_eq!(body.need_rotation_correction, Some(true));
    }
}

//! 上传图片
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/image/create

use openlark_core::{api::ApiRequest, config::Config, error, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    endpoints::IM_V1_IMAGES,
    im::im::v1::image::models::{CreateImageResponse, ImageType},
};

/// 上传图片请求
pub struct CreateImageRequest {
    config: Config,
    image_type: Option<ImageType>,
    file_name: Option<String>,
}

impl CreateImageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            image_type: None,
            file_name: None,
        }
    }

    /// 图片类型（必填）
    pub fn image_type(mut self, image_type: ImageType) -> Self {
        self.image_type = Some(image_type);
        self
    }

    /// 可选：设置上传文件名（仅用于 multipart 的文件名，不作为字段发送）
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口为 multipart 上传，请直接传入图片二进制内容。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/image/create
    pub async fn execute(self, image_bytes: Vec<u8>) -> SDKResult<CreateImageResponse> {
        self.execute_with_options(
            image_bytes,
            openlark_core::req_option::RequestOption::default(),
        )
        .await
    }

    pub async fn execute_with_options(
        self,
        image_bytes: Vec<u8>,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateImageResponse> {
        let image_type = self.image_type.ok_or_else(|| {
            error::validation_error(
                "image_type 不能为空".to_string(),
                "上传图片需要指定 image_type".to_string(),
            )
        })?;

        let mut body = serde_json::json!({
            "image_type": image_type.as_str(),
        });

        if let Some(file_name) = self.file_name {
            // 仅用于 multipart 文件名，不作为字段发送
            if let Some(obj) = body.as_object_mut() {
                obj.insert(
                    "__file_name".to_string(),
                    serde_json::Value::String(file_name),
                );
            }
        }

        // url: POST:/open-apis/im/v1/images
        let req: ApiRequest<CreateImageResponse> = ApiRequest::post(IM_V1_IMAGES)
            .body(body)
            .file_content(image_bytes);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "上传图片")
    }
}

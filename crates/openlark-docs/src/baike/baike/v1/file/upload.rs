//! 上传图片
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/file/upload

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 上传图片响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadFileResponse {
    pub file_token: String,
}

impl ApiResponseTrait for UploadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传图片请求
pub struct UploadFileRequest {
    config: Config,
    name: String,
    file: Vec<u8>,
}

impl UploadFileRequest {
    /// 创建上传图片请求
    ///
    /// `name`：表单字段 `name`，同时会用于 multipart 的文件名（内部字段 `file_name`）。
    pub fn new(config: Config, name: impl Into<String>, file: Vec<u8>) -> Self {
        Self {
            config,
            name: name.into(),
            file,
        }
    }

    pub async fn send(self) -> SDKResult<UploadFileResponse> {
        use crate::common::api_endpoints::BaikeApiV1;
        validate_required!(self.name, "name 不能为空");
        validate_required!(self.file, "file 不能为空");

        // multipart/form-data：
        // - name：文档要求的文件名称字段
        // - file：二进制文件内容（由 core 层 MultipartBuilder 处理）
        // - file_name：仅用于设置 multipart 的文件名（不会作为表单字段发送）
        let name = self.name;
        let file_name = name.clone();
        let body = serde_json::json!({
            "name": name,
            "file_name": file_name,
        });

        let api_request: ApiRequest<UploadFileResponse> =
            ApiRequest::post(&BaikeApiV1::FileUpload.to_url())
                .body(body)
                .file_content(self.file);

        let response: Response<UploadFileResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

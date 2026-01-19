//! 上传图片
//!
//! docPath: https://open.feishu.cn/document/lingo-v1/file/upload

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::LingoApiV1;

/// 上传图片响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileResp {
    /// 文件 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
}

impl ApiResponseTrait for UploadFileResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传图片请求（multipart/form-data）
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

    pub async fn execute(self) -> SDKResult<UploadFileResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<UploadFileResp> {
        validate_required!(self.name, "name 不能为空");
        validate_required!(self.file, "file 不能为空");

        // multipart/form-data：
        // - name：文档要求的文件名称字段
        // - file：二进制文件内容（由 core 层 MultipartBuilder 处理）
        // - __file_name：仅用于设置 multipart 的文件名（不会作为表单字段发送）
        let name = self.name;
        let body = serde_json::json!({
            "name": name,
            "__file_name": name,
        });

        let api_request: ApiRequest<UploadFileResp> =
            ApiRequest::post(&LingoApiV1::FileUpload.to_url())
                .body(body)
                .file_content(self.file);

        let response: Response<UploadFileResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

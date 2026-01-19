//! 上传文件
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/file/create

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_FILES,
    im::im::v1::file::models::CreateFileResponse,
};

/// 上传文件请求体（multipart 表单字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileBody {
    pub file_type: String,
    pub file_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

/// 上传文件请求
pub struct CreateFileRequest {
    config: Config,
}

impl CreateFileRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口为 multipart 上传，请传入文件元信息 + 文件二进制内容。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/file/create
    pub async fn execute(
        self,
        body: CreateFileBody,
        file_bytes: Vec<u8>,
    ) -> SDKResult<CreateFileResponse> {
        self.execute_with_options(body, file_bytes, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateFileBody,
        file_bytes: Vec<u8>,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateFileResponse> {
        validate_required!(body.file_type, "file_type 不能为空");
        validate_required!(body.file_name, "file_name 不能为空");
        if file_bytes.is_empty() {
            return Err(error::validation_error(
                "file 不能为空".to_string(),
                "不允许上传空文件".to_string(),
            ));
        }

        // url: POST:/open-apis/im/v1/files
        let req: ApiRequest<CreateFileResponse> = ApiRequest::post(IM_V1_FILES)
            .body(serialize_params(&body, "上传文件")?)
            .file_content(file_bytes);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "上传文件")
}
}

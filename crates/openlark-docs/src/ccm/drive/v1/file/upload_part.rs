use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 上传分片
///
/// 上传文件分片数据。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/upload_part
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 上传分片请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartRequest {
    /// 上传会话ID
    pub upload_id: String,
    /// 分片序号
    pub seq: i32,
    /// 分片大小
    pub size: i64,
}

impl UploadPartRequest {
    pub fn new(upload_id: impl Into<String>, seq: i32, size: i64) -> Self {
        Self {
            upload_id: upload_id.into(),
            seq,
            size,
        }
    }
}

/// 上传分片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartResponse {
    /// 是否成功
    pub success: Option<bool>,
}

impl ApiResponseTrait for UploadPartResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传分片
pub async fn upload_part(
    request: UploadPartRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UploadPartResponse>> {
    let api_endpoint = DriveApi::UploadPart;

    let mut api_request: ApiRequest<UploadPartResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serde_json::json!(&request));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}
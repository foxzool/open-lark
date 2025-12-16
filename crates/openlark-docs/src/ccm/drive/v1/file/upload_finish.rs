use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 完成分片上传
///
/// 完成文件分片上传。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/upload_finish
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 完成分片上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFinishRequest {
    /// 上传会话ID
    pub upload_id: String,
    /// 分片数量
    pub block_num: i32,
}

impl UploadFinishRequest {
    pub fn new(upload_id: impl Into<String>, block_num: i32) -> Self {
        Self {
            upload_id: upload_id.into(),
            block_num,
        }
    }
}

/// 完成分片上传响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFinishResponse {
    /// 文件token
    pub file_token: Option<String>,
}

impl ApiResponseTrait for UploadFinishResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 完成分片上传
pub async fn upload_finish(
    request: UploadFinishRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UploadFinishResponse>> {
    let api_endpoint = DriveApi::UploadFinish;

    let mut api_request: ApiRequest<UploadFinishResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serde_json::json!(&request));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}
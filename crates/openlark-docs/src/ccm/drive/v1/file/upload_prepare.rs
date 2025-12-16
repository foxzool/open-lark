use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 文件分片上传预备
///
/// 发送初始化请求获取上传会话ID，用于大文件分片上传。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/upload_prepare
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 分片上传预备请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareRequest {
    /// 文件名
    pub file_name: String,
    /// 父文件夹token
    pub parent_folder_token: String,
    /// 文件大小（字节）
    pub size: i64,
}

impl UploadPrepareRequest {
    pub fn new(file_name: impl Into<String>, parent_folder_token: impl Into<String>, size: i64) -> Self {
        Self {
            file_name: file_name.into(),
            parent_folder_token: parent_folder_token.into(),
            size,
        }
    }
}

/// 分片上传预备响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareResponse {
    /// 上传会话ID
    pub upload_id: Option<String>,
    /// 分片大小
    pub block_size: Option<i64>,
    /// 分片数量
    pub block_num: Option<i32>,
}

impl ApiResponseTrait for UploadPrepareResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文件分片上传预备
pub async fn upload_prepare(
    request: UploadPrepareRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UploadPrepareResponse>> {
    let api_endpoint = DriveApi::UploadPrepare;

    let mut api_request: ApiRequest<UploadPrepareResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serde_json::json!(&request));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}
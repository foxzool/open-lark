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
    #[serde(skip)]
    config: Config,
    /// 上传会话ID
    pub upload_id: String,
    /// 分片数量
    pub block_num: i32,
}

impl UploadFinishRequest {
    pub fn new(config: Config, upload_id: impl Into<String>, block_num: i32) -> Self {
        Self {
            config,
            upload_id: upload_id.into(),
            block_num,
        }
    }

    pub async fn execute(self) -> SDKResult<Response<UploadFinishResponse>> {
        let api_endpoint = DriveApi::UploadFinish;
        let request = ApiRequest::<UploadFinishResponse>::post(&api_endpoint.to_url())
            .json_body(&self);

        Transport::request(request, &self.config, None).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_finish_request_builder() {
        let config = Config::default();
        let request = UploadFinishRequest::new(config, "upload_id", 10);
        assert_eq!(request.upload_id, "upload_id");
        assert_eq!(request.block_num, 10);
    }
}
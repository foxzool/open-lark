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
    #[serde(skip)]
    config: Config,
    /// 上传会话ID
    pub upload_id: String,
    /// 分片序号
    pub seq: i32,
    /// 分片大小
    pub size: i64,
    /// 分片数据
    #[serde(skip)]
    pub data: Vec<u8>,
}

impl UploadPartRequest {
    pub fn new(config: Config, upload_id: impl Into<String>, seq: i32, size: i64, data: Vec<u8>) -> Self {
        Self {
            config,
            upload_id: upload_id.into(),
            seq,
            size,
            data,
        }
    }

    pub async fn execute(self) -> SDKResult<Response<UploadPartResponse>> {
        let api_endpoint = DriveApi::UploadPart;
        
        // Metadata for JSON body
        #[derive(Serialize)]
        struct PartMeta {
            upload_id: String,
            seq: i32,
            size: i64,
        }
        
        let meta = PartMeta {
            upload_id: self.upload_id,
            seq: self.seq,
            size: self.size,
        };

        let request = ApiRequest::<UploadPartResponse>::post(&api_endpoint.to_url())
            .json_body(&meta)
            .file_content(self.data);

        Transport::request(request, &self.config, None).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_part_request_builder() {
        let config = Config::default();
        let request = UploadPartRequest::new(config, "upload_id", 1, 1024, vec![0; 1024]);
        assert_eq!(request.upload_id, "upload_id");
        assert_eq!(request.seq, 1);
        assert_eq!(request.size, 1024);
        assert_eq!(request.data.len(), 1024);
    }
}
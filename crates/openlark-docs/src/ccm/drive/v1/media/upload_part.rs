/// 分片上传素材-上传分片
///
/// 上传对应的文件块。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_part
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 分片上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartRequest {
    #[serde(skip)]
    config: Config,
    /// 上传事务ID
    pub transaction_id: String,
    /// 分片编号
    pub part_number: i32,
    /// 分片数据（base64编码）
    pub part_data: String,
    /// 分片大小
    pub part_size: i32,
}

impl UploadPartRequest {
    pub fn new(
        config: Config,
        transaction_id: impl Into<String>,
        part_number: i32,
        part_data: impl Into<String>,
        part_size: i32,
    ) -> Self {
        Self {
            config,
            transaction_id: transaction_id.into(),
            part_number,
            part_data: part_data.into(),
            part_size,
        }
    }

    pub async fn execute(self) -> SDKResult<Response<UploadPartResponse>> {
        let url = "/open-apis/drive/v1/medias/upload_part";

        let api_request: ApiRequest<UploadPartResponse> = ApiRequest::post(url).json_body(&self);

        Transport::request(api_request, &self.config, None).await
    }
}

/// 分片上传响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartResponse {
    /// 分片编号
    pub part_number: i32,
    /// 上传状态
    pub status: String,
    /// ETag
    pub etag: Option<String>,
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
    fn test_upload_part_request() {
        let config = Config::default();
        let request = UploadPartRequest::new(
            config,
            "txn_media_123456",
            1,
            "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==",
            67
        );

        assert_eq!(request.transaction_id, "txn_media_123456");
        assert_eq!(request.part_number, 1);
        assert_eq!(request.part_size, 67);
    }

    #[test]
    fn test_upload_part_response() {
        let response = UploadPartResponse {
            part_number: 1,
            status: "completed".to_string(),
            etag: Some("etag_media_123456".to_string()),
        };

        assert_eq!(response.part_number, 1);
        assert_eq!(response.status, "completed");
        assert_eq!(response.etag, Some("etag_media_123456".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UploadPartResponse::data_format(), ResponseFormat::Data);
    }
}

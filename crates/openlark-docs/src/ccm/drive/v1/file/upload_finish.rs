//! 分片上传文件-完成上传
//!
//! 上传分片全部完成后，调用该接口触发完成上传。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_finish

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

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

    pub async fn execute(self) -> SDKResult<UploadFinishResponse> {
        if self.upload_id.is_empty() {
            return Err(openlark_core::error::validation_error(
                "upload_id",
                "upload_id 不能为空",
            ));
        }
        if self.block_num <= 0 {
            return Err(openlark_core::error::validation_error(
                "block_num",
                "block_num 必须为正整数",
            ));
        }

        let api_endpoint = DriveApi::UploadFinish;
        let request = ApiRequest::<UploadFinishResponse>::post(&api_endpoint.to_url())
            .body(serialize_params(&self, "分片上传文件-完成上传")?);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "分片上传文件-完成上传")
    }
}

/// 完成分片上传响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFinishResponse {
    /// 文件token
    pub file_token: String,
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

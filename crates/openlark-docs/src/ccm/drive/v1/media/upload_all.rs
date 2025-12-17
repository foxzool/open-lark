/// 上传文件
///
/// 向云空间指定目录下上传一个小文件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/upload_all

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};



/// 上传文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileRequest {
    #[serde(skip)]
    config: Config,
    /// 文件名
    pub file_name: String,
    /// 父节点类型：explorer, bitable_image, bitable_file
    pub parent_type: String,
    /// 父节点token
    pub parent_node: String,
    /// 文件大小
    pub size: usize,
    /// 额外参数
    pub extra: Option<String>,
    /// 文件内容
    #[serde(skip)]
    pub file: Vec<u8>,
}

impl UploadFileRequest {
    pub fn new(
        config: Config,
        file_name: impl Into<String>,
        parent_type: impl Into<String>,
        parent_node: impl Into<String>,
        size: usize,
        file: Vec<u8>
    ) -> Self {
        Self {
            config,
            file_name: file_name.into(),
            parent_type: parent_type.into(),
            parent_node: parent_node.into(),
            size,
            extra: None,
            file,
        }
    }

    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    pub async fn execute(self) -> SDKResult<Response<UploadFileResponse>> {
        let api_endpoint = crate::common::api_endpoints::DriveApi::UploadFile;
        
        #[derive(Serialize)]
        struct UploadMeta {
            file_name: String,
            parent_type: String,
            parent_node: String,
            size: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            extra: Option<String>,
        }

        let meta = UploadMeta {
            file_name: self.file_name,
            parent_type: self.parent_type,
            parent_node: self.parent_node,
            size: self.size,
            extra: self.extra,
        };

        let request = ApiRequest::<UploadFileResponse>::post(&api_endpoint.to_url())
            .json_body(&meta)
            .file_content(self.file);
        
        Transport::request(request, &self.config, None).await
    }
}

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileResponse {
    /// 上传结果信息
    pub data: Option<UploadFileData>,
}

/// 上传文件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileData {
    /// 文件token
    pub file_token: String,
    /// 文件名称
    pub name: String,
    /// 文件大小
    pub size: i64,
    /// 文件类型
    pub r#type: String,
    /// 上传时间
    pub created_time: String,
}

impl ApiResponseTrait for UploadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_file_request_builder() {
        let config = Config::default();
        let request = UploadFileRequest::new(config, "test.png", "explorer", "folder_token", 100, vec![0; 100])
            .extra("extra_info");

        assert_eq!(request.file_name, "test.png");
        assert_eq!(request.parent_type, "explorer");
        assert_eq!(request.extra, Some("extra_info".to_string()));
    }
}
/// 分片上传素材-预上传
///
/// 发送初始化请求获取上传事务ID和分块策略，目前是以4MB大小进行定长分片。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_prepare
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 分片上传素材-预上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareRequest {
    #[serde(skip)]
    config: Config,
    /// 父文件夹token
    pub parent_folder_token: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小（字节）
    pub size: i64,
    /// 文件类型（MIME）
    pub content_type: Option<String>,
}

impl UploadPrepareRequest {
    /// 创建分片上传素材-预上传请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `parent_folder_token` - 父文件夹token
    /// * `file_name` - 文件名
    /// * `size` - 文件大小
    pub fn new(
        config: Config,
        parent_folder_token: impl Into<String>,
        file_name: impl Into<String>,
        size: i64,
    ) -> Self {
        Self {
            config,
            parent_folder_token: parent_folder_token.into(),
            file_name: file_name.into(),
            size,
            content_type: None,
        }
    }

    /// 设置文件类型
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<Response<UploadPrepareResponse>> {
        // 构建API端点 - media upload prepare uses different endpoint than file upload prepare
        // Using manual path as per original code
        let url = "/open-apis/drive/v1/medias/upload_prepare";

        let api_request: ApiRequest<UploadPrepareResponse> = ApiRequest::post(url).json_body(&self);

        Transport::request(api_request, &self.config, None).await
    }
}

/// 分片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartInfo {
    /// 分片编号
    pub part_number: i32,
    /// 分片起始位置
    pub start_position: i64,
    /// 分片大小
    pub part_size: i32,
    /// 上传URL
    pub upload_url: String,
}

/// 分片上传素材准备响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareResponse {
    /// 上传事务ID
    pub transaction_id: String,
    /// 文件token
    pub file_token: String,
    /// 分片信息列表
    pub parts: Option<Vec<PartInfo>>,
}

impl ApiResponseTrait for UploadPrepareResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_prepare_request() {
        let config = Config::default();
        let request = UploadPrepareRequest::new(config, "folder_token", "image.jpg", 2048576)
            .content_type("image/jpeg");

        assert_eq!(request.parent_folder_token, "folder_token");
        assert_eq!(request.file_name, "image.jpg");
        assert_eq!(request.size, 2048576);
        assert_eq!(request.content_type, Some("image/jpeg".to_string()));
    }

    #[test]
    fn test_part_info_structure() {
        let part_info = PartInfo {
            part_number: 1,
            start_position: 0,
            part_size: 4194304, // 4MB
            upload_url: "https://upload.example.com/part1".to_string(),
        };

        assert_eq!(part_info.part_number, 1);
        assert_eq!(part_info.start_position, 0);
        assert_eq!(part_info.part_size, 4194304);
        assert_eq!(part_info.upload_url, "https://upload.example.com/part1");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UploadPrepareResponse::data_format(), ResponseFormat::Data);
    }
}

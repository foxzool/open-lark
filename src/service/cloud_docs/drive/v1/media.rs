use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, BinaryResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        validation::{validate_file_name, validate_upload_file, ValidateBuilder, ValidationResult},
        SDKResult,
    },
    impl_executable_builder_owned,
};
use log;

/// 素材服务
pub struct MediaService {
    config: Config,
}

impl MediaService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建上传素材Builder
    pub fn upload_all_builder(&self) -> UploadMediaRequestBuilder {
        UploadMediaRequestBuilder::default()
    }

    /// 使用Builder上传素材（带验证）
    pub async fn upload_all_with_builder(
        &self,
        builder_result: SDKResult<UploadMediaRequest>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadMediaRespData>> {
        let request = builder_result?;
        self.upload_all(request, option).await
    }

    /// 上传素材
    ///
    /// 该接口用于上传素材文件。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_all>
    pub async fn upload_all(
        &self,
        request: UploadMediaRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadMediaRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = DRIVE_V1_MEDIAS_UPLOAD_ALL.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 分片上传素材-预上传
    ///
    /// 该接口用于分片上传的预上传步骤。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_prepare>
    pub async fn upload_prepare(
        &self,
        request: UploadPrepareRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadPrepareRespData>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_MEDIAS_UPLOAD_PREPARE.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 分片上传素材-上传分片
    ///
    /// 该接口用于上传文件分片。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_part>
    pub async fn upload_part(
        &self,
        request: UploadPartRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadPartRespData>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = DRIVE_V1_MEDIAS_UPLOAD_PART.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 分片上传素材-完成上传
    ///
    /// 该接口用于完成分片上传。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_finish>
    pub async fn upload_finish(
        &self,
        request: UploadFinishRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadFinishRespData>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_MEDIAS_UPLOAD_FINISH.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 下载素材
    ///
    /// 该接口用于下载素材文件。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/download>
    pub async fn download(
        &self,
        request: DownloadMediaRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BinaryResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_MEDIAS_DOWNLOAD.replace("{}", &request.file_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 获取素材临时下载链接
    ///
    /// 该接口用于获取素材的临时下载链接。
    ///
    /// <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/batch_get_tmp_download_url>
    pub async fn batch_get_tmp_download_url(
        &self,
        request: BatchGetTmpDownloadUrlRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BatchGetTmpDownloadUrlRespData>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_MEDIAS_BATCH_GET_TMP_DOWNLOAD_URL.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        // 添加查询参数
        let file_tokens = request.file_tokens.join(",");
        api_req.query_params.insert("file_tokens", file_tokens);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// === 数据结构定义 ===

/// 上传素材请求参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadMediaRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 素材名称
    file_name: String,
    /// 父文件夹token
    parent_token: String,
    /// 文件大小
    size: i32,
    /// 校验和（可选）
    checksum: Option<String>,
}

impl UploadMediaRequest {
    pub fn builder() -> UploadMediaRequestBuilder {
        UploadMediaRequestBuilder::default()
    }
}

/// 上传素材请求构建器
#[derive(Default)]
pub struct UploadMediaRequestBuilder {
    request: UploadMediaRequest,
}

impl UploadMediaRequestBuilder {
    pub fn file_name(mut self, file_name: impl ToString) -> Self {
        self.request.file_name = file_name.to_string();
        self
    }

    pub fn parent_token(mut self, parent_token: impl ToString) -> Self {
        self.request.parent_token = parent_token.to_string();
        self
    }

    pub fn size(mut self, size: i32) -> Self {
        self.request.size = size;
        self
    }

    pub fn checksum(mut self, checksum: impl ToString) -> Self {
        self.request.checksum = Some(checksum.to_string());
        self
    }

    pub fn file(mut self, file: Vec<u8>) -> Self {
        self.request.api_req.file = file;
        self
    }

    pub fn build(mut self) -> UploadMediaRequest {
        // 验证必填字段
        if self.request.file_name.is_empty() {
            log::error!("file_name is required for media upload");
            return UploadMediaRequest {
                api_req: ApiRequest {
                    body: Vec::new(),
                    ..Default::default()
                },
                ..self.request
            };
        }

        if self.request.parent_token.is_empty() {
            log::error!("parent_token is required for media upload");
            return UploadMediaRequest {
                api_req: ApiRequest {
                    body: Vec::new(),
                    ..Default::default()
                },
                ..self.request
            };
        }

        if self.request.size <= 0 {
            log::error!("file size must be greater than 0");
            return UploadMediaRequest {
                api_req: ApiRequest {
                    body: Vec::new(),
                    ..Default::default()
                },
                ..self.request
            };
        }

        // 验证文件名
        let (_, name_result) = validate_file_name(&self.request.file_name);
        if !name_result.is_valid() {
            log::error!(
                "Invalid file_name: {}",
                name_result.error().unwrap_or("unknown error")
            );
            return UploadMediaRequest {
                api_req: ApiRequest {
                    body: Vec::new(),
                    ..Default::default()
                },
                ..self.request
            };
        }

        // 验证文件数据（如果有）
        if !self.request.api_req.file.is_empty() {
            let upload_result =
                validate_upload_file(&self.request.api_req.file, &self.request.file_name, false);
            if !upload_result.is_valid() {
                log::error!(
                    "File validation failed: {}",
                    upload_result.error().unwrap_or("unknown error")
                );
                return UploadMediaRequest {
                    api_req: ApiRequest {
                        body: Vec::new(),
                        ..Default::default()
                    },
                    ..self.request
                };
            }
        }

        self.request.api_req.body = match serde_json::to_vec(&self.request) {
            Ok(body) => body,
            Err(e) => {
                log::error!("Failed to serialize upload media request: {}", e);
                return UploadMediaRequest {
                    api_req: ApiRequest {
                        body: Vec::new(),
                        ..Default::default()
                    },
                    ..self.request
                };
            }
        };
        self.request
    }
}

impl ValidateBuilder for UploadMediaRequestBuilder {
    fn validate(&self) -> ValidationResult {
        // 验证必填字段
        if self.request.file_name.is_empty() {
            return ValidationResult::Invalid("file_name is required".to_string());
        }

        if self.request.parent_token.is_empty() {
            return ValidationResult::Invalid("parent_token is required".to_string());
        }

        if self.request.size <= 0 {
            return ValidationResult::Invalid("file size must be greater than 0".to_string());
        }

        // 验证文件名
        let (_, name_result) = validate_file_name(&self.request.file_name);
        if !name_result.is_valid() {
            return name_result;
        }

        // 验证文件数据（如果有）
        if !self.request.api_req.file.is_empty() {
            validate_upload_file(&self.request.api_req.file, &self.request.file_name, false)
        } else {
            ValidationResult::Valid
        }
    }
}

/// 上传素材响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadMediaRespData {
    /// 素材token
    pub file_token: String,
}

impl ApiResponseTrait for UploadMediaRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 分片上传预上传请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareRequest {
    /// 文件名称
    pub file_name: String,
    /// 父文件夹token
    pub parent_token: String,
    /// 文件大小
    pub size: i64,
    /// 分片大小（可选）
    pub block_size: Option<i32>,
    /// 文件校验和（可选）
    pub checksum: Option<String>,
}

impl UploadPrepareRequest {
    pub fn new(file_name: impl Into<String>, parent_token: impl Into<String>, size: i64) -> Self {
        Self {
            file_name: file_name.into(),
            parent_token: parent_token.into(),
            size,
            block_size: None,
            checksum: None,
        }
    }
}

/// 分片上传预上传响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareRespData {
    /// 上传事务ID
    pub upload_id: String,
    /// 分片大小
    pub block_size: i32,
    /// 分片数量
    pub block_num: i32,
}

impl ApiResponseTrait for UploadPrepareRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传分片请求参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadPartRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 上传事务ID
    upload_id: String,
    /// 分片序号
    seq: i32,
    /// 分片大小
    size: i32,
    /// 分片校验和（可选）
    checksum: Option<String>,
}

impl UploadPartRequest {
    pub fn builder() -> UploadPartRequestBuilder {
        UploadPartRequestBuilder::default()
    }
}

/// 上传分片请求构建器
#[derive(Default)]
pub struct UploadPartRequestBuilder {
    request: UploadPartRequest,
}

impl UploadPartRequestBuilder {
    pub fn upload_id(mut self, upload_id: impl ToString) -> Self {
        self.request.upload_id = upload_id.to_string();
        self
    }

    pub fn seq(mut self, seq: i32) -> Self {
        self.request.seq = seq;
        self
    }

    pub fn size(mut self, size: i32) -> Self {
        self.request.size = size;
        self
    }

    pub fn checksum(mut self, checksum: impl ToString) -> Self {
        self.request.checksum = Some(checksum.to_string());
        self
    }

    pub fn file_chunk(mut self, chunk: Vec<u8>) -> Self {
        self.request.api_req.file = chunk;
        self
    }

    pub fn build(mut self) -> UploadPartRequest {
        self.request.api_req.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 上传分片响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartRespData {
    /// 分片ETag
    pub etag: String,
}

impl ApiResponseTrait for UploadPartRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 完成上传请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFinishRequest {
    /// 上传事务ID
    pub upload_id: String,
    /// 分片信息列表
    pub block_infos: Vec<BlockInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockInfo {
    /// 分片ETag
    pub etag: String,
    /// 分片序号
    pub seq: i32,
}

impl UploadFinishRequest {
    pub fn new(upload_id: impl Into<String>, block_infos: Vec<BlockInfo>) -> Self {
        Self {
            upload_id: upload_id.into(),
            block_infos,
        }
    }
}

/// 完成上传响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFinishRespData {
    /// 素材token
    pub file_token: String,
}

impl ApiResponseTrait for UploadFinishRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载素材请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadMediaRequest {
    /// 素材token
    pub file_token: String,
}

impl DownloadMediaRequest {
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
        }
    }
}

/// 批量获取临时下载链接请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetTmpDownloadUrlRequest {
    /// 素材token列表
    pub file_tokens: Vec<String>,
}

impl BatchGetTmpDownloadUrlRequest {
    pub fn new(file_tokens: Vec<String>) -> Self {
        Self { file_tokens }
    }
}

/// 批量获取临时下载链接响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetTmpDownloadUrlRespData {
    /// 临时下载链接信息
    pub tmp_download_urls: Vec<TmpDownloadUrl>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TmpDownloadUrl {
    /// 素材token
    pub file_token: String,
    /// 临时下载链接
    pub tmp_download_url: String,
}

impl ApiResponseTrait for BatchGetTmpDownloadUrlRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// === 宏实现 ===

impl_executable_builder_owned!(
    UploadMediaRequestBuilder,
    MediaService,
    UploadMediaRequest,
    BaseResponse<UploadMediaRespData>,
    upload_all
);

impl_executable_builder_owned!(
    UploadPartRequestBuilder,
    MediaService,
    UploadPartRequest,
    BaseResponse<UploadPartRespData>,
    upload_part
);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    fn mock_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    // === MediaService Tests ===

    #[test]
    fn test_media_service_new() {
        let config = mock_config();
        let service = MediaService::new(config.clone());
        assert_eq!(service.config.app_id, config.app_id);
    }

    #[test]
    fn test_upload_all_builder() {
        let service = MediaService::new(mock_config());
        let builder = service.upload_all_builder();
        assert_eq!(builder.request.file_name, "");
        assert_eq!(builder.request.parent_token, "");
        assert_eq!(builder.request.size, 0);
    }

    // === UploadMediaRequest Tests ===

    #[test]
    fn test_upload_media_request_builder() {
        let request = UploadMediaRequest::builder()
            .file_name("test.pdf")
            .parent_token("parent123")
            .size(1024)
            .build();
        assert_eq!(request.file_name, "test.pdf");
        assert_eq!(request.parent_token, "parent123");
        assert_eq!(request.size, 1024);
        assert!(request.checksum.is_none());
    }

    // === UploadMediaRequestBuilder Tests ===

    #[test]
    fn test_upload_media_builder_basic() {
        let builder = UploadMediaRequestBuilder::default()
            .file_name("test_file.txt")
            .parent_token("parent_token_123")
            .size(1024);

        assert_eq!(builder.request.file_name, "test_file.txt");
        assert_eq!(builder.request.parent_token, "parent_token_123");
        assert_eq!(builder.request.size, 1024);
    }

    #[test]
    fn test_upload_media_builder_with_checksum() {
        let builder = UploadMediaRequestBuilder::default()
            .file_name("image.png")
            .parent_token("images_folder")
            .size(512000)
            .checksum("md5:hash123");

        assert_eq!(builder.request.file_name, "image.png");
        assert_eq!(builder.request.checksum, Some("md5:hash123".to_string()));
    }

    #[test]
    fn test_upload_media_builder_with_file_data() {
        let file_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
        let builder = UploadMediaRequestBuilder::default()
            .file_name("test.png")
            .parent_token("parent")
            .size(4)
            .file(file_data.clone());

        assert_eq!(builder.request.api_req.file, file_data);
    }

    #[test]
    fn test_upload_media_builder_build_valid() {
        let file_data = b"Hello World".to_vec();
        let request = UploadMediaRequestBuilder::default()
            .file_name("hello.txt")
            .parent_token("parent123")
            .size(11)
            .file(file_data)
            .build();

        assert_eq!(request.file_name, "hello.txt");
        assert_eq!(request.parent_token, "parent123");
        assert_eq!(request.size, 11);
        assert!(!request.api_req.body.is_empty());
    }

    #[test]
    fn test_upload_media_builder_build_missing_filename() {
        let request = UploadMediaRequestBuilder::default()
            .parent_token("parent123")
            .size(100)
            .build();

        // Should return invalid request due to missing filename
        assert!(request.api_req.body.is_empty());
    }

    #[test]
    fn test_upload_media_builder_build_missing_parent_token() {
        let request = UploadMediaRequestBuilder::default()
            .file_name("test.txt")
            .size(100)
            .build();

        // Should return invalid request due to missing parent_token
        assert!(request.api_req.body.is_empty());
    }

    #[test]
    fn test_upload_media_builder_build_invalid_size() {
        let request = UploadMediaRequestBuilder::default()
            .file_name("test.txt")
            .parent_token("parent123")
            .size(0) // Invalid size
            .build();

        // Should return invalid request due to invalid size
        assert!(request.api_req.body.is_empty());
    }

    #[test]
    fn test_upload_media_builder_build_negative_size() {
        let request = UploadMediaRequestBuilder::default()
            .file_name("test.txt")
            .parent_token("parent123")
            .size(-1) // Negative size
            .build();

        // Should return invalid request due to negative size
        assert!(request.api_req.body.is_empty());
    }

    // === UploadPrepareRequest Tests ===

    #[test]
    fn test_upload_prepare_request_new() {
        let request = UploadPrepareRequest::new("large_file.zip", "uploads", 10485760);
        assert_eq!(request.file_name, "large_file.zip");
        assert_eq!(request.parent_token, "uploads");
        assert_eq!(request.size, 10485760);
        assert!(request.block_size.is_none());
    }

    #[test]
    fn test_upload_prepare_request_with_block_size() {
        let mut request = UploadPrepareRequest::new("video.mp4", "videos", 104857600);
        request.block_size = Some(1048576); // 1MB blocks
        assert_eq!(request.file_name, "video.mp4");
        assert_eq!(request.block_size, Some(1048576));
    }

    // === UploadPartRequest Tests ===

    #[test]
    fn test_upload_part_request_builder() {
        let request = UploadPartRequest::builder()
            .upload_id("upload123")
            .seq(1)
            .build();
        assert_eq!(request.upload_id, "upload123");
        assert_eq!(request.seq, 1);
        assert_eq!(request.api_req.file.len(), 0);
    }

    #[test]
    fn test_upload_part_request_builder_with_file() {
        let _file_chunk = [0x01, 0x02, 0x03, 0x04];
        let request = UploadPartRequest::builder()
            .upload_id("upload456")
            .seq(2)
            .size(4)
            .checksum("crc32:abc123")
            .build();

        assert_eq!(request.upload_id, "upload456");
        assert_eq!(request.seq, 2);
        assert_eq!(request.size, 4);
        assert_eq!(request.checksum, Some("crc32:abc123".to_string()));
    }

    #[test]
    fn test_upload_part_request_builder_minimal() {
        let request = UploadPartRequestBuilder::default()
            .upload_id("minimal")
            .seq(1)
            .build();

        assert_eq!(request.upload_id, "minimal");
        assert_eq!(request.seq, 1);
        assert_eq!(request.size, 0);
        assert!(request.checksum.is_none());
    }

    // === UploadFinishRequest Tests ===

    #[test]
    fn test_upload_finish_request_new() {
        let block_infos = vec![
            BlockInfo {
                etag: "hash1".to_string(),
                seq: 1,
            },
            BlockInfo {
                etag: "hash2".to_string(),
                seq: 2,
            },
        ];
        let request = UploadFinishRequest::new("upload789", block_infos.clone());
        assert_eq!(request.upload_id, "upload789");
        assert_eq!(request.block_infos, block_infos);
    }

    // === DownloadMediaRequest Tests ===

    #[test]
    fn test_download_media_request_new() {
        let request = DownloadMediaRequest::new("media_token_abc");
        assert_eq!(request.file_token, "media_token_abc");
    }

    // === BatchGetTmpDownloadUrlRequest Tests ===

    #[test]
    fn test_batch_get_tmp_download_url_request_new() {
        let request = BatchGetTmpDownloadUrlRequest::new(vec!["temp_token_xyz".to_string()]);
        assert_eq!(request.file_tokens, vec!["temp_token_xyz".to_string()]);
    }

    // === Response Data Structure Tests ===

    #[test]
    fn test_upload_media_resp_data() {
        let data = UploadMediaRespData {
            file_token: "uploaded_file_token".to_string(),
        };
        assert_eq!(data.file_token, "uploaded_file_token");
    }

    #[test]
    fn test_upload_prepare_resp_data() {
        let data = UploadPrepareRespData {
            upload_id: "prepared_upload_123".to_string(),
            block_size: 1048576,
            block_num: 10,
        };
        assert_eq!(data.upload_id, "prepared_upload_123");
        assert_eq!(data.block_size, 1048576);
        assert_eq!(data.block_num, 10);
    }

    #[test]
    fn test_upload_part_resp_data() {
        let data = UploadPartRespData {
            etag: "part_etag_456".to_string(),
        };
        assert_eq!(data.etag, "part_etag_456");
    }

    #[test]
    fn test_upload_finish_resp_data() {
        let data = UploadFinishRespData {
            file_token: "finished_file_token".to_string(),
        };
        assert_eq!(data.file_token, "finished_file_token");
    }

    #[test]
    fn test_batch_get_tmp_download_url_resp_data() {
        let data = BatchGetTmpDownloadUrlRespData {
            tmp_download_urls: vec![TmpDownloadUrl {
                file_token: "token123".to_string(),
                tmp_download_url: "https://temp.example.com/download/abc123".to_string(),
            }],
        };
        assert_eq!(
            data.tmp_download_urls[0].tmp_download_url,
            "https://temp.example.com/download/abc123"
        );
    }

    // === Serialization Tests ===

    #[rstest]
    #[case("upload_media_request")]
    #[case("upload_prepare_request")]
    #[case("upload_part_request")]
    #[case("upload_finish_request")]
    #[case("download_media_request")]
    #[case("batch_get_tmp_download_url_request")]
    fn test_request_serialization_roundtrip(#[case] request_type: &str) {
        match request_type {
            "upload_media_request" => {
                let original = UploadMediaRequest::builder()
                    .file_name("test.txt")
                    .parent_token("parent123")
                    .size(100)
                    .checksum("sha256:test")
                    .build();
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: UploadMediaRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(original.file_name, deserialized.file_name);
                assert_eq!(original.parent_token, deserialized.parent_token);
                assert_eq!(original.size, deserialized.size);
                assert_eq!(original.checksum, deserialized.checksum);
            }
            "upload_prepare_request" => {
                let mut original = UploadPrepareRequest::new("large.zip", "uploads", 1000000);
                original.block_size = Some(4096);
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: UploadPrepareRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(original.file_name, deserialized.file_name);
                assert_eq!(original.parent_token, deserialized.parent_token);
                assert_eq!(original.size, deserialized.size);
                assert_eq!(original.block_size, deserialized.block_size);
            }
            "upload_part_request" => {
                let original = UploadPartRequest::builder()
                    .upload_id("test_upload")
                    .seq(1)
                    .size(1024)
                    .checksum("test_checksum")
                    .build();
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: UploadPartRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(original.upload_id, deserialized.upload_id);
                assert_eq!(original.seq, deserialized.seq);
                assert_eq!(original.size, deserialized.size);
                assert_eq!(original.checksum, deserialized.checksum);
            }
            "upload_finish_request" => {
                let checksums = vec!["hash1".to_string(), "hash2".to_string()];
                let block_infos: Vec<BlockInfo> = checksums
                    .into_iter()
                    .enumerate()
                    .map(|(i, etag)| BlockInfo {
                        etag,
                        seq: i as i32,
                    })
                    .collect();
                let original = UploadFinishRequest::new("upload123", block_infos.clone());
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: UploadFinishRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(original.upload_id, deserialized.upload_id);
                assert_eq!(original.block_infos, deserialized.block_infos);
            }
            "download_media_request" => {
                let original = DownloadMediaRequest::new("download_token");
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: DownloadMediaRequest = serde_json::from_str(&json).unwrap();
                assert_eq!(original.file_token, deserialized.file_token);
            }
            "batch_get_tmp_download_url_request" => {
                let original = BatchGetTmpDownloadUrlRequest::new(vec!["temp_token".to_string()]);
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: BatchGetTmpDownloadUrlRequest =
                    serde_json::from_str(&json).unwrap();
                assert_eq!(original.file_tokens, deserialized.file_tokens);
            }
            _ => panic!("Unknown request type: {}", request_type),
        }
    }

    #[rstest]
    #[case("upload_media_resp")]
    #[case("upload_prepare_resp")]
    #[case("upload_part_resp")]
    #[case("upload_finish_resp")]
    #[case("batch_get_tmp_download_url_resp")]
    fn test_response_serialization_roundtrip(#[case] response_type: &str) {
        match response_type {
            "upload_media_resp" => {
                let original = UploadMediaRespData {
                    file_token: "response_token".to_string(),
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: UploadMediaRespData = serde_json::from_str(&json).unwrap();
                assert_eq!(original.file_token, deserialized.file_token);
            }
            "upload_prepare_resp" => {
                let original = UploadPrepareRespData {
                    upload_id: "prepared_id".to_string(),
                    block_size: 4096,
                    block_num: 5,
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: UploadPrepareRespData = serde_json::from_str(&json).unwrap();
                assert_eq!(original.upload_id, deserialized.upload_id);
                assert_eq!(original.block_size, deserialized.block_size);
                assert_eq!(original.block_num, deserialized.block_num);
            }
            "upload_part_resp" => {
                let original = UploadPartRespData {
                    etag: "part_etag".to_string(),
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: UploadPartRespData = serde_json::from_str(&json).unwrap();
                assert_eq!(original.etag, deserialized.etag);
            }
            "upload_finish_resp" => {
                let original = UploadFinishRespData {
                    file_token: "final_token".to_string(),
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: UploadFinishRespData = serde_json::from_str(&json).unwrap();
                assert_eq!(original.file_token, deserialized.file_token);
            }
            "batch_get_tmp_download_url_resp" => {
                let original = BatchGetTmpDownloadUrlRespData {
                    tmp_download_urls: vec![TmpDownloadUrl {
                        file_token: "token123".to_string(),
                        tmp_download_url: "https://example.com/temp".to_string(),
                    }],
                };
                let json = serde_json::to_string(&original).unwrap();
                let deserialized: BatchGetTmpDownloadUrlRespData =
                    serde_json::from_str(&json).unwrap();
                assert_eq!(original.tmp_download_urls, deserialized.tmp_download_urls);
            }
            _ => panic!("Unknown response type: {}", response_type),
        }
    }

    // === ApiResponseTrait Tests ===

    #[rstest]
    #[case("UploadMediaRespData")]
    #[case("UploadPrepareRespData")]
    #[case("UploadPartRespData")]
    #[case("UploadFinishRespData")]
    #[case("BatchGetTmpDownloadUrlRespData")]
    fn test_api_response_trait(#[case] response_type: &str) {
        let format = match response_type {
            "UploadMediaRespData" => UploadMediaRespData::data_format(),
            "UploadPrepareRespData" => UploadPrepareRespData::data_format(),
            "UploadPartRespData" => UploadPartRespData::data_format(),
            "UploadFinishRespData" => UploadFinishRespData::data_format(),
            "BatchGetTmpDownloadUrlRespData" => BatchGetTmpDownloadUrlRespData::data_format(),
            _ => panic!("Unknown response type: {}", response_type),
        };
        assert_eq!(format, ResponseFormat::Data);
    }

    // === Validation Tests ===

    #[test]
    fn test_upload_media_builder_validate_success() {
        let builder = UploadMediaRequestBuilder::default()
            .file_name("valid.txt")
            .parent_token("valid_parent")
            .size(100);

        let result = builder.validate();
        assert!(result.is_valid());
    }

    #[test]
    fn test_upload_media_builder_validate_missing_filename() {
        let builder = UploadMediaRequestBuilder::default()
            .parent_token("valid_parent")
            .size(100);

        let result = builder.validate();
        assert!(!result.is_valid());
        assert!(result.error().unwrap().contains("file_name is required"));
    }

    #[test]
    fn test_upload_media_builder_validate_missing_parent() {
        let builder = UploadMediaRequestBuilder::default()
            .file_name("valid.txt")
            .size(100);

        let result = builder.validate();
        assert!(!result.is_valid());
        assert!(result.error().unwrap().contains("parent_token is required"));
    }

    #[test]
    fn test_upload_media_builder_validate_invalid_size() {
        let builder = UploadMediaRequestBuilder::default()
            .file_name("valid.txt")
            .parent_token("valid_parent")
            .size(0);

        let result = builder.validate();
        assert!(!result.is_valid());
        assert!(result
            .error()
            .unwrap()
            .contains("file size must be greater than 0"));
    }

    // === Edge Cases and Boundary Tests ===

    #[test]
    fn test_upload_large_file_size() {
        let request = UploadPrepareRequest::new("huge_file.bin", "storage", i64::MAX);
        assert_eq!(request.size, i64::MAX);
    }

    #[test]
    fn test_upload_zero_block_size() {
        let mut request = UploadPrepareRequest::new("test.txt", "parent", 1000);
        request.block_size = Some(0);
        assert_eq!(request.block_size, Some(0));
    }

    #[test]
    fn test_upload_part_maximum_sequence() {
        let request = UploadPartRequest::builder()
            .upload_id("upload123")
            .seq(i32::MAX)
            .size(0)
            .build();
        assert_eq!(request.seq, i32::MAX);
    }

    #[test]
    fn test_unicode_filename() {
        let unicode_name = "测试文件🎉.docx";
        let request = UploadMediaRequest::builder()
            .file_name(unicode_name)
            .parent_token("folder")
            .size(1024)
            .build();
        assert_eq!(request.file_name, unicode_name);
    }

    #[test]
    fn test_special_characters_in_filename() {
        let special_name = "file with spaces & symbols @#$.txt";
        let request = UploadMediaRequest::builder()
            .file_name(special_name)
            .parent_token("parent")
            .size(512)
            .build();
        assert_eq!(request.file_name, special_name);
    }

    #[test]
    fn test_empty_block_checksums() {
        let empty_block_infos: Vec<BlockInfo> = vec![];
        let request = UploadFinishRequest::new("upload456", empty_block_infos);
        assert_eq!(request.block_infos.len(), 0);
    }

    #[test]
    fn test_many_block_checksums() {
        let many_checksums: Vec<String> = (0..1000).map(|i| format!("hash_{}", i)).collect();
        let many_block_infos: Vec<BlockInfo> = many_checksums
            .into_iter()
            .enumerate()
            .map(|(i, etag)| BlockInfo {
                etag,
                seq: i as i32,
            })
            .collect();
        let request = UploadFinishRequest::new("upload789", many_block_infos.clone());
        assert_eq!(request.block_infos.len(), 1000);
        assert_eq!(request.block_infos[0].etag, "hash_0");
        assert_eq!(request.block_infos[999].etag, "hash_999");
    }

    #[test]
    fn test_very_long_token() {
        let long_token = "a".repeat(1000);
        let request = DownloadMediaRequest::new(&long_token);
        assert_eq!(request.file_token, long_token);
    }

    #[test]
    fn test_very_long_url() {
        let long_url = format!("https://example.com/{}", "a".repeat(1000));
        let data = BatchGetTmpDownloadUrlRespData {
            tmp_download_urls: vec![TmpDownloadUrl {
                file_token: "token123".to_string(),
                tmp_download_url: long_url.clone(),
            }],
        };
        assert_eq!(data.tmp_download_urls[0].tmp_download_url, long_url);
    }

    // === Clone and Debug Tests ===

    #[test]
    fn test_request_cloning() {
        let original = UploadMediaRequest::builder()
            .file_name("clone_test.txt")
            .parent_token("parent")
            .size(256)
            .checksum("test_hash")
            .build();
        let cloned = original.clone();

        assert_eq!(original.file_name, cloned.file_name);
        assert_eq!(original.parent_token, cloned.parent_token);
        assert_eq!(original.size, cloned.size);
        assert_eq!(original.checksum, cloned.checksum);
    }

    #[test]
    fn test_response_debug() {
        let data = UploadPrepareRespData {
            upload_id: "debug_test".to_string(),
            block_size: 4096,
            block_num: 2,
        };
        let debug_str = format!("{:?}", data);
        assert!(debug_str.contains("debug_test"));
        assert!(debug_str.contains("4096"));
        assert!(debug_str.contains("2"));
    }

    // === Builder Chain Tests ===

    #[test]
    fn test_upload_part_builder_method_chaining() {
        let chunk_data = vec![0xFF; 1024];
        let request = UploadPartRequest::builder()
            .upload_id("chain_test")
            .seq(3)
            .size(1024)
            .checksum("md5:abc123")
            .file_chunk(chunk_data.clone())
            .build();

        assert_eq!(request.upload_id, "chain_test");
        assert_eq!(request.seq, 3);
        assert_eq!(request.size, 1024);
        assert_eq!(request.checksum, Some("md5:abc123".to_string()));
        assert_eq!(request.api_req.file, chunk_data);
    }

    #[test]
    fn test_upload_part_builder_overwrite_values() {
        let request = UploadPartRequest::builder()
            .upload_id("first")
            .upload_id("second") // Overwrite
            .seq(1)
            .seq(5) // Overwrite
            .build();

        assert_eq!(request.upload_id, "second");
        assert_eq!(request.seq, 5);
    }

    #[test]
    fn test_default_values() {
        let request = UploadPartRequestBuilder::default().build();
        assert_eq!(request.upload_id, "");
        assert_eq!(request.seq, 0);
        assert_eq!(request.size, 0);
        assert!(request.checksum.is_none());
        assert!(request.api_req.file.is_empty());
    }
}

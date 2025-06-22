use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, BinaryResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 素材服务
pub struct MediaService {
    config: Config,
}

impl MediaService {
    pub fn new(config: Config) -> Self {
        Self { config }
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
        api_req.api_path = "/open-apis/drive/v1/medias/upload_all".to_string();
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
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/drive/v1/medias/upload_prepare".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = serde_json::to_vec(&request)?;

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
        api_req.api_path = "/open-apis/drive/v1/medias/upload_part".to_string();
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
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/drive/v1/medias/upload_finish".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = serde_json::to_vec(&request)?;

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
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::GET;
        api_req.api_path = format!("/open-apis/drive/v1/medias/{}/download", request.file_token);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

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
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/drive/v1/medias/batch_get_tmp_download_url".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        // 添加查询参数
        let file_tokens = request.file_tokens.join(",");
        api_req
            .query_params
            .insert("file_tokens".to_string(), file_tokens);

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
        self.request.api_req.body = serde_json::to_vec(&self.request).unwrap();
        self.request
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

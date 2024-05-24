use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ ApiResponseTrait, BinaryResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use crate::core::api_resp::BaseResp;


pub struct FilesService {
    config: Config,
}

/// 上传文件 请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadAllRequest {
    /// 请求体
    #[serde(skip)]
    api_req: ApiRequest,
    /// 文件名。
    ///
    /// 示例值："demo.pdf"
    file_name: String,
    /// 上传点类型。
    ///
    /// 示例值："explorer"
    parent_type: String,
    /// 文件夹token，获取方式见 概述
    ///
    /// 示例值："fldbcO1UuPz8VwnpPx5a92abcef"
    parent_node: String,
    /// 文件大小（以字节为单位）。
    ///
    /// 示例值：1024
    size: i32,
    /// 文件adler32校验和(可选)。
    checksum: Option<String>,
}

impl UploadAllRequest {
    pub fn builder() -> UploadAllRequestBuilder {
        UploadAllRequestBuilder::default()
    }
}

/// 上传文件 请求体构建器
#[derive(Default)]
pub struct UploadAllRequestBuilder {
    request: UploadAllRequest,
}

impl UploadAllRequestBuilder {
    /// 文件名
    pub fn file_name(mut self, file_name: impl ToString) -> Self {
        self.request.file_name = file_name.to_string();
        self
    }

    /// 上传点类型。
    pub fn parent_type(mut self, parent_type: impl ToString) -> Self {
        self.request.parent_type = parent_type.to_string();
        self
    }

    /// 文件夹token
    pub fn parent_node(mut self, parent_node: impl ToString) -> Self {
        self.request.parent_node = parent_node.to_string();
        self
    }

    /// 文件大小（以字节为单位）
    pub fn size(mut self, size: i32) -> Self {
        self.request.size = size;
        self
    }

    /// 文件adler32校验和(可选)
    pub fn checksum(mut self, checksum: impl ToString) -> Self {
        self.request.checksum = Some(checksum.to_string());
        self
    }

    /// 文件二进制内容。
    pub fn file(mut self, file: Vec<u8>) -> Self {
        self.request.api_req.file = file;
        self
    }

    pub fn build(mut self) -> UploadAllRequest {
        self.request.api_req.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}


impl FilesService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传文件
    pub async fn upload_all(
        &self,
        upload_all_request: UploadAllRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResp<UploadAllResponse>> {
        let mut api_req = upload_all_request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = "/open-apis/drive/v1/files/upload_all".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 下载文件
    pub async fn download(
        &self,
        request: DownloadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResp<BinaryResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = format!("/open-apis/drive/v1/files/{}/download", request.file_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 上传文件响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadAllResponse {
    /// 新创建文件的 token
    file_token: String,
}

impl ApiResponseTrait for UploadAllResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载文件 请求体
#[derive(Default, Serialize, Deserialize)]
pub struct DownloadRequest {
    /// 请求体
    #[serde(skip)]
    api_req: ApiRequest,
    /// 文件的 token
    file_token: String,
}

/// 下载文件 请求体构建器
#[derive(Default)]
pub struct DownloadRequestBuilder {
    req: DownloadRequest,
}

impl DownloadRequestBuilder {
    pub fn file_token(mut self, file_token: impl ToString) -> Self {
        self.req.file_token = file_token.to_string();
        self
    }

    pub fn build(self) -> DownloadRequest {
        self.req
    }
}

impl DownloadRequest {
    pub fn builder() -> DownloadRequestBuilder {
        DownloadRequestBuilder::default()
    }
}

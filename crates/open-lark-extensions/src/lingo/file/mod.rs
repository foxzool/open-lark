use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, LINGO_FILE_DOWNLOAD, LINGO_FILE_UPLOAD},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::lingo::models::FileInfo,
};

/// 图片管理服务
pub struct FileService {
    pub config: Config,
}

impl FileService {
    /// 创建图片管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传图片
    ///
    /// 上传图片资源，用于词条中的图片引用。
    ///
    /// # Arguments
    ///
    /// * `request` - 图片上传请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回上传后的图片信息
    pub async fn upload_image(
        &self,
        request: FileUploadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FileUploadResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: LINGO_FILE_UPLOAD.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 下载图片
    ///
    /// 通过文件 token 下载图片资源。
    ///
    /// # Arguments
    ///
    /// * `file_token` - 文件 token
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回图片的二进制数据
    pub async fn download_image(
        &self,
        file_token: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FileDownloadResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                LINGO_FILE_DOWNLOAD,
                "{file_token}",
                file_token,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 文件上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadRequest {
    /// 文件名
    pub file_name: String,
    /// 文件内容（base64编码）
    pub file_content: String,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

/// 文件上传响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FileUploadResponse {
    /// 上传的文件信息
    pub file: FileInfo,
}

impl ApiResponseTrait for FileUploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文件下载响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FileDownloadResponse {
    /// 文件内容（二进制数据）
    pub content: Vec<u8>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
}

impl ApiResponseTrait for FileDownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

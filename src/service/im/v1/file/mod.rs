use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 文件服务
pub struct FileService {
    pub config: Config,
}

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileResponse {
    /// 文件的key
    pub file_key: String,
}

impl ApiResponseTrait for CreateFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileResponse {
    /// 文件数据
    pub data: Vec<u8>,
}

impl ApiResponseTrait for GetFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl FileService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传文件
    pub async fn create(
        &self,
        file_type: &str,
        file_name: &str,
        file_data: Vec<u8>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateFileResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("file_type".to_string(), file_type.to_string());
        query_params.insert("file_name".to_string(), file_name.to_string());

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/im/v1/files".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: file_data,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 下载文件
    pub async fn get(
        &self,
        file_key: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetFileResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/im/v1/files/{}", file_key),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

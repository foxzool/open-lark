use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use open_lark_core::core::standard_response::StandardResponse;
use open_lark_core::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 文件服务
pub struct FileService {
    pub config: Config,
}

// 接入统一 Service 抽象（IM v1 - FileService）
// impl_full_service!(FileService, "im.file", "v1");

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
    ) -> SDKResult<CreateFileResponse> {
        let mut query_params = HashMap::new();
        query_params.insert("file_type", file_type.to_string());
        query_params.insert("file_name", file_name.to_string());

        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V1_FILES.to_string());
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params = query_params;
        api_req.body = file_data;

        let api_resp: BaseResponse<CreateFileResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 下载文件
    pub async fn get(
        &self,
        file_key: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<GetFileResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            open_lark_core::core::endpoints::im::IM_V1_DOWNLOAD_FILE,
            "file_key",
            file_key,
        ));
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        let api_resp: BaseResponse<GetFileResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use open_lark_core::core::config::Config;
    fn create_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_file_service_creation() {
        let config = create_test_config();
        let service = FileService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_create_file_response_format() {
        assert_eq!(CreateFileResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_file_response_format() {
        assert_eq!(GetFileResponse::data_format(), ResponseFormat::Data);
    }
}

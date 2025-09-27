use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::approval::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::approval::models::ApprovalFile,
};

/// 审批文件服务
pub struct FileService {
    pub config: Config,
}

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileResponse {
    /// 文件信息
    pub file: ApprovalFile,
}

impl ApiResponseTrait for UploadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl FileService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传文件
    pub async fn upload(
        &self,
        file_name: &str,
        file_content: Vec<u8>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadFileResponse>> {
        use crate::core::error::LarkAPIError;

        // 构建表单元数据
        let metadata = serde_json::json!({
            "file_name": file_name
        });

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: APPROVAL_V4_FILE_UPLOAD.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&metadata)
                .map_err(|e| LarkAPIError::DeserializeError(e.to_string()))?,
            file: file_content,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for FileService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "approval_file"
    }

    fn service_version() -> &'static str {
        "v4"
    }
}

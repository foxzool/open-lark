use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use crate::{
    core::{,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::approval::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    service::approval::models::ApprovalFile,
};
/// 审批文件服务
pub struct FileService {
}
    pub config: Config,
/// 上传文件响应
#[derive(Debug, Clone)]
pub struct UploadFileResponse {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl FileService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 上传文件
    ///,
/// 上传文件到审批系统，支持各种文件格式，为审批流程提供附件支持。
    /// 上传的文件可以在审批单中引用，提供审批依据和材料支撑。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDOyUjL1gjM14SN4ITN
pub async fn upload(,
        &self,
        file_name: &str,
        file_content: Vec<u8>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadFileResponse>> {,
use crate::core::error::LarkAPIError;
        // 构建表单元数据
let metadata = serde_json::json!({,
            "file_name": file_name,
});
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: APPROVAL_V4_FILE_UPLOAD.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&metadata)
                .map_err(|e| LarkAPIError::DeserializeError(e.to_string()))?,
            file: file_content,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
impl Service for FileService {,
    fn config(&self) -> &Config {,
&self.config,
    fn service_name() -> &'static str {,
        "approval_file",
fn service_version() -> &'static str {,
        "v4",
}
}}}}}}}}}
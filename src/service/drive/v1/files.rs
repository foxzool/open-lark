use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiReq,
    api_resp::{ApiResponse, ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 上传文件 请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadAllRequest {
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
    #[serde(skip)]
    /// 文件二进制内容。
    file: Vec<u8>,
}

impl UploadAllRequest {
    pub fn builder() -> Self {
        Self::default()
    }

    /// 文件名
    pub fn file_name(mut self, file_name: impl ToString) -> Self {
        self.file_name = file_name.to_string();
        self
    }

    /// 上传点类型。
    pub fn parent_type(mut self, parent_type: impl ToString) -> Self {
        self.parent_type = parent_type.to_string();
        self
    }

    /// 文件夹token
    pub fn parent_node(mut self, parent_node: impl ToString) -> Self {
        self.parent_node = parent_node.to_string();
        self
    }

    /// 文件大小（以字节为单位）
    pub fn size(mut self, size: i32) -> Self {
        self.size = size;
        self
    }

    /// 文件adler32校验和(可选)
    pub fn checksum(mut self, checksum: impl ToString) -> Self {
        self.checksum = Some(checksum.to_string());
        self
    }

    /// 文件二进制内容。
    pub fn file(mut self, file: Vec<u8>) -> Self {
        self.file = file;
        self
    }

    pub fn build(self) -> ApiReq {
        let mut api_req = ApiReq::default();
        api_req.body = serde_json::to_vec(&self).unwrap().into();
        api_req.file = self.file;
        api_req
    }
}

pub struct FilesService {
    config: Config,
}

impl FilesService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传文件
    pub fn upload_all(
        &self,
        mut api_req: ApiReq,
        option: Option<RequestOption>,
    ) -> SDKResult<ApiResponse<UploadAllResponse>> {
        api_req.http_method = "POST".to_string();
        api_req.api_path = "/open-apis/drive/v1/files/upload_all".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option)?;

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
    fn standard_data_format() -> bool {
        true
    }
}

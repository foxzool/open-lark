//! 下载文件
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/file/get

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{common::api_utils::extract_response_data, endpoints::IM_V1_FILES};

/// 下载文件请求
pub struct GetFileRequest {
    config: Config,
    file_key: String,
}

impl GetFileRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            file_key: String::new(),
        }
    }

    /// 文件 key（路径参数）
    pub fn file_key(mut self, file_key: impl Into<String>) -> Self {
        self.file_key = file_key.into();
        self
    }

    /// 执行请求（返回二进制内容）
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/file/get
    pub async fn execute(self) -> SDKResult<Vec<u8>> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<Vec<u8>> {
        validate_required!(self.file_key, "file_key 不能为空");

        // url: GET:/open-apis/im/v1/files/:file_key
        let req: ApiRequest<Vec<u8>> =
            ApiRequest::get(format!("{}/{}", IM_V1_FILES, self.file_key));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "下载文件")
    }
}

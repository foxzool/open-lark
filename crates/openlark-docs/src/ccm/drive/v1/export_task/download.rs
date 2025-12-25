use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 下载导出文件
///
/// 下载导出的文件内容。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/download
/// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/download

use crate::common::api_endpoints::DriveApi;

/// 下载导出文件请求
#[derive(Debug, Clone)]
pub struct DownloadExportRequest {
    config: Config,
    /// 文件token
    pub file_token: String,
}

impl DownloadExportRequest {
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
        }
    }

    /// 执行下载请求，返回二进制内容
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }

        let api_endpoint = DriveApi::DownloadExportFile(self.file_token.clone());

        let api_request = ApiRequest::<Vec<u8>>::get(&api_endpoint.to_url());

        Transport::request(api_request, &self.config, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_export_request_builder() {
        let config = Config::default();
        let request = DownloadExportRequest::new(config, "file_token");
        assert_eq!(request.file_token, "file_token");
    }
}

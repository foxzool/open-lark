//! 下载导出文件
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/export/download

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 下载导出文件请求

#[derive(Debug, Clone)]
pub struct DownloadExportRequest {
    /// 配置信息
    config: Config,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 下载导出文件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadExportResponse {
    /// 下载 URL
    pub url: String,
}

impl ApiResponseTrait for DownloadExportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DownloadExportRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/export/download
    pub async fn execute(self) -> SDKResult<DownloadExportResponse> {
        let api_endpoint = VcApiV1::ExportGet("download".to_string());
        let mut api_request: ApiRequest<DownloadExportResponse> =
            ApiRequest::get(api_endpoint.to_url());

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "下载导出文件")
    }
}

/// 下载导出文件请求构建器

#[derive(Debug, Clone)]
pub struct DownloadExportRequestBuilder {
    request: DownloadExportRequest,
}

impl DownloadExportRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DownloadExportRequest::new(config),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request = self.request.query_param(key, value);
        self
    }

    /// 构建请求
    pub fn build(self) -> DownloadExportRequest {
        self.request
    }
}

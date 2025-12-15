/// 下载图片
///
/// 通过 file_token 下载原图片。
/// docPath: https://open.feishu.cn/document/lingo-v1/file/download
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct DownloadFileResponse {
    pub data: Option<FileData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FileData {
    pub file_token: String,
    pub file_url: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_type: String,
    pub download_time: String,
}

impl ApiResponseTrait for DownloadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载图片
///
/// 通过 file_token 下载原图片。
/// docPath: https://open.feishu.cn/document/lingo-v1/file/download
pub async fn download_file(
    config: &Config,
    file_token: &str,
) -> SDKResult<FileData> {
    // 验证必填字段
    validate_required_field("文件令牌", Some(file_token), "文件令牌不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::FileDownload(file_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DownloadFileResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: DownloadFileResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("file_data", "File data is missing")
    })
}
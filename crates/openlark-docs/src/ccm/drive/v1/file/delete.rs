/// 删除文件或文件夹
///
/// 删除用户在云空间内的文件或者文件夹。文件或者文件夹被删除后，会进入用户回收站里。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/delete

use serde::{Deserialize, Serialize};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 删除文件或文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileResponse {
    /// 删除结果
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除文件或文件夹
///
/// 删除用户在云空间内的文件或者文件夹。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/delete
pub async fn delete_file(
    config: &Config,
    file_token: &str,
) -> SDKResult<DeleteFileResponse> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::DeleteFile(file_token.to_string());

    // 创建API请求
    let api_request: ApiRequest<DeleteFileResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除文件")
}
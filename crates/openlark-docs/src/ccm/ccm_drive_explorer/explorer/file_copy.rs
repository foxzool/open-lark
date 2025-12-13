/// 复制文件
///
/// 此接口用于复制指定文件到目标位置，支持跨文件夹复制。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/explorer/file_copy
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{
    requests::FileCopyRequest,
    responses::FileCopyData,
};

/// 复制文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCopyResponse {
    /// 复制后的文件信息
    pub data: Option<FileCopyData>,
}

impl ApiResponseTrait for FileCopyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 复制文件
pub async fn file_copy(
    request: FileCopyRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<FileCopyData> {
    use crate::common::api_endpoints::CcmDriveExplorerApi;
    use serde_json::json;

    // 使用CcmDriveExplorerApi枚举生成API端点
    let api_endpoint = CcmDriveExplorerApi::FileCopy(request.file_token.clone());

    // 构建请求体
    let mut body = json!({
        "parent_folder_token": request.folder_token
    });

    if let Some(name) = &request.name {
        body["name"] = json!(name);
    }

    // 创建API请求
    let mut api_request: ApiRequest<FileCopyResponse> = ApiRequest::post(&api_endpoint.to_url())
        .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    let response = Transport::request(api_request, config, None).await?;

    // 返回数据
    response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })
}

/// 上传文件
///
/// 向云空间指定目录下上传一个小文件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/upload_all

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};



// 导入序列化支持
use serde::{Deserialize, Serialize};
use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileResponse {
    /// 上传结果信息
    pub data: Option<UploadFileData>,
}

/// 上传文件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileData {
    /// 文件token
    pub file_token: String,
    /// 文件名称
    pub name: String,
    /// 文件大小
    pub size: i64,
    /// 文件类型
    pub r#type: String,
    /// 上传时间
    pub created_time: String,
}

impl ApiResponseTrait for UploadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传文件
///
/// 向云空间指定目录下上传一个小文件。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/upload_all
pub async fn upload_file(
    config: &Config,
    params: serde_json::Value,
) -> SDKResult<UploadFileResponse> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::UploadFile;

    // 创建API请求
    let api_request: ApiRequest<UploadFileResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(params);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "上传文件")
}
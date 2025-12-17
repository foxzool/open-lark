/// 新建文件
///
/// 根据 folderToken 创建 Doc、 Sheet 或 Bitable 。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create-online-document
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 新建文件请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileParams {
    /// 父文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 文件类型，可选值：doc、sheet、bitable、mindnote
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 文件标题，长度限制：1-100字符
    pub title: String,
}

/// 新建文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileResponse {
    /// 文件信息
    pub data: Option<FileInfo>,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
    /// 文件标题
    pub title: String,
    /// 文件类型
    #[serde(rename = "doc_type")]
    pub doc_type: String,
    /// 预览URL
    #[serde(rename = "preview_url")]
    pub preview_url: Option<String>,
}

impl ApiResponseTrait for CreateFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新建文件请求
pub struct CreateFileRequest {
    config: Config,
}

impl CreateFileRequest {
    /// 创建新建文件请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create-online-document
    pub async fn execute(self, params: CreateFileParams) -> SDKResult<CreateFileResponse> {
        // 验证必填字段
        validate_required!(params.folder_token, "父文件夹token不能为空");
        validate_required!(params.doc_type, "文件类型不能为空");
        validate_required!(params.title, "文件标题不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDriveExplorerApiOld::File(params.folder_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CreateFileResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_value(params).map_err(|e| {
            openlark_core::error::validation_error(
                "参数序列化失败",
                &format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

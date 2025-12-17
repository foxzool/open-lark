/// 复制文档
///
/// 复制指定的文档或表格。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/copy-a-doc-or-sheet

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 复制文档请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileParams {
    /// 源文件token
    #[serde(rename = "file_token")]
    pub file_token: String,
    /// 目标文件夹token，不填则复制到"我的空间"
    #[serde(rename = "folder_token")]
    pub folder_token: Option<String>,
    /// 新文件名称，不填则使用原名称加"副本"
    #[serde(rename = "name")]
    pub name: Option<String>,
}

/// 复制文档响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileResponse {
    /// 复制结果
    pub data: Option<CopyResult>,
}

/// 复制结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyResult {
    /// 新文件token
    #[serde(rename = "file_token")]
    pub file_token: String,
    /// 新文件类型
    #[serde(rename = "file_type")]
    pub file_type: String,
    /// 新文件名称
    pub name: String,
}

impl ApiResponseTrait for CopyFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 复制文档请求
pub struct CopyFileRequest {
    config: Config,
}

impl CopyFileRequest {
    /// 创建复制文档请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/copy-files
    pub async fn execute(
        self,
        params: CopyFileParams,
    ) -> SDKResult<CopyFileResponse> {
        // 验证必填字段
        validate_required!(params.file_token, "源文件token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDriveExplorerApiOld::FileCopy(params.file_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CopyFileResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serde_json::to_value(params).map_err(|e| {
                    openlark_core::error::validation_error(
                        "参数序列化失败",
                        &format!("无法序列化请求参数: {}", e)
                    )
                })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

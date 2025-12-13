/// 新建文件夹
///
/// 根据 folderToken 在该 folder 下创建文件夹。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/create-a-new-folder

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 新建文件夹请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderParams {
    /// 父文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 文件夹名称，长度限制：1-100字符
    pub name: String,
}

/// 新建文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderResponse {
    /// 文件夹信息
    pub data: Option<FolderInfo>,
}

/// 文件夹信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    /// 文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新建文件夹请求
pub struct CreateFolderRequest {
    config: Config,
}

impl CreateFolderRequest {
    /// 创建新建文件夹请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/create-a-new-folder
    pub async fn execute(
        self,
        params: CreateFolderParams,
    ) -> SDKResult<CreateFolderResponse> {
        // 验证必填字段
        validate_required!(params.folder_token, "父文件夹token不能为空");
        validate_required!(params.name, "文件夹名称不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDriveExplorerApiOld::Folder(params.folder_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<CreateFolderResponse> =
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
//! 获取文件夹元数据
//!
//! docPath: /document/ukTMukTMukTM/uAjNzUjLwYzM14CM2MTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uAjNzUjLwYzM14CM2MTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDriveExplorerApiOld, api_utils::*};

/// 获取文件夹元数据响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderMetaResp {
    /// 文件夹 id
    pub id: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹 token
    pub token: String,
    /// 创建者 uid
    #[serde(rename = "createUid")]
    pub create_uid: String,
    /// 最后编辑者 uid
    #[serde(rename = "editUid")]
    pub edit_uid: String,
    /// 父目录 id
    #[serde(rename = "parentId")]
    pub parent_id: String,
    /// 所有者 uid
    #[serde(rename = "ownUid")]
    pub own_uid: String,
}

impl ApiResponseTrait for GetFolderMetaResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹元数据请求
pub struct GetFolderMetaRequest {
    config: Config,
    folder_token: String,
}

impl GetFolderMetaRequest {
    pub fn new(config: Config, folder_token: impl Into<String>) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetFolderMetaResp> {
            self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
        }

        pub async fn execute_with_options(
            self,
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<GetFolderMetaResp> {

        validate_required!(self.folder_token, "folderToken 不能为空");

        let api_request: ApiRequest<GetFolderMetaResp> =
            ApiRequest::get(&CcmDriveExplorerApiOld::FolderMeta(self.folder_token).to_url());

        
            let response = Transport::request(api_request, &self.config, 
Some(option)).await?;
                extract_response_data(response, "操作")}
}

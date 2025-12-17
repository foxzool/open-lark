//! 根据 folderToken 在该 folder 下创建文件夹。
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/create-a-new-folder

pub mod children;
pub mod meta;
pub use children::*;
pub use meta::*;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateFolderReq {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateFolderResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新建文件夹请求
pub struct CreateFolderRequest {
    config: Config,
    folder_token: String,
    req: CreateFolderReq,
}

impl CreateFolderRequest {
    pub fn new(config: Config, folder_token: impl Into<String>) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            req: CreateFolderReq::default(),
        }
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        self.req.name = name.to_string();
        self
    }

    pub async fn send(self) -> SDKResult<CreateFolderResponse> {
        use crate::common::api_endpoints::CcmDriveExplorerApiOld;

        let api_request: ApiRequest<CreateFolderResponse> =
            ApiRequest::post(&CcmDriveExplorerApiOld::Folder(self.folder_token).to_url())
                .body(serde_json::to_value(&self.req)?);

        let response: Response<CreateFolderResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

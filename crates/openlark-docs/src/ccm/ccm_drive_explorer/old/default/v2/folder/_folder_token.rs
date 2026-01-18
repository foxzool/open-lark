//! 根据 folderToken 在该 folder 下创建文件夹。
//!
//! docPath: /document/ukTMukTMukTM/ukTNzUjL5UzM14SO1MTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/ukTNzUjL5UzM14SO1MTN

pub mod children;
pub mod meta;
pub use children::*;
pub use meta::*;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateFolderReq {
    pub title: String,
}

/// 新建文件夹响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateFolderResp {
    /// 文件夹 URL
    pub url: String,
    /// 修订版本号
    pub revision: i32,
    /// 文件夹 token
    pub token: String,
}

impl ApiResponseTrait for CreateFolderResp {
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

    pub fn title(mut self, title: impl ToString) -> Self {
        self.req.title = title.to_string();
        self
    }

    pub async fn execute(self) -> SDKResult<CreateFolderResp> {
        use crate::common::api_endpoints::CcmDriveExplorerApiOld;

        validate_required!(self.folder_token, "folderToken 不能为空");
        validate_required!(self.req.title, "title 不能为空");

        let api_request: ApiRequest<CreateFolderResp> =
            ApiRequest::post(&CcmDriveExplorerApiOld::Folder(self.folder_token).to_url())
                .body(serialize_params(&self.req, "新建文件夹")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "新建文件夹")
    }
}

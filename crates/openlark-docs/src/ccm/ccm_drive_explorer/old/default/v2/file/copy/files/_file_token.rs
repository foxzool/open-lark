//! 复制文档/表格
//!
//! docPath: /document/ukTMukTMukTM/uYTNzUjL2UzM14iN1MTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uYTNzUjL2UzM14iN1MTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDriveExplorerApiOld, api_utils::*};

/// 复制文件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyFileType {
    /// 文档
    Doc,
    /// 电子表格
    Sheet,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopyFileReq {
    /// 文件类型
    #[serde(rename = "type")]
    pub type_: CopyFileType,
    /// 目标文件夹 token，不传默认复制到「我的空间」
    #[serde(rename = "dstFolderToken", skip_serializing_if = "Option::is_none")]
    pub dst_folder_token: Option<String>,
    /// 新文件名，不传默认「原名 + 副本」
    #[serde(rename = "dstName", skip_serializing_if = "Option::is_none")]
    pub dst_name: Option<String>,
    /// 是否需要评论（可选）
    #[serde(rename = "commentNeeded", skip_serializing_if = "Option::is_none")]
    pub comment_needed: Option<bool>,
}

impl CopyFileReq {
    pub fn new(file_type: CopyFileType) -> Self {
        Self {
            type_: file_type,
            dst_folder_token: None,
            dst_name: None,
            comment_needed: None,
        }
    }
}

/// 复制文件响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopyFileResp {
    /// 目标文件夹 token
    #[serde(rename = "folderToken")]
    pub folder_token: String,
    /// 修订版本号
    pub revision: i32,
    /// 新文件 token
    pub token: String,
    /// 新文件类型
    #[serde(rename = "type")]
    pub type_: String,
    /// 新文件 URL
    pub url: String,
}

impl ApiResponseTrait for CopyFileResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 复制文档/表格请求
pub struct CopyFileRequest {
    config: Config,
    file_token: String,
    req: CopyFileReq,
}

impl CopyFileRequest {
    pub fn new(config: Config, file_token: impl Into<String>, file_type: CopyFileType) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            req: CopyFileReq::new(file_type),
        }
    }

    /// 目标文件夹 token（可选）
    pub fn dst_folder_token(mut self, dst_folder_token: impl Into<String>) -> Self {
        self.req.dst_folder_token = Some(dst_folder_token.into());
        self
    }

    /// 新文件名（可选）
    pub fn dst_name(mut self, dst_name: impl Into<String>) -> Self {
        self.req.dst_name = Some(dst_name.into());
        self
    }

    /// 是否需要评论（可选）
    pub fn comment_needed(mut self, comment_needed: bool) -> Self {
        self.req.comment_needed = Some(comment_needed);
        self
    }

    pub async fn send(self) -> SDKResult<CopyFileResp> {
        validate_required!(self.file_token, "fileToken 不能为空");

        let api_request: ApiRequest<CopyFileResp> =
            ApiRequest::post(&CcmDriveExplorerApiOld::FileCopy(self.file_token).to_url())
                .body(serialize_params(&self.req, "复制文档")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "复制文档")
    }
}

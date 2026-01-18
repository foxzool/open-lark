//! 新建文件
//!
//! 根据 folderToken 创建 Doc、 Sheet 或 Bitable。
//!
//! docPath: /document/ukTMukTMukTM/uQTNzUjL0UzM14CN1MTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uQTNzUjL0UzM14CN1MTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDriveExplorerApiOld, api_utils::*};

/// 新建文件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateFileType {
    /// 电子表格
    Sheet,
    /// 多维表格
    Bitable,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateFileReq {
    /// 文件标题
    pub title: String,
    /// 文件类型（不传默认创建 doc）
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateFileType>,
}

/// 新建文件响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateFileResp {
    /// 文件 URL
    pub url: String,
    /// 文件 token
    pub token: String,
    /// 修订版本号
    pub revision: i32,
}

impl ApiResponseTrait for CreateFileResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新建文件请求
pub struct CreateFileRequest {
    config: Config,
    folder_token: String,
    req: CreateFileReq,
}

impl CreateFileRequest {
    pub fn new(config: Config, folder_token: impl Into<String>) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            req: CreateFileReq::default(),
        }
    }

    /// 文件标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.req.title = title.into();
        self
    }

    /// 指定创建 Sheet
    pub fn sheet(mut self) -> Self {
        self.req.type_ = Some(CreateFileType::Sheet);
        self
    }

    /// 指定创建 Bitable
    pub fn bitable(mut self) -> Self {
        self.req.type_ = Some(CreateFileType::Bitable);
        self
    }

    pub async fn execute(self) -> SDKResult<CreateFileResp> {
        validate_required!(self.folder_token, "folderToken 不能为空");
        validate_required!(self.req.title, "title 不能为空");

        let api_request: ApiRequest<CreateFileResp> =
            ApiRequest::post(&CcmDriveExplorerApiOld::File(self.folder_token).to_url())
                .body(serialize_params(&self.req, "新建文件")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "新建文件")
    }
}

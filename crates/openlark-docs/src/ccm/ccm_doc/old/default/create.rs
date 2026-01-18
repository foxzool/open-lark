//! 创建并初始化文档
//!
//! docPath: /document/ukTMukTMukTM/ugDM2YjL4AjN24COwYjN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/ugDM2YjL4AjN24COwYjN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;
use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateDocReq {
    /// 文档所在文件夹 token
    #[serde(skip_serializing_if = "Option::is_none", rename = "FolderToken")]
    pub folder_token: Option<String>,
    /// 初始内容（文档数据结构序列化后的字符串）
    #[serde(skip_serializing_if = "Option::is_none", rename = "Content")]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateDocResp {
    #[serde(rename = "objToken")]
    pub obj_token: String,
    pub url: String,
}

impl ApiResponseTrait for CreateDocResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建旧版文档请求
pub struct CreateDocRequest {
    config: Config,
    req: CreateDocReq,
}

impl CreateDocRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            req: CreateDocReq::default(),
        }
    }

    /// 文档所在文件夹 token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.req.folder_token = Some(folder_token.into());
        self
    }

    /// 初始内容（可选）
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.req.content = Some(content.into());
        self
    }

    pub async fn execute(self) -> SDKResult<CreateDocResp> {
        let api_request: ApiRequest<CreateDocResp> =
            ApiRequest::post(&CcmDocApiOld::Create.to_url())
                .body(serialize_params(&self.req, "创建旧版文档")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建旧版文档")
    }
}

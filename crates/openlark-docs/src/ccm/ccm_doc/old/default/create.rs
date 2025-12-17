//! 创建并初始化文档
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/apiRef/create-document

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateDocReq {
    /// 文档所在文件夹 token
    #[serde(skip_serializing_if = "Option::is_none", rename = "folderToken")]
    pub folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub async fn send(self) -> SDKResult<CreateDocResp> {
        let api_request: ApiRequest<CreateDocResp> =
            ApiRequest::post(&CcmDocApiOld::Create.to_url()).body(serde_json::to_value(&self.req)?);

        let response: Response<CreateDocResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

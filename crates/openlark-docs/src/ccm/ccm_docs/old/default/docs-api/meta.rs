/// 获取元数据
///
/// 根据 token 获取各类文件的元数据。
/// docPath: /document/ukTMukTMukTM/uMjN3UjLzYzN14yM2cTN
/// doc: https://open.feishu.cn/document/ukTMukTMukTM/uMjN3UjLzYzN14yM2cTN
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaReq {
    pub request_docs: Vec<RequestDoc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDoc {
    pub docs_token: String,
    pub docs_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaResponse {
    #[serde(default)]
    pub docs_metas: Vec<DocsMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsMeta {
    pub docs_token: String,
    pub docs_type: String,
    pub title: String,
    pub owner_id: String,
    pub create_time: i64,
    pub latest_modify_user: String,
    pub latest_modify_time: i64,
}

impl ApiResponseTrait for GetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取元数据请求（旧版）
pub struct GetMetaRequest {
    config: Config,
    req: GetMetaReq,
}

impl GetMetaRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            req: GetMetaReq {
                request_docs: Vec::new(),
            },
        }
    }

    /// 请求文档列表
    pub fn request_docs(mut self, request_docs: Vec<RequestDoc>) -> Self {
        self.req.request_docs = request_docs;
        self
    }

    pub async fn execute(self) -> SDKResult<GetMetaResponse> {
        if self.req.request_docs.is_empty() {
            return Err(openlark_core::error::validation_error(
                "request_docs",
                "request_docs 不能为空",
            ));
        }
        if self.req.request_docs.len() > 200 {
            return Err(openlark_core::error::validation_error(
                "request_docs",
                "request_docs 一次不超过 200 个",
            ));
        }
        for (idx, doc) in self.req.request_docs.iter().enumerate() {
            if doc.docs_token.is_empty() {
                return Err(openlark_core::error::validation_error(
                    &format!("request_docs[{}].docs_token", idx),
                    "docs_token 不能为空",
                ));
            }
            if doc.docs_type.is_empty() {
                return Err(openlark_core::error::validation_error(
                    &format!("request_docs[{}].docs_type", idx),
                    "docs_type 不能为空",
                ));
            }
        }

        use crate::common::api_endpoints::CcmDocsApiOld;

        let api_request: ApiRequest<GetMetaResponse> =
            ApiRequest::post(&CcmDocsApiOld::Meta.to_url())
                .body(serialize_params(&self.req, "获取元数据")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取元数据")
    }
}

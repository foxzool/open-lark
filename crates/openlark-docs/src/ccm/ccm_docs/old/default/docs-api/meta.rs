/// 获取元数据
///
/// 根据 token 获取各类文件的元数据。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

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

    pub async fn send(self) -> SDKResult<GetMetaResponse> {
        use crate::common::api_endpoints::CcmDocsApiOld;

        let api_request: ApiRequest<GetMetaResponse> =
            ApiRequest::post(&CcmDocsApiOld::Meta.to_url()).body(serde_json::to_value(&self.req)?);

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

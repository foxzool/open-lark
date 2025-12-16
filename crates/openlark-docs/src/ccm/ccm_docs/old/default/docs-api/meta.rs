/// 获取元数据
///
/// 根据 token 获取各类文件的元数据。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/obtain-metadata
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaRequest {
    pub request_docs: Vec<RequestDoc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDoc {
    pub docs_token: String,
    pub docs_type: String,
}

impl GetMetaRequest {
    pub fn new(request_docs: Vec<RequestDoc>) -> Self {
        Self { request_docs }
    }
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

pub async fn get_meta(
    request: GetMetaRequest,
    config: &Config,
) -> SDKResult<Response<GetMetaResponse>> {
    let mut api_request: ApiRequest<GetMetaResponse> = ApiRequest::post("/open-apis/suite/docs-api/meta")
        .body(serde_json::to_value(request)?);
    
    Transport::request(api_request, config, None).await
}

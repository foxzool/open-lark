//! 获取旧版文档元信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-document-meta

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocMetaReq {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocMetaResponse {
    pub create_date: String,
    pub create_time: i64,
    pub creator: String,
    pub create_user_name: String,
    pub delete_flag: i32,
    pub edit_time: i64,
    pub edit_user_name: String,
    pub owner_user_name: String,
    pub server_time: i64,
    pub tenant_id: String,
    pub title: String,
    pub type_: i32,
    pub url: String,
}

impl ApiResponseTrait for GetDocMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档元信息请求
pub struct GetDocMetaRequest {
    config: Config,
    doc_token: String,
}

impl GetDocMetaRequest {
    pub fn new(config: Config, doc_token: impl Into<String>) -> Self {
        Self {
            config,
            doc_token: doc_token.into(),
        }
    }

    pub async fn send(self) -> SDKResult<GetDocMetaResponse> {
        use crate::common::api_endpoints::CcmDocApiOld;

        let api_request: ApiRequest<GetDocMetaResponse> =
            ApiRequest::get(&CcmDocApiOld::Meta(self.doc_token).to_url());
        let response: Response<GetDocMetaResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

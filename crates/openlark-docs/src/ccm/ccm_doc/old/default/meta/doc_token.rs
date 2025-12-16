use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 获取旧版文档元信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocMetaRequest {
    /// 文档 token
    pub doc_token: String,
}

impl GetDocMetaRequest {
    /// 创建新的 GetDocMetaRequest
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            doc_token: doc_token.into(),
        }
    }
}

/// 获取旧版文档元信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocMetaResponse {
    /// 创建日期
    pub create_date: String,
    /// 创建时间戳
    pub create_time: i64,
    /// 创建者 OpenID
    pub creator: String,
    /// 删除状态，0：未删除，1：已删除
    pub is_deleted: i32,
    /// 拥有者 OpenID
    pub owner: String,
    /// 文档标题
    pub title: String,
    /// 文档类型
    pub r#type: String,
    /// 更新时间戳
    pub update_time: i64,
}

impl ApiResponseTrait for GetDocMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档元信息
///
/// 根据 docToken 获取元数据。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-document-meta
pub async fn get_doc_meta(
    request: GetDocMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetDocMetaResponse>> {
    let api_endpoint = CcmDocApiOld::Meta(request.doc_token.clone());
    let mut api_request: ApiRequest<GetDocMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_doc_meta_request() {
        let request = GetDocMetaRequest::new("doc_token");
        assert_eq!(request.doc_token, "doc_token");
    }
}

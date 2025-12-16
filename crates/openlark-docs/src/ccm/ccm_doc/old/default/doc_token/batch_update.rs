use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 编辑旧版文档内容请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateDocRequest {
    /// 文档 token
    #[serde(skip)]
    pub doc_token: String,
    /// 文档修订版号，为空时会从最新的版本开始编辑
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<i64>,
    /// 操作序列，支持插入、删除、替换等操作
    pub requests: Vec<serde_json::Value>,
}

impl BatchUpdateDocRequest {
    /// 创建新的 BatchUpdateDocRequest
    pub fn new(doc_token: impl Into<String>, requests: Vec<serde_json::Value>) -> Self {
        Self {
            doc_token: doc_token.into(),
            revision_id: None,
            requests,
        }
    }

    /// 设置 revision_id
    pub fn revision_id(mut self, revision_id: i64) -> Self {
        self.revision_id = Some(revision_id);
        self
    }
}

/// 编辑旧版文档内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateDocResponse {
    /// 文档 token
    pub doc_token: String,
    /// 成功更新后的文档修订版号
    pub revision_id: i64,
}

impl ApiResponseTrait for BatchUpdateDocResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 编辑旧版文档内容
///
/// 批量编辑更新文档内容，包括更新标题、范围删除、插入内容。
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs/docs/content/batch-update-document
pub async fn batch_update_doc(
    request: BatchUpdateDocRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchUpdateDocResponse>> {
    let api_endpoint = CcmDocApiOld::BatchUpdate(request.doc_token.clone());
    let mut api_request: ApiRequest<BatchUpdateDocResponse> = ApiRequest::post(&api_endpoint.to_url())
        .body(serde_json::json!(&request));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_update_doc_request() {
        let request = BatchUpdateDocRequest::new("doc_token", vec![]);
        assert_eq!(request.doc_token, "doc_token");
        assert!(request.revision_id.is_none());
    }
}

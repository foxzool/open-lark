//! 编辑旧版文档内容
//!
//! docPath: /document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateDocRequest {
    /// 文档版本号（>=0）
    #[serde(rename = "Revision")]
    pub revision: i32,
    /// OperationRequest 序列化后的 JSON 字符串数组
    #[serde(rename = "Requests")]
    pub requests: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateDocResponse {}

impl ApiResponseTrait for BatchUpdateDocResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量编辑更新旧版文档内容请求
pub struct BatchUpdateDocRequestBuilder {
    config: Config,
    doc_token: String,
    req: BatchUpdateDocRequest,
}

impl BatchUpdateDocRequestBuilder {
    pub fn new(config: Config, doc_token: impl Into<String>) -> Self {
        Self {
            config,
            doc_token: doc_token.into(),
            req: BatchUpdateDocRequest::default(),
        }
    }

    pub fn revision(mut self, revision: i32) -> Self {
        self.req.revision = revision;
        self
    }

    /// 设置 OperationRequest 序列化后的 JSON 字符串数组
    pub fn requests(mut self, requests: Vec<String>) -> Self {
        self.req.requests = requests;
        self
    }

    /// 追加单个 OperationRequest（以 JSON 形式传入，内部会序列化为字符串）
    pub fn push_request_json(mut self, request: serde_json::Value) -> Self {
        self.req.requests.push(request.to_string());
        self
    }

    pub async fn execute(self) -> SDKResult<BatchUpdateDocResponse> {
            self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
        }

        pub async fn execute_with_options(
            self,
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<BatchUpdateDocResponse> {

        validate_required!(self.doc_token, "doc_token 不能为空");
        if self.req.revision < 0 {
            return Err(openlark_core::error::validation_error(
                "revision",
                "revision 必须 >= 0",
            ));
        }
        validate_required!(self.req.requests, "Requests 不能为空");

        use crate::common::api_endpoints::CcmDocApiOld;

        let api_request: ApiRequest<BatchUpdateDocResponse> =
            ApiRequest::post(&CcmDocApiOld::BatchUpdate(self.doc_token).to_url())
                .body(serialize_params(&self.req, "编辑旧版文档内容")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "编辑旧版文档内容")
    }
}

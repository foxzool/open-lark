//! 词条高亮
//!
//! docPath: https://open.feishu.cn/document/lingo-v1/entity/highlight

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::LingoApiV1;

/// 词条所在位置（span）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    /// 关键词开始位置，从 0 开始计数（utf-8）
    pub start: i32,
    /// 关键词结束位置，从 0 开始计数（utf-8）
    pub end: i32,
}

/// 识别到的词条信息（phrase）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phrase {
    /// 识别到的关键词
    pub name: String,
    /// 对应的词条 ID
    pub entity_ids: Vec<String>,
    /// 词条所在位置
    pub span: Span,
}

/// 词条高亮响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightEntityResp {
    /// 识别到的词条信息
    #[serde(default)]
    pub phrases: Vec<Phrase>,
}

impl ApiResponseTrait for HighlightEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条高亮请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightEntityBody {
    /// 需要识别词条的内容（不超过 1000 字）
    pub text: String,
}

/// 词条高亮请求
pub struct HighlightEntityRequest {
    config: Config,
    body: HighlightEntityBody,
}

impl HighlightEntityRequest {
    pub fn new(config: Config, text: impl Into<String>) -> Self {
        Self {
            config,
            body: HighlightEntityBody { text: text.into() },
        }
    }

    pub async fn execute(self) -> SDKResult<HighlightEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<HighlightEntityResp> {
        validate_required!(self.body.text, "text 不能为空");

        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::serialization_error("序列化词条高亮请求体失败", Some(e))
        })?;

        let api_request: ApiRequest<HighlightEntityResp> =
            ApiRequest::post(&LingoApiV1::EntityHighlight.to_url()).body(body);

        let response: Response<HighlightEntityResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

//! 词条高亮
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/highlight
//! doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/highlight

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HighlightEntityReqBody {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HighlightEntityResponse {
    #[serde(default)]
    pub phrases: Vec<Phrase>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Phrase {
    pub name: String,
    pub entity_ids: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Span {
    pub start: i32,
    pub end: i32,
}

impl ApiResponseTrait for HighlightEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条高亮请求
pub struct HighlightEntityRequest {
    config: Config,
    req: HighlightEntityReqBody,
}

impl HighlightEntityRequest {
    pub fn new(config: Config, text: impl Into<String>) -> Self {
        Self {
            config,
            req: HighlightEntityReqBody { text: text.into() },
        }
    }

    pub async fn send(self) -> SDKResult<HighlightEntityResponse> {
        use crate::common::api_endpoints::BaikeApiV1;
        validate_required!(self.req.text, "text 不能为空");
        let len = self.req.text.chars().count();
        if !(1..=1000).contains(&len) {
            return Err(openlark_core::error::validation_error(
                "text",
                "text 长度必须在 1~1000 字符之间",
            ));
        }

        let api_request: ApiRequest<HighlightEntityResponse> =
            ApiRequest::post(&BaikeApiV1::EntityHighlight.to_url())
                .body(serde_json::to_value(&self.req)?);

        let response: Response<HighlightEntityResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

//! 精准搜索词条
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/match

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BaikeApiV1;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchEntityReq {
    /// 搜索关键词，将与词条名、别名进行精准匹配
    pub word: String,
}

/// 精准搜索词条响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchEntityResp {
    #[serde(default)]
    pub results: Vec<MatchEntityResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchEntityResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// 匹配类型（文档示例为 int，如 0）
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
}

impl ApiResponseTrait for MatchEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 精准搜索词条请求
pub struct MatchEntityRequest {
    config: Config,
    req: MatchEntityReq,
}

impl MatchEntityRequest {
    pub fn new(config: Config, word: impl Into<String>) -> Self {
        Self {
            config,
            req: MatchEntityReq { word: word.into() },
        }
    }

    pub async fn send(self) -> SDKResult<MatchEntityResp> {
        validate_required!(self.req.word, "word 不能为空");
        let len = self.req.word.chars().count();
        if !(1..=100).contains(&len) {
            return Err(openlark_core::error::validation_error(
                "word",
                "word 长度必须在 1~100 字符之间",
            ));
        }

        let api_request: ApiRequest<MatchEntityResp> =
            ApiRequest::post(&BaikeApiV1::EntityMatch.to_url())
                .body(serde_json::to_value(&self.req)?);

        let response: Response<MatchEntityResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

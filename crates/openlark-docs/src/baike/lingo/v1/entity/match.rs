//! 精准搜索词条
//!
//! docPath: https://open.feishu.cn/document/lingo-v1/entity/match

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::common::api_endpoints::LingoApiV1;

/// 匹配字段（schema: TermType）
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(i32)]
pub enum TermType {
    /// 词条名
    MainKey = 0,
    /// 全称
    FullName = 1,
    /// 别名
    Alias = 2,
}

/// 匹配结果项（match_info）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchInfo {
    /// 词条 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// 匹配中的字段
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_type: Option<TermType>,
}

/// 精准搜索词条响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEntityResp {
    /// 搜索结果
    #[serde(default)]
    pub results: Vec<MatchInfo>,
}

impl ApiResponseTrait for MatchEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEntityBody {
    /// 搜索关键词，将与词条名、别名进行精准匹配
    pub word: String,
}

/// 精准搜索词条请求
pub struct MatchEntityRequest {
    config: Config,
    repo_id: Option<String>,
    body: MatchEntityBody,
}

impl MatchEntityRequest {
    pub fn new(config: Config, word: impl Into<String>) -> Self {
        Self {
            config,
            repo_id: None,
            body: MatchEntityBody { word: word.into() },
        }
    }

    /// 词库ID（不传时默认在全员词库内搜索）
    pub fn repo_id(mut self, repo_id: impl Into<String>) -> Self {
        self.repo_id = Some(repo_id.into());
        self
    }

    pub async fn execute(self) -> SDKResult<MatchEntityResp> {
        validate_required!(self.body.word, "word 不能为空");

        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::serialization_error("序列化精准搜索词条请求体失败", Some(e))
        })?;

        let mut api_request: ApiRequest<MatchEntityResp> =
            ApiRequest::post(&LingoApiV1::EntityMatch.to_url()).body(body);
        if let Some(repo_id) = &self.repo_id {
            api_request = api_request.query("repo_id", repo_id);
        }

        let response: Response<MatchEntityResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

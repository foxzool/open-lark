//! 精准搜索词条
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/match

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
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

    pub async fn execute(self) -> SDKResult<MatchEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<MatchEntityResp> {
        // ===== 验证必填字段 =====
        validate_required!(self.req.word, "word 不能为空");
        // ===== 验证字段长度 =====
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
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    /// 测试构建器模式
    #[test]
    fn test_match_entity_request_builder() {
        let config = Config::default();
        let request = MatchEntityRequest::new(config, "搜索词");
        assert_eq!(request.req.word, "搜索词");
    }

    /// 测试 word 为空时的验证
    #[test]
    fn test_empty_word_validation() {
        let config = Config::default();
        let request = MatchEntityRequest::new(config, "");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 word 长度边界值
    #[test]
    fn test_word_length_boundaries() {
        let config = Config::default();

        // 最小长度 1
        let request1 = MatchEntityRequest::new(config.clone(), "a");
        assert_eq!(request1.req.word, "a");

        // 最大长度 100
        let word_100 = "a".repeat(100);
        let request2 = MatchEntityRequest::new(config.clone(), word_100);
        assert_eq!(request2.req.word.chars().count(), 100);

        // 超过 100
        let word_101 = "a".repeat(101);
        let request3 = MatchEntityRequest::new(config, word_101);

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request3.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 Unicode 字符计数
    #[test]
    fn test_unicode_character_count() {
        let config = Config::default();
        let word = "🎉🎊🎈"; // 3 个 Unicode 码点
        let request = MatchEntityRequest::new(config, word);
        assert_eq!(request.req.word.chars().count(), 3);
    }
}

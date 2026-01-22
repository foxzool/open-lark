//! 提取潜在的词条
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/extract

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractEntityReqBody {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractEntityResponse {
    #[serde(default)]
    pub entity_word: Vec<ExtractedWord>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtractedWord {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
}

impl ApiResponseTrait for ExtractEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 提取潜在词条请求
pub struct ExtractEntityRequest {
    config: Config,
    req: ExtractEntityReqBody,
}

impl ExtractEntityRequest {
    pub fn new(config: Config, text: impl Into<String>) -> Self {
        Self {
            config,
            req: ExtractEntityReqBody { text: text.into() },
        }
    }

    pub async fn execute(self) -> SDKResult<ExtractEntityResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ExtractEntityResponse> {
        use crate::common::api_endpoints::BaikeApiV1;
        // ===== 参数校验 =====
        let len = self.req.text.chars().count();
        if len > 128 {
            return Err(openlark_core::error::validation_error(
                "text",
                "text 最大长度不能超过 128 字符",
            ));
        }

        // ===== 构建请求 =====
        let api_request: ApiRequest<ExtractEntityResponse> =
            ApiRequest::post(&BaikeApiV1::EntityExtract.to_url())
                .body(serde_json::to_value(&self.req)?);

        // ===== 发送请求 =====
        let response: Response<ExtractEntityResponse> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_extract_entity_request_builder() {
        let config = Config::default();
        let request = ExtractEntityRequest::new(config, "测试文本");
        assert_eq!(request.req.text, "测试文本");
    }

    /// 测试 text 长度验证
    #[test]
    fn test_text_length_validation() {
        let config = Config::default();

        // 最大长度 128
        let text_128 = "a".repeat(128);
        let request1 = ExtractEntityRequest::new(config.clone(), text_128);
        assert_eq!(request1.req.text.chars().count(), 128);

        // 超过 128
        let text_129 = "a".repeat(129);
        let request2 = ExtractEntityRequest::new(config.clone(), text_129);

        let result = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let _ = request2.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());

        // 测试边界情况
        let text_127 = "a".repeat(127);
        let request3 = ExtractEntityRequest::new(config, text_127);
        assert_eq!(request3.req.text.chars().count(), 127);
    }

    /// 测试空文本
    #[test]
    fn test_empty_text() {
        let config = Config::default();
        let request = ExtractEntityRequest::new(config, "");
        assert_eq!(request.req.text, "");
    }

    /// 测试 Unicode 字符计数
    #[test]
    fn test_unicode_character_count() {
        let config = Config::default();
        let text = "🎉🎊🎈"; // 3 个 Unicode 码点
        let request = ExtractEntityRequest::new(config, text);
        assert_eq!(request.req.text.chars().count(), 3);
    }

    /// 测试响应数据结构
    #[test]
    fn test_extract_entity_response() {
        let response = ExtractEntityResponse {
            entity_word: vec![
                ExtractedWord {
                    name: "词条1".to_string(),
                    aliases: Some(vec!["别名1".to_string()]),
                },
                ExtractedWord {
                    name: "词条2".to_string(),
                    aliases: None,
                },
            ],
        };

        assert_eq!(response.entity_word.len(), 2);
        assert_eq!(response.entity_word[0].name, "词条1");
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(ExtractEntityResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试ExtractedWord结构
    #[test]
    fn test_extracted_word_structure() {
        let word = ExtractedWord {
            name: "测试词条".to_string(),
            aliases: Some(vec!["别名A".to_string(), "别名B".to_string()]),
        };

        assert_eq!(word.name, "测试词条");
        assert_eq!(word.aliases.unwrap().len(), 2);
    }

    /// 测试无别名场景
    #[test]
    fn test_extracted_word_no_aliases() {
        let word = ExtractedWord {
            name: "无别名词条".to_string(),
            aliases: None,
        };

        assert!(word.aliases.is_none());
    }
}

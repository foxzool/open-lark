//! 精准搜索词条
//!
//! 
//!
//! [官方文档](https://open.feishu.cn/document/server-docs/baike-v1/entity/match)

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestBuilder, Send},
    config::Config,
    error::Error,
    response::Response,
};
use serde::{Deserialize, Serialize};

/// 精准搜索词条
#[derive(Debug)]
pub struct Match {
    config: Config,
    req: MatchReq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchReq {
    /// 搜索关键词，将与词条名、别名进行精准匹配
    pub word: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchResp {
    /// 匹配结果
    pub results: Vec<MatchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchResult {
    /// 词条 ID
    pub entity_id: String,
    /// 词条名
    pub main_keys: Vec<TermKey>,
    /// 别名
    pub aliases: Vec<TermKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TermKey {
    /// 名称的值
    pub key: String,
    /// 名称的展示状态
    pub display_status: DisplayStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayStatus {
    /// 对应名称是否在词条卡片允许展示
    pub allow: bool,
    /// 对应名称是否在词条卡片不允许展示
    pub deny: bool,
}

impl Match {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            req: MatchReq {
                word: String::new(),
            },
        }
    }

    /// 搜索关键词
    pub fn word(mut self, word: impl Into<String>) -> Self {
        self.req.word = word.into();
        self
    }

    pub async fn send(self) -> Result<Response<MatchResp>, Error> {
        let url = format!(
            "{}/open-apis/baike/v1/entities/match",
            self.config.base_url
        );
        let request = ApiRequest::post(&url).body(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}

impl ApiResponseTrait for MatchResp {}

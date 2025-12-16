/// 获取词典分类
///
/// API文档: https://open.feishu.cn/document/lingo-v1/classification/list
///
/// 获取飞书词典当前分类。飞书词典目前为二级分类体系，每个词条可添加多个二级分类，
/// 但选择的二级分类必须从属于不同的一级分类。

use openlark_core::{
    error::SDKResult,
    config::Config,
    request_builder::UnifiedRequestBuilder,
    constants::AccessTokenType,
    api::{ApiRequest, Response},
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use crate::baike::models::*;

/// 获取词典分类响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListClassificationResponse {
    /// 分类列表
    pub classifications: Vec<Classification>,
    /// 词典ID
    pub repo_id: String,
}

/// 获取词典分类构建器
pub struct ListClassificationBuilder<'a> {
    config: &'a Config,
    /// 词典ID
    repo_id: Option<String>,
}

impl<'a> ListClassificationBuilder<'a> {
    /// 创建新的获取词典分类构建器
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            repo_id: None,
        }
    }

    /// 设置词典ID
    pub fn repo_id(mut self, repo_id: String) -> Self {
        self.repo_id = Some(repo_id);
        self
    }

    /// 执行获取词典分类操作
    pub async fn execute(self) -> SDKResult<ListClassificationResponse> {
        let mut api_request: ApiRequest<ListClassificationResponse> = ApiRequest::get("/open-apis/lingo/v1/classifications");

        if let Some(repo_id) = &self.repo_id {
            api_request = api_request.query("repo_id", repo_id);
        }

        let http_request = UnifiedRequestBuilder::build(
            &mut api_request,
            AccessTokenType::App,
            self.config,
            &RequestOption::default(),
        ).await?;

        let response = http_request.send().await?;
        let resp: Response<_> = response.json().await?;
        resp.into_result()
    }
}
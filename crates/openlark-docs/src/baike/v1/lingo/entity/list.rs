/// 获取词条列表
///
/// API文档: https://open.feishu.cn/document/lingo-v1/entity/list
///
/// 分页拉取词条列表数据，支持拉取租户内的全部词条。

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

/// 获取词条列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEntityResponse {
    /// 词条列表
    pub items: Vec<Entity>,
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有下一页
    pub has_more: bool,
}

/// 获取词条列表构建器
pub struct ListEntityBuilder<'a> {
    config: &'a Config,
    /// 分页token
    page_token: Option<String>,
    /// 每页大小
    page_size: Option<i32>,
    /// 分类ID过滤
    classification_id: Option<String>,
    /// 词典ID过滤
    repo_id: Option<String>,
    /// 词条状态过滤
    status: Option<String>,
}

impl<'a> ListEntityBuilder<'a> {
    /// 创建新的获取词条列表构建器
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            page_token: None,
            page_size: Some(20),
            classification_id: None,
            repo_id: None,
            status: None,
        }
    }

    /// 设置分页token
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置每页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分类ID过滤
    pub fn classification_id(mut self, classification_id: String) -> Self {
        self.classification_id = Some(classification_id);
        self
    }

    /// 设置词典ID过滤
    pub fn repo_id(mut self, repo_id: String) -> Self {
        self.repo_id = Some(repo_id);
        self
    }

    /// 设置词条状态过滤
    pub fn status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    /// 执行获取词条列表操作
    pub async fn execute(self) -> SDKResult<ListEntityResponse> {
        let mut api_request = ApiRequest::get("/open-apis/lingo/v1/entities");

        // 添加查询参数
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(classification_id) = &self.classification_id {
            api_request = api_request.query("classification_id", classification_id);
        }
        if let Some(repo_id) = &self.repo_id {
            api_request = api_request.query("repo_id", repo_id);
        }
        if let Some(status) = &self.status {
            api_request = api_request.query("status", status);
        }

        let http_request = UnifiedRequestBuilder::build(
            &mut api_request,
            AccessTokenType::App,
            self.config,
            &RequestOption::default(),
        ).await?;

        let response = self.config.http_client().execute(http_request).await?;
        let raw_response = Response::from_reqwest_response(response).await?;

        raw_response.into_result()
    }
}
/// 获取词库列表
///
/// API文档: https://open.feishu.cn/document/lingo-v1/repo/list

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

/// 获取词库列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRepoResponse {
    /// 词典列表
    pub repositories: Vec<Repository>,
}

/// 获取词库列表构建器
pub struct ListRepoBuilder<'a> {
    config: &'a Config,
}

impl<'a> ListRepoBuilder<'a> {
    /// 创建新的获取词库列表构建器
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// 执行获取词库列表操作
    pub async fn execute(self) -> SDKResult<ListRepoResponse> {
        let mut api_request: ApiRequest<ListRepoResponse> = ApiRequest::get("/open-apis/lingo/v1/repos");

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
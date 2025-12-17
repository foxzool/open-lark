/// 获取词库列表
///
/// 获取当前可用的词库列表。
/// docPath: https://open.feishu.cn/document/lingo-v1/repo/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct ListRepoResponse {
    pub data: Option<RepoData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RepoData {
    pub repos: Vec<RepoItem>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RepoItem {
    pub repo_id: String,
    pub repo_name: String,
    pub description: Option<String>,
    pub repo_type: String,
    pub is_default: bool,
    pub entity_count: i32,
    pub create_time: String,
    pub update_time: String,
}

impl ApiResponseTrait for ListRepoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词库列表
///
/// 获取当前可用的词库列表。
/// docPath: https://open.feishu.cn/document/lingo-v1/repo/list
pub async fn list_repo(config: &Config) -> SDKResult<Vec<RepoItem>> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::RepoList;

    // 创建API请求
    let api_request: ApiRequest<ListRepoResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: ListRepoResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data
        .map(|data| data.repos)
        .ok_or_else(|| openlark_core::error::validation_error("repo_data", "Repo data is missing"))
}

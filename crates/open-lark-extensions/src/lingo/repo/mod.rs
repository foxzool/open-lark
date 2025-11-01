use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::LINGO_REPO_LIST,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::lingo::models::{PageResponse, Repo},
};

/// 词库管理服务
pub struct RepoService {
    pub config: Config,
}

impl RepoService {
    /// 创建词库管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取词库列表
    ///
    /// 获取当前用户可访问的词库列表，支持分页查询。
    ///
    /// # Arguments
    ///
    /// * `request` - 词库列表查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回词库列表
    pub async fn list_repos(
        &self,
        request: RepoListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RepoListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: LINGO_REPO_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }
}

/// 词库列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RepoListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 词库列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoListResponse {
    /// 词库列表
    #[serde(flatten)]
    pub repos: PageResponse<Repo>,
}

impl ApiResponseTrait for RepoListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

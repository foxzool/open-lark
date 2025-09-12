use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::lingo::models::{Classification, PageResponse},
};

/// 分类管理服务
pub struct ClassificationService {
    pub config: Config,
}

impl ClassificationService {
    /// 创建分类管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取词典分类
    ///
    /// 获取词典中的分类列表，支持分页查询。
    ///
    /// # Arguments
    ///
    /// * `request` - 分类列表查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回分类列表
    pub async fn list_classifications(
        &self,
        request: ClassificationListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ClassificationListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/lingo/v1/classifications".to_string(),
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

        if let Some(repo_id) = request.repo_id {
            api_req.query_params.insert("repo_id", repo_id);
        }

        Transport::request(api_req, &self.config, option).await
    }
}

/// 分类列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClassificationListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 词库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
}

/// 分类列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassificationListResponse {
    /// 分类列表
    #[serde(flatten)]
    pub classifications: PageResponse<Classification>,
}

impl ApiResponseTrait for ClassificationListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

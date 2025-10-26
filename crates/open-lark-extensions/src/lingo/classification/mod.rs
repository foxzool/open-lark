use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::LINGO_CLASSIFICATION_LIST,
        http::Transport,
        req_option::RequestOption,
        trait_system::AsyncServiceOperation,
        SDKResult,
    },
    impl_basic_service, impl_service_constructor,
    service::lingo::models::{Classification, PageResponse},
};

/// 分类管理服务
#[derive(Debug, Clone)]
pub struct ClassificationService {
    pub config: Config,
}

impl ClassificationService {
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
        <Self as AsyncServiceOperation<
            ClassificationListRequest,
            BaseResponse<ClassificationListResponse>,
        >>::execute_with_observability(self, "list_classifications", || async {
            let mut api_req = ApiRequest {
                http_method: Method::GET,
                api_path: LINGO_CLASSIFICATION_LIST.to_string(),
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
        })
        .await
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

// 使用宏实现基础 Service traits
impl_basic_service!(ClassificationService, "lingo.classification", "v1");
impl_service_constructor!(ClassificationService);

// 实现异步操作支持
impl AsyncServiceOperation<ClassificationListRequest, BaseResponse<ClassificationListResponse>>
    for ClassificationService
{
}

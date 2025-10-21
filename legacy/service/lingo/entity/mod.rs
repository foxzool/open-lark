use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{
            EndpointBuilder, LINGO_ENTITY_CREATE, LINGO_ENTITY_GET, LINGO_ENTITY_HIGHLIGHT,
            LINGO_ENTITY_MATCH, LINGO_ENTITY_SEARCH, LINGO_ENTITY_UPDATE,
        },
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::lingo::models::{
        Entity, EntityMatchResult, EntitySearchResult, HighlightResult, OuterInfo, PageResponse,
        RelatedMeta,
    },
};

/// 词条管理服务
pub struct EntityService {
    pub config: Config,
}

impl EntityService {
    /// 创建词条管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建免审词条
    ///
    /// 通过此接口创建的词条，无需经过词典管理员审核，直接写入词库。
    ///
    /// # Arguments
    ///
    /// * `request` - 词条创建请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回创建的词条信息
    pub async fn create_entity(
        &self,
        request: EntityCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityCreateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: LINGO_ENTITY_CREATE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新免审词条
    ///
    /// 通过此接口更新已有词条，无需经过词典管理员审核，直接写入词库。
    ///
    /// # Arguments
    ///
    /// * `entity_id` - 词条ID
    /// * `request` - 词条更新请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回更新后的词条信息
    pub async fn update_entity(
        &self,
        entity_id: &str,
        request: EntityUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityUpdateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(LINGO_ENTITY_UPDATE, "{entity_id}", entity_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除免审词条
    ///
    /// 通过词条ID删除已有词条。
    ///
    /// # Arguments
    ///
    /// * `entity_id` - 词条ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回删除结果
    pub async fn delete_entity(
        &self,
        entity_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityDeleteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(LINGO_ENTITY_UPDATE, "{entity_id}", entity_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取词条详情
    ///
    /// 通过词条ID获取词条的详细信息。
    ///
    /// # Arguments
    ///
    /// * `entity_id` - 词条ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回词条详细信息
    pub async fn get_entity(
        &self,
        entity_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(LINGO_ENTITY_GET, "{entity_id}", entity_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取词条列表
    ///
    /// 分页获取词条列表，支持多种筛选条件。
    ///
    /// # Arguments
    ///
    /// * `request` - 词条列表查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回词条列表
    pub async fn list_entities(
        &self,
        request: EntityListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: LINGO_ENTITY_CREATE.to_string(),
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

        if let Some(classification_id) = request.classification_id {
            api_req
                .query_params
                .insert("classification_id", classification_id);
        }

        if let Some(creator) = request.creator {
            api_req.query_params.insert("creator", creator);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 精准搜索词条
    ///
    /// 传入关键词，与词条的名称和别名精准匹配。
    ///
    /// # Arguments
    ///
    /// * `request` - 词条精准搜索请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回匹配的词条列表
    pub async fn match_entities(
        &self,
        request: EntityMatchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityMatchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: LINGO_ENTITY_MATCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 模糊搜索词条
    ///
    /// 传入关键词，对词条的名称、别名和释义等信息进行模糊匹配。
    ///
    /// # Arguments
    ///
    /// * `request` - 词条模糊搜索请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回搜索结果列表
    pub async fn search_entities(
        &self,
        request: EntitySearchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntitySearchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: LINGO_ENTITY_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 词条高亮
    ///
    /// 传入一段文本，返回文本中匹配到的词条位置信息，可用于高亮显示。
    ///
    /// # Arguments
    ///
    /// * `request` - 词条高亮请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回高亮信息
    pub async fn highlight_entities(
        &self,
        request: EntityHighlightRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EntityHighlightResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: LINGO_ENTITY_HIGHLIGHT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 词条创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityCreateRequest {
    /// 主名称
    pub main_keys: Vec<String>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// 词条释义富文本
    pub description: String,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id: Option<String>,
    /// 外链（用于跳转到释义页面）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 相关词条ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
    /// 词库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
}

/// 词条创建响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityCreateResponse {
    /// 创建的词条信息
    pub entity: Entity,
}

impl ApiResponseTrait for EntityCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityUpdateRequest {
    /// 主名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_keys: Option<Vec<String>>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// 词条释义富文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id: Option<String>,
    /// 外链（用于跳转到释义页面）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_info: Option<OuterInfo>,
    /// 相关词条ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_meta: Option<RelatedMeta>,
}

/// 词条更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityUpdateResponse {
    /// 更新后的词条信息
    pub entity: Entity,
}

impl ApiResponseTrait for EntityUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityDeleteResponse {
    /// 删除成功标识
    pub success: bool,
}

impl ApiResponseTrait for EntityDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条详情查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityGetResponse {
    /// 词条详细信息
    pub entity: Entity,
}

impl ApiResponseTrait for EntityGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 词库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id: Option<String>,
    /// 创建者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
}

/// 词条列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityListResponse {
    /// 词条列表
    #[serde(flatten)]
    pub entities: PageResponse<Entity>,
}

impl ApiResponseTrait for EntityListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条精准搜索请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMatchRequest {
    /// 搜索词
    pub word: String,
    /// 词库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
}

/// 词条精准搜索响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityMatchResponse {
    /// 匹配结果列表
    pub results: Vec<EntityMatchResult>,
}

impl ApiResponseTrait for EntityMatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条模糊搜索请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntitySearchRequest {
    /// 搜索关键词
    pub query: String,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_id: Option<String>,
    /// 词库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 搜索来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 词条模糊搜索响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EntitySearchResponse {
    /// 搜索结果列表
    #[serde(flatten)]
    pub results: PageResponse<EntitySearchResult>,
}

impl ApiResponseTrait for EntitySearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 词条高亮请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityHighlightRequest {
    /// 要高亮的文本
    pub text: String,
    /// 词库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
}

/// 词条高亮响应
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityHighlightResponse {
    /// 高亮结果
    pub result: HighlightResult,
}

impl ApiResponseTrait for EntityHighlightResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

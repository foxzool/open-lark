use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::{
        api::ApiRequest,
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::hire::*,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    };
use crate::hire::models::{CommonResponse, I18nText, PageResponse, Talent, UserId};

/// 人才库服务
pub struct TalentPoolService {
    pub config: Config,
}

/// 人才库信息
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentPool {
    /// 人才库ID
    pub id: String,
    /// 人才库名称
    pub name: I18nText,
    /// 人才库描述
    pub description: Option<I18nText>,
    /// 人才库类型
    pub pool_type: String,
    /// 是否公开
    pub is_public: bool,
    /// 人才库状态
    pub status: String,
    /// 人才库拥有者
    pub owner: Option<UserId>,
    /// 人才库管理员
    pub managers: Vec<UserId>,
    /// 人才数量
    pub talent_count: u32,
    /// 人才库设置
    pub settings: Option<TalentPoolSettings>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 人才库设置
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentPoolSettings {
    /// 自动匹配设置
    pub auto_match_enabled: Option<bool>,
    /// 人才可见性设置
    pub talent_visibility: Option<String>,
    /// 标签配置
    pub tag_config: Option<serde_json::Value>,
    /// 自定义字段配置
    pub custom_field_config: Option<serde_json::Value>,
}

/// 人才库创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TalentPoolCreateRequest {
    /// 人才库名称
    pub name: I18nText,
    /// 人才库描述
    pub description: Option<I18nText>,
    /// 人才库类型
    pub pool_type: String,
    /// 是否公开
    pub is_public: Option<bool>,
    /// 人才库拥有者ID
    pub owner_id: Option<String>,
    /// 人才库管理员ID列表
    pub manager_ids: Vec<String>,
    /// 人才库设置
    pub settings: Option<TalentPoolSettings>,
}

/// 人才库列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TalentPoolListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 人才库类型
    pub pool_type: Option<String>,
    /// 人才库状态
    pub status: Option<String>,
    /// 拥有者ID
    pub owner_id: Option<String>,
}

/// 人才库中人才列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TalentPoolTalentListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 人才标签
    pub tags: Vec<String>,
    /// 工作年限筛选
    pub work_experience: Option<u32>,
    /// 学历筛选
    pub education: Option<String>,
}

/// 人才库列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentPoolListResponse {
    /// 人才库列表
    #[serde(flatten)]
    pub pools: PageResponse<TalentPool>,
}

impl ApiResponseTrait for TalentPoolListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人才库详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentPoolDetailResponse {
    /// 人才库信息
    pub pool: TalentPool,
}

impl ApiResponseTrait for TalentPoolDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人才库操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentPoolOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 人才库ID
    pub pool_id: Option<String>,
}

impl ApiResponseTrait for TalentPoolOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人才库中人才列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct TalentPoolTalentListResponse {
    /// 人才列表
    #[serde(flatten)]
    pub talents: PageResponse<Talent>,
}

impl ApiResponseTrait for TalentPoolTalentListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TalentPoolService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建人才库
    ///
    /// 该接口用于创建新的人才库，设置人才库的基本信息、
    /// 访问权限、管理员等配置。创建的人才库可用于
    /// 组织和管理相关的人才信息。
    ///
    /// # 参数
    ///
    /// - `request`: 人才库创建请求参数，包括：
    ///   - `name`: 人才库名称（必填）
    ///   - `pool_type`: 人才库类型（必填）
    ///   - `description`: 人才库描述
    ///   - `is_public`: 是否公开
    ///   - `owner_id`: 拥有者ID
    ///   - `manager_ids`: 管理员ID列表
    ///   - `settings`: 人才库设置
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回人才库创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `pool_id`: 创建的人才库ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::talent_pool::TalentPoolCreateRequest;
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let request = TalentPoolCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("技术人才库".to_string()),
    ///         en_us: Some("Technical Talent Pool".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     pool_type: "technical".to_string(),
    ///     description: Some(I18nText {
    ///         zh_cn: Some("存储技术类候选人信息".to_string()),
    ///         en_us: Some("Storage for technical candidates".to_string()),
    ///         ja_jp: None,
    ///     }),
    ///     is_public: Some(false),
    ///     owner_id: Some("user_123456".to_string()),
    ///     manager_ids: vec!["user_789".to_string()],
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.candidate_management.talent_pool.create_pool(request, None).await?;
    /// ```
    pub async fn create_pool(
        &self,
        request: TalentPoolCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentPoolOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_TALENT_POOLS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取人才库详情
    ///
    /// 该接口用于获取指定人才库的详细信息，包括人才库
    /// 基本信息、权限设置、人才统计等完整数据。
    ///
    /// # 参数
    ///
    /// - `pool_id`: 人才库ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回人才库详细信息，包括：
    /// - 人才库基本信息（名称、类型、状态等）
    /// - 拥有者和管理员信息
    /// - 人才数量统计
    /// - 人才库设置配置
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let pool_id = "pool_123456";
    /// let response = client.hire.candidate_management.talent_pool.get_pool_detail(pool_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("人才库名称: {:?}", data.pool.name.zh_cn);
    ///     println!("人才库类型: {}", data.pool.pool_type);
    ///     println!("人才数量: {}", data.pool.talent_count);
    /// }
    /// ```
    pub async fn get_pool_detail(
        &self,
        pool_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentPoolDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_TALENT_POOL_GET, "pool_id", pool_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取人才库列表
    ///
    /// 该接口用于获取企业的人才库列表，支持按类型、
    /// 状态、拥有者等条件筛选。返回的列表包含人才库
    /// 基本信息，可用于人才库管理和选择。
    ///
    /// # 参数
    ///
    /// - `request`: 人才库列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `pool_type`: 人才库类型筛选
    ///   - `status`: 人才库状态筛选
    ///   - `owner_id`: 拥有者ID筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的人才库列表，包括：
    /// - 人才库基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::talent_pool::TalentPoolListRequest;
    ///
    /// let request = TalentPoolListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     pool_type: Some("technical".to_string()),
    ///     status: Some("active".to_string()),
    ///     owner_id: Some("user_123456".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.talent_pool.list_pools(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("人才库总数: {}", data.pools.items.len());
    ///     for pool in &data.pools.items {
    ///         println!("人才库: {:?} ({}人)", pool.name.zh_cn, pool.talent_count);
    ///     }
    /// }
    /// ```
    pub async fn list_pools(
        &self,
        request: TalentPoolListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentPoolListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_TALENT_POOLS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(pool_type) = request.pool_type {
            api_req.query_params.insert("pool_type", pool_type);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(owner_id) = request.owner_id {
            api_req.query_params.insert("owner_id", owner_id);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取人才库中的人才列表
    ///
    /// 该接口用于获取指定人才库中的人才列表，支持按
    /// 标签、工作年限、学历等条件筛选。返回的列表
    /// 包含人才基本信息，可用于人才筛选和匹配。
    ///
    /// # 参数
    ///
    /// - `pool_id`: 人才库ID
    /// - `request`: 人才列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `tags`: 人才标签筛选
    ///   - `work_experience`: 工作年限筛选
    ///   - `education`: 学历筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的人才列表，包括：
    /// - 人才基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::talent_pool::TalentPoolTalentListRequest;
    ///
    /// let pool_id = "pool_123456";
    /// let request = TalentPoolTalentListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     tags: vec!["Java".to_string(), "微服务".to_string()],
    ///     work_experience: Some(3),
    ///     education: Some("本科".to_string()),
    /// };
    ///
    /// let response = client.hire.candidate_management.talent_pool.list_pool_talents(pool_id, request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("符合条件的人才数: {}", data.talents.items.len());
    ///     for talent in &data.talents.items {
    ///         println!("人才: {} ({}年经验)", talent.name, talent.work_experience.unwrap_or(0));
    ///     }
    /// }
    /// ```
    pub async fn list_pool_talents(
        &self,
        pool_id: &str,
        request: TalentPoolTalentListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentPoolTalentListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_TALENT_POOL_TALENTS,
            "pool_id",
            pool_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if !request.tags.is_empty() {
            api_req.query_params.insert("tags", request.tags.join(","));
        }

        if let Some(work_experience) = request.work_experience {
            api_req
                .query_params
                .insert("work_experience", work_experience.to_string());
        }

        if let Some(education) = request.education {
            api_req.query_params.insert("education", education);
        }

        Transport::request(api_req, &self.config, option).await
    }
    /// 向人才库添加人才
    ///
    /// 该接口用于将指定的人才添加到人才库中，
    /// 建立人才与人才库的关联关系。
    ///
    /// # 参数
    ///
    /// - `pool_id`: 人才库ID
    /// - `talent_id`: 人才ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let pool_id = "pool_123456";
    /// let talent_id = "talent_789";
    /// let response = client.hire.candidate_management.talent_pool.add_talent_to_pool(pool_id, talent_id, None).await?;
    /// ```
    pub async fn add_talent_to_pool(
        &self,
        pool_id: &str,
        talent_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentPoolOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            HIRE_V1_TALENT_POOL_ADD_TALENT,
            "talent_pool_id",
            pool_id
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 从人才库移除人才
    ///
    /// 该接口用于将指定的人才从人才库中移除，
    /// 解除人才与人才库的关联关系。
    ///
    /// # 参数
    ///
    /// - `pool_id`: 人才库ID
    /// - `talent_id`: 人才ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let pool_id = "pool_123456";
    /// let talent_id = "talent_789";
    /// let response = client.hire.candidate_management.talent_pool.remove_talent_from_pool(pool_id, talent_id, None).await?;
    /// ```
    pub async fn remove_talent_from_pool(
        &self,
        pool_id: &str,
        talent_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentPoolOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_params(
            HIRE_V1_TALENT_POOL_REMOVE_TALENT,
            &std::collections::HashMap::from([
                ("talent_pool_id", pool_id.to_string()),
                ("talent_id", talent_id.to_string()),
            ])
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
    /// 更新人才库
    ///
    /// 该接口用于更新现有人才库的信息，支持修改人才库
    /// 名称、描述、管理员、设置等配置。
    ///
    /// # 参数
    ///
    /// - `pool_id`: 人才库ID
    /// - `request`: 人才库更新请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::candidate_management::talent_pool::TalentPoolCreateRequest;
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let pool_id = "pool_123456";
    /// let request = TalentPoolCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("高级技术人才库".to_string()),
    ///         en_us: Some("Senior Technical Talent Pool".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     pool_type: "senior_technical".to_string(),
    ///     manager_ids: vec!["user_789".to_string(), "user_456".to_string()],
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.candidate_management.talent_pool.update_pool(pool_id, request, None).await?;
    /// ```
    pub async fn update_pool(
        &self,
        pool_id: &str,
        request: TalentPoolCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentPoolOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_TALENT_POOL_GET, "pool_id", pool_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 删除人才库
    ///
    /// 该接口用于删除指定的人才库。删除后的人才库
    /// 将不再可用，但其中的人才信息不会被删除。
    ///
    /// # 参数
    ///
    /// - `pool_id`: 人才库ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let pool_id = "pool_123456";
    /// let response = client.hire.candidate_management.talent_pool.delete_pool(pool_id, None).await?;
    /// ```
    pub async fn delete_pool(
        &self,
        pool_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<TalentPoolOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_TALENT_POOL_GET, "pool_id", pool_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
}

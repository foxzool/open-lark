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

use crate::hire::models::{CommonResponse, I18nText, PageResponse, UserId};

/// 项目服务
pub struct SubjectService {
    pub config: Config,
}

/// 项目信息
#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    /// 项目ID
    pub id: String,
    /// 项目名称
    pub name: I18nText,
    /// 项目描述
    pub description: Option<I18nText>,
    /// 项目类型
    pub subject_type: String,
    /// 项目状态
    pub status: String,
    /// 项目负责人
    pub owner: Option<UserId>,
    /// 项目成员
    pub members: Vec<UserId>,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 项目设置
    pub settings: Option<SubjectSettings>,
    /// 创建时间
    pub created_time: Option<String>,
    /// 更新时间
    pub updated_time: Option<String>,
}

/// 项目设置
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectSettings {
    /// 是否启用候选人可见性
    pub candidate_visibility: Option<bool>,
    /// 数据权限设置
    pub data_permissions: Option<Vec<String>>,
    /// 自定义字段配置
    pub custom_field_config: Option<serde_json::Value>,
    /// 通知设置
    pub notification_settings: Option<serde_json::Value>,
}

/// 项目创建请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubjectCreateRequest {
    /// 项目名称
    pub name: I18nText,
    /// 项目描述
    pub description: Option<I18nText>,
    /// 项目类型
    pub subject_type: String,
    /// 项目负责人ID
    pub owner_id: Option<String>,
    /// 项目成员ID列表
    pub member_ids: Vec<String>,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 项目设置
    pub settings: Option<SubjectSettings>,
}

/// 项目列表请求
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubjectListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 项目类型
    pub subject_type: Option<String>,
    /// 项目状态
    pub status: Option<String>,
    /// 负责人ID
    pub owner_id: Option<String>,
}

/// 项目列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectListResponse {
    /// 项目列表
    #[serde(flatten)]
    pub subjects: PageResponse<Subject>,
}

impl ApiResponseTrait for SubjectListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 项目详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectDetailResponse {
    /// 项目信息
    pub subject: Subject,
}

impl ApiResponseTrait for SubjectDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 项目操作响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectOperationResponse {
    /// 操作结果
    #[serde(flatten)]
    pub result: CommonResponse,
    /// 项目ID
    pub subject_id: Option<String>,
}

impl ApiResponseTrait for SubjectOperationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SubjectService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建项目
    ///
    /// 该接口用于创建新的招聘项目，定义项目范围、
    /// 负责人、成员和相关设置。项目可用于组织和
    /// 管理相关的招聘活动。
    ///
    /// # 参数
    ///
    /// - `request`: 项目创建请求参数，包括：
    ///   - `name`: 项目名称（必填）
    ///   - `subject_type`: 项目类型（必填）
    ///   - `description`: 项目描述
    ///   - `owner_id`: 项目负责人ID
    ///   - `member_ids`: 项目成员ID列表
    ///   - `start_time`: 开始时间
    ///   - `end_time`: 结束时间
    ///   - `settings`: 项目设置
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回项目创建操作结果，包括：
    /// - `success`: 创建是否成功
    /// - `subject_id`: 创建的项目ID
    /// - `message`: 操作结果消息
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::subject::SubjectCreateRequest;
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let request = SubjectCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("2024年春季校园招聘".to_string()),
    ///         en_us: Some("2024 Spring Campus Recruitment".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     subject_type: "campus_recruitment".to_string(),
    ///     description: Some(I18nText {
    ///         zh_cn: Some("面向应届毕业生的春季招聘项目".to_string()),
    ///         en_us: Some("Spring recruitment project for fresh graduates".to_string()),
    ///         ja_jp: None,
    ///     }),
    ///     owner_id: Some("user_123456".to_string()),
    ///     member_ids: vec!["user_789".to_string(), "user_456".to_string()],
    ///     start_time: Some("2024-02-01T00:00:00Z".to_string()),
    ///     end_time: Some("2024-05-31T23:59:59Z".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.subject.create_subject(request, None).await?;
    /// ```
    pub async fn create_subject(
        &self,
        request: SubjectCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<SubjectOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_SUBJECTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取项目详情
    ///
    /// 该接口用于获取指定项目的详细信息，包括项目
    /// 基本信息、负责人、成员、设置等完整数据。
    ///
    /// # 参数
    ///
    /// - `subject_id`: 项目ID
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回项目详细信息，包括：
    /// - 项目基本信息（名称、类型、状态等）
    /// - 负责人和成员信息
    /// - 项目时间范围
    /// - 项目设置配置
    /// - 创建和更新时间
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let subject_id = "subject_123456";
    /// let response = client.hire.recruitment_config.subject.get_subject_detail(subject_id, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("项目名称: {:?}", data.subject.name.zh_cn);
    ///     println!("项目类型: {}", data.subject.subject_type);
    ///     println!("项目状态: {}", data.subject.status);
    /// }
    /// ```
    pub async fn get_subject_detail(
        &self,
        subject_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<SubjectDetailResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_SUBJECT_GET, "subject_id", subject_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 获取项目列表
    ///
    /// 该接口用于获取企业的项目列表，支持按类型、
    /// 状态、负责人等条件筛选。返回的列表包含项目
    /// 基本信息，可用于项目管理和选择。
    ///
    /// # 参数
    ///
    /// - `request`: 项目列表查询请求参数，包括：
    ///   - `page_size`: 分页大小，最大值100
    ///   - `page_token`: 分页标记
    ///   - `subject_type`: 项目类型筛选
    ///   - `status`: 项目状态筛选
    ///   - `owner_id`: 负责人ID筛选
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回分页的项目列表，包括：
    /// - 项目基本信息列表
    /// - 分页信息（是否有更多数据、下一页标记）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::subject::SubjectListRequest;
    ///
    /// let request = SubjectListRequest {
    ///     page_size: Some(50),
    ///     page_token: None,
    ///     subject_type: Some("campus_recruitment".to_string()),
    ///     status: Some("active".to_string()),
    ///     owner_id: Some("user_123456".to_string()),
    /// };
    ///
    /// let response = client.hire.recruitment_config.subject.list_subjects(request, None).await?;
    ///
    /// if let Some(data) = &response.data {
    ///     println!("项目总数: {}", data.subjects.items.len());
    ///     for subject in &data.subjects.items {
    ///         println!("项目: {:?} (类型: {})", subject.name.zh_cn, subject.subject_type);
    ///     }
    /// }
    /// ```
    pub async fn list_subjects(
        &self,
        request: SubjectListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<SubjectListResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(HIRE_V1_SUBJECTS.to_string());
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

        if let Some(subject_type) = request.subject_type {
            api_req.query_params.insert("subject_type", subject_type);
        }

        if let Some(status) = request.status {
            api_req.query_params.insert("status", status);
        }

        if let Some(owner_id) = request.owner_id {
            api_req.query_params.insert("owner_id", owner_id);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新项目
    ///
    /// 该接口用于更新现有项目的信息，支持修改项目
    /// 名称、描述、成员、设置等配置。
    ///
    /// # 参数
    ///
    /// - `subject_id`: 项目ID
    /// - `request`: 项目更新请求参数
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::hire::recruitment_config::subject::SubjectCreateRequest;
    /// use open_lark::crate::hire::models::I18nText;
    ///
    /// let subject_id = "subject_123456";
    /// let request = SubjectCreateRequest {
    ///     name: I18nText {
    ///         zh_cn: Some("2024年春季校园招聘(更新)".to_string()),
    ///         en_us: Some("2024 Spring Campus Recruitment (Updated)".to_string()),
    ///         ja_jp: None,
    ///     },
    ///     subject_type: "campus_recruitment".to_string(),
    ///     member_ids: vec!["user_789".to_string(), "user_456".to_string(), "user_321".to_string()],
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.hire.recruitment_config.subject.update_subject(subject_id, request, None).await?;
    /// ```
    pub async fn update_subject(
        &self,
        subject_id: &str,
        request: SubjectCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<SubjectOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_SUBJECT_GET, "subject_id", subject_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(&request).unwrap_or_default();
        Transport::request(api_req, &self.config, option).await
    }
    /// 删除项目
    ///
    /// 该接口用于删除指定的项目。删除后的项目
    /// 将不再可用于组织招聘活动。
    ///
    /// # 参数
    ///
    /// - `subject_id`: 项目ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let subject_id = "subject_123456";
    /// let response = client.hire.recruitment_config.subject.delete_subject(subject_id, None).await?;
    /// ```
    pub async fn delete_subject(
        &self,
        subject_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<SubjectOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
        api_req.set_api_path(EndpointBuilder::replace_param(HIRE_V1_SUBJECT_GET, "subject_id", subject_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 启用项目
    ///
    /// 该接口用于启用指定的项目，使其可用于
    /// 组织招聘活动。
    ///
    /// # 参数
    ///
    /// - `subject_id`: 项目ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let subject_id = "subject_123456";
    /// let response = client.hire.recruitment_config.subject.enable_subject(subject_id, None).await?;
    /// ```
    pub async fn enable_subject(
        &self,
        subject_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<SubjectOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_SUBJECT_ENABLE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }

    /// 禁用项目
    ///
    /// 该接口用于禁用指定的项目，暂停其相关的
    /// 招聘活动。
    ///
    /// # 参数
    ///
    /// - `subject_id`: 项目ID
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let subject_id = "subject_123456";
    /// let response = client.hire.recruitment_config.subject.disable_subject(subject_id, None).await?;
    /// ```
    pub async fn disable_subject(
        &self,
        subject_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<SubjectOperationResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(HIRE_V1_SUBJECT_DISABLE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request(api_req, &self.config, option).await
    }
}

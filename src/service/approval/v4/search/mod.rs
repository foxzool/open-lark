//! 审批查询服务 V4
//!
//! 提供企业级的审批查询功能，支持：
//! - 实例查询：查询审批实例列表，支持多维度筛选
//! - 任务查询：查询审批任务列表，支持状态和时间筛选
//! - 抄送查询：查询审批抄送列表，支持用户和实例筛选
//! - 审批ID查询：根据审批名称查询审批定义ID，支持模糊搜索
//! - 用户任务查询：查询指定用户的审批任务列表
//! - 完整的Builder模式API设计
//! - 自动分页验证和错误处理
//!
//! # 核心功能
//!
//! ## 实例查询
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::approval::v4::search::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 查询审批实例列表
//!     let response = client.approval.v4.search
//!         .search_instances_builder()
//!         .approval_code("LEAVE_001")
//!         .instance_status(ApprovalStatus::Pending)
//!         .page_size(20)
//!         .execute(&client.approval.v4.search)
//!         .await?;
//!
//!     println!("查询到 {} 个审批实例", response.data?.instances.len());
//!     Ok(())
//! }
//! ```
//!
//! ## 任务查询
//! ```rust,no_run
//! // 查询待处理任务
//! let response = client.approval.v4.search
//!     .search_tasks_builder()
//!     .task_status(TaskStatus::Pending)
//!     .user_id("user_123")
//!     .user_id_type(UserIdType::OpenId)
//!     .page_size(50)
//!     .execute(&client.approval.v4.search)
//!     .await?;
//!
//! println!("待处理任务数量: {}", response.data?.tasks.len());
//! ```
//!
//! ## 抄送查询
//! ```rust,no_run
//! // 查询抄送记录
//! let response = client.approval.v4.search
//!     .search_cc_builder()
//!     .user_id("user_123")
//!     .user_id_type(UserIdType::OpenId)
//!     .page_size(20)
//!     .execute(&client.approval.v4.search)
//!     .await?;
//! ```
//!
//! ## 审批ID查询
//! ```rust,no_run
//! // 根据名称查询审批定义ID
//! let response = client.approval.v4.search
//!     .search_approval_id_builder()
//!     .approval_name("请假申请")
//!     .user_id_type(UserIdType::OpenId)
//!     .execute(&client.approval.v4.search)
//!     .await?;
//!
//! if let Some(approval_id) = response.data?.approval_id {
//!     println!("找到审批ID: {}", approval_id);
//! }
//! ```
//!
//! ## 用户任务查询
//! ```rust,no_run
//! // 查询指定用户的任务
//! let response = client.approval.v4.search
//!     .search_user_tasks_builder()
//!     .user_id("user_123")
//!     .user_id_type(UserIdType::OpenId)
//!     .task_status(TaskStatus::Pending)
//!     .page_size(20)
//!     .execute(&client.approval.v4.search)
//!     .await?;
//! ```
//!
//! # API文档
//!
//! 详细API文档请参考：[飞书开放平台 - 审批查询](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/approval-v4/approval/search)

// 导入核心依赖
use open_lark_core::core::api_req::ApiRequest;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 导入SDK核心模块
use crate::core::{error::LarkAPIError, 
    error::LarkAPIError,
use crate::core::{error::LarkAPIError, SDKResult, api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat}},
    error::LarkAPIError,
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

// 导入端点常量
use crate::service::endpoints::{
    APPROVAL_V4_APPROVALS_SEARCH, APPROVAL_V4_INSTANCES_SEARCH,
    APPROVAL_V4_INSTANCES_SEARCH_CC, APPROVAL_V4_TASKS_QUERY, APPROVAL_V4_TASKS_SEARCH,
};

// 导入模型
use crate::service::approval::models::{ApprovalInstance, ApprovalStatus, ApprovalTask, TaskStatus, UserIdType};

/// 审批查询服务 V4
///
/// 提供企业级的审批查询功能，支持多种查询场景和高性能筛选。
/// 专为企业应用设计，具备完善的错误处理和性能优化。
///
/// # 主要特性
///
/// - **多维度查询**：支持按审批编码、状态、时间范围、用户等多维度筛选
/// - **分页处理**：自动处理大数据集的分页逻辑
/// - **Builder模式**：现代化的流式API设计
/// - **类型安全**：完整的Rust类型系统支持
/// - **状态管理**：支持各种审批和任务状态的查询
///
/// # 使用示例
///
/// ```rust,no_run
/// use open_lark::prelude::*;
/// use open_lark::service::approval::v4::search::SearchService;
///
/// let config = Config::new("app_id", "app_secret");
/// let service = SearchService::new(config);
/// ```
#[derive(Debug, Clone)]
pub struct SearchService {
    pub config: Config,
}

impl SearchService {
    /// 创建审批查询服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息，包含应用ID、密钥等
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::approval::v4::search::SearchService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = SearchService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询审批实例列表
    ///
    /// 根据指定条件查询审批实例列表，支持按审批编码、状态、时间范围、用户等条件筛选。
    /// 用于获取特定的审批流程实例，便于后续的统计分析和跟踪管理。
    ///
    /// # API文档
    ///
    /// 查询审批实例接口，支持多维度筛选条件查询。
    ///
    /// # 参数
    ///
    /// * `request` - 查询请求参数，包含筛选条件和分页信息
    /// * `option` - 可选的请求配置，如超时时间、重试次数等
    ///
    /// # 返回值
    ///
    /// 返回查询结果，包含实例列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::approval::v4::search::*;
    ///
    /// let request = SearchInstancesRequest::builder()
    ///     .approval_code("LEAVE_001")
    ///     .instance_status(ApprovalStatus::Pending)
    ///     .page_size(20)
    ///     .build();
    ///
    /// let response = client.approval.v4.search.search_instances(&request, None).await?;
    /// println!("查询到 {} 个实例", response.data?.instances.len());
    /// ```
    pub async fn search_instances(
        &self,
        request: &SearchInstancesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchInstancesResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(APPROVAL_V4_INSTANCES_SEARCH.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params = request.to_query_params();

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询审批任务列表
    ///
    /// 根据指定条件查询审批任务列表，支持按审批编码、实例编码、状态、时间范围、用户等条件筛选。
    /// 用于获取需要处理的审批任务或查看已处理的任务记录，支持分页查询。
    ///
    /// # API文档
    ///
    /// 查询审批任务接口，支持状态和用户筛选。
    ///
    /// # 参数
    ///
    /// * `request` - 查询请求参数，包含筛选条件和分页信息
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回查询结果，包含任务列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::approval::v4::search::*;
    ///
    /// let request = SearchTasksRequest::builder()
    ///     .task_status(TaskStatus::Pending)
    ///     .user_id("user_123")
    ///     .user_id_type(UserIdType::OpenId)
    ///     .page_size(20)
    ///     .build();
    ///
    /// let response = client.approval.v4.search.search_tasks(&request, None).await?;
    /// println!("查询到 {} 个任务", response.data?.tasks.len());
    /// ```
    pub async fn search_tasks(
        &self,
        request: &SearchTasksRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchTasksResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(APPROVAL_V4_TASKS_SEARCH.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params = request.to_query_params();

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询审批抄送列表
    ///
    /// 根据指定条件查询审批抄送列表，支持按审批编码、实例编码、用户等条件筛选。
    /// 用于获取用户被抄送的审批记录，帮助用户了解相关审批的进展情况。
    ///
    /// # API文档
    ///
    /// 查询审批抄送接口，支持用户和实例筛选。
    ///
    /// # 参数
    ///
    /// * `request` - 查询请求参数，包含筛选条件和分页信息
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回查询结果，包含抄送列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::approval::v4::search::*;
    ///
    /// let request = SearchCcRequest::builder()
    ///     .user_id("user_123")
    ///     .user_id_type(UserIdType::OpenId)
    ///     .page_size(20)
    ///     .build();
    ///
    /// let response = client.approval.v4.search.search_cc(&request, None).await?;
    /// println!("查询到 {} 个抄送记录", response.data?.cc_instances.len());
    /// ```
    pub async fn search_cc(
        &self,
        request: &SearchCcRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchCcResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(APPROVAL_V4_INSTANCES_SEARCH_CC.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params = request.to_query_params();

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询审批定义ID
    ///
    /// 根据审批名称查询审批定义ID，支持模糊搜索。用于获取特定审批模板的标识符，
    /// 便于后续发起审批流程或查询相关实例。
    ///
    /// # API文档
    ///
    /// 查询审批定义ID接口，支持名称模糊搜索。
    ///
    /// # 参数
    ///
    /// * `request` - 查询请求参数，包含审批名称和用户ID类型
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回查询结果，包含审批定义ID
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::approval::v4::search::*;
    ///
    /// let request = SearchApprovalIdRequest::builder()
    ///     .approval_name("请假申请")
    ///     .user_id_type(UserIdType::OpenId)
    ///     .build();
    ///
    /// let response = client.approval.v4.search.search_approval_id(&request, None).await?;
    /// if let Some(approval_id) = response.data?.approval_id {
    ///     println!("找到审批ID: {}", approval_id);
    /// }
    /// ```
    pub async fn search_approval_id(
        &self,
        request: &SearchApprovalIdRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchApprovalIdResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(APPROVAL_V4_APPROVALS_SEARCH.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params = request.to_query_params();

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询用户的审批任务列表
    ///
    /// 查询指定用户的审批任务列表，支持按审批编码、状态、时间范围等条件筛选。
    /// 用于获取用户需要处理的待办任务或已处理的历史任务，便于任务管理和统计。
    ///
    /// # API文档
    ///
    /// 查询用户任务接口，专用于特定用户的任务查询。
    ///
    /// # 参数
    ///
    /// * `request` - 查询请求参数，包含用户ID和筛选条件
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回查询结果，包含用户任务列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::approval::v4::search::*;
    ///
    /// let request = SearchUserTasksRequest::builder()
    ///     .user_id("user_123")
    ///     .user_id_type(UserIdType::OpenId)
    ///     .task_status(TaskStatus::Pending)
    ///     .page_size(20)
    ///     .build();
    ///
    /// let response = client.approval.v4.search.search_user_tasks(&request, None).await?;
    /// println!("用户待处理任务: {}", response.data?.tasks.len());
    /// ```
    pub async fn search_user_tasks(
        &self,
        request: &SearchUserTasksRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchTasksResponse>> {
        // 构建API请求
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(APPROVAL_V4_TASKS_QUERY.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params = request.to_query_params();

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    // ==================== Builder模式实现 ====================

    /// 搜索审批实例构建器
    ///
    /// 提供流式API来构建搜索审批实例的请求参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::approval::v4::search::*;
    ///
    /// let builder = client.approval.v4.search
    ///     .search_instances_builder()
    ///     .approval_code("LEAVE_001")
    ///     .instance_status(ApprovalStatus::Pending)
    ///     .page_size(20);
    ///
    /// let response = builder.execute(&client.approval.v4.search).await?;
    /// ```
    pub fn search_instances_builder(&self) -> SearchInstancesRequestBuilder {
        SearchInstancesRequestBuilder::new()
    }

    /// 搜索审批任务构建器
    ///
    /// 提供流式API来构建搜索审批任务的请求参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let builder = client.approval.v4.search
    ///     .search_tasks_builder()
    ///     .task_status(TaskStatus::Pending)
    ///     .user_id("user_123")
    ///     .user_id_type(UserIdType::OpenId)
    ///     .page_size(20);
    ///
    /// let response = builder.execute(&client.approval.v4.search).await?;
    /// ```
    pub fn search_tasks_builder(&self) -> SearchTasksRequestBuilder {
        SearchTasksRequestBuilder::new()
    }

    /// 搜索审批抄送构建器
    ///
    /// 提供流式API来构建搜索审批抄送的请求参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let builder = client.approval.v4.search
    ///     .search_cc_builder()
    ///     .user_id("user_123")
    ///     .user_id_type(UserIdType::OpenId)
    ///     .page_size(20);
    ///
    /// let response = builder.execute(&client.approval.v4.search).await?;
    /// ```
    pub fn search_cc_builder(&self) -> SearchCcRequestBuilder {
        SearchCcRequestBuilder::new()
    }

    /// 搜索审批定义ID构建器
    ///
    /// 提供流式API来构建搜索审批定义ID的请求参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let builder = client.approval.v4.search
    ///     .search_approval_id_builder()
    ///     .approval_name("请假申请")
    ///     .user_id_type(UserIdType::OpenId);
    ///
    /// let response = builder.execute(&client.approval.v4.search).await?;
    /// ```
    pub fn search_approval_id_builder(&self) -> SearchApprovalIdRequestBuilder {
        SearchApprovalIdRequestBuilder::new()
    }

    /// 搜索用户任务构建器
    ///
    /// 提供流式API来构建搜索用户任务的请求参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// let builder = client.approval.v4.search
    ///     .search_user_tasks_builder()
    ///     .user_id("user_123")
    ///     .user_id_type(UserIdType::OpenId)
    ///     .task_status(TaskStatus::Pending)
    ///     .page_size(20);
    ///
    /// let response = builder.execute(&client.approval.v4.search).await?;
    /// ```
    pub fn search_user_tasks_builder(&self) -> SearchUserTasksRequestBuilder {
        SearchUserTasksRequestBuilder::new()
    }
}

// ==================== 请求和响应结构体 ====================

/// 搜索审批实例请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchInstancesRequest {
    /// 审批定义编码，用于筛选特定类型的审批
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    /// 审批实例状态，用于筛选特定状态的实例
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<ApprovalStatus>,
    /// 开始时间，格式为Unix时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间，格式为Unix时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 用户ID，用于筛选特定用户相关的实例
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
    /// 页面大小，范围1-200，默认值为20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 分页令牌，用于获取指定页的数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl SearchInstancesRequest {
    /// 创建搜索审批实例请求构建器
    pub fn builder() -> SearchInstancesRequestBuilder {
        SearchInstancesRequestBuilder::new()
    }

    /// 将请求转换为查询参数
    fn to_query_params(&self) -> HashMap<&'static str, String> {
        let mut params = HashMap::new();

        if let Some(approval_code) = &self.approval_code {
            params.insert("approval_code", approval_code.clone());
        }

        if let Some(instance_status) = &self.instance_status {
            params.insert("instance_status", serde_json::to_string(instance_status).unwrap());
        }

        if let Some(start_time) = self.start_time {
            params.insert("start_time", start_time.to_string());
        }

        if let Some(end_time) = self.end_time {
            params.insert("end_time", end_time.to_string());
        }

        if let Some(user_id) = &self.user_id {
            params.insert("user_id", user_id.clone());
        }

        if let Some(user_id_type) = &self.user_id_type {
            params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        if let Some(page_size) = self.page_size {
            params.insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = &self.page_token {
            params.insert("page_token", page_token.clone());
        }

        params
    }
}

/// 搜索审批实例请求构建器
#[derive(Debug, Clone, Default)]
pub struct SearchInstancesRequestBuilder {
    request: SearchInstancesRequest,
}

impl SearchInstancesRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchInstancesRequest::default(),
        }
    }

    /// 设置审批定义编码
    pub fn approval_code(mut self, approval_code: impl ToString) -> Self {
        self.request.approval_code = Some(approval_code.to_string());
        self
    }

    /// 设置审批实例状态
    pub fn instance_status(mut self, instance_status: ApprovalStatus) -> Self {
        self.request.instance_status = Some(instance_status);
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.request.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.request.end_time = Some(end_time);
        self
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl ToString) -> Self {
        self.request.user_id = Some(user_id.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: u32) -> SDKResult<Self> {
        if page_size == 0 {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(
                "Page size cannot be 0, minimum value is 1".to_string(),
            ));
        }
        if page_size > 200 {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(format!(
                "Page size {} exceeds maximum limit of 200",
                page_size
            )));
        }
        self.request.page_size = Some(page_size);
        Ok(self)
    }

    /// 设置分页令牌
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> SearchInstancesRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchInstancesRequestBuilder,
//    SearchService,
//    SearchInstancesRequest,
//    BaseResponse<SearchInstancesResponse>,
//    search_instances
//);

/// 搜索审批实例响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchInstancesResponse {
    /// 审批实例列表
    pub instances: Vec<ApprovalInstance>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchInstancesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索审批任务请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchTasksRequest {
    /// 审批定义编码，用于筛选特定类型的审批
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    /// 审批实例编码，用于筛选特定实例的任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    /// 任务状态，用于筛选特定状态的任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<TaskStatus>,
    /// 开始时间，格式为Unix时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间，格式为Unix时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 用户ID，用于筛选特定用户的任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
    /// 页面大小，范围1-200，默认值为20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 分页令牌，用于获取指定页的数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl SearchTasksRequest {
    /// 创建搜索审批任务请求构建器
    pub fn builder() -> SearchTasksRequestBuilder {
        SearchTasksRequestBuilder::new()
    }

    /// 将请求转换为查询参数
    fn to_query_params(&self) -> HashMap<&'static str, String> {
        let mut params = HashMap::new();

        if let Some(approval_code) = &self.approval_code {
            params.insert("approval_code", approval_code.clone());
        }

        if let Some(instance_code) = &self.instance_code {
            params.insert("instance_code", instance_code.clone());
        }

        if let Some(task_status) = &self.task_status {
            params.insert("task_status", serde_json::to_string(task_status).unwrap());
        }

        if let Some(start_time) = self.start_time {
            params.insert("start_time", start_time.to_string());
        }

        if let Some(end_time) = self.end_time {
            params.insert("end_time", end_time.to_string());
        }

        if let Some(user_id) = &self.user_id {
            params.insert("user_id", user_id.clone());
        }

        if let Some(user_id_type) = &self.user_id_type {
            params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        if let Some(page_size) = self.page_size {
            params.insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = &self.page_token {
            params.insert("page_token", page_token.clone());
        }

        params
    }
}

/// 搜索审批任务请求构建器
#[derive(Debug, Clone, Default)]
pub struct SearchTasksRequestBuilder {
    request: SearchTasksRequest,
}

impl SearchTasksRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchTasksRequest::default(),
        }
    }

    /// 设置审批定义编码
    pub fn approval_code(mut self, approval_code: impl ToString) -> Self {
        self.request.approval_code = Some(approval_code.to_string());
        self
    }

    /// 设置审批实例编码
    pub fn instance_code(mut self, instance_code: impl ToString) -> Self {
        self.request.instance_code = Some(instance_code.to_string());
        self
    }

    /// 设置任务状态
    pub fn task_status(mut self, task_status: TaskStatus) -> Self {
        self.request.task_status = Some(task_status);
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.request.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.request.end_time = Some(end_time);
        self
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl ToString) -> Self {
        self.request.user_id = Some(user_id.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: u32) -> SDKResult<Self> {
        if page_size == 0 {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(
                "Page size cannot be 0, minimum value is 1".to_string(),
            ));
        }
        if page_size > 200 {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(format!(
                "Page size {} exceeds maximum limit of 200",
                page_size
            )));
        }
        self.request.page_size = Some(page_size);
        Ok(self)
    }

    /// 设置分页令牌
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> SearchTasksRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchTasksRequestBuilder,
//    SearchService,
//    SearchTasksRequest,
//    BaseResponse<SearchTasksResponse>,
//    search_tasks
//);

/// 搜索审批任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTasksResponse {
    /// 审批任务列表
    pub tasks: Vec<ApprovalTask>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchTasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索审批抄送请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchCcRequest {
    /// 审批定义编码，用于筛选特定类型的审批
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    /// 审批实例编码，用于筛选特定实例的抄送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_code: Option<String>,
    /// 用户ID，用于筛选特定用户的抄送记录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
    /// 页面大小，范围1-200，默认值为20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 分页令牌，用于获取指定页的数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl SearchCcRequest {
    /// 创建搜索审批抄送请求构建器
    pub fn builder() -> SearchCcRequestBuilder {
        SearchCcRequestBuilder::new()
    }

    /// 将请求转换为查询参数
    fn to_query_params(&self) -> HashMap<&'static str, String> {
        let mut params = HashMap::new();

        if let Some(approval_code) = &self.approval_code {
            params.insert("approval_code", approval_code.clone());
        }

        if let Some(instance_code) = &self.instance_code {
            params.insert("instance_code", instance_code.clone());
        }

        if let Some(user_id) = &self.user_id {
            params.insert("user_id", user_id.clone());
        }

        if let Some(user_id_type) = &self.user_id_type {
            params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        if let Some(page_size) = self.page_size {
            params.insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = &self.page_token {
            params.insert("page_token", page_token.clone());
        }

        params
    }
}

/// 搜索审批抄送请求构建器
#[derive(Debug, Clone, Default)]
pub struct SearchCcRequestBuilder {
    request: SearchCcRequest,
}

impl SearchCcRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchCcRequest::default(),
        }
    }

    /// 设置审批定义编码
    pub fn approval_code(mut self, approval_code: impl ToString) -> Self {
        self.request.approval_code = Some(approval_code.to_string());
        self
    }

    /// 设置审批实例编码
    pub fn instance_code(mut self, instance_code: impl ToString) -> Self {
        self.request.instance_code = Some(instance_code.to_string());
        self
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl ToString) -> Self {
        self.request.user_id = Some(user_id.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: u32) -> SDKResult<Self> {
        if page_size == 0 {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(
                "Page size cannot be 0, minimum value is 1".to_string(),
            ));
        }
        if page_size > 200 {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(format!(
                "Page size {} exceeds maximum limit of 200",
                page_size
            )));
        }
        self.request.page_size = Some(page_size);
        Ok(self)
    }

    /// 设置分页令牌
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> SearchCcRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchCcRequestBuilder,
//    SearchService,
//    SearchCcRequest,
//    BaseResponse<SearchCcResponse>,
//    search_cc
//);

/// 搜索审批抄送响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCcResponse {
    /// 抄送实例列表
    pub cc_instances: Vec<ApprovalInstance>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchCcResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索审批定义ID请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchApprovalIdRequest {
    /// 审批名称，支持模糊搜索
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

impl SearchApprovalIdRequest {
    /// 创建搜索审批定义ID请求构建器
    pub fn builder() -> SearchApprovalIdRequestBuilder {
        SearchApprovalIdRequestBuilder::new()
    }

    /// 将请求转换为查询参数
    fn to_query_params(&self) -> HashMap<&'static str, String> {
        let mut params = HashMap::new();

        if let Some(approval_name) = &self.approval_name {
            params.insert("approval_name", approval_name.clone());
        }

        if let Some(user_id_type) = &self.user_id_type {
            params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        params
    }
}

/// 搜索审批定义ID请求构建器
#[derive(Debug, Clone, Default)]
pub struct SearchApprovalIdRequestBuilder {
    request: SearchApprovalIdRequest,
}

impl SearchApprovalIdRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchApprovalIdRequest::default(),
        }
    }

    /// 设置审批名称
    pub fn approval_name(mut self, approval_name: impl ToString) -> Self {
        self.request.approval_name = Some(approval_name.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> SearchApprovalIdRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchApprovalIdRequestBuilder,
//    SearchService,
//    SearchApprovalIdRequest,
//    BaseResponse<SearchApprovalIdResponse>,
//    search_approval_id
//);

/// 搜索审批定义ID响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchApprovalIdResponse {
    /// 审批定义ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    /// 审批定义名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_name: Option<String>,
}

impl ApiResponseTrait for SearchApprovalIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索用户任务请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchUserTasksRequest {
    /// 用户ID，必填
    pub user_id: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
    /// 审批定义编码，用于筛选特定类型的审批
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_code: Option<String>,
    /// 任务状态，用于筛选特定状态的任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<TaskStatus>,
    /// 开始时间，格式为Unix时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间，格式为Unix时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 页面大小，范围1-200，默认值为20
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// 分页令牌，用于获取指定页的数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl SearchUserTasksRequest {
    /// 创建搜索用户任务请求构建器
    pub fn builder() -> SearchUserTasksRequestBuilder {
        SearchUserTasksRequestBuilder::new()
    }

    /// 将请求转换为查询参数
    fn to_query_params(&self) -> HashMap<&'static str, String> {
        let mut params = HashMap::new();

        params.insert("user_id", self.user_id.clone());

        if let Some(user_id_type) = &self.user_id_type {
            params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        if let Some(approval_code) = &self.approval_code {
            params.insert("approval_code", approval_code.clone());
        }

        if let Some(task_status) = &self.task_status {
            params.insert("task_status", serde_json::to_string(task_status).unwrap());
        }

        if let Some(start_time) = self.start_time {
            params.insert("start_time", start_time.to_string());
        }

        if let Some(end_time) = self.end_time {
            params.insert("end_time", end_time.to_string());
        }

        if let Some(page_size) = self.page_size {
            params.insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = &self.page_token {
            params.insert("page_token", page_token.clone());
        }

        params
    }
}

/// 搜索用户任务请求构建器
#[derive(Debug, Clone, Default)]
pub struct SearchUserTasksRequestBuilder {
    request: SearchUserTasksRequest,
}

impl SearchUserTasksRequestBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchUserTasksRequest::default(),
        }
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl ToString) -> Self {
        self.request.user_id = user_id.to_string();
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 设置审批定义编码
    pub fn approval_code(mut self, approval_code: impl ToString) -> Self {
        self.request.approval_code = Some(approval_code.to_string());
        self
    }

    /// 设置任务状态
    pub fn task_status(mut self, task_status: TaskStatus) -> Self {
        self.request.task_status = Some(task_status);
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.request.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.request.end_time = Some(end_time);
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: u32) -> SDKResult<Self> {
        if page_size == 0 {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(
                "Page size cannot be 0, minimum value is 1".to_string(),
            ));
        }
        if page_size > 200 {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(format!(
                "Page size {} exceeds maximum limit of 200",
                page_size
            )));
        }
        self.request.page_size = Some(page_size);
        Ok(self)
    }

    /// 设置分页令牌
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> SearchUserTasksRequest {
        self.request
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchUserTasksRequestBuilder,
//    SearchService,
//    SearchUserTasksRequest,
//    BaseResponse<SearchTasksResponse>,
//    search_user_tasks
//);
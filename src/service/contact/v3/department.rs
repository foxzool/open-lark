//! Department部门管理服务
//!
//! 提供完整的部门管理功能：
//! - 创建、修改、删除部门
//! - 获取部门信息（单个/批量）
//! - 获取子部门列表
//! - 获取父部门信息
//! - 搜索部门
//! - 部门ID更新
//! - 企业级部门架构管理

use open_lark_core::core::api_req::ApiRequest; // trait_system::ExecutableBuilder temporarily disabled
use crate::core::{
    api_resp::BaseResponse,
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// 导入核心类型
use super::types::*;

/// 部门管理服务
#[derive(Debug, Clone)]
pub struct DepartmentService {
    pub config: Config,
}

impl DepartmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建部门
    /// 创建新的部门组织单元
    ///
    /// # API文档
    ///
    /// 创建新的部门，支持设置部门名称、父部门、部门主管等信息。
    /// 适用于企业组织架构的动态调整和扩展。
    ///
    /// # 参数
    ///
    /// * `request` - 包含部门信息的请求参数
    ///
    /// # 返回值
    ///
    /// 返回创建成功的部门详细信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::department::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder()
    ///         .app_id("your_app_id")
    ///         .app_secret("your_app_secret")
    ///         .build()?;
    ///
    ///     let department = Department {
    ///         name: Some("技术部".to_string()),
    ///         parent_department_id: Some("root_dept".to_string()),
    ///         ..Default::default()
    ///     };
    ///
    ///     let request = CreateDepartmentRequest {
    ///         department,
    ///         user_id_type: Some("open_id".to_string()),
    ///         department_id_type: Some("department_id".to_string()),
    ///         ..Default::default()
    ///     };
    ///
    ///     let response = client.contact.v3.department
    ///         .create(&request).await?;
    ///
    ///     println!("部门创建成功: {:?}", response.data.department.name);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateDepartmentRequest,
    ) -> SDKResult<BaseResponse<CreateDepartmentResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/departments".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 修改部门部分信息
    /// 部分更新部门信息（不覆盖未提供的字段）
    pub async fn patch(
        &self,
        department_id: &str,
        request: &PatchDepartmentRequest,
    ) -> SDKResult<BaseResponse<PatchDepartmentResponse>> {
        let api_path = format!("/open-apis/contact/v3/departments/{}", department_id);

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::PATCH,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 更新部门所有信息
    /// 完全更新部门信息（覆盖所有字段）
    pub async fn update(
        &self,
        department_id: &str,
        request: &UpdateDepartmentRequest,
    ) -> SDKResult<BaseResponse<UpdateDepartmentResponse>> {
        let api_path = format!("/open-apis/contact/v3/departments/{}", department_id);

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 更新部门ID
    /// 修改现有部门的部门ID标识
    pub async fn update_department_id(
        &self,
        department_id: &str,
        request: &UpdateDepartmentIdRequest,
    ) -> SDKResult<BaseResponse<UpdateDepartmentIdResponse>> {
        let api_path = format!("/open-apis/contact/v3/departments/{}/update_id", department_id);

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::PATCH,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取单个部门信息
    /// 获取指定部门的详细信息
    pub async fn get(
        &self,
        department_id: &str,
        request: &GetDepartmentRequest,
    ) -> SDKResult<BaseResponse<GetDepartmentResponse>> {
        let mut api_path = format!("/open-apis/contact/v3/departments/{}", department_id);

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 批量获取部门信息
    /// 一次性获取多个部门的详细信息
    pub async fn batch_get(
        &self,
        request: &BatchGetDepartmentsRequest,
    ) -> SDKResult<BaseResponse<BatchGetDepartmentsResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/departments/batch_get".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取子部门列表
    /// 获取指定部门的直接下级部门列表
    pub async fn get_children(
        &self,
        request: &GetChildrenDepartmentsRequest,
    ) -> SDKResult<BaseResponse<GetChildrenDepartmentsResponse>> {
        let mut api_path = "/open-apis/contact/v3/departments/sub_departments".to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(parent_department_id) = &request.parent_department_id {
            query_params.push(format!("parent_department_id={}", parent_department_id));
        }
        if let Some(user_id_type) = &request.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }
        if let Some(fetch_child) = &request.fetch_child {
            query_params.push(format!("fetch_child={}", fetch_child));
        }
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取父部门信息
    /// 获取指定部门的上级部门信息
    pub async fn get_parent(
        &self,
        request: &GetParentDepartmentRequest,
    ) -> SDKResult<BaseResponse<GetParentDepartmentResponse>> {
        let mut api_path = "/open-apis/contact/v3/departments/parent_department".to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(department_id) = &request.department_id {
            query_params.push(format!("department_id={}", department_id));
        }
        if let Some(user_id_type) = &request.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 搜索部门
    /// 根据关键词搜索部门信息
    pub async fn search(
        &self,
        request: &SearchDepartmentsRequest,
    ) -> SDKResult<BaseResponse<SearchDepartmentsResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/departments/search".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 删除部门
    /// 删除指定的部门（请谨慎操作）
    pub async fn delete(
        &self,
        department_id: &str,
        request: &DeleteDepartmentRequest,
    ) -> SDKResult<BaseResponse<DeleteDepartmentResponse>> {
        let mut api_path = format!("/open-apis/contact/v3/departments/{}", department_id);

        // 添加查询参数
        if let Some(department_id_type) = &request.department_id_type {
            api_path.push('?');
            api_path.push_str(&format!("department_id_type={}", department_id_type));
        }

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }
}

// ==================== 数据模型 ====================

/// 创建部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentRequest {
    /// 部门信息
    pub department: Department,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 客户端令牌，用于幂等性控制
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl Default for CreateDepartmentRequest {
    fn default() -> Self {
        Self {
            department: Department::default(),
            user_id_type: None,
            department_id_type: None,
            client_token: None,
        }
    }
}

/// 创建部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

/// 修改部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDepartmentRequest {
    /// 部门信息
    pub department: Department,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for PatchDepartmentRequest {
    fn default() -> Self {
        Self {
            department: Department::default(),
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 修改部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

/// 更新部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentRequest {
    /// 部门信息
    pub department: Department,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for UpdateDepartmentRequest {
    fn default() -> Self {
        Self {
            department: Department::default(),
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 更新部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

/// 更新部门ID请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentIdRequest {
    /// 新的部门ID
    pub new_department_id: String,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for UpdateDepartmentIdRequest {
    fn default() -> Self {
        Self {
            new_department_id: String::new(),
            department_id_type: None,
        }
    }
}

/// 更新部门ID响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentIdResponse {
    /// 操作结果
    pub success: bool,
}

impl Default for UpdateDepartmentIdResponse {
    fn default() -> Self {
        Self { success: true }
    }
}

/// 获取部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentRequest {
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for GetDepartmentRequest {
    fn default() -> Self {
        Self {
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 获取部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

/// 批量获取部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetDepartmentsRequest {
    /// 部门ID列表
    pub department_ids: Vec<String>,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for BatchGetDepartmentsRequest {
    fn default() -> Self {
        Self {
            department_ids: Vec::new(),
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 批量获取部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetDepartmentsResponse {
    /// 部门列表
    pub departments: Vec<Department>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 获取子部门列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChildrenDepartmentsRequest {
    /// 父部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 是否递归获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_child: Option<bool>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for GetChildrenDepartmentsRequest {
    fn default() -> Self {
        Self {
            parent_department_id: None,
            user_id_type: None,
            department_id_type: None,
            fetch_child: None,
            page_size: None,
            page_token: None,
        }
    }
}

/// 获取子部门列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChildrenDepartmentsResponse {
    /// 部门列表
    pub departments: Vec<Department>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 获取父部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParentDepartmentRequest {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for GetParentDepartmentRequest {
    fn default() -> Self {
        Self {
            department_id: None,
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 获取父部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetParentDepartmentResponse {
    /// 父部门信息
    pub parent_department: Option<Department>,
}

/// 搜索部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDepartmentsRequest {
    /// 搜索关键词
    pub query: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for SearchDepartmentsRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 搜索部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDepartmentsResponse {
    /// 部门列表
    pub departments: Vec<Department>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 删除部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDepartmentRequest {
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for DeleteDepartmentRequest {
    fn default() -> Self {
        Self {
            department_id_type: None,
        }
    }
}

/// 删除部门响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDepartmentResponse {
    /// 操作结果
    pub success: bool,
}

impl Default for DeleteDepartmentResponse {
    fn default() -> Self {
        Self { success: true }
    }
}

// ==================== Builder 模式实现 ====================

/// 创建部门请求构建器
#[derive(Debug, Clone)]
pub struct CreateDepartmentBuilder {
    request: CreateDepartmentRequest,
}

impl CreateDepartmentBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: CreateDepartmentRequest::default(),
        }
    }

    /// 设置部门信息
    pub fn department(mut self, department: Department) -> Self {
        self.request.department = department;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 设置客户端令牌
    pub fn client_token(mut self, client_token: &str) -> Self {
        self.request.client_token = Some(client_token.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> CreateDepartmentRequest {
        self.request
    }
}

impl Default for CreateDepartmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    CreateDepartmentBuilder,
//    DepartmentService,
//    CreateDepartmentRequest,
//    BaseResponse<CreateDepartmentResponse>,
//    create
//);

/// 修改部门请求构建器
#[derive(Debug, Clone)]
pub struct PatchDepartmentBuilder {
    request: PatchDepartmentRequest,
}

impl PatchDepartmentBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: PatchDepartmentRequest::default(),
        }
    }

    /// 设置部门信息
    pub fn department(mut self, department: Department) -> Self {
        self.request.department = department;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> PatchDepartmentRequest {
        self.request
    }
}

impl Default for PatchDepartmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    PatchDepartmentBuilder,
//    DepartmentService,
//    PatchDepartmentRequest,
//    BaseResponse<PatchDepartmentResponse>,
//    patch
//);

/// 更新部门请求构建器
#[derive(Debug, Clone)]
pub struct UpdateDepartmentBuilder {
    request: UpdateDepartmentRequest,
}

impl UpdateDepartmentBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: UpdateDepartmentRequest::default(),
        }
    }

    /// 设置部门信息
    pub fn department(mut self, department: Department) -> Self {
        self.request.department = department;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> UpdateDepartmentRequest {
        self.request
    }
}

impl Default for UpdateDepartmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    UpdateDepartmentBuilder,
//    DepartmentService,
//    UpdateDepartmentRequest,
//    BaseResponse<UpdateDepartmentResponse>,
//    update
//);

/// 批量获取部门请求构建器
#[derive(Debug, Clone)]
pub struct BatchGetDepartmentsBuilder {
    request: BatchGetDepartmentsRequest,
}

impl BatchGetDepartmentsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: BatchGetDepartmentsRequest::default(),
        }
    }

    /// 设置部门ID列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.request.department_ids = department_ids;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> BatchGetDepartmentsRequest {
        self.request
    }
}

impl Default for BatchGetDepartmentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    BatchGetDepartmentsBuilder,
//    DepartmentService,
//    BatchGetDepartmentsRequest,
//    BaseResponse<BatchGetDepartmentsResponse>,
//    batch_get
//);

/// 获取子部门列表请求构建器
#[derive(Debug, Clone)]
pub struct GetChildrenDepartmentsBuilder {
    request: GetChildrenDepartmentsRequest,
}

impl GetChildrenDepartmentsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetChildrenDepartmentsRequest::default(),
        }
    }

    /// 设置父部门ID
    pub fn parent_department_id(mut self, parent_department_id: &str) -> Self {
        self.request.parent_department_id = Some(parent_department_id.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 设置是否递归获取
    pub fn fetch_child(mut self, fetch_child: bool) -> Self {
        self.request.fetch_child = Some(fetch_child);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetChildrenDepartmentsRequest {
        self.request
    }
}

impl Default for GetChildrenDepartmentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetChildrenDepartmentsBuilder,
//    DepartmentService,
//    GetChildrenDepartmentsRequest,
//    BaseResponse<GetChildrenDepartmentsResponse>,
//    get_children
//);

/// 搜索部门请求构建器
#[derive(Debug, Clone)]
pub struct SearchDepartmentsBuilder {
    request: SearchDepartmentsRequest,
}

impl SearchDepartmentsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchDepartmentsRequest::default(),
        }
    }

    /// 设置搜索关键词
    pub fn query(mut self, query: &str) -> Self {
        self.request.query = query.to_string();
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> SearchDepartmentsRequest {
        self.request
    }
}

impl Default for SearchDepartmentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchDepartmentsBuilder,
//    DepartmentService,
//    SearchDepartmentsRequest,
//    BaseResponse<SearchDepartmentsResponse>,
//    search
//);

impl DepartmentService {
    /// 创建部门构建器
    pub fn create_department_builder(&self) -> CreateDepartmentBuilder {
        CreateDepartmentBuilder::new()
    }

    /// 修改部门构建器
    pub fn patch_department_builder(&self) -> PatchDepartmentBuilder {
        PatchDepartmentBuilder::new()
    }

    /// 更新部门构建器
    pub fn update_department_builder(&self) -> UpdateDepartmentBuilder {
        UpdateDepartmentBuilder::new()
    }

    /// 批量获取部门构建器
    pub fn batch_get_departments_builder(&self) -> BatchGetDepartmentsBuilder {
        BatchGetDepartmentsBuilder::new()
    }

    /// 获取子部门列表构建器
    pub fn get_children_departments_builder(&self) -> GetChildrenDepartmentsBuilder {
        GetChildrenDepartmentsBuilder::new()
    }

    /// 搜索部门构建器
    pub fn search_departments_builder(&self) -> SearchDepartmentsBuilder {
        SearchDepartmentsBuilder::new()
    }
}
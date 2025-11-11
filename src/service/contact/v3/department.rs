#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 部门管理服务
//!
//! 提供完整的部门管理功能：
//! - 创建、修改、删除部门
//! - 获取部门信息（单个/批量）
//! - 获取子部门列表
//! - 获取父部门信息
//! - 搜索部门
//! - 部门ID更新
//! - 企业级部门架构管理

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 部门信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Department {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 主管用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<Vec<String>>,
    /// 部门描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 部门在父部门中的排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    /// 部门状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DepartmentStatus>,
    /// 部门标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_tag: Option<Vec<String>>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 部门类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_type: Option<String>,
    /// 部门路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_path: Option<String>,
}

impl Default for Department {
    fn default() -> Self {
        Self {
            department_id: None,
            name: None,
            parent_department_id: None,
            leader_user_id: None,
            description: None,
            order: None,
            status: None,
            department_tag: None,
            create_time: None,
            update_time: None,
            department_type: None,
            department_path: None,
        }
    }
}

/// 部门状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DepartmentStatus {
    /// 正常
    Normal,
    /// 禁用
    Disabled,
}

impl Default for DepartmentStatus {
    fn default() -> Self {
        Self::Normal
    }
}

/// 部门管理服务
#[derive(Debug, Clone)]
pub struct DepartmentService {
    config: Config,
}

impl DepartmentService {
    /// 创建新的部门服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建部门
    ///
    /// 创建新的部门组织单元
    ///
    /// # 参数
    /// * `req` - 创建部门请求
    ///
    /// # 返回值
    /// 返回创建成功的部门详细信息
    pub async fn create(
        &self,
        req: &CreateDepartmentRequest,
    ) -> SDKResult<CreateDepartmentResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENTS
                .to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 修改部门部分信息
    ///
    /// 部分更新部门信息（不覆盖未提供的字段）
    ///
    /// # 参数
    /// * `department_id` - 部门ID
    /// * `req` - 修改部门请求
    ///
    /// # 返回值
    /// 返回修改后的部门详细信息
    pub async fn patch(
        &self,
        department_id: &str,
        req: &PatchDepartmentRequest,
    ) -> SDKResult<PatchDepartmentResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENT_GET
            .replace("{department_id}", department_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<PatchDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新部门所有信息
    ///
    /// 完全更新部门信息（覆盖所有字段）
    ///
    /// # 参数
    /// * `department_id` - 部门ID
    /// * `req` - 更新部门请求
    ///
    /// # 返回值
    /// 返回更新后的部门详细信息
    pub async fn update(
        &self,
        department_id: &str,
        req: &UpdateDepartmentRequest,
    ) -> SDKResult<UpdateDepartmentResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENT_GET
            .replace("{department_id}", department_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取部门信息
    ///
    /// 根据部门ID获取部门的详细信息
    ///
    /// # 参数
    /// * `department_id` - 部门ID
    /// * `req` - 获取部门信息请求
    ///
    /// # 返回值
    /// 返回部门的详细信息
    pub async fn get(
        &self,
        department_id: &str,
        req: &GetDepartmentRequest,
    ) -> SDKResult<GetDepartmentResponse> {
        let mut api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENT_GET
            .replace("{department_id}", department_id);

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除部门
    ///
    /// 删除指定的部门
    ///
    /// # 参数
    /// * `department_id` - 部门ID
    ///
    /// # 返回值
    /// 返回删除操作的结果
    pub async fn delete(&self, department_id: &str) -> SDKResult<DeleteDepartmentResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENT_GET
            .replace("{department_id}", department_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<DeleteDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取子部门列表
    ///
    /// 获取指定部门的直接子部门列表
    ///
    /// # 参数
    /// * `department_id` - 部门ID
    /// * `req` - 获取子部门列表请求
    ///
    /// # 返回值
    /// 返回子部门列表，支持分页
    pub async fn get_sub_department_list(
        &self,
        department_id: &str,
        req: &GetSubDepartmentListRequest,
    ) -> SDKResult<GetSubDepartmentListResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENTS_CHILDREN.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        query_params.push(format!("department_id={}", department_id));
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<GetSubDepartmentListResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取部门ID列表
    ///
    /// 获取租户下所有部门的ID列表
    ///
    /// # 参数
    /// * `req` - 获取部门ID列表请求
    ///
    /// # 返回值
    /// 返回部门ID列表，支持分页
    pub async fn get_id_list(
        &self,
        req: &GetDepartmentIdListRequest,
    ) -> SDKResult<GetDepartmentIdListResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENTS_BATCH.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<GetDepartmentIdListResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 搜索部门
    ///
    /// 根据关键词搜索部门
    ///
    /// # 参数
    /// * `req` - 搜索部门请求
    ///
    /// # 返回值
    /// 返回搜索结果，支持分页
    pub async fn search(
        &self,
        req: &SearchDepartmentRequest,
    ) -> SDKResult<SearchDepartmentResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENTS_SEARCH.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(query) = &req.query {
            query_params.push(format!("query={}", query));
        }
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<SearchDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 创建部门请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateDepartmentRequest {
    /// 部门信息
    pub department: Department,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 创建部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

impl ApiResponseTrait for CreateDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改部门请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchDepartmentRequest {
    /// 部门信息
    pub department: Department,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 修改部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

impl ApiResponseTrait for PatchDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新部门请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateDepartmentRequest {
    /// 部门信息
    pub department: Department,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 更新部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

impl ApiResponseTrait for UpdateDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取部门信息请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDepartmentRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
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

/// 获取部门信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

impl ApiResponseTrait for GetDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeleteDepartmentResponse {}

impl ApiResponseTrait for DeleteDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取子部门列表请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetSubDepartmentListRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for GetSubDepartmentListRequest {
    fn default() -> Self {
        Self {
            user_id_type: None,
            department_id_type: None,
            page_size: None,
            page_token: None,
        }
    }
}

/// 获取子部门列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetSubDepartmentListResponse {
    /// 部门列表
    pub items: Vec<Department>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetSubDepartmentListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取部门ID列表请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDepartmentIdListRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for GetDepartmentIdListRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
            department_id_type: None,
        }
    }
}

/// 获取部门ID列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetDepartmentIdListResponse {
    /// 部门ID列表
    pub items: Vec<DepartmentIdItem>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetDepartmentIdListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 部门ID项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentIdItem {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 搜索部门请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchDepartmentRequest {
    /// 搜索关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for SearchDepartmentRequest {
    fn default() -> Self {
        Self {
            query: None,
            user_id_type: None,
            department_id_type: None,
            page_size: None,
            page_token: None,
        }
    }
}

/// 搜索部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchDepartmentResponse {
    /// 搜索结果
    pub items: Vec<Department>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建部门构建器
#[derive(Debug, Clone)]
pub struct CreateDepartmentBuilder {
    request: CreateDepartmentRequest,
}

impl CreateDepartmentBuilder {
    /// 创建新的部门构建器
    pub fn new() -> Self {
        Self {
            request: CreateDepartmentRequest {
                department: Department::default(),
                user_id_type: None,
                department_id_type: None,
            },
        }
    }

    /// 设置部门信息
    pub fn department(mut self, department: Department) -> Self {
        self.request.department = department;
        self
    }

    /// 设置部门名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.department.name = Some(name.into());
        self
    }

    /// 设置父部门ID
    pub fn parent_department_id(mut self, parent_department_id: impl Into<String>) -> Self {
        self.request.department.parent_department_id = Some(parent_department_id.into());
        self
    }

    /// 设置主管用户ID列表
    pub fn leader_user_id(mut self, leader_user_id: Vec<String>) -> Self {
        self.request.department.leader_user_id = Some(leader_user_id);
        self
    }

    /// 设置部门描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.department.description = Some(description.into());
        self
    }

    /// 设置排序
    pub fn order(mut self, order: i64) -> Self {
        self.request.department.order = Some(order);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.request.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行创建操作
    pub async fn execute(self, service: &DepartmentService) -> SDKResult<CreateDepartmentResponse> {
        service.create(&self.request).await
    }
}

impl Default for CreateDepartmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 搜索部门构建器
#[derive(Debug, Clone)]
pub struct SearchDepartmentBuilder {
    request: SearchDepartmentRequest,
}

impl SearchDepartmentBuilder {
    /// 创建新的搜索构建器
    pub fn new() -> Self {
        Self {
            request: SearchDepartmentRequest::default(),
        }
    }

    /// 设置搜索关键词
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.request.query = Some(query.into());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.request.department_id_type = Some(department_id_type.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 执行搜索操作
    pub async fn execute(self, service: &DepartmentService) -> SDKResult<SearchDepartmentResponse> {
        service.search(&self.request).await
    }
}

impl Default for SearchDepartmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DepartmentService {
    /// 创建部门构建器
    pub fn create_department_builder(&self) -> CreateDepartmentBuilder {
        CreateDepartmentBuilder::new()
    }

    /// 创建搜索部门构建器
    pub fn search_department_builder(&self) -> SearchDepartmentBuilder {
        SearchDepartmentBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_department_service_creation() {
        let config = Config::default();
        let service = DepartmentService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_department_default_creation() {
        let department = Department::default();
        assert_eq!(department.department_id, None);
        assert_eq!(department.name, None);
        assert_eq!(department.parent_department_id, None);
        assert_eq!(department.leader_user_id, None);
        assert_eq!(department.description, None);
        assert_eq!(department.order, None);
        assert_eq!(department.status, None);
        assert_eq!(department.department_tag, None);
        assert_eq!(department.create_time, None);
        assert_eq!(department.update_time, None);
        assert_eq!(department.department_type, None);
        assert_eq!(department.department_path, None);
    }

    #[test]
    fn test_department_with_data() {
        let department = Department {
            department_id: Some("dept_123".to_string()),
            name: Some("技术部".to_string()),
            parent_department_id: Some("root".to_string()),
            leader_user_id: Some(vec!["user_001".to_string(), "user_002".to_string()]),
            description: Some("负责技术研发的部门".to_string()),
            order: Some(1),
            status: Some(DepartmentStatus::Normal),
            department_tag: Some(vec!["技术".to_string(), "研发".to_string()]),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-12-31T23:59:59Z".to_string()),
            department_type: Some("business".to_string()),
            department_path: Some("/技术部".to_string()),
        };

        assert_eq!(department.department_id, Some("dept_123".to_string()));
        assert_eq!(department.name, Some("技术部".to_string()));
        assert_eq!(department.parent_department_id, Some("root".to_string()));
        assert_eq!(
            department.leader_user_id,
            Some(vec!["user_001".to_string(), "user_002".to_string()])
        );
        assert_eq!(
            department.description,
            Some("负责技术研发的部门".to_string())
        );
        assert_eq!(department.order, Some(1));
        assert_eq!(department.status, Some(DepartmentStatus::Normal));
        assert_eq!(
            department.department_tag,
            Some(vec!["技术".to_string(), "研发".to_string()])
        );
        assert_eq!(
            department.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            department.update_time,
            Some("2023-12-31T23:59:59Z".to_string())
        );
        assert_eq!(department.department_type, Some("business".to_string()));
        assert_eq!(department.department_path, Some("/技术部".to_string()));
    }

    #[test]
    fn test_department_status_enum() {
        let normal_status = DepartmentStatus::Normal;
        let disabled_status = DepartmentStatus::Disabled;

        assert_eq!(normal_status, DepartmentStatus::Normal);
        assert_eq!(disabled_status, DepartmentStatus::Disabled);
        assert_ne!(normal_status, disabled_status);

        // Test default
        let default_status = DepartmentStatus::default();
        assert_eq!(default_status, DepartmentStatus::Normal);
    }

    #[test]
    fn test_create_department_request() {
        let department = Department {
            name: Some("产品部".to_string()),
            parent_department_id: Some("root".to_string()),
            description: Some("负责产品设计和管理".to_string()),
            ..Default::default()
        };

        let request = CreateDepartmentRequest {
            department: department.clone(),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        assert_eq!(request.department.name, Some("产品部".to_string()));
        assert_eq!(
            request.department.parent_department_id,
            Some("root".to_string())
        );
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_get_department_request_default() {
        let request = GetDepartmentRequest::default();
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_get_department_request_with_types() {
        let request = GetDepartmentRequest {
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_get_sub_department_list_request_default() {
        let request = GetSubDepartmentListRequest::default();
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_get_sub_department_list_request_with_pagination() {
        let request = GetSubDepartmentListRequest {
            user_id_type: Some("union_id".to_string()),
            department_id_type: Some("open_department_id".to_string()),
            page_size: Some(50),
            page_token: Some("page_token_123".to_string()),
        };

        assert_eq!(request.user_id_type, Some("union_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("open_department_id".to_string())
        );
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("page_token_123".to_string()));
    }

    #[test]
    fn test_search_department_request_default() {
        let request = SearchDepartmentRequest::default();
        assert_eq!(request.query, None);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_search_department_request_with_query() {
        let request = SearchDepartmentRequest {
            query: Some("技术".to_string()),
            user_id_type: Some("user_id".to_string()),
            department_id_type: Some("department_id".to_string()),
            page_size: Some(20),
            page_token: Some("search_token".to_string()),
        };

        assert_eq!(request.query, Some("技术".to_string()));
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("search_token".to_string()));
    }

    #[test]
    fn test_department_id_item() {
        let item = DepartmentIdItem {
            department_id: "dept_001".to_string(),
            name: Some("市场部".to_string()),
        };

        assert_eq!(item.department_id, "dept_001");
        assert_eq!(item.name, Some("市场部".to_string()));

        let item_without_name = DepartmentIdItem {
            department_id: "dept_002".to_string(),
            name: None,
        };

        assert_eq!(item_without_name.department_id, "dept_002");
        assert_eq!(item_without_name.name, None);
    }

    #[test]
    fn test_response_default_creation() {
        let create_response = CreateDepartmentResponse::default();
        let patch_response = PatchDepartmentResponse::default();
        let update_response = UpdateDepartmentResponse::default();
        let get_response = GetDepartmentResponse::default();
        let delete_response = DeleteDepartmentResponse::default();
        let sub_list_response = GetSubDepartmentListResponse::default();
        let id_list_response = GetDepartmentIdListResponse::default();
        let search_response = SearchDepartmentResponse::default();

        assert!(!format!("{:?}", create_response).is_empty());
        assert!(!format!("{:?}", patch_response).is_empty());
        assert!(!format!("{:?}", update_response).is_empty());
        assert!(!format!("{:?}", get_response).is_empty());
        assert!(!format!("{:?}", delete_response).is_empty());
        assert_eq!(sub_list_response.items.len(), 0);
        assert_eq!(id_list_response.items.len(), 0);
        assert_eq!(search_response.items.len(), 0);
    }

    #[test]
    fn test_response_with_data() {
        let mut create_response = CreateDepartmentResponse::default();
        create_response.department = Department {
            department_id: Some("dept_new".to_string()),
            name: Some("新部门".to_string()),
            ..Default::default()
        };

        assert_eq!(
            create_response.department.department_id,
            Some("dept_new".to_string())
        );
        assert_eq!(create_response.department.name, Some("新部门".to_string()));

        let mut sub_list_response = GetSubDepartmentListResponse::default();
        sub_list_response.items = vec![
            Department {
                department_id: Some("dept_sub1".to_string()),
                name: Some("子部门1".to_string()),
                ..Default::default()
            },
            Department {
                department_id: Some("dept_sub2".to_string()),
                name: Some("子部门2".to_string()),
                ..Default::default()
            },
        ];
        sub_list_response.has_more = Some(true);
        sub_list_response.page_token = Some("next_page".to_string());

        assert_eq!(sub_list_response.items.len(), 2);
        assert_eq!(
            sub_list_response.items[0].department_id,
            Some("dept_sub1".to_string())
        );
        assert_eq!(sub_list_response.has_more, Some(true));
        assert_eq!(sub_list_response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementations() {
        assert_eq!(
            CreateDepartmentResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(PatchDepartmentResponse::data_format(), ResponseFormat::Data);
        assert_eq!(
            UpdateDepartmentResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(GetDepartmentResponse::data_format(), ResponseFormat::Data);
        assert_eq!(
            DeleteDepartmentResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            GetSubDepartmentListResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            GetDepartmentIdListResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            SearchDepartmentResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_create_department_builder() {
        let builder = CreateDepartmentBuilder::new()
            .name("研发部")
            .parent_department_id("root")
            .description("负责研发工作")
            .order(1)
            .user_id_type("open_id")
            .department_id_type("department_id");

        assert_eq!(builder.request.department.name, Some("研发部".to_string()));
        assert_eq!(
            builder.request.department.parent_department_id,
            Some("root".to_string())
        );
        assert_eq!(
            builder.request.department.description,
            Some("负责研发工作".to_string())
        );
        assert_eq!(builder.request.department.order, Some(1));
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            builder.request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_search_department_builder() {
        let builder = SearchDepartmentBuilder::new()
            .query("技术")
            .user_id_type("user_id")
            .department_id_type("department_id")
            .page_size(30)
            .page_token("search_page_token");

        assert_eq!(builder.request.query, Some("技术".to_string()));
        assert_eq!(builder.request.user_id_type, Some("user_id".to_string()));
        assert_eq!(
            builder.request.department_id_type,
            Some("department_id".to_string())
        );
        assert_eq!(builder.request.page_size, Some(30));
        assert_eq!(
            builder.request.page_token,
            Some("search_page_token".to_string())
        );
    }

    #[test]
    fn test_request_serialization() {
        let department = Department {
            name: Some("测试部".to_string()),
            parent_department_id: Some("parent".to_string()),
            description: Some("测试用途部门".to_string()),
            ..Default::default()
        };

        let request = CreateDepartmentRequest {
            department,
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateDepartmentRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.department.name, deserialized.department.name);
        assert_eq!(request.user_id_type, deserialized.user_id_type);
        assert_eq!(request.department_id_type, deserialized.department_id_type);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = GetSubDepartmentListRequest {
            user_id_type: Some("union_id".to_string()),
            department_id_type: Some("open_department_id".to_string()),
            page_size: Some(25),
            page_token: Some("test_page_token".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        assert_eq!(query_params.len(), 4);
        assert!(query_params.contains(&"user_id_type=union_id".to_string()));
        assert!(query_params.contains(&"department_id_type=open_department_id".to_string()));
        assert!(query_params.contains(&"page_size=25".to_string()));
        assert!(query_params.contains(&"page_token=test_page_token".to_string()));
    }

    #[test]
    fn test_department_complex_scenarios() {
        // Test department with all fields
        let comprehensive_department = Department {
            department_id: Some("dept_comprehensive".to_string()),
            name: Some("综合管理部".to_string()),
            parent_department_id: Some("root_admin".to_string()),
            leader_user_id: Some(vec![
                "leader_001".to_string(),
                "leader_002".to_string(),
                "leader_003".to_string(),
            ]),
            description: Some("负责综合管理事务的部门".to_string()),
            order: Some(10),
            status: Some(DepartmentStatus::Normal),
            department_tag: Some(vec![
                "管理".to_string(),
                "行政".to_string(),
                "综合".to_string(),
            ]),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
            department_type: Some("administrative".to_string()),
            department_path: Some("/根部门/综合管理部".to_string()),
        };

        assert_eq!(
            comprehensive_department.department_id,
            Some("dept_comprehensive".to_string())
        );
        assert_eq!(
            comprehensive_department.name,
            Some("综合管理部".to_string())
        );
        assert_eq!(
            comprehensive_department.parent_department_id,
            Some("root_admin".to_string())
        );
        assert_eq!(
            comprehensive_department
                .leader_user_id
                .as_ref()
                .unwrap()
                .len(),
            3
        );
        assert_eq!(
            comprehensive_department.description,
            Some("负责综合管理事务的部门".to_string())
        );
        assert_eq!(comprehensive_department.order, Some(10));
        assert_eq!(
            comprehensive_department.status,
            Some(DepartmentStatus::Normal)
        );
        assert_eq!(
            comprehensive_department
                .department_tag
                .as_ref()
                .unwrap()
                .len(),
            3
        );
        assert_eq!(
            comprehensive_department.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_department.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_department.department_type,
            Some("administrative".to_string())
        );
        assert_eq!(
            comprehensive_department.department_path,
            Some("/根部门/综合管理部".to_string())
        );
    }

    #[test]
    fn test_department_status_variations() {
        // Test different status values
        let normal_dept = Department {
            name: Some("正常部门".to_string()),
            status: Some(DepartmentStatus::Normal),
            ..Default::default()
        };

        let disabled_dept = Department {
            name: Some("禁用部门".to_string()),
            status: Some(DepartmentStatus::Disabled),
            ..Default::default()
        };

        assert_eq!(normal_dept.status, Some(DepartmentStatus::Normal));
        assert_eq!(disabled_dept.status, Some(DepartmentStatus::Disabled));
        assert_ne!(normal_dept.status, disabled_dept.status);
    }

    #[test]
    fn test_leader_user_id_variations() {
        // Test with different leader configurations
        let single_leader = Department {
            name: Some("单主管部门".to_string()),
            leader_user_id: Some(vec!["user_001".to_string()]),
            ..Default::default()
        };

        let multiple_leaders = Department {
            name: Some("多主管部门".to_string()),
            leader_user_id: Some(vec![
                "user_002".to_string(),
                "user_003".to_string(),
                "user_004".to_string(),
            ]),
            ..Default::default()
        };

        let no_leader = Department {
            name: Some("无主管部门".to_string()),
            leader_user_id: None,
            ..Default::default()
        };

        assert_eq!(single_leader.leader_user_id.as_ref().unwrap().len(), 1);
        assert_eq!(multiple_leaders.leader_user_id.as_ref().unwrap().len(), 3);
        assert!(no_leader.leader_user_id.is_none());
    }

    #[test]
    fn test_department_tag_scenarios() {
        // Test with different tag configurations
        let tagged_dept = Department {
            name: Some("标签部门".to_string()),
            department_tag: Some(vec![
                "研发".to_string(),
                "核心".to_string(),
                "重要".to_string(),
            ]),
            ..Default::default()
        };

        let untagged_dept = Department {
            name: Some("无标签部门".to_string()),
            department_tag: None,
            ..Default::default()
        };

        assert_eq!(tagged_dept.department_tag.as_ref().unwrap().len(), 3);
        assert!(tagged_dept
            .department_tag
            .as_ref()
            .unwrap()
            .contains(&"研发".to_string()));
        assert!(tagged_dept
            .department_tag
            .as_ref()
            .unwrap()
            .contains(&"核心".to_string()));
        assert!(tagged_dept
            .department_tag
            .as_ref()
            .unwrap()
            .contains(&"重要".to_string()));
        assert!(untagged_dept.department_tag.is_none());
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENTS,
            "/open-apis/contact/v3/departments"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENT_GET,
            "/open-apis/contact/v3/departments/{department_id}"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENTS_CHILDREN,
            "/open-apis/contact/v3/departments/children"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENTS_BATCH,
            "/open-apis/contact/v3/departments/batch"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_DEPARTMENTS_SEARCH,
            "/open-apis/contact/v3/departments/search"
        );
    }
}

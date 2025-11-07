#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 单位管理服务
//!
//! 提供完整的单位管理功能：
//! - 创建单位
//! - 更新单位
//! - 获取单个单位信息
//! - 获取单位列表
//! - 删除单位
//! - 单位与部门的绑定/解绑
//! - 获取单位下的部门列表

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 单位信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    /// 单位ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    /// 单位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 单位编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<String>,
    /// 父单位ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_unit_id: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            unit_id: None,
            name: None,
            unit_code: None,
            parent_unit_id: None,
            description: None,
            status: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 单位服务
#[derive(Debug, Clone)]
pub struct UnitService {
    config: Config,
}

impl UnitService {
    /// 创建新的单位服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建单位
    ///
    /// 创建一个新的单位
    ///
    /// # 参数
    /// * `req` - 创建单位请求
    ///
    /// # 返回值
    /// 返回创建的单位信息
    pub async fn create(&self, req: &CreateUnitRequest) -> SDKResult<CreateUnitResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::CONTACT_V3_UNITS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateUnitResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新单位
    ///
    /// 更新指定单位的信息
    ///
    /// # 参数
    /// * `unit_id` - 单位ID
    /// * `req` - 更新单位请求
    ///
    /// # 返回值
    /// 返回更新后的单位信息
    pub async fn update(
        &self,
        unit_id: &str,
        req: &UpdateUnitRequest,
    ) -> SDKResult<UpdateUnitResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_UNIT_GET
            .replace("{unit_id}", unit_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<UpdateUnitResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个单位信息
    ///
    /// 根据单位ID获取单位的详细信息
    ///
    /// # 参数
    /// * `unit_id` - 单位ID
    ///
    /// # 返回值
    /// 返回单位的详细信息
    pub async fn get(&self, unit_id: &str) -> SDKResult<GetUnitResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_UNIT_GET
            .replace("{unit_id}", unit_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetUnitResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单位列表
    ///
    /// 获取企业内所有单位的列表，支持分页查询
    ///
    /// # 参数
    /// * `req` - 查询单位列表请求
    ///
    /// # 返回值
    /// 返回单位列表，支持分页
    pub async fn list(&self, req: &ListUnitsRequest) -> SDKResult<ListUnitsResponse> {
        let mut api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_UNITS.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(parent_unit_id) = &req.parent_unit_id {
            query_params.push(format!("parent_unit_id={}", parent_unit_id));
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

        let resp = Transport::<ListUnitsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除单位
    ///
    /// 删除指定的单位
    ///
    /// # 参数
    /// * `unit_id` - 单位ID
    ///
    /// # 返回值
    /// 返回删除操作的结果
    pub async fn delete(&self, unit_id: &str) -> SDKResult<DeleteUnitResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_UNIT_GET
            .replace("{unit_id}", unit_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<DeleteUnitResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 单位绑定部门
    ///
    /// 将部门绑定到指定单位
    ///
    /// # 参数
    /// * `unit_id` - 单位ID
    /// * `req` - 绑定部门请求
    ///
    /// # 返回值
    /// 返回绑定操作的结果
    pub async fn bind_department(
        &self,
        unit_id: &str,
        req: &BindDepartmentRequest,
    ) -> SDKResult<BindDepartmentResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_UNIT_BIND_DEPARTMENT
            .replace("{unit_id}", unit_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BindDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 单位解绑部门
    ///
    /// 将部门从指定单位解绑
    ///
    /// # 参数
    /// * `unit_id` - 单位ID
    /// * `req` - 解绑部门请求
    ///
    /// # 返回值
    /// 返回解绑操作的结果
    pub async fn unbind_department(
        &self,
        unit_id: &str,
        req: &UnbindDepartmentRequest,
    ) -> SDKResult<UnbindDepartmentResponse> {
        let api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_UNIT_UNBIND_DEPARTMENT
                .replace("{unit_id}", unit_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UnbindDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单位下的部门列表
    ///
    /// 获取指定单位下的所有部门列表
    ///
    /// # 参数
    /// * `unit_id` - 单位ID
    /// * `req` - 查询部门列表请求
    ///
    /// # 返回值
    /// 返回部门列表，支持分页
    pub async fn list_departments(
        &self,
        unit_id: &str,
        req: &ListUnitDepartmentsRequest,
    ) -> SDKResult<ListUnitDepartmentsResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_UNIT_LIST_DEPARTMENT
                .replace("{unit_id}", unit_id);

        // 添加查询参数
        let mut query_params = Vec::new();
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
            Transport::<ListUnitDepartmentsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 创建单位请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUnitRequest {
    /// 单位名称
    pub name: String,
    /// 单位编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<String>,
    /// 父单位ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_unit_id: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建单位响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateUnitResponse {
    /// 单位信息
    pub unit: Unit,
}

impl ApiResponseTrait for CreateUnitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新单位请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUnitRequest {
    /// 单位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 单位编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<String>,
    /// 父单位ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_unit_id: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 更新单位响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateUnitResponse {
    /// 单位信息
    pub unit: Unit,
}

impl ApiResponseTrait for UpdateUnitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取单个单位信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUnitResponse {
    /// 单位信息
    pub unit: Unit,
}

impl ApiResponseTrait for GetUnitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询单位列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUnitsRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 父单位ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_unit_id: Option<String>,
}

impl Default for ListUnitsRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
            parent_unit_id: None,
        }
    }
}

/// 查询单位列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListUnitsResponse {
    /// 单位列表
    pub items: Vec<Unit>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListUnitsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除单位响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteUnitResponse {
    /// 是否成功删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteUnitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 绑定部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindDepartmentRequest {
    /// 部门ID列表
    pub department_ids: Vec<String>,
}

/// 绑定部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BindDepartmentResponse {
    /// 是否成功绑定
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for BindDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 解绑部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindDepartmentRequest {
    /// 部门ID列表
    pub department_ids: Vec<String>,
}

/// 解绑部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnbindDepartmentResponse {
    /// 是否成功解绑
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for UnbindDepartmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询单位部门列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUnitDepartmentsRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for ListUnitDepartmentsRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }
}

/// 单位下的部门信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitDepartment {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 绑定时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_time: Option<String>,
}

impl Default for UnitDepartment {
    fn default() -> Self {
        Self {
            department_id: None,
            name: None,
            bind_time: None,
        }
    }
}

/// 查询单位部门列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListUnitDepartmentsResponse {
    /// 部门列表
    pub items: Vec<UnitDepartment>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListUnitDepartmentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建单位构建器
#[derive(Debug, Clone)]
pub struct CreateUnitBuilder {
    request: CreateUnitRequest,
}

impl Default for CreateUnitBuilder {
    fn default() -> Self {
        Self {
            request: CreateUnitRequest {
                name: String::new(),
                unit_code: None,
                parent_unit_id: None,
                description: None,
            },
        }
    }
}

impl CreateUnitBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置单位名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    /// 设置单位编码
    pub fn unit_code(mut self, unit_code: impl Into<String>) -> Self {
        self.request.unit_code = Some(unit_code.into());
        self
    }

    /// 设置父单位ID
    pub fn parent_unit_id(mut self, parent_unit_id: impl Into<String>) -> Self {
        self.request.parent_unit_id = Some(parent_unit_id.into());
        self
    }

    /// 设置描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 执行创建
    pub async fn execute(self, service: &UnitService) -> SDKResult<CreateUnitResponse> {
        service.create(&self.request).await
    }
}

/// 查询单位列表构建器
#[derive(Debug, Clone)]
pub struct ListUnitsBuilder {
    request: ListUnitsRequest,
}

impl Default for ListUnitsBuilder {
    fn default() -> Self {
        Self {
            request: ListUnitsRequest::default(),
        }
    }
}

impl ListUnitsBuilder {
    /// 创建新的查询构建器
    pub fn new() -> Self {
        Self::default()
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

    /// 设置父单位ID
    pub fn parent_unit_id(mut self, parent_unit_id: impl Into<String>) -> Self {
        self.request.parent_unit_id = Some(parent_unit_id.into());
        self
    }

    /// 执行查询
    pub async fn execute(self, service: &UnitService) -> SDKResult<ListUnitsResponse> {
        service.list(&self.request).await
    }
}

impl UnitService {
    /// 创建单位构建器
    pub fn create_unit_builder(&self) -> CreateUnitBuilder {
        CreateUnitBuilder::new()
    }

    /// 创建查询构建器
    pub fn list_units_builder(&self) -> ListUnitsBuilder {
        ListUnitsBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_service_creation() {
        let config = Config::default();
        let service = UnitService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_unit_default_creation() {
        let unit = Unit::default();
        assert_eq!(unit.unit_id, None);
        assert_eq!(unit.name, None);
        assert_eq!(unit.unit_code, None);
        assert_eq!(unit.parent_unit_id, None);
        assert_eq!(unit.description, None);
        assert_eq!(unit.status, None);
        assert_eq!(unit.create_time, None);
        assert_eq!(unit.update_time, None);
    }

    #[test]
    fn test_unit_with_data() {
        let unit = Unit {
            unit_id: Some("unit_123".to_string()),
            name: Some("研发中心".to_string()),
            unit_code: Some("RD001".to_string()),
            parent_unit_id: Some("unit_root".to_string()),
            description: Some("负责研发工作的中心".to_string()),
            status: Some(1),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(unit.unit_id, Some("unit_123".to_string()));
        assert_eq!(unit.name, Some("研发中心".to_string()));
        assert_eq!(unit.unit_code, Some("RD001".to_string()));
        assert_eq!(unit.parent_unit_id, Some("unit_root".to_string()));
        assert_eq!(unit.description, Some("负责研发工作的中心".to_string()));
        assert_eq!(unit.status, Some(1));
        assert_eq!(unit.create_time, Some("2023-01-01T00:00:00Z".to_string()));
        assert_eq!(unit.update_time, Some("2023-01-02T00:00:00Z".to_string()));
    }

    #[test]
    fn test_create_unit_request() {
        let request = CreateUnitRequest {
            name: "市场部".to_string(),
            unit_code: Some("MKT001".to_string()),
            parent_unit_id: Some("unit_root".to_string()),
            description: Some("负责市场营销工作".to_string()),
        };

        assert_eq!(request.name, "市场部".to_string());
        assert_eq!(request.unit_code, Some("MKT001".to_string()));
        assert_eq!(request.parent_unit_id, Some("unit_root".to_string()));
        assert_eq!(request.description, Some("负责市场营销工作".to_string()));
    }

    #[test]
    fn test_list_units_request_default() {
        let request = ListUnitsRequest::default();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.parent_unit_id, None);
    }

    #[test]
    fn test_list_units_request_with_pagination() {
        let request = ListUnitsRequest {
            page_size: Some(50),
            page_token: Some("token123".to_string()),
            parent_unit_id: Some("parent_unit".to_string()),
        };

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
        assert_eq!(request.parent_unit_id, Some("parent_unit".to_string()));
    }

    #[test]
    fn test_unit_department_default_creation() {
        let dept = UnitDepartment::default();
        assert_eq!(dept.department_id, None);
        assert_eq!(dept.name, None);
        assert_eq!(dept.bind_time, None);
    }

    #[test]
    fn test_unit_department_with_data() {
        let dept = UnitDepartment {
            department_id: Some("dept_456".to_string()),
            name: Some("前端开发组".to_string()),
            bind_time: Some("2023-01-01T00:00:00Z".to_string()),
        };

        assert_eq!(dept.department_id, Some("dept_456".to_string()));
        assert_eq!(dept.name, Some("前端开发组".to_string()));
        assert_eq!(dept.bind_time, Some("2023-01-01T00:00:00Z".to_string()));
    }

    #[test]
    fn test_bind_department_request() {
        let request = BindDepartmentRequest {
            department_ids: vec![
                "dept_1".to_string(),
                "dept_2".to_string(),
                "dept_3".to_string(),
            ],
        };

        assert_eq!(request.department_ids.len(), 3);
        assert_eq!(request.department_ids[0], "dept_1".to_string());
        assert_eq!(request.department_ids[1], "dept_2".to_string());
        assert_eq!(request.department_ids[2], "dept_3".to_string());
    }

    #[test]
    fn test_create_unit_builder() {
        let builder = CreateUnitBuilder::new()
            .name("测试单位")
            .unit_code("TEST001")
            .parent_unit_id("parent")
            .description("测试描述");

        assert_eq!(builder.request.name, "测试单位");
        assert_eq!(builder.request.unit_code, Some("TEST001".to_string()));
        assert_eq!(builder.request.parent_unit_id, Some("parent".to_string()));
        assert_eq!(builder.request.description, Some("测试描述".to_string()));
    }

    #[test]
    fn test_list_units_builder() {
        let builder = ListUnitsBuilder::new()
            .page_size(20)
            .page_token("test_token")
            .parent_unit_id("parent_unit");

        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("test_token".to_string()));
        assert_eq!(
            builder.request.parent_unit_id,
            Some("parent_unit".to_string())
        );
    }

    #[test]
    fn test_response_default_creation() {
        let create_response = CreateUnitResponse::default();
        assert_eq!(create_response.unit.unit_id, None);

        let list_response = ListUnitsResponse::default();
        assert_eq!(list_response.items.len(), 0);
        assert_eq!(list_response.has_more, None);
        assert_eq!(list_response.page_token, None);

        let delete_response = DeleteUnitResponse::default();
        assert_eq!(delete_response.success, None);

        let bind_response = BindDepartmentResponse::default();
        assert_eq!(bind_response.success, None);

        let dept_list_response = ListUnitDepartmentsResponse::default();
        assert_eq!(dept_list_response.items.len(), 0);
    }

    #[test]
    fn test_response_with_data() {
        let mut create_response = CreateUnitResponse::default();
        create_response.unit = Unit {
            unit_id: Some("unit_789".to_string()),
            name: Some("测试单位".to_string()),
            ..Default::default()
        };

        assert_eq!(create_response.unit.unit_id, Some("unit_789".to_string()));
        assert_eq!(create_response.unit.name, Some("测试单位".to_string()));

        let mut list_response = ListUnitsResponse::default();
        list_response.items = vec![Unit {
            unit_id: Some("unit_101".to_string()),
            name: Some("单位1".to_string()),
            ..Default::default()
        }];
        list_response.has_more = Some(true);
        list_response.page_token = Some("next_page".to_string());

        assert_eq!(list_response.items.len(), 1);
        assert_eq!(list_response.items[0].unit_id, Some("unit_101".to_string()));
        assert_eq!(list_response.has_more, Some(true));
        assert_eq!(list_response.page_token, Some("next_page".to_string()));

        let mut bind_response = BindDepartmentResponse::default();
        bind_response.success = Some(true);

        assert_eq!(bind_response.success, Some(true));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(CreateUnitResponse::data_format(), ResponseFormat::Data);
        assert_eq!(UpdateUnitResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetUnitResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListUnitsResponse::data_format(), ResponseFormat::Data);
        assert_eq!(DeleteUnitResponse::data_format(), ResponseFormat::Data);
        assert_eq!(BindDepartmentResponse::data_format(), ResponseFormat::Data);
        assert_eq!(
            UnbindDepartmentResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            ListUnitDepartmentsResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_request_serialization() {
        let request = CreateUnitRequest {
            name: "测试单位".to_string(),
            unit_code: Some("TEST001".to_string()),
            parent_unit_id: Some("parent".to_string()),
            description: Some("测试描述".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateUnitRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.unit_code, deserialized.unit_code);
        assert_eq!(request.parent_unit_id, deserialized.parent_unit_id);
        assert_eq!(request.description, deserialized.description);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = ListUnitsRequest {
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
            parent_unit_id: Some("parent_unit".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(parent_unit_id) = &request.parent_unit_id {
            query_params.push(format!("parent_unit_id={}", parent_unit_id));
        }

        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&"page_size=20".to_string()));
        assert!(query_params.contains(&"page_token=test_token".to_string()));
        assert!(query_params.contains(&"parent_unit_id=parent_unit".to_string()));
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_UNITS,
            "/open-apis/contact/v3/units"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_UNIT_GET,
            "/open-apis/contact/v3/units/{unit_id}"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_UNIT_BIND_DEPARTMENT,
            "/open-apis/contact/v3/units/{unit_id}/bind_department"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_UNIT_UNBIND_DEPARTMENT,
            "/open-apis/contact/v3/units/{unit_id}/unbind_department"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_UNIT_LIST_DEPARTMENT,
            "/open-apis/contact/v3/units/{unit_id}/list_department"
        );
    }

    #[test]
    fn test_complex_unit_hierarchy() {
        // Test complex unit hierarchy relationships
        let root_unit = Unit {
            unit_id: Some("root".to_string()),
            name: Some("集团总部".to_string()),
            ..Default::default()
        };

        let child_unit = Unit {
            unit_id: Some("child".to_string()),
            name: Some("技术中心".to_string()),
            parent_unit_id: Some("root".to_string()),
            ..Default::default()
        };

        assert_eq!(root_unit.unit_id, Some("root".to_string()));
        assert_eq!(root_unit.name, Some("集团总部".to_string()));
        assert_eq!(root_unit.parent_unit_id, None);

        assert_eq!(child_unit.unit_id, Some("child".to_string()));
        assert_eq!(child_unit.name, Some("技术中心".to_string()));
        assert_eq!(child_unit.parent_unit_id, Some("root".to_string()));
    }
}

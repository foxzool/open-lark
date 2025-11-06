#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 人员类型枚举服务
//!
//! 提供完整的人员类型枚举管理功能：
//! - 创建人员类型
//! - 更新人员类型
//! - 获取单个人员类型信息
//! - 获取人员类型列表
//! - 删除人员类型
//! - 支持分页查询

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 人员类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeTypeEnum {
    /// 人员类型ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_id: Option<String>,
    /// 人员类型名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 人员类型描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for EmployeeTypeEnum {
    fn default() -> Self {
        Self {
            enum_id: None,
            name: None,
            description: None,
            order: None,
            enabled: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 人员类型枚举服务
#[derive(Debug, Clone)]
pub struct EmployeeTypeEnumService {
    config: Config,
}

impl EmployeeTypeEnumService {
    /// 创建新的人员类型枚举服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建人员类型
    ///
    /// 创建新的人员类型枚举
    ///
    /// # 参数
    /// * `req` - 创建人员类型请求
    ///
    /// # 返回值
    /// 返回创建的人员类型信息
    pub async fn create(
        &self,
        req: &CreateEmployeeTypeRequest,
    ) -> SDKResult<CreateEmployeeTypeResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::CONTACT_V3_EMPLOYEE_TYPE_ENUMS
                .to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateEmployeeTypeResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新人员类型
    ///
    /// 更新指定人员类型的信息
    ///
    /// # 参数
    /// * `enum_id` - 人员类型ID
    /// * `req` - 更新人员类型请求
    ///
    /// # 返回值
    /// 返回更新后的人员类型信息
    pub async fn update(
        &self,
        enum_id: &str,
        req: &UpdateEmployeeTypeRequest,
    ) -> SDKResult<UpdateEmployeeTypeResponse> {
        let api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_EMPLOYEE_TYPE_ENUM_GET
                .replace("{enum_id}", enum_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateEmployeeTypeResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个人员类型信息
    ///
    /// 根据人员类型ID获取人员类型的详细信息
    ///
    /// # 参数
    /// * `enum_id` - 人员类型ID
    ///
    /// # 返回值
    /// 返回人员类型的详细信息
    pub async fn get(&self, enum_id: &str) -> SDKResult<GetEmployeeTypeResponse> {
        let api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_EMPLOYEE_TYPE_ENUM_GET
                .replace("{enum_id}", enum_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<GetEmployeeTypeResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取人员类型列表
    ///
    /// 获取企业内所有人员类型的列表，支持分页查询
    ///
    /// # 参数
    /// * `req` - 查询人员类型列表请求
    ///
    /// # 返回值
    /// 返回人员类型列表，支持分页
    pub async fn list(
        &self,
        req: &ListEmployeeTypesRequest,
    ) -> SDKResult<ListEmployeeTypesResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_EMPLOYEE_TYPE_ENUMS.to_string();

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
            Transport::<ListEmployeeTypesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除人员类型
    ///
    /// 删除指定的人员类型
    ///
    /// # 参数
    /// * `enum_id` - 人员类型ID
    ///
    /// # 返回值
    /// 返回删除操作的结果
    pub async fn delete(&self, enum_id: &str) -> SDKResult<DeleteEmployeeTypeResponse> {
        let api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_EMPLOYEE_TYPE_ENUM_GET
                .replace("{enum_id}", enum_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<DeleteEmployeeTypeResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 创建人员类型请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployeeTypeRequest {
    /// 人员类型名称
    pub name: String,
    /// 人员类型描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 创建人员类型响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateEmployeeTypeResponse {
    /// 人员类型信息
    pub employee_type_enum: EmployeeTypeEnum,
}

impl ApiResponseTrait for CreateEmployeeTypeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新人员类型请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmployeeTypeRequest {
    /// 人员类型名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 人员类型描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// 更新人员类型响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateEmployeeTypeResponse {
    /// 人员类型信息
    pub employee_type_enum: EmployeeTypeEnum,
}

impl ApiResponseTrait for UpdateEmployeeTypeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取单个人员类型信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetEmployeeTypeResponse {
    /// 人员类型信息
    pub employee_type_enum: EmployeeTypeEnum,
}

impl ApiResponseTrait for GetEmployeeTypeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询人员类型列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEmployeeTypesRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for ListEmployeeTypesRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }
}

/// 查询人员类型列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListEmployeeTypesResponse {
    /// 人员类型列表
    pub items: Vec<EmployeeTypeEnum>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListEmployeeTypesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除人员类型响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteEmployeeTypeResponse {
    /// 是否成功删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteEmployeeTypeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建人员类型构建器
#[derive(Debug, Clone)]
pub struct CreateEmployeeTypeBuilder {
    request: CreateEmployeeTypeRequest,
}

impl Default for CreateEmployeeTypeBuilder {
    fn default() -> Self {
        Self {
            request: CreateEmployeeTypeRequest {
                name: String::new(),
                description: None,
                order: None,
            },
        }
    }
}

impl CreateEmployeeTypeBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置人员类型名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    /// 设置人员类型描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置排序
    pub fn order(mut self, order: i32) -> Self {
        self.request.order = Some(order);
        self
    }

    /// 执行创建
    pub async fn execute(
        self,
        service: &EmployeeTypeEnumService,
    ) -> SDKResult<CreateEmployeeTypeResponse> {
        service.create(&self.request).await
    }
}

/// 查询人员类型列表构建器
#[derive(Debug, Clone)]
pub struct ListEmployeeTypesBuilder {
    request: ListEmployeeTypesRequest,
}

impl Default for ListEmployeeTypesBuilder {
    fn default() -> Self {
        Self {
            request: ListEmployeeTypesRequest::default(),
        }
    }
}

impl ListEmployeeTypesBuilder {
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

    /// 执行查询
    pub async fn execute(
        self,
        service: &EmployeeTypeEnumService,
    ) -> SDKResult<ListEmployeeTypesResponse> {
        service.list(&self.request).await
    }
}

impl EmployeeTypeEnumService {
    /// 创建人员类型构建器
    pub fn create_employee_type_builder(&self) -> CreateEmployeeTypeBuilder {
        CreateEmployeeTypeBuilder::new()
    }

    /// 创建查询构建器
    pub fn list_employee_types_builder(&self) -> ListEmployeeTypesBuilder {
        ListEmployeeTypesBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employee_type_enum_service_creation() {
        let config = Config::default();
        let service = EmployeeTypeEnumService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_employee_type_enum_default_creation() {
        let employee_type = EmployeeTypeEnum::default();
        assert_eq!(employee_type.enum_id, None);
        assert_eq!(employee_type.name, None);
        assert_eq!(employee_type.description, None);
        assert_eq!(employee_type.order, None);
        assert_eq!(employee_type.enabled, None);
        assert_eq!(employee_type.create_time, None);
        assert_eq!(employee_type.update_time, None);
    }

    #[test]
    fn test_employee_type_enum_with_data() {
        let employee_type = EmployeeTypeEnum {
            enum_id: Some("enum_123".to_string()),
            name: Some("正式员工".to_string()),
            description: Some("正式员工类型".to_string()),
            order: Some(1),
            enabled: Some(true),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(employee_type.enum_id, Some("enum_123".to_string()));
        assert_eq!(employee_type.name, Some("正式员工".to_string()));
        assert_eq!(employee_type.description, Some("正式员工类型".to_string()));
        assert_eq!(employee_type.order, Some(1));
        assert_eq!(employee_type.enabled, Some(true));
        assert_eq!(
            employee_type.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            employee_type.update_time,
            Some("2023-01-02T00:00:00Z".to_string())
        );
    }

    #[test]
    fn test_create_employee_type_request() {
        let request = CreateEmployeeTypeRequest {
            name: "实习生".to_string(),
            description: Some("实习生类型".to_string()),
            order: Some(2),
        };

        assert_eq!(request.name, "实习生".to_string());
        assert_eq!(request.description, Some("实习生类型".to_string()));
        assert_eq!(request.order, Some(2));
    }

    #[test]
    fn test_update_employee_type_request() {
        let request = UpdateEmployeeTypeRequest {
            name: Some("外包人员".to_string()),
            description: Some("外包人员类型".to_string()),
            order: Some(3),
            enabled: Some(false),
        };

        assert_eq!(request.name, Some("外包人员".to_string()));
        assert_eq!(request.description, Some("外包人员类型".to_string()));
        assert_eq!(request.order, Some(3));
        assert_eq!(request.enabled, Some(false));
    }

    #[test]
    fn test_list_employee_types_request_default() {
        let request = ListEmployeeTypesRequest::default();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_list_employee_types_request_with_pagination() {
        let request = ListEmployeeTypesRequest {
            page_size: Some(50),
            page_token: Some("token123".to_string()),
        };

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_create_employee_type_builder() {
        let builder = CreateEmployeeTypeBuilder::new()
            .name("测试类型")
            .description("测试描述")
            .order(1);

        assert_eq!(builder.request.name, "测试类型");
        assert_eq!(builder.request.description, Some("测试描述".to_string()));
        assert_eq!(builder.request.order, Some(1));
    }

    #[test]
    fn test_list_employee_types_builder() {
        let builder = ListEmployeeTypesBuilder::new()
            .page_size(20)
            .page_token("test_token");

        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("test_token".to_string()));
    }

    #[test]
    fn test_response_default_creation() {
        let create_response = CreateEmployeeTypeResponse::default();
        assert_eq!(create_response.employee_type_enum.enum_id, None);

        let list_response = ListEmployeeTypesResponse::default();
        assert_eq!(list_response.items.len(), 0);
        assert_eq!(list_response.has_more, None);
        assert_eq!(list_response.page_token, None);

        let delete_response = DeleteEmployeeTypeResponse::default();
        assert_eq!(delete_response.success, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut create_response = CreateEmployeeTypeResponse::default();
        create_response.employee_type_enum = EmployeeTypeEnum {
            enum_id: Some("enum_456".to_string()),
            name: Some("兼职人员".to_string()),
            ..Default::default()
        };

        assert_eq!(
            create_response.employee_type_enum.enum_id,
            Some("enum_456".to_string())
        );
        assert_eq!(
            create_response.employee_type_enum.name,
            Some("兼职人员".to_string())
        );

        let mut list_response = ListEmployeeTypesResponse::default();
        list_response.items = vec![EmployeeTypeEnum {
            enum_id: Some("enum_789".to_string()),
            name: Some("顾问".to_string()),
            ..Default::default()
        }];
        list_response.has_more = Some(true);
        list_response.page_token = Some("next_page".to_string());

        assert_eq!(list_response.items.len(), 1);
        assert_eq!(list_response.items[0].enum_id, Some("enum_789".to_string()));
        assert_eq!(list_response.items[0].name, Some("顾问".to_string()));
        assert_eq!(list_response.has_more, Some(true));
        assert_eq!(list_response.page_token, Some("next_page".to_string()));

        let mut delete_response = DeleteEmployeeTypeResponse::default();
        delete_response.success = Some(true);

        assert_eq!(delete_response.success, Some(true));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            CreateEmployeeTypeResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            UpdateEmployeeTypeResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(GetEmployeeTypeResponse::data_format(), ResponseFormat::Data);
        assert_eq!(
            ListEmployeeTypesResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            DeleteEmployeeTypeResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_request_serialization() {
        let request = CreateEmployeeTypeRequest {
            name: "测试人员类型".to_string(),
            description: Some("测试描述".to_string()),
            order: Some(1),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateEmployeeTypeRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.description, deserialized.description);
        assert_eq!(request.order, deserialized.order);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = ListEmployeeTypesRequest {
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        assert_eq!(query_params.len(), 2);
        assert!(query_params.contains(&"page_size=20".to_string()));
        assert!(query_params.contains(&"page_token=test_token".to_string()));
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_EMPLOYEE_TYPE_ENUMS,
            "/open-apis/contact/v3/employee_type_enums"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_EMPLOYEE_TYPE_ENUM_GET,
            "/open-apis/contact/v3/employee_type_enums/{enum_id}"
        );
    }

    #[test]
    fn test_employee_type_enabled_status() {
        // Test different enabled status values
        let enabled_type = EmployeeTypeEnum {
            enum_id: Some("enabled_001".to_string()),
            name: Some("启用类型".to_string()),
            enabled: Some(true),
            ..Default::default()
        };

        let disabled_type = EmployeeTypeEnum {
            enum_id: Some("disabled_001".to_string()),
            name: Some("禁用类型".to_string()),
            enabled: Some(false),
            ..Default::default()
        };

        let no_status_type = EmployeeTypeEnum {
            enum_id: Some("no_status_001".to_string()),
            name: Some("无状态类型".to_string()),
            enabled: None,
            ..Default::default()
        };

        assert_eq!(enabled_type.enabled, Some(true));
        assert_eq!(disabled_type.enabled, Some(false));
        assert_eq!(no_status_type.enabled, None);
    }

    #[test]
    fn test_employee_type_ordering() {
        // Test type ordering
        let first_type = EmployeeTypeEnum {
            enum_id: Some("order_1".to_string()),
            name: Some("第一类型".to_string()),
            order: Some(1),
            ..Default::default()
        };

        let second_type = EmployeeTypeEnum {
            enum_id: Some("order_2".to_string()),
            name: Some("第二类型".to_string()),
            order: Some(2),
            ..Default::default()
        };

        let unordered_type = EmployeeTypeEnum {
            enum_id: Some("order_none".to_string()),
            name: Some("无序类型".to_string()),
            order: None,
            ..Default::default()
        };

        assert_eq!(first_type.order, Some(1));
        assert_eq!(second_type.order, Some(2));
        assert_eq!(unordered_type.order, None);
    }

    #[test]
    fn test_comprehensive_employee_type_data() {
        // Test comprehensive employee type data with all fields
        let comprehensive_type = EmployeeTypeEnum {
            enum_id: Some("comprehensive_001".to_string()),
            name: Some("高级工程师".to_string()),
            description: Some("高级工程师级别人员".to_string()),
            order: Some(5),
            enabled: Some(true),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
        };

        assert_eq!(
            comprehensive_type.enum_id,
            Some("comprehensive_001".to_string())
        );
        assert_eq!(comprehensive_type.name, Some("高级工程师".to_string()));
        assert_eq!(
            comprehensive_type.description,
            Some("高级工程师级别人员".to_string())
        );
        assert_eq!(comprehensive_type.order, Some(5));
        assert_eq!(comprehensive_type.enabled, Some(true));
        assert_eq!(
            comprehensive_type.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_type.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
    }

    #[test]
    fn test_employee_type_name_variations() {
        // Test different employee type names
        let formal_type = EmployeeTypeEnum {
            enum_id: Some("formal".to_string()),
            name: Some("正式员工".to_string()),
            ..Default::default()
        };

        let intern_type = EmployeeTypeEnum {
            enum_id: Some("intern".to_string()),
            name: Some("实习生".to_string()),
            ..Default::default()
        };

        let contractor_type = EmployeeTypeEnum {
            enum_id: Some("contractor".to_string()),
            name: Some("外包人员".to_string()),
            ..Default::default()
        };

        let consultant_type = EmployeeTypeEnum {
            enum_id: Some("consultant".to_string()),
            name: Some("顾问".to_string()),
            ..Default::default()
        };

        assert_eq!(formal_type.name, Some("正式员工".to_string()));
        assert_eq!(intern_type.name, Some("实习生".to_string()));
        assert_eq!(contractor_type.name, Some("外包人员".to_string()));
        assert_eq!(consultant_type.name, Some("顾问".to_string()));
    }
}

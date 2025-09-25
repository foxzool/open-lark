use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
    },
    service::contact::models::*,
};
use serde::{Deserialize, Serialize};

/// 部门管理服务
///
/// 提供完整的部门管理功能，包括：
/// - 创建、修改、删除部门
/// - 获取部门信息（单个/批量）
/// - 获取子部门列表
/// - 获取父部门信息
/// - 搜索部门
pub struct DepartmentService {
    config: Config,
}

impl DepartmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建部门
    pub async fn create(
        &self,
        req: &CreateDepartmentRequest,
    ) -> crate::core::SDKResult<CreateDepartmentResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints::contact::CONTACT_V3_DEPARTMENTS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 修改部门部分信息
    pub async fn patch(
        &self,
        department_id: &str,
        req: &PatchDepartmentRequest,
    ) -> crate::core::SDKResult<PatchDepartmentResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_DEPARTMENT_GET,
                "department_id",
                department_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<PatchDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新部门所有信息
    pub async fn update(
        &self,
        department_id: &str,
        req: &UpdateDepartmentRequest,
    ) -> crate::core::SDKResult<UpdateDepartmentResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_DEPARTMENT_GET,
                "department_id",
                department_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新部门 ID
    pub async fn update_department_id(
        &self,
        department_id: &str,
        req: &UpdateDepartmentIdRequest,
    ) -> crate::core::SDKResult<UpdateDepartmentIdResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_DEPARTMENT_UPDATE_ID,
                "department_id",
                department_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateDepartmentIdResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个部门信息
    pub async fn get(
        &self,
        department_id: &str,
        _req: &GetDepartmentRequest,
    ) -> crate::core::SDKResult<GetDepartmentResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_DEPARTMENT_GET,
                "department_id",
                department_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<GetDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量获取部门信息
    pub async fn batch(
        &self,
        req: &BatchGetDepartmentsRequest,
    ) -> crate::core::SDKResult<BatchGetDepartmentsResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints::contact::CONTACT_V3_DEPARTMENTS_BATCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchGetDepartmentsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取子部门列表
    pub async fn children(
        &self,
        _req: &GetChildrenDepartmentsRequest,
    ) -> crate::core::SDKResult<GetChildrenDepartmentsResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: crate::core::endpoints::contact::CONTACT_V3_DEPARTMENTS_CHILDREN.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp =
            Transport::<GetChildrenDepartmentsResponse>::request(api_req, &self.config, None)
                .await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取父部门信息
    pub async fn parent(
        &self,
        _req: &GetParentDepartmentRequest,
    ) -> crate::core::SDKResult<GetParentDepartmentResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: crate::core::endpoints::contact::CONTACT_V3_DEPARTMENTS_PARENT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp =
            Transport::<GetParentDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 搜索部门
    pub async fn search(
        &self,
        req: &SearchDepartmentsRequest,
    ) -> crate::core::SDKResult<SearchDepartmentsResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints::contact::CONTACT_V3_DEPARTMENTS_SEARCH.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<SearchDepartmentsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除部门
    pub async fn delete(
        &self,
        department_id: &str,
        _req: &DeleteDepartmentRequest,
    ) -> crate::core::SDKResult<DeleteDepartmentResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_DEPARTMENT_GET,
                "department_id",
                department_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp =
            Transport::<DeleteDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// 请求/响应结构体定义

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
    /// 客户端令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

/// 创建部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

impl ApiResponseTrait for CreateDepartmentResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
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

/// 修改部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

impl ApiResponseTrait for PatchDepartmentResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
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

/// 更新部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

impl ApiResponseTrait for UpdateDepartmentResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
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

/// 更新部门ID响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateDepartmentIdResponse {}

impl ApiResponseTrait for UpdateDepartmentIdResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 获取部门请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDepartmentRequest {
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 获取部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDepartmentResponse {
    /// 部门信息
    pub department: Department,
}

impl ApiResponseTrait for GetDepartmentResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
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

/// 批量获取部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchGetDepartmentsResponse {
    /// 部门列表
    pub items: Vec<Department>,
}

impl ApiResponseTrait for BatchGetDepartmentsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 获取子部门列表请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

/// 获取子部门列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetChildrenDepartmentsResponse {
    /// 部门列表
    pub items: Vec<Department>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetChildrenDepartmentsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 获取父部门请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

/// 获取父部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetParentDepartmentResponse {
    /// 部门列表
    pub items: Vec<Department>,
}

impl ApiResponseTrait for GetParentDepartmentResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
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

/// 搜索部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchDepartmentsResponse {
    /// 部门列表
    pub items: Vec<Department>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchDepartmentsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 删除部门请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteDepartmentRequest {
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 删除部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteDepartmentResponse {}

impl ApiResponseTrait for DeleteDepartmentResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use crate::service::contact::models::Department;

    #[test]
    fn test_department_service_creation() {
        let config = Config::default();
        let service = DepartmentService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_department_service_with_custom_config() {
        let config = Config::builder()
            .app_id("dept_test_app")
            .app_secret("dept_test_secret")
            .build();

        let service = DepartmentService::new(config.clone());

        assert_eq!(service.config.app_id, "dept_test_app");
        assert_eq!(service.config.app_secret, "dept_test_secret");
    }

    #[test]
    fn test_create_department_request_construction() {
        let department = Department {
            department_id: Some("dept_123".to_string()),
            name: Some("Engineering".to_string()),
            ..Default::default()
        };

        let request = CreateDepartmentRequest {
            department,
            user_id_type: Some("user_id".to_string()),
            department_id_type: Some("open_id".to_string()),
            client_token: Some("token_123".to_string()),
        };

        assert_eq!(
            request.department.department_id,
            Some("dept_123".to_string())
        );
        assert_eq!(request.department.name, Some("Engineering".to_string()));
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.department_id_type, Some("open_id".to_string()));
        assert_eq!(request.client_token, Some("token_123".to_string()));
    }

    #[test]
    fn test_create_department_request_with_none_values() {
        let department = Department {
            name: Some("HR".to_string()),
            ..Default::default()
        };

        let request = CreateDepartmentRequest {
            department,
            user_id_type: None,
            department_id_type: None,
            client_token: None,
        };

        assert_eq!(request.department.name, Some("HR".to_string()));
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
        assert_eq!(request.client_token, None);
    }

    #[test]
    fn test_patch_department_request_construction() {
        let department = Department {
            department_id: Some("dept_456".to_string()),
            name: Some("Marketing".to_string()),
            ..Default::default()
        };

        let request = PatchDepartmentRequest {
            department,
            user_id_type: Some("union_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        assert_eq!(
            request.department.department_id,
            Some("dept_456".to_string())
        );
        assert_eq!(request.department.name, Some("Marketing".to_string()));
        assert_eq!(request.user_id_type, Some("union_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_update_department_request_construction() {
        let department = Department {
            department_id: Some("dept_789".to_string()),
            name: Some("Sales".to_string()),
            ..Default::default()
        };

        let request = UpdateDepartmentRequest {
            department,
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("open_id".to_string()),
        };

        assert_eq!(
            request.department.department_id,
            Some("dept_789".to_string())
        );
        assert_eq!(request.department.name, Some("Sales".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.department_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_update_department_id_request_construction() {
        let request = UpdateDepartmentIdRequest {
            new_department_id: "new_dept_id_123".to_string(),
            department_id_type: Some("open_id".to_string()),
        };

        assert_eq!(request.new_department_id, "new_dept_id_123");
        assert_eq!(request.department_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_update_department_id_request_with_none_type() {
        let request = UpdateDepartmentIdRequest {
            new_department_id: "new_dept_id_456".to_string(),
            department_id_type: None,
        };

        assert_eq!(request.new_department_id, "new_dept_id_456");
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_get_department_request_construction() {
        let request = GetDepartmentRequest {
            user_id_type: Some("user_id".to_string()),
            department_id_type: Some("open_id".to_string()),
        };

        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.department_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_department_request_default() {
        let request = GetDepartmentRequest::default();

        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_batch_get_departments_request_construction() {
        let request = BatchGetDepartmentsRequest {
            department_ids: vec![
                "dept_1".to_string(),
                "dept_2".to_string(),
                "dept_3".to_string(),
            ],
            user_id_type: Some("union_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        assert_eq!(request.department_ids.len(), 3);
        assert_eq!(request.department_ids[0], "dept_1");
        assert_eq!(request.department_ids[2], "dept_3");
        assert_eq!(request.user_id_type, Some("union_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_batch_get_departments_request_with_empty_list() {
        let request = BatchGetDepartmentsRequest {
            department_ids: vec![],
            user_id_type: None,
            department_id_type: None,
        };

        assert!(request.department_ids.is_empty());
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_get_children_departments_request_construction() {
        let request = GetChildrenDepartmentsRequest {
            parent_department_id: Some("parent_dept_123".to_string()),
            user_id_type: Some("user_id".to_string()),
            department_id_type: Some("open_id".to_string()),
            fetch_child: Some(true),
            page_size: Some(50),
            page_token: Some("page_token_123".to_string()),
        };

        assert_eq!(
            request.parent_department_id,
            Some("parent_dept_123".to_string())
        );
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.department_id_type, Some("open_id".to_string()));
        assert_eq!(request.fetch_child, Some(true));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("page_token_123".to_string()));
    }

    #[test]
    fn test_get_children_departments_request_default() {
        let request = GetChildrenDepartmentsRequest::default();

        assert_eq!(request.parent_department_id, None);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
        assert_eq!(request.fetch_child, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_get_parent_department_request_construction() {
        let request = GetParentDepartmentRequest {
            department_id: Some("dept_456".to_string()),
            user_id_type: Some("union_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        assert_eq!(request.department_id, Some("dept_456".to_string()));
        assert_eq!(request.user_id_type, Some("union_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_get_parent_department_request_default() {
        let request = GetParentDepartmentRequest::default();

        assert_eq!(request.department_id, None);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_search_departments_request_construction() {
        let request = SearchDepartmentsRequest {
            query: "Engineering".to_string(),
            page_size: Some(20),
            page_token: Some("search_token_123".to_string()),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("open_id".to_string()),
        };

        assert_eq!(request.query, "Engineering");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("search_token_123".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.department_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_search_departments_request_with_minimal_data() {
        let request = SearchDepartmentsRequest {
            query: "HR".to_string(),
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        };

        assert_eq!(request.query, "HR");
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_delete_department_request_construction() {
        let request = DeleteDepartmentRequest {
            department_id_type: Some("department_id".to_string()),
        };

        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_delete_department_request_default() {
        let request = DeleteDepartmentRequest::default();

        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_create_department_response_default() {
        let response = CreateDepartmentResponse::default();

        assert_eq!(response.department.name, None);
        assert_eq!(response.department.department_id, None);
    }

    #[test]
    fn test_patch_department_response_default() {
        let response = PatchDepartmentResponse::default();

        assert_eq!(response.department.name, None);
        assert_eq!(response.department.department_id, None);
    }

    #[test]
    fn test_update_department_response_default() {
        let response = UpdateDepartmentResponse::default();

        assert_eq!(response.department.name, None);
        assert_eq!(response.department.department_id, None);
    }

    #[test]
    fn test_update_department_id_response_default() {
        let _response = UpdateDepartmentIdResponse::default();
        // UpdateDepartmentIdResponse is an empty struct, just test it can be created
    }

    #[test]
    fn test_get_department_response_default() {
        let response = GetDepartmentResponse::default();

        assert_eq!(response.department.name, None);
        assert_eq!(response.department.department_id, None);
    }

    #[test]
    fn test_batch_get_departments_response_default() {
        let response = BatchGetDepartmentsResponse::default();

        assert!(response.items.is_empty());
    }

    #[test]
    fn test_get_children_departments_response_default() {
        let response = GetChildrenDepartmentsResponse::default();

        assert!(response.items.is_empty());
        assert_eq!(response.has_more, None);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_get_parent_department_response_default() {
        let response = GetParentDepartmentResponse::default();

        assert!(response.items.is_empty());
    }

    #[test]
    fn test_search_departments_response_default() {
        let response = SearchDepartmentsResponse::default();

        assert!(response.items.is_empty());
        assert_eq!(response.has_more, None);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_delete_department_response_default() {
        let _response = DeleteDepartmentResponse::default();
        // DeleteDepartmentResponse is an empty struct, just test it can be created
    }

    #[test]
    fn test_request_structs_debug_trait() {
        let department = Department {
            name: Some("Debug Test Dept".to_string()),
            ..Default::default()
        };

        let create_request = CreateDepartmentRequest {
            department: department.clone(),
            user_id_type: None,
            department_id_type: None,
            client_token: None,
        };

        let debug_str = format!("{:?}", create_request);
        assert!(debug_str.contains("CreateDepartmentRequest"));
        assert!(debug_str.contains("Debug Test Dept"));
    }

    #[test]
    fn test_search_departments_request_edge_cases() {
        // Test with very long query string
        let long_query = "a".repeat(1000);
        let request_long = SearchDepartmentsRequest {
            query: long_query.clone(),
            page_size: Some(100),
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        };

        assert_eq!(request_long.query, long_query);
        assert_eq!(request_long.page_size, Some(100));

        // Test with zero page size
        let request_zero = SearchDepartmentsRequest {
            query: "Test".to_string(),
            page_size: Some(0),
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        };

        assert_eq!(request_zero.page_size, Some(0));

        // Test with empty query
        let request_empty = SearchDepartmentsRequest {
            query: "".to_string(),
            page_size: Some(10),
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        };

        assert_eq!(request_empty.query, "");
    }

    #[test]
    fn test_batch_get_departments_request_edge_cases() {
        // Test with very large department ID list
        let large_list: Vec<String> = (0..1000).map(|i| format!("dept_{}", i)).collect();
        let request_large = BatchGetDepartmentsRequest {
            department_ids: large_list.clone(),
            user_id_type: None,
            department_id_type: None,
        };

        assert_eq!(request_large.department_ids.len(), 1000);
        assert_eq!(request_large.department_ids[999], "dept_999");

        // Test with single department ID
        let request_single = BatchGetDepartmentsRequest {
            department_ids: vec!["single_dept".to_string()],
            user_id_type: None,
            department_id_type: None,
        };

        assert_eq!(request_single.department_ids.len(), 1);
        assert_eq!(request_single.department_ids[0], "single_dept");
    }

    #[test]
    fn test_get_children_departments_request_edge_cases() {
        // Test with very large page size
        let request_large_page = GetChildrenDepartmentsRequest {
            parent_department_id: Some("parent_123".to_string()),
            user_id_type: None,
            department_id_type: None,
            fetch_child: Some(true),
            page_size: Some(10000),
            page_token: None,
        };

        assert_eq!(request_large_page.page_size, Some(10000));
        assert_eq!(request_large_page.fetch_child, Some(true));

        // Test with fetch_child set to false
        let request_no_fetch = GetChildrenDepartmentsRequest {
            parent_department_id: None,
            user_id_type: None,
            department_id_type: None,
            fetch_child: Some(false),
            page_size: Some(20),
            page_token: None,
        };

        assert_eq!(request_no_fetch.fetch_child, Some(false));
        assert_eq!(request_no_fetch.parent_department_id, None);
    }

    #[test]
    fn test_department_service_config_independence() {
        let config1 = Config::builder().app_id("dept_app_1").build();

        let config2 = Config::builder().app_id("dept_app_2").build();

        let service1 = DepartmentService::new(config1);
        let service2 = DepartmentService::new(config2);

        assert_eq!(service1.config.app_id, "dept_app_1");
        assert_eq!(service2.config.app_id, "dept_app_2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_api_response_trait_implementations() {
        // Test that all response types implement ApiResponseTrait correctly
        assert_eq!(
            CreateDepartmentResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
        assert_eq!(
            PatchDepartmentResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
        assert_eq!(
            UpdateDepartmentResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
        assert_eq!(
            UpdateDepartmentIdResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
        assert_eq!(
            GetDepartmentResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
        assert_eq!(
            BatchGetDepartmentsResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
        assert_eq!(
            GetChildrenDepartmentsResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
        assert_eq!(
            GetParentDepartmentResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
        assert_eq!(
            SearchDepartmentsResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
        assert_eq!(
            DeleteDepartmentResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
    }
}

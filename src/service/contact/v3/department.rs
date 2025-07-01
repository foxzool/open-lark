use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, http::Transport,
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
            api_path: "/open-apis/contact/v3/departments".to_string(),
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
            api_path: format!("/open-apis/contact/v3/departments/{department_id}"),
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
            api_path: format!("/open-apis/contact/v3/departments/{department_id}"),
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
            api_path: format!(
                "/open-apis/contact/v3/departments/{department_id}/update_department_id"
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
            api_path: format!("/open-apis/contact/v3/departments/{department_id}"),
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
            api_path: "/open-apis/contact/v3/departments/batch".to_string(),
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
            api_path: "/open-apis/contact/v3/departments/children".to_string(),
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
            api_path: "/open-apis/contact/v3/departments/parent".to_string(),
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
            api_path: "/open-apis/contact/v3/departments/search".to_string(),
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
            api_path: format!("/open-apis/contact/v3/departments/{department_id}"),
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

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 职族管理服务
//!
//! 提供完整的职位族管理功能：
//! - 创建职位族
//! - 更新职位族
//! - 获取单个职位族信息
//! - 获取租户职位族列表
//! - 删除职位族
//! - 支持分页查询

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 职位族信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobFamily {
    /// 职位族ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 职位族名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职位族描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for JobFamily {
    fn default() -> Self {
        Self {
            job_family_id: None,
            name: None,
            description: None,
            order: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 职位族服务
#[derive(Debug, Clone)]
pub struct JobFamilyService {
    config: Config,
}

impl JobFamilyService {
    /// 创建新的职位族服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建职位族
    ///
    /// 在企业中创建一个新的职位族
    ///
    /// # 参数
    /// * `req` - 创建职位族请求
    ///
    /// # 返回值
    /// 返回创建的职位族信息
    pub async fn create(&self, req: &CreateJobFamilyRequest) -> SDKResult<CreateJobFamilyResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_FAMILIES
                .to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateJobFamilyResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新职位族
    ///
    /// 更新指定职位族的信息
    ///
    /// # 参数
    /// * `job_family_id` - 职位族ID
    /// * `req` - 更新职位族请求
    ///
    /// # 返回值
    /// 返回更新后的职位族信息
    pub async fn update(
        &self,
        job_family_id: &str,
        req: &UpdateJobFamilyRequest,
    ) -> SDKResult<UpdateJobFamilyResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_FAMILY_GET
            .replace("{job_family_id}", job_family_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateJobFamilyResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个职位族信息
    ///
    /// 根据职位族ID获取职位族的详细信息
    ///
    /// # 参数
    /// * `job_family_id` - 职位族ID
    ///
    /// # 返回值
    /// 返回职位族的详细信息
    pub async fn get(&self, job_family_id: &str) -> SDKResult<GetJobFamilyResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_FAMILY_GET
            .replace("{job_family_id}", job_family_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetJobFamilyResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取租户职位族列表
    ///
    /// 获取企业内所有职位族的列表，支持分页查询
    ///
    /// # 参数
    /// * `req` - 查询职位族列表请求
    ///
    /// # 返回值
    /// 返回职位族列表，支持分页
    pub async fn list(&self, req: &ListJobFamiliesRequest) -> SDKResult<ListJobFamiliesResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_FAMILIES.to_string();

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
            Transport::<ListJobFamiliesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除职位族
    ///
    /// 删除指定的职位族
    ///
    /// # 参数
    /// * `job_family_id` - 职位族ID
    ///
    /// # 返回值
    /// 返回删除操作的结果
    pub async fn delete(&self, job_family_id: &str) -> SDKResult<DeleteJobFamilyResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_FAMILY_GET
            .replace("{job_family_id}", job_family_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<DeleteJobFamilyResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 创建职位族请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJobFamilyRequest {
    /// 职位族名称
    pub name: String,
    /// 职位族描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 创建职位族响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateJobFamilyResponse {
    /// 职位族信息
    pub job_family: JobFamily,
}

impl ApiResponseTrait for CreateJobFamilyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新职位族请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateJobFamilyRequest {
    /// 职位族名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职位族描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 更新职位族响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateJobFamilyResponse {
    /// 职位族信息
    pub job_family: JobFamily,
}

impl ApiResponseTrait for UpdateJobFamilyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取单个职位族信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetJobFamilyResponse {
    /// 职位族信息
    pub job_family: JobFamily,
}

impl ApiResponseTrait for GetJobFamilyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询职位族列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListJobFamiliesRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for ListJobFamiliesRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }
}

/// 查询职位族列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListJobFamiliesResponse {
    /// 职位族列表
    pub items: Vec<JobFamily>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListJobFamiliesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除职位族响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteJobFamilyResponse {
    /// 是否成功删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteJobFamilyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建职位族构建器
#[derive(Debug, Clone)]
pub struct CreateJobFamilyBuilder {
    request: CreateJobFamilyRequest,
}

impl Default for CreateJobFamilyBuilder {
    fn default() -> Self {
        Self {
            request: CreateJobFamilyRequest {
                name: String::new(),
                description: None,
                order: None,
            },
        }
    }
}

impl CreateJobFamilyBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置职位族名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    /// 设置职位族描述
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
    pub async fn execute(self, service: &JobFamilyService) -> SDKResult<CreateJobFamilyResponse> {
        service.create(&self.request).await
    }
}

/// 查询职位族列表构建器
#[derive(Debug, Clone)]
pub struct ListJobFamiliesBuilder {
    request: ListJobFamiliesRequest,
}

impl Default for ListJobFamiliesBuilder {
    fn default() -> Self {
        Self {
            request: ListJobFamiliesRequest::default(),
        }
    }
}

impl ListJobFamiliesBuilder {
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
    pub async fn execute(self, service: &JobFamilyService) -> SDKResult<ListJobFamiliesResponse> {
        service.list(&self.request).await
    }
}

impl JobFamilyService {
    /// 创建职位族构建器
    pub fn create_job_family_builder(&self) -> CreateJobFamilyBuilder {
        CreateJobFamilyBuilder::new()
    }

    /// 创建查询构建器
    pub fn list_job_families_builder(&self) -> ListJobFamiliesBuilder {
        ListJobFamiliesBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_family_service_creation() {
        let config = Config::default();
        let service = JobFamilyService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_job_family_default_creation() {
        let job_family = JobFamily::default();
        assert_eq!(job_family.job_family_id, None);
        assert_eq!(job_family.name, None);
        assert_eq!(job_family.description, None);
        assert_eq!(job_family.order, None);
        assert_eq!(job_family.create_time, None);
        assert_eq!(job_family.update_time, None);
    }

    #[test]
    fn test_job_family_with_data() {
        let job_family = JobFamily {
            job_family_id: Some("family_123".to_string()),
            name: Some("技术族".to_string()),
            description: Some("技术研发相关职位族".to_string()),
            order: Some(1),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(job_family.job_family_id, Some("family_123".to_string()));
        assert_eq!(job_family.name, Some("技术族".to_string()));
        assert_eq!(
            job_family.description,
            Some("技术研发相关职位族".to_string())
        );
        assert_eq!(job_family.order, Some(1));
        assert_eq!(
            job_family.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            job_family.update_time,
            Some("2023-01-02T00:00:00Z".to_string())
        );
    }

    #[test]
    fn test_create_job_family_request() {
        let request = CreateJobFamilyRequest {
            name: "产品族".to_string(),
            description: Some("产品设计相关职位族".to_string()),
            order: Some(2),
        };

        assert_eq!(request.name, "产品族".to_string());
        assert_eq!(request.description, Some("产品设计相关职位族".to_string()));
        assert_eq!(request.order, Some(2));
    }

    #[test]
    fn test_list_job_families_request_default() {
        let request = ListJobFamiliesRequest::default();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_list_job_families_request_with_pagination() {
        let request = ListJobFamiliesRequest {
            page_size: Some(50),
            page_token: Some("token123".to_string()),
        };

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_create_job_family_builder() {
        let builder = CreateJobFamilyBuilder::new()
            .name("测试职位族")
            .description("测试描述")
            .order(1);

        assert_eq!(builder.request.name, "测试职位族");
        assert_eq!(builder.request.description, Some("测试描述".to_string()));
        assert_eq!(builder.request.order, Some(1));
    }

    #[test]
    fn test_list_job_families_builder() {
        let builder = ListJobFamiliesBuilder::new()
            .page_size(20)
            .page_token("test_token");

        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("test_token".to_string()));
    }

    #[test]
    fn test_response_default_creation() {
        let create_response = CreateJobFamilyResponse::default();
        assert_eq!(create_response.job_family.job_family_id, None);

        let list_response = ListJobFamiliesResponse::default();
        assert_eq!(list_response.items.len(), 0);
        assert_eq!(list_response.has_more, None);
        assert_eq!(list_response.page_token, None);

        let delete_response = DeleteJobFamilyResponse::default();
        assert_eq!(delete_response.success, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut create_response = CreateJobFamilyResponse::default();
        create_response.job_family = JobFamily {
            job_family_id: Some("family_456".to_string()),
            name: Some("市场族".to_string()),
            ..Default::default()
        };

        assert_eq!(
            create_response.job_family.job_family_id,
            Some("family_456".to_string())
        );
        assert_eq!(create_response.job_family.name, Some("市场族".to_string()));

        let mut list_response = ListJobFamiliesResponse::default();
        list_response.items = vec![JobFamily {
            job_family_id: Some("family_789".to_string()),
            name: Some("运营族".to_string()),
            ..Default::default()
        }];
        list_response.has_more = Some(true);
        list_response.page_token = Some("next_page".to_string());

        assert_eq!(list_response.items.len(), 1);
        assert_eq!(
            list_response.items[0].job_family_id,
            Some("family_789".to_string())
        );
        assert_eq!(list_response.items[0].name, Some("运营族".to_string()));
        assert_eq!(list_response.has_more, Some(true));
        assert_eq!(list_response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(CreateJobFamilyResponse::data_format(), ResponseFormat::Data);
        assert_eq!(UpdateJobFamilyResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetJobFamilyResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListJobFamiliesResponse::data_format(), ResponseFormat::Data);
        assert_eq!(DeleteJobFamilyResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = CreateJobFamilyRequest {
            name: "测试职位族".to_string(),
            description: Some("测试描述".to_string()),
            order: Some(1),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateJobFamilyRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.description, deserialized.description);
        assert_eq!(request.order, deserialized.order);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = ListJobFamiliesRequest {
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
            crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_FAMILIES,
            "/open-apis/contact/v3/job_families"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_FAMILY_GET,
            "/open-apis/contact/v3/job_families/{job_family_id}"
        );
    }
}

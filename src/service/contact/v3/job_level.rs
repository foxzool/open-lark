#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 职级管理服务
//!
//! 提供完整的职级管理功能：
//! - 创建职级
//! - 更新职级
//! - 获取单个职级信息
//! - 获取租户职级列表
//! - 删除职级
//! - 支持分页查询

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 职级信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobLevel {
    /// 职级ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 职级名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职级描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
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

impl Default for JobLevel {
    fn default() -> Self {
        Self {
            job_level_id: None,
            name: None,
            description: None,
            job_family_id: None,
            order: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 职级服务
#[derive(Debug, Clone)]
pub struct JobLevelService {
    config: Config,
}

impl JobLevelService {
    /// 创建新的职级服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建职级
    ///
    /// 在企业中创建一个新的职级
    ///
    /// # 参数
    /// * `req` - 创建职级请求
    ///
    /// # 返回值
    /// 返回创建的职级信息
    pub async fn create(&self, req: &CreateJobLevelRequest) -> SDKResult<CreateJobLevelResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_LEVELS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateJobLevelResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新职级
    ///
    /// 更新指定职级的信息
    ///
    /// # 参数
    /// * `job_level_id` - 职级ID
    /// * `req` - 更新职级请求
    ///
    /// # 返回值
    /// 返回更新后的职级信息
    pub async fn update(
        &self,
        job_level_id: &str,
        req: &UpdateJobLevelRequest,
    ) -> SDKResult<UpdateJobLevelResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_LEVEL_GET
            .replace("{job_level_id}", job_level_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateJobLevelResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个职级信息
    ///
    /// 根据职级ID获取职级的详细信息
    ///
    /// # 参数
    /// * `job_level_id` - 职级ID
    ///
    /// # 返回值
    /// 返回职级的详细信息
    pub async fn get(&self, job_level_id: &str) -> SDKResult<GetJobLevelResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_LEVEL_GET
            .replace("{job_level_id}", job_level_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetJobLevelResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取租户职级列表
    ///
    /// 获取企业内所有职级的列表，支持分页查询
    ///
    /// # 参数
    /// * `req` - 查询职级列表请求
    ///
    /// # 返回值
    /// 返回职级列表，支持分页
    pub async fn list(&self, req: &ListJobLevelsRequest) -> SDKResult<ListJobLevelsResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_LEVELS.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(job_family_id) = &req.job_family_id {
            query_params.push(format!("job_family_id={}", job_family_id));
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

        let resp = Transport::<ListJobLevelsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除职级
    ///
    /// 删除指定的职级
    ///
    /// # 参数
    /// * `job_level_id` - 职级ID
    ///
    /// # 返回值
    /// 返回删除操作的结果
    pub async fn delete(&self, job_level_id: &str) -> SDKResult<DeleteJobLevelResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_LEVEL_GET
            .replace("{job_level_id}", job_level_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<DeleteJobLevelResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 创建职级请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJobLevelRequest {
    /// 职级名称
    pub name: String,
    /// 职级描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 创建职级响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateJobLevelResponse {
    /// 职级信息
    pub job_level: JobLevel,
}

impl ApiResponseTrait for CreateJobLevelResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新职级请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateJobLevelRequest {
    /// 职级名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职级描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 序列ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 更新职级响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateJobLevelResponse {
    /// 职级信息
    pub job_level: JobLevel,
}

impl ApiResponseTrait for UpdateJobLevelResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取单个职级信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetJobLevelResponse {
    /// 职级信息
    pub job_level: JobLevel,
}

impl ApiResponseTrait for GetJobLevelResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询职级列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListJobLevelsRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 序列ID过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
}

impl Default for ListJobLevelsRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
            job_family_id: None,
        }
    }
}

/// 查询职级列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListJobLevelsResponse {
    /// 职级列表
    pub items: Vec<JobLevel>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListJobLevelsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除职级响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteJobLevelResponse {
    /// 是否成功删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl ApiResponseTrait for DeleteJobLevelResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建职级构建器
#[derive(Debug, Clone)]
pub struct CreateJobLevelBuilder {
    request: CreateJobLevelRequest,
}

impl Default for CreateJobLevelBuilder {
    fn default() -> Self {
        Self {
            request: CreateJobLevelRequest {
                name: String::new(),
                description: None,
                job_family_id: None,
                order: None,
            },
        }
    }
}

impl CreateJobLevelBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置职级名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    /// 设置职级描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 设置序列ID
    pub fn job_family_id(mut self, job_family_id: impl Into<String>) -> Self {
        self.request.job_family_id = Some(job_family_id.into());
        self
    }

    /// 设置排序
    pub fn order(mut self, order: i32) -> Self {
        self.request.order = Some(order);
        self
    }

    /// 执行创建
    pub async fn execute(self, service: &JobLevelService) -> SDKResult<CreateJobLevelResponse> {
        service.create(&self.request).await
    }
}

/// 查询职级列表构建器
#[derive(Debug, Clone)]
pub struct ListJobLevelsBuilder {
    request: ListJobLevelsRequest,
}

impl Default for ListJobLevelsBuilder {
    fn default() -> Self {
        Self {
            request: ListJobLevelsRequest::default(),
        }
    }
}

impl ListJobLevelsBuilder {
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

    /// 设置序列ID过滤
    pub fn job_family_id(mut self, job_family_id: impl Into<String>) -> Self {
        self.request.job_family_id = Some(job_family_id.into());
        self
    }

    /// 执行查询
    pub async fn execute(self, service: &JobLevelService) -> SDKResult<ListJobLevelsResponse> {
        service.list(&self.request).await
    }
}

impl JobLevelService {
    /// 创建职级构建器
    pub fn create_job_level_builder(&self) -> CreateJobLevelBuilder {
        CreateJobLevelBuilder::new()
    }

    /// 创建查询构建器
    pub fn list_job_levels_builder(&self) -> ListJobLevelsBuilder {
        ListJobLevelsBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_level_service_creation() {
        let config = Config::default();
        let service = JobLevelService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_job_level_default_creation() {
        let job_level = JobLevel::default();
        assert_eq!(job_level.job_level_id, None);
        assert_eq!(job_level.name, None);
        assert_eq!(job_level.description, None);
        assert_eq!(job_level.job_family_id, None);
        assert_eq!(job_level.order, None);
        assert_eq!(job_level.create_time, None);
        assert_eq!(job_level.update_time, None);
    }

    #[test]
    fn test_job_level_with_data() {
        let job_level = JobLevel {
            job_level_id: Some("level_123".to_string()),
            name: Some("高级工程师".to_string()),
            description: Some("高级技术职级".to_string()),
            job_family_id: Some("family_456".to_string()),
            order: Some(3),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(job_level.job_level_id, Some("level_123".to_string()));
        assert_eq!(job_level.name, Some("高级工程师".to_string()));
        assert_eq!(job_level.description, Some("高级技术职级".to_string()));
        assert_eq!(job_level.job_family_id, Some("family_456".to_string()));
        assert_eq!(job_level.order, Some(3));
        assert_eq!(
            job_level.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            job_level.update_time,
            Some("2023-01-02T00:00:00Z".to_string())
        );
    }

    #[test]
    fn test_create_job_level_request() {
        let request = CreateJobLevelRequest {
            name: "中级工程师".to_string(),
            description: Some("中级技术职级".to_string()),
            job_family_id: Some("family_789".to_string()),
            order: Some(2),
        };

        assert_eq!(request.name, "中级工程师".to_string());
        assert_eq!(request.description, Some("中级技术职级".to_string()));
        assert_eq!(request.job_family_id, Some("family_789".to_string()));
        assert_eq!(request.order, Some(2));
    }

    #[test]
    fn test_list_job_levels_request_default() {
        let request = ListJobLevelsRequest::default();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.job_family_id, None);
    }

    #[test]
    fn test_list_job_levels_request_with_filters() {
        let request = ListJobLevelsRequest {
            page_size: Some(50),
            page_token: Some("token123".to_string()),
            job_family_id: Some("family_456".to_string()),
        };

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
        assert_eq!(request.job_family_id, Some("family_456".to_string()));
    }

    #[test]
    fn test_create_job_level_builder() {
        let builder = CreateJobLevelBuilder::new()
            .name("测试职级")
            .description("测试描述")
            .job_family_id("family_123")
            .order(1);

        assert_eq!(builder.request.name, "测试职级");
        assert_eq!(builder.request.description, Some("测试描述".to_string()));
        assert_eq!(
            builder.request.job_family_id,
            Some("family_123".to_string())
        );
        assert_eq!(builder.request.order, Some(1));
    }

    #[test]
    fn test_list_job_levels_builder() {
        let builder = ListJobLevelsBuilder::new()
            .page_size(20)
            .page_token("test_token")
            .job_family_id("family_789");

        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("test_token".to_string()));
        assert_eq!(
            builder.request.job_family_id,
            Some("family_789".to_string())
        );
    }

    #[test]
    fn test_response_default_creation() {
        let create_response = CreateJobLevelResponse::default();
        assert_eq!(create_response.job_level.job_level_id, None);

        let list_response = ListJobLevelsResponse::default();
        assert_eq!(list_response.items.len(), 0);
        assert_eq!(list_response.has_more, None);
        assert_eq!(list_response.page_token, None);

        let delete_response = DeleteJobLevelResponse::default();
        assert_eq!(delete_response.success, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut create_response = CreateJobLevelResponse::default();
        create_response.job_level = JobLevel {
            job_level_id: Some("level_456".to_string()),
            name: Some("产品经理".to_string()),
            ..Default::default()
        };

        assert_eq!(
            create_response.job_level.job_level_id,
            Some("level_456".to_string())
        );
        assert_eq!(create_response.job_level.name, Some("产品经理".to_string()));

        let mut list_response = ListJobLevelsResponse::default();
        list_response.items = vec![JobLevel {
            job_level_id: Some("level_789".to_string()),
            name: Some("设计师".to_string()),
            ..Default::default()
        }];
        list_response.has_more = Some(true);
        list_response.page_token = Some("next_page".to_string());

        assert_eq!(list_response.items.len(), 1);
        assert_eq!(
            list_response.items[0].job_level_id,
            Some("level_789".to_string())
        );
        assert_eq!(list_response.items[0].name, Some("设计师".to_string()));
        assert_eq!(list_response.has_more, Some(true));
        assert_eq!(list_response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(CreateJobLevelResponse::data_format(), ResponseFormat::Data);
        assert_eq!(UpdateJobLevelResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetJobLevelResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListJobLevelsResponse::data_format(), ResponseFormat::Data);
        assert_eq!(DeleteJobLevelResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = CreateJobLevelRequest {
            name: "测试职级".to_string(),
            description: Some("测试描述".to_string()),
            job_family_id: Some("family_123".to_string()),
            order: Some(1),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateJobLevelRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.description, deserialized.description);
        assert_eq!(request.job_family_id, deserialized.job_family_id);
        assert_eq!(request.order, deserialized.order);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = ListJobLevelsRequest {
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
            job_family_id: Some("family_789".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(job_family_id) = &request.job_family_id {
            query_params.push(format!("job_family_id={}", job_family_id));
        }

        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&"page_size=20".to_string()));
        assert!(query_params.contains(&"page_token=test_token".to_string()));
        assert!(query_params.contains(&"job_family_id=family_789".to_string()));
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_LEVELS,
            "/open-apis/contact/v3/job_levels"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_LEVEL_GET,
            "/open-apis/contact/v3/job_levels/{job_level_id}"
        );
    }
}

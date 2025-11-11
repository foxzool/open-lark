#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 职务管理服务
//!
//! 提供完整的职务管理功能：
//! - 获取单个职务信息
//! - 获取租户职务列表
//! - 支持分页查询

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 职务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobTitle {
    /// 职务ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title_id: Option<String>,
    /// 职务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for JobTitle {
    fn default() -> Self {
        Self {
            job_title_id: None,
            name: None,
            description: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 职务服务
#[derive(Debug, Clone)]
pub struct JobTitleService {
    config: Config,
}

impl JobTitleService {
    /// 创建新的职务服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取单个职务信息
    ///
    /// 根据职务ID获取职务的详细信息
    ///
    /// # 参数
    /// * `job_title_id` - 职务ID
    ///
    /// # 返回值
    /// 返回职务的详细信息
    pub async fn get(&self, job_title_id: &str) -> SDKResult<GetJobTitleResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_TITLE_GET
            .replace("{job_title_id}", job_title_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetJobTitleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取租户职务列表
    ///
    /// 获取企业内所有职务的列表，支持分页查询
    ///
    /// # 参数
    /// * `req` - 查询职务列表请求
    ///
    /// # 返回值
    /// 返回职务列表，支持分页
    pub async fn list(&self, req: &ListJobTitlesRequest) -> SDKResult<ListJobTitlesResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_TITLES.to_string();

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

        let resp = Transport::<ListJobTitlesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 获取单个职务信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetJobTitleResponse {
    /// 职务信息
    pub job_title: JobTitle,
}

impl ApiResponseTrait for GetJobTitleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询职务列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListJobTitlesRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for ListJobTitlesRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }
}

/// 查询职务列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListJobTitlesResponse {
    /// 职务列表
    pub items: Vec<JobTitle>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListJobTitlesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 查询职务列表构建器
#[derive(Debug, Clone)]
pub struct ListJobTitlesBuilder {
    request: ListJobTitlesRequest,
}

impl Default for ListJobTitlesBuilder {
    fn default() -> Self {
        Self {
            request: ListJobTitlesRequest::default(),
        }
    }
}

impl ListJobTitlesBuilder {
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
    pub async fn execute(self, service: &JobTitleService) -> SDKResult<ListJobTitlesResponse> {
        service.list(&self.request).await
    }
}

impl JobTitleService {
    /// 创建查询构建器
    pub fn list_job_titles_builder(&self) -> ListJobTitlesBuilder {
        ListJobTitlesBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_title_service_creation() {
        let config = Config::default();
        let service = JobTitleService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_job_title_default_creation() {
        let job_title = JobTitle::default();
        assert_eq!(job_title.job_title_id, None);
        assert_eq!(job_title.name, None);
        assert_eq!(job_title.description, None);
        assert_eq!(job_title.create_time, None);
        assert_eq!(job_title.update_time, None);
    }

    #[test]
    fn test_job_title_with_data() {
        let job_title = JobTitle {
            job_title_id: Some("title_123".to_string()),
            name: Some("软件工程师".to_string()),
            description: Some("负责软件开发工作".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(job_title.job_title_id, Some("title_123".to_string()));
        assert_eq!(job_title.name, Some("软件工程师".to_string()));
        assert_eq!(job_title.description, Some("负责软件开发工作".to_string()));
        assert_eq!(
            job_title.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            job_title.update_time,
            Some("2023-01-02T00:00:00Z".to_string())
        );
    }

    #[test]
    fn test_list_job_titles_request_default() {
        let request = ListJobTitlesRequest::default();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_list_job_titles_request_with_pagination() {
        let request = ListJobTitlesRequest {
            page_size: Some(50),
            page_token: Some("token123".to_string()),
        };

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_list_job_titles_builder() {
        let builder = ListJobTitlesBuilder::new()
            .page_size(20)
            .page_token("test_token");

        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("test_token".to_string()));
    }

    #[test]
    fn test_list_job_titles_builder_default() {
        let builder = ListJobTitlesBuilder::default();
        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.page_token, None);
    }

    #[test]
    fn test_response_default_creation() {
        let get_response = GetJobTitleResponse::default();
        assert_eq!(get_response.job_title.job_title_id, None);

        let list_response = ListJobTitlesResponse::default();
        assert_eq!(list_response.items.len(), 0);
        assert_eq!(list_response.has_more, None);
        assert_eq!(list_response.page_token, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut get_response = GetJobTitleResponse::default();
        get_response.job_title = JobTitle {
            job_title_id: Some("title_456".to_string()),
            name: Some("产品经理".to_string()),
            ..Default::default()
        };

        assert_eq!(
            get_response.job_title.job_title_id,
            Some("title_456".to_string())
        );
        assert_eq!(get_response.job_title.name, Some("产品经理".to_string()));

        let mut list_response = ListJobTitlesResponse::default();
        list_response.items = vec![JobTitle {
            job_title_id: Some("title_789".to_string()),
            name: Some("设计师".to_string()),
            ..Default::default()
        }];
        list_response.has_more = Some(true);
        list_response.page_token = Some("next_page".to_string());

        assert_eq!(list_response.items.len(), 1);
        assert_eq!(
            list_response.items[0].job_title_id,
            Some("title_789".to_string())
        );
        assert_eq!(list_response.items[0].name, Some("设计师".to_string()));
        assert_eq!(list_response.has_more, Some(true));
        assert_eq!(list_response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(GetJobTitleResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListJobTitlesResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = ListJobTitlesRequest {
            page_size: Some(10),
            page_token: Some("token".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ListJobTitlesRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.page_size, deserialized.page_size);
        assert_eq!(request.page_token, deserialized.page_token);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = ListJobTitlesRequest {
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
            crate::core::endpoints_original::Endpoints::CONTACT_V3_JOB_TITLES,
            "/open-apis/contact/v3/job_titles"
        );
    }
}

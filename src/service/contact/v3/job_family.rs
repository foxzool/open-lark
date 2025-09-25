use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
    },
    service::contact::models::*,
};
use serde::{Deserialize, Serialize};

/// 序列管理服务
#[derive(Debug)]
pub struct JobFamilyService {
    config: Config,
}

impl JobFamilyService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建序列
    pub async fn create(
        &self,
        req: &CreateJobFamilyRequest,
    ) -> crate::core::SDKResult<CreateJobFamilyResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints::contact::CONTACT_V3_JOB_FAMILIES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateJobFamilyResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新序列
    pub async fn update(
        &self,
        job_family_id: &str,
        req: &UpdateJobFamilyRequest,
    ) -> crate::core::SDKResult<UpdateJobFamilyResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_JOB_FAMILY_GET,
                "job_family_id",
                job_family_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateJobFamilyResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个序列信息
    pub async fn get(&self, job_family_id: &str) -> crate::core::SDKResult<GetJobFamilyResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_JOB_FAMILY_GET,
                "job_family_id",
                job_family_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetJobFamilyResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取租户序列列表
    pub async fn list(
        &self,
        _req: &ListJobFamiliesRequest,
    ) -> crate::core::SDKResult<ListJobFamiliesResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: crate::core::endpoints::contact::CONTACT_V3_JOB_FAMILIES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp =
            Transport::<ListJobFamiliesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除序列
    pub async fn delete(
        &self,
        job_family_id: &str,
    ) -> crate::core::SDKResult<DeleteJobFamilyResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_JOB_FAMILY_GET,
                "job_family_id",
                job_family_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<DeleteJobFamilyResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJobFamilyRequest {
    pub job_family: JobFamily,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateJobFamilyResponse {
    pub job_family: JobFamily,
}

impl ApiResponseTrait for CreateJobFamilyResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateJobFamilyRequest {
    pub job_family: JobFamily,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateJobFamilyResponse {
    pub job_family: JobFamily,
}

impl ApiResponseTrait for UpdateJobFamilyResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetJobFamilyResponse {
    pub job_family: JobFamily,
}

impl ApiResponseTrait for GetJobFamilyResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListJobFamiliesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListJobFamiliesResponse {
    pub items: Vec<JobFamily>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListJobFamiliesResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteJobFamilyResponse {}

impl ApiResponseTrait for DeleteJobFamilyResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    #[test]
    fn test_job_family_service_creation() {
        let config = Config::default();
        let service = JobFamilyService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_job_family_service_creation_with_custom_config() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_secret")
            .build();
        let service = JobFamilyService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_create_job_family_request_construction() {
        let job_family = JobFamily {
            job_family_id: Some("family_123".to_string()),
            name: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("技术序列".to_string()),
            }]),
            description: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("技术类职业序列".to_string()),
            }]),
            status: Some(true),
        };
        let request = CreateJobFamilyRequest { job_family };
        assert_eq!(
            request.job_family.job_family_id,
            Some("family_123".to_string())
        );
        assert!(request.job_family.name.is_some());
    }

    #[test]
    fn test_create_job_family_request_with_minimal_data() {
        let job_family = JobFamily {
            job_family_id: None,
            name: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("简单序列".to_string()),
            }]),
            description: None,
            status: None,
        };
        let request = CreateJobFamilyRequest { job_family };
        assert!(request.job_family.name.is_some());
        assert_eq!(request.job_family.description, None);
    }

    #[test]
    fn test_create_job_family_request_with_empty_values() {
        let job_family = JobFamily {
            job_family_id: Some("".to_string()),
            name: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("".to_string()),
            }]),
            description: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("".to_string()),
            }]),
            status: Some(false),
        };
        let request = CreateJobFamilyRequest { job_family };
        assert_eq!(request.job_family.job_family_id, Some("".to_string()));
        assert!(request.job_family.name.is_some());
    }

    #[test]
    fn test_create_job_family_request_with_long_values() {
        let long_name = "a".repeat(1000);
        let long_desc = "b".repeat(2000);
        let job_family = JobFamily {
            job_family_id: Some("family_long".to_string()),
            name: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some(long_name.clone()),
            }]),
            description: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some(long_desc.clone()),
            }]),
            status: Some(true),
        };
        let request = CreateJobFamilyRequest { job_family };
        assert!(request.job_family.name.is_some());
        assert!(request.job_family.description.is_some());
    }

    #[test]
    fn test_create_job_family_response_default() {
        let response = CreateJobFamilyResponse::default();
        assert_eq!(response.job_family.job_family_id, None);
        assert_eq!(response.job_family.name, None);
    }

    #[test]
    fn test_create_job_family_response_data_format() {
        assert_eq!(
            CreateJobFamilyResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_update_job_family_request_construction() {
        let job_family = JobFamily {
            job_family_id: Some("family_456".to_string()),
            name: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("更新序列".to_string()),
            }]),
            description: Some(vec![I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("更新后的描述".to_string()),
            }]),
            status: Some(true),
        };
        let request = UpdateJobFamilyRequest { job_family };
        assert_eq!(
            request.job_family.job_family_id,
            Some("family_456".to_string())
        );
        assert!(request.job_family.name.is_some());
    }

    #[test]
    fn test_update_job_family_request_with_none_values() {
        let job_family = JobFamily {
            job_family_id: None,
            name: None,
            description: None,
            status: None,
        };
        let request = UpdateJobFamilyRequest { job_family };
        assert_eq!(request.job_family.job_family_id, None);
        assert_eq!(request.job_family.name, None);
    }

    #[test]
    fn test_update_job_family_response_default() {
        let response = UpdateJobFamilyResponse::default();
        assert_eq!(response.job_family.job_family_id, None);
        assert_eq!(response.job_family.name, None);
    }

    #[test]
    fn test_update_job_family_response_data_format() {
        assert_eq!(
            UpdateJobFamilyResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_get_job_family_response_default() {
        let response = GetJobFamilyResponse::default();
        assert_eq!(response.job_family.job_family_id, None);
        assert_eq!(response.job_family.name, None);
    }

    #[test]
    fn test_get_job_family_response_data_format() {
        assert_eq!(
            GetJobFamilyResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
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
            page_size: Some(20),
            page_token: Some("token_abc".to_string()),
        };
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token_abc".to_string()));
    }

    #[test]
    fn test_list_job_families_request_with_large_page_size() {
        let request = ListJobFamiliesRequest {
            page_size: Some(10000),
            page_token: Some("large_token".to_string()),
        };
        assert_eq!(request.page_size, Some(10000));
        assert_eq!(request.page_token, Some("large_token".to_string()));
    }

    #[test]
    fn test_list_job_families_request_with_zero_page_size() {
        let request = ListJobFamiliesRequest {
            page_size: Some(0),
            page_token: None,
        };
        assert_eq!(request.page_size, Some(0));
    }

    #[test]
    fn test_list_job_families_request_with_negative_page_size() {
        let request = ListJobFamiliesRequest {
            page_size: Some(-1),
            page_token: None,
        };
        assert_eq!(request.page_size, Some(-1));
    }

    #[test]
    fn test_list_job_families_request_with_empty_token() {
        let request = ListJobFamiliesRequest {
            page_size: Some(50),
            page_token: Some("".to_string()),
        };
        assert_eq!(request.page_token, Some("".to_string()));
    }

    #[test]
    fn test_list_job_families_request_with_long_token() {
        let long_token = "x".repeat(3000);
        let request = ListJobFamiliesRequest {
            page_size: Some(100),
            page_token: Some(long_token.clone()),
        };
        assert_eq!(request.page_token, Some(long_token));
    }

    #[test]
    fn test_list_job_families_response_default() {
        let response = ListJobFamiliesResponse::default();
        assert_eq!(response.items.len(), 0);
        assert_eq!(response.has_more, None);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_list_job_families_response_with_items() {
        let items = vec![
            JobFamily {
                job_family_id: Some("family_1".to_string()),
                name: Some(vec![I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("序列1".to_string()),
                }]),
                description: Some(vec![I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("描述1".to_string()),
                }]),
                status: Some(true),
            },
            JobFamily {
                job_family_id: Some("family_2".to_string()),
                name: Some(vec![I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("序列2".to_string()),
                }]),
                description: Some(vec![I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("描述2".to_string()),
                }]),
                status: Some(true),
            },
        ];
        let response = ListJobFamiliesResponse {
            items,
            has_more: Some(true),
            page_token: Some("next_page".to_string()),
        };
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.has_more, Some(true));
        assert_eq!(response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_list_job_families_response_with_large_list() {
        let mut items = Vec::new();
        for i in 0..500 {
            items.push(JobFamily {
                job_family_id: Some(format!("family_{}", i)),
                name: Some(vec![I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some(format!("序列{}", i)),
                }]),
                description: Some(vec![I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some(format!("描述{}", i)),
                }]),
                status: Some(true),
            });
        }
        let response = ListJobFamiliesResponse {
            items,
            has_more: Some(false),
            page_token: None,
        };
        assert_eq!(response.items.len(), 500);
        assert_eq!(response.has_more, Some(false));
    }

    #[test]
    fn test_list_job_families_response_data_format() {
        assert_eq!(
            ListJobFamiliesResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_delete_job_family_response_default() {
        let response = DeleteJobFamilyResponse::default();
        assert!(!format!("{:?}", response).is_empty());
    }

    #[test]
    fn test_delete_job_family_response_data_format() {
        assert_eq!(
            DeleteJobFamilyResponse::data_format(),
            crate::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_config_independence() {
        let config1 = Config::default();
        let config2 = Config::default();
        let service1 = JobFamilyService::new(config1);
        let service2 = JobFamilyService::new(config2);
        assert!(!format!("{:?}", service1).is_empty());
        assert!(!format!("{:?}", service2).is_empty());
    }

    #[test]
    fn test_all_structs_debug_trait() {
        let job_family = JobFamily {
            job_family_id: Some("test".to_string()),
            name: Some(vec![I18nContent {
                locale: Some("en_us".to_string()),
                value: Some("test".to_string()),
            }]),
            description: Some(vec![I18nContent {
                locale: Some("en_us".to_string()),
                value: Some("test description".to_string()),
            }]),
            status: Some(true),
        };

        let create_request = CreateJobFamilyRequest {
            job_family: job_family.clone(),
        };
        let update_request = UpdateJobFamilyRequest {
            job_family: job_family.clone(),
        };
        let list_request = ListJobFamiliesRequest {
            page_size: Some(10),
            page_token: Some("test".to_string()),
        };

        assert!(format!("{:?}", create_request).contains("test"));
        assert!(format!("{:?}", update_request).contains("test"));
        assert!(format!("{:?}", list_request).contains("test"));

        let create_response = CreateJobFamilyResponse::default();
        let update_response = UpdateJobFamilyResponse::default();
        let get_response = GetJobFamilyResponse::default();
        let list_response = ListJobFamiliesResponse::default();
        let delete_response = DeleteJobFamilyResponse::default();

        assert!(!format!("{:?}", create_response).is_empty());
        assert!(!format!("{:?}", update_response).is_empty());
        assert!(!format!("{:?}", get_response).is_empty());
        assert!(!format!("{:?}", list_response).is_empty());
        assert!(!format!("{:?}", delete_response).is_empty());
    }
}

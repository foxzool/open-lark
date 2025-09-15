use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::ApiResponseTrait,
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
    },
    service::contact::models::*,
};
use serde::{Deserialize, Serialize};

/// 序列管理服务
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
            api_path: Endpoints::CONTACT_V3_JOB_FAMILIES.to_string(),
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
                Endpoints::CONTACT_V3_JOB_FAMILY_GET,
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
                Endpoints::CONTACT_V3_JOB_FAMILY_GET,
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
            api_path: Endpoints::CONTACT_V3_JOB_FAMILIES.to_string(),
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
                Endpoints::CONTACT_V3_JOB_FAMILY_GET,
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

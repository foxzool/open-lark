use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
    },
    service::contact::models::*,
};
use serde::{Deserialize, Serialize};

/// 职务服务
pub struct JobTitleService {
    config: Config,
}

impl JobTitleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取单个职务信息
    pub async fn get(&self, job_title_id: &str) -> crate::core::SDKResult<GetJobTitleResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_JOB_TITLE_GET,
                "job_title_id",
                job_title_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetJobTitleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取租户职务列表
    pub async fn list(
        &self,
        _req: &ListJobTitlesRequest,
    ) -> crate::core::SDKResult<ListJobTitlesResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: crate::core::endpoints::contact::CONTACT_V3_JOB_TITLES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<ListJobTitlesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetJobTitleResponse {
    pub job_title: JobTitle,
}

impl ApiResponseTrait for GetJobTitleResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListJobTitlesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListJobTitlesResponse {
    pub items: Vec<JobTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListJobTitlesResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

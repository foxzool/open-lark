use open_lark_core::core::{
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
    constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
};
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 职级管理服务
pub struct JobLevelService {
    config: Config,
}

impl JobLevelService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 创建职级
    pub async fn create(
        &self,
        req: &CreateJobLevelRequest,
    ) -> open_lark_core::core::SDKResult<CreateJobLevelResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::POST);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_JOB_LEVELS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(req)?;

        let resp =
            Transport::<CreateJobLevelResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新职级
    pub async fn update(
        &self,
        job_level_id: &str,
        req: &UpdateJobLevelRequest,
    ) -> open_lark_core::core::SDKResult<UpdateJobLevelResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::PUT);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_JOB_LEVELS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(req)?;

        let resp =
            Transport::<UpdateJobLevelResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact/get


    /// 获取单个职级信息
    pub async fn get(&self, job_level_id: &str) -> open_lark_core::core::SDKResult<GetJobLevelResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::GET);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_JOB_LEVELS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = Vec::new();

        let resp = Transport::<GetJobLevelResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取租户职级列表
    pub async fn list(
        &self,
        _req: &ListJobLevelsRequest,
    ) -> open_lark_core::core::SDKResult<ListJobLevelsResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::GET);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_JOB_LEVELS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = Vec::new();
        api_req.query_params = std::collections::HashMap::new();

        let resp = Transport::<ListJobLevelsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除职级
    pub async fn delete(
        &self,
        job_level_id: &str,
    ) -> open_lark_core::core::SDKResult<DeleteJobLevelResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::DELETE);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_JOB_LEVELS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = Vec::new();

        let resp =
            Transport::<DeleteJobLevelResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJobLevelRequest {
    pub job_level: JobLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateJobLevelResponse {
    pub job_level: JobLevel,
}

impl ApiResponseTrait for CreateJobLevelResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateJobLevelRequest {
    pub job_level: JobLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateJobLevelResponse {
    pub job_level: JobLevel,
}

impl ApiResponseTrait for UpdateJobLevelResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetJobLevelResponse {
    pub job_level: JobLevel,
}

impl ApiResponseTrait for GetJobLevelResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListJobLevelsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListJobLevelsResponse {
    pub items: Vec<JobLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListJobLevelsResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteJobLevelResponse {}

impl ApiResponseTrait for DeleteJobLevelResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}
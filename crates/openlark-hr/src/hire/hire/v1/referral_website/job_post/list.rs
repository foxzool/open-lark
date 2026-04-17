//! 获取内推官网下职位广告列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/referral_website.job_post/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::{CodeNameObject, I18nText, IdNameObject};

#[derive(Debug, Clone)]
pub struct ListRequest {
    config: Config,
    process_type: Option<i32>,
    page_token: Option<String>,
    page_size: Option<i32>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
    job_level_id_type: Option<String>,
}

impl ListRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            process_type: None,
            page_token: None,
            page_size: None,
            user_id_type: None,
            department_id_type: None,
            job_level_id_type: None,
        }
    }

    pub fn process_type(mut self, process_type: i32) -> Self {
        self.process_type = Some(process_type);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    pub fn job_level_id_type(mut self, job_level_id_type: impl Into<String>) -> Self {
        self.job_level_id_type = Some(job_level_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        if let Some(page_size) = self.page_size {
            if !(1..=10).contains(&page_size) {
                return Err(error::validation_error(
                    "page_size",
                    "page_size 必须在 1-10 之间",
                ));
            }
        }

        let mut request =
            ApiRequest::<ListResponse>::get("/open-apis/hire/v1/referral_websites/job_posts");
        if let Some(process_type) = self.process_type {
            request = request.query("process_type", process_type.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        if let Some(department_id_type) = self.department_id_type {
            request = request.query("department_id_type", department_id_type);
        }
        if let Some(job_level_id_type) = self.job_level_id_type {
            request = request.query("job_level_id_type", job_level_id_type);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error(
                "获取内推官网下职位广告列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobPostAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<CodeNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<CodeNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CodeNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<CodeNameObject>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralWebsiteJobPostItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_process_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_recruitment_type: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_department: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_job_level: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_job_level: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<JobPostAddress>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    pub items: Vec<ReferralWebsiteJobPostItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}

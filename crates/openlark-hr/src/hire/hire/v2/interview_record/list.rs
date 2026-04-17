//! 批量获取面试评价详细信息（新版）
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v2/interview_record/list

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

use crate::hire::hire::common_models::{AttachmentMeta, I18nText, IdNameObject, ScoreInfo};

#[derive(Debug, Clone)]
pub struct ListRequest {
    config: Config,
    ids: Option<Vec<String>>,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
}

impl ListRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ids: None,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.ids = Some(ids);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
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
            if !(0..=100).contains(&page_size) {
                return Err(error::validation_error(
                    "page_size",
                    "page_size 必须在 0-100 之间",
                ));
            }
        }

        if let Some(ref ids) = self.ids {
            if ids.len() > 100 {
                return Err(error::validation_error("ids", "ids 不能超过 100 个"));
            }
        }

        let mut request = ApiRequest::<ListResponse>::get("/open-apis/hire/v2/interview_records");
        if let Some(ids) = self.ids {
            request = request.query(
                "ids",
                serde_json::to_string(&ids).map_err(|e| {
                    error::validation_error("ids", format!("无法序列化数组查询参数: {}", e))
                })?,
            );
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error(
                "批量获取面试评价详细信息（新版）响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecordDimensionAssessment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_feedback_form_dimension_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_score: Option<f64>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecordModuleAssessment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_feedback_form_module_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_assessments: Option<Vec<InterviewRecordDimensionAssessment>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRecordItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_form_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_score: Option<ScoreInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interviewer: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentMeta>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_assessments: Option<Vec<InterviewRecordModuleAssessment>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    pub items: Vec<InterviewRecordItem>,
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

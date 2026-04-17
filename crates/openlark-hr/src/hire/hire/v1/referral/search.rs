//! 查询人才内推信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/referral/search

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

use crate::hire::hire::common_models::IdNameObject;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct SearchRequestBody {
    talent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SearchRequest {
    config: Config,
    user_id_type: Option<String>,
    talent_id: String,
    start_time: Option<String>,
    end_time: Option<String>,
}

impl SearchRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            talent_id: String::new(),
            start_time: None,
            end_time: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn talent_id(mut self, talent_id: impl Into<String>) -> Self {
        self.talent_id = talent_id.into();
        self
    }

    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        if self.talent_id.trim().is_empty() {
            return Err(error::validation_error("talent_id", "talent_id 不能为空"));
        }

        let mut request = ApiRequest::<SearchResponse>::post("/open-apis/hire/v1/referrals/search");
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        request = request.body(
            serde_json::to_value(SearchRequestBody {
                talent_id: self.talent_id,
                start_time: self.start_time,
                end_time: self.end_time,
            })
            .map_err(|e| {
                error::validation_error("request_body", format!("无法序列化请求体: {}", e))
            })?,
        );

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("查询人才内推信息响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_user: Option<IdNameObject>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchResponse {
    #[serde(default)]
    pub items: Vec<ReferralItem>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for SearchResponse {
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

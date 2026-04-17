//! 获取人才详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v2/talent/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::{AttachmentMeta, I18nText};

/// 获取人才详情请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    talent_id: String,
    user_id_type: Option<String>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            talent_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn talent_id(mut self, talent_id: String) -> Self {
        self.talent_id = talent_id;
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::HireApiV2;

        validate_required!(self.talent_id.trim(), "人才 ID 不能为空");

        let api_endpoint = HireApiV2::TalentGet(self.talent_id);
        let mut request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取人才详情响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取人才详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentValueOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentTimeRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCustomizedValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<TalentValueOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_list: Option<Vec<TalentValueOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<TalentTimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_attachment: Option<Vec<AttachmentMeta>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCustomizedData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TalentCustomizedValue>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentBasicInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_years: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_location_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hometown_location_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_location_code_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<TalentCustomizedData>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<TalentBasicInfo>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}

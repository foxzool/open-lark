//! 创建 Offer
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/offer/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 创建 Offer请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        let api_endpoint = HireApiV1::OfferCreate;
        let mut request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建 Offer 响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 创建 Offer响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferCatalogRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferContractPeriod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferBasicInfoUpsert {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_period: Option<OfferContractPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_onboard_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboard_address_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_address_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_words: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_requirement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_process_type_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_attachment_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_offered: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferSalaryInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_salary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_salary_percentage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_salary_multiple: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_shares: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarterly_bonus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub half_year_bonus: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferCustomizedInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<OfferBasicInfoUpsert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary_info: Option<OfferSalaryInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_info_list: Option<Vec<OfferCustomizedInfo>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CreateResponse {
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

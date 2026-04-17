//! 获取 Offer 详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/offer/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 获取 Offer 详情请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    offer_id: String,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            offer_id: String::new(),
        }
    }

    pub fn offer_id(mut self, offer_id: String) -> Self {
        self.offer_id = offer_id;
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
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.offer_id.trim(), "Offer ID 不能为空");

        let api_endpoint = HireApiV1::OfferGet(self.offer_id);
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取 Offer 详情响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取 Offer 详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetCatalogRef {
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
pub struct OfferGetContractPeriod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetLocationNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<OfferGetLocationNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<OfferGetLocationNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<OfferGetLocationNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<OfferGetLocationNode>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetSimpleAddress {
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
pub struct OfferWorkLocationAddressInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_info: Option<OfferGetSimpleAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info: Option<OfferGetSimpleAddress>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetCustomInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customize_value: Option<Value>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferGetBasicInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<OfferGetCatalogRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboard_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_period: Option<OfferGetContractPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recruitment_type: Option<OfferGetCatalogRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<OfferGetCatalogRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<OfferGetCatalogRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboard_address: Option<OfferGetAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_address: Option<OfferGetAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customize_info_list: Option<Vec<OfferGetCustomInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location_address_info: Option<OfferWorkLocationAddressInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_offered: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_attachment_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferSalaryPlan {
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
pub struct OfferDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<OfferGetBasicInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary_plan: Option<OfferSalaryPlan>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer: Option<OfferDetail>,
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

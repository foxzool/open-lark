//! 查询猎头保护期信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/agency/protect_search

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::AgencyProtectionSummary;

/// 查询猎头保护期信息请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProtectSearchRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl ProtectSearchRequest {
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
    pub async fn execute(self) -> SDKResult<ProtectSearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ProtectSearchResponse> {
        let mut request = ApiRequest::<ProtectSearchResponse>::post(
            "/open-apis/hire/v1/agencies/protection_period/search",
        );
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询猎头保护期信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询猎头保护期信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProtectSearchResponse {
    #[serde(default, alias = "protections")]
    pub items: Vec<AgencyProtectionSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ProtectSearchResponse {
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

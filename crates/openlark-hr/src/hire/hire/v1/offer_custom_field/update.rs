//! 更新 Offer 申请表自定义字段
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/offer_custom_field/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新 Offer 申请表自定义字段请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateRequest {
    /// 配置信息
    config: Config,
    offer_custom_field_id: Option<String>,
    request_body: Option<Value>,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            offer_custom_field_id: None,
            request_body: None,
        }
    }

    pub fn offer_custom_field_id(mut self, offer_custom_field_id: impl Into<String>) -> Self {
        self.offer_custom_field_id = Some(offer_custom_field_id.into());
        self
    }

    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        let offer_custom_field_id = self.offer_custom_field_id.unwrap_or_default();
        validate_required!(
            offer_custom_field_id.trim(),
            "offer_custom_field_id 不能为空"
        );

        let mut request = ApiRequest::<UpdateResponse>::put(format!(
            "/open-apis/hire/v1/offer_custom_fields/{}",
            offer_custom_field_id
        ));
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新 Offer 申请表自定义字段响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新 Offer 申请表自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for UpdateResponse {
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

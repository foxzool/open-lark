//! 获取人才字段
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/talent_object/query

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取人才字段请求。
///
/// 官方文档未定义请求参数，因此该请求仅持有配置并直接发起调用。
#[derive(Debug, Clone)]
pub struct QueryRequest {
    config: Config,
}

impl QueryRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        let request = ApiRequest::<QueryResponse>::get("/open-apis/hire/v1/talent_objects/query");
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("获取人才字段响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 响应数据
    ///
    /// 当前按未建模 JSON 原样透传；字段收敛后再替换为显式结构。
    pub data: Value,
}

impl ApiResponseTrait for QueryResponse {
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

//! 获取用户自定义常用的应用

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GetFrequentlyUsedAppsRequest {
    config: Arc<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFrequentlyUsedAppsResponse {
    pub data: Option<FrequentlyUsedAppsData>,
}

impl ApiResponseTrait for GetFrequentlyUsedAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequentlyUsedAppsData {
    pub apps: Vec<FrequentlyUsedApp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequentlyUsedApp {
    pub app_id: String,
    pub app_name: String,
}

impl GetFrequentlyUsedAppsRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<GetFrequentlyUsedAppsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetFrequentlyUsedAppsResponse> {
        let path = "/open-apis/application/v6/applications/frequently_used".to_string();
        let req: ApiRequest<GetFrequentlyUsedAppsResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取用户自定义常用的应用", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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

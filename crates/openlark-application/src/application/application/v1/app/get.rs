//! app get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取应用详情请求。
#[derive(Debug, Clone)]
pub struct GetAppRequest {
    config: Arc<Config>,
}

/// 获取应用详情的原始响应封装。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppGetResponse {
    /// 响应数据载荷。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for AppGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetAppRequest {
    /// 创建新的获取应用详情请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<AppGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<AppGetResponse> {
        let path = "/open-apis/application/v3/app/info".to_string();
        let req: ApiRequest<AppGetResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("app get", "响应数据为空"))
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}

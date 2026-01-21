//! 获取应用详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/application-api/app/get

use crate::common::{api_endpoints::AppApiV1, api_utils::*};
use crate::v1::app::models::GetAppResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use std::sync::Arc;

/// 获取应用请求
#[derive(Debug, Clone)]
pub struct GetAppRequest {
    /// 配置信息
    config: Arc<Config>,
}

impl GetAppRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetAppResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetAppResponse> {
        let api_endpoint = AppApiV1::AppGet;
        let request = ApiRequest::<GetAppResponse>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取应用")
    }
}

impl ApiResponseTrait for GetAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_app_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = GetAppRequest::new(Arc::new(config));

        // 测试请求创建成功
        let _ = request;
    }

    #[test]
    fn test_app_api_v1_url() {
        let endpoint = AppApiV1::AppGet;
        assert_eq!(endpoint.to_url(), "/open-apis/application/v1/applications");
    }
}

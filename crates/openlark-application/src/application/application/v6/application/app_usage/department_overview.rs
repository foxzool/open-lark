//! 获取多部门应用使用概览

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
pub struct GetApplicationUsageDepartmentOverviewRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationUsageDepartmentOverviewResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationUsageDepartmentOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationUsageDepartmentOverviewRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetApplicationUsageDepartmentOverviewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationUsageDepartmentOverviewResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/app_usage/department_overview", self.app_id);
        let req: ApiRequest<GetApplicationUsageDepartmentOverviewResponse> = ApiRequest::post(&path);

        let _resp: openlark_core::api::Response<GetApplicationUsageDepartmentOverviewResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetApplicationUsageDepartmentOverviewResponse { data: None })
    }
}

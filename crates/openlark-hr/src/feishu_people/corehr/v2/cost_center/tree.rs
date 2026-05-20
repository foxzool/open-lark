//! 查询指定生效日期的成本中心架构树
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

/// 查询指定生效日期的成本中心架构树请求。
#[derive(Debug, Clone)]
pub struct CostCenterTreeRequest {
    config: Arc<Config>,
}

impl CostCenterTreeRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let path = "/open-apis/corehr/v2/cost_centers/tree".to_string();
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(path).body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询指定生效日期的成本中心架构树",
                "响应数据为空",
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = CostCenterTreeRequest::new(config);
    }
}

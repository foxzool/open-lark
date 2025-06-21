pub mod copy;
pub mod list;

use crate::core::config::Config;

pub use copy::*;
pub use list::*;

/// 仪表盘服务
pub struct AppDashboardService {
    config: Config,
}

impl AppDashboardService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 复制仪表盘
    pub async fn copy(
        &self,
        request: CopyDashboardRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<CopyDashboardResponse>> {
        copy::copy_dashboard(request, &self.config, option).await
    }

    /// 列出仪表盘
    pub async fn list(
        &self,
        request: ListDashboardRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListDashboardResponse>> {
        list::list_dashboard(request, &self.config, option).await
    }
}

pub mod field;
pub mod get;
pub mod models;
pub mod patch;

pub use field::*;
pub use get::*;
pub use models::*;
use openlark_core::config::Config;
pub use patch::*;

/// 表单服务
pub struct FormService {
    pub config: Config,
}

impl FormService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取表单元数据
    pub async fn get(&self, request: GetFormRequest) -> openlark_core::SDKResult<GetFormResponse> {
        request.execute().await
    }

    /// 更新表单元数据
    pub async fn patch(
        &self,
        request: PatchFormRequest,
    ) -> openlark_core::SDKResult<PatchFormResponse> {
        request.execute().await
    }
}

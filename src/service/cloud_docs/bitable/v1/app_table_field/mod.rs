pub mod create;
pub mod delete;
pub mod list;
pub mod types;
pub mod update;

use crate::core::config::Config;

pub use create::*;
pub use delete::*;
pub use list::*;
pub use types::*;
pub use update::*;

/// 字段服务
pub struct AppTableFieldService {
    config: Config,
}

impl AppTableFieldService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 新增字段
    pub async fn create(
        &self,
        request: CreateFieldRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<CreateFieldResponse>> {
        create::create_field(request, &self.config, option).await
    }

    /// 更新字段
    pub async fn update(
        &self,
        request: UpdateFieldRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<UpdateFieldResponse>> {
        update::update_field(request, &self.config, option).await
    }

    /// 列出字段
    pub async fn list(
        &self,
        request: &ListFieldRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListFieldResponse>> {
        list::list_field(request.clone(), &self.config, option).await
    }

    /// 删除字段
    pub async fn delete(
        &self,
        request: DeleteFieldRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<DeleteFieldResponse>> {
        delete::delete_field(request, &self.config, option).await
    }
}

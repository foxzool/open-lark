pub mod create;
pub mod delete;
pub mod list;
pub mod update;

use crate::core::config::Config;

pub use create::*;
pub use delete::*;
pub use list::*;
pub use update::*;

/// 自定义角色服务
pub struct AppRoleService {
    config: Config,
}

impl AppRoleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 新增自定义角色
    pub async fn create(
        &self,
        request: CreateAppRoleRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<CreateAppRoleResponse>> {
        create::create_app_role(request, &self.config, option).await
    }

    /// 更新自定义角色
    pub async fn update(
        &self,
        request: UpdateAppRoleRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<UpdateAppRoleResponse>> {
        update::update_app_role(request, &self.config, option).await
    }

    /// 列出自定义角色
    pub async fn list(
        &self,
        request: ListAppRoleRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListAppRoleResponse>> {
        list::list_app_roles(request, &self.config, option).await
    }

    /// 删除自定义角色
    pub async fn delete(
        &self,
        request: DeleteAppRoleRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<DeleteAppRoleResponse>> {
        delete::delete_app_role(request, &self.config, option).await
    }
}
pub mod batch_create;
pub mod batch_delete;
pub mod create;
pub mod delete;
pub mod list;

use crate::core::config::Config;

pub use batch_create::*;
pub use batch_delete::*;
pub use create::*;
pub use delete::*;

/// 协作者服务
pub struct AppRoleMemberService {
    config: Config,
}

impl AppRoleMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量新增协作者
    pub async fn batch_create(
        &self,
        request: BatchCreateRoleMemberRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchCreateRoleMemberResponse>>
    {
        batch_create::batch_create_role_members(request, &self.config, option).await
    }

    /// 删除协作者
    pub async fn delete(
        &self,
        request: DeleteRoleMemberRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<DeleteRoleMemberResponse>> {
        delete::delete_role_member(request, &self.config, option).await
    }

    /// 批量删除协作者
    pub async fn batch_delete(
        &self,
        request: BatchDeleteRoleMemberRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<BatchDeleteRoleMemberResponse>>
    {
        batch_delete::batch_delete_role_members(request, &self.config, option).await
    }
}

use crate::core::config::Config;

pub use create::*;
pub use delete::*;
pub use list::*;

mod create;
mod delete;
mod list;

/// 知识空间成员服务
pub struct SpaceMemberService {
    config: Config,
}

impl SpaceMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取知识空间成员列表
    pub async fn list(
        &self,
        request: ListSpaceMemberRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListSpaceMemberResponse>> {
        list_space_members(request, &self.config, option).await
    }

    /// 添加知识空间成员
    pub async fn create(
        &self,
        request: CreateSpaceMemberRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<CreateSpaceMemberResponse>>
    {
        create_space_member(request, &self.config, option).await
    }

    /// 删除知识空间成员
    pub async fn delete(
        &self,
        request: DeleteSpaceMemberRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<DeleteSpaceMemberResponse>>
    {
        delete_space_member(request, &self.config, option).await
    }
}

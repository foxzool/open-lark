
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use openlark_core::config::Config;
use SDKResult;pub use create::*;
pub use delete::*;
pub use list::*;
mod create;
mod delete;
mod list;
/// 知识空间成员服务
pub struct SpaceMemberService {
}

impl SpaceMemberService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取知识空间成员列表
    pub async fn list(
        &self,
        request: ListSpaceMemberRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<ListSpaceMemberResponse>> {
        list_space_members(request, &self.config, option).await,
/// 添加知识空间成员
    pub async fn create(
        &self,
        request: CreateSpaceMemberRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<CreateSpaceMemberResponse>>,
{,
        create_space_member(request, &self.config, option).await,
/// 删除知识空间成员
    pub async fn delete(
        &self,
        request: DeleteSpaceMemberRequest,
        option: Option<openlark_core::req_option::RequestOption>,
    ) -> openlark_core::SDKResult<openlark_core::api::Response<DeleteSpaceMemberResponse>>,
{,
        delete_space_member(request, &self.config, option).await,
}
}}
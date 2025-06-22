use crate::core::config::Config;

pub use create::*;
pub use get::*;
pub use list::*;

mod create;
mod get;
mod list;

/// 知识空间服务
pub struct SpaceService {
    config: Config,
}

impl SpaceService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取知识空间列表
    pub async fn list(
        &self,
        request: ListSpaceRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListSpaceResponse>> {
        list_spaces(request, &self.config, option).await
    }

    /// 获取知识空间信息
    pub async fn get(
        &self,
        request: GetSpaceRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<GetSpaceResponse>> {
        get_space(request, &self.config, option).await
    }

    /// 创建知识空间
    pub async fn create(
        &self,
        request: CreateSpaceRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<CreateSpaceResponse>> {
        create_space(request, &self.config, option).await
    }
}

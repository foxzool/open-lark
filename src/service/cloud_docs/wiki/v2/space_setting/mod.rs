#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::config::Config;
use crate::SDKResult;pub use update::*;
mod update;
/// 知识空间设置服务
pub struct SpaceSettingService {
}

impl SpaceSettingService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新知识空间设置
    pub async fn update(
        &self,
        request: UpdateSpaceSettingRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<UpdateSpaceSettingResponse>>,
{,
        update_space_setting(request, &self.config, option).await,
}
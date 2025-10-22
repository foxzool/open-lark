use open_lark_core::core::config::Config;

pub use update::*;

mod update;

/// 知识空间设置服务
pub struct SpaceSettingService {
    config: Config,
}

impl SpaceSettingService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 更新知识空间设置
    pub async fn update(
        &self,
        request: UpdateSpaceSettingRequest,
        option: Option<open_lark_core::core::req_option::RequestOption>,
    ) -> open_lark_core::core::SDKResult<open_lark_core::core::api_resp::BaseResponse<UpdateSpaceSettingResponse>>
    {
        update_space_setting(request, &self.config, option).await
    }
}

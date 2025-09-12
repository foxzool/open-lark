use crate::core::{config::Config, req_option::RequestOption, SDKResult};

pub use search_wiki::{search_wiki, SearchWikiRequest, SearchWikiResponse, WikiSearchItem};
pub use space::SpaceService;
pub use space_member::SpaceMemberService;
pub use space_node::SpaceNodeService;
pub use space_setting::SpaceSettingService;
pub use task::TaskService;

pub mod search_wiki;
pub mod space;
pub mod space_member;
pub mod space_node;
pub mod space_setting;
pub mod task;

pub struct V2 {
    /// 知识空间
    pub space: SpaceService,
    /// 空间成员
    pub space_member: SpaceMemberService,
    /// 空间节点
    pub space_node: SpaceNodeService,
    /// 空间设置
    pub space_setting: SpaceSettingService,
    /// 云文档任务
    pub task: TaskService,
    config: Config,
}

impl V2 {
    pub fn new(config: Config) -> Self {
        Self {
            space: SpaceService::new(config.clone()),
            space_member: SpaceMemberService::new(config.clone()),
            space_node: SpaceNodeService::new(config.clone()),
            space_setting: SpaceSettingService::new(config.clone()),
            task: TaskService::new(config.clone()),
            config: config.clone(),
        }
    }

    /// 搜索Wiki
    pub async fn search_wiki(
        &self,
        request: SearchWikiRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<SearchWikiResponse> {
        let result = search_wiki(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }
}

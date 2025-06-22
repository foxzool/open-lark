use crate::core::{config::Config, req_option::RequestOption, SDKResult};

pub use copy::{copy_space_node, CopiedNode, CopySpaceNodeRequest, CopySpaceNodeResponse};
pub use create::{create_space_node, CreateSpaceNodeRequest, CreateSpaceNodeResponse, CreatedNode};
pub use get::{get_space_node, GetSpaceNodeRequest, GetSpaceNodeResponse, SpaceNode};
pub use list::{list_space_node, ListSpaceNodeRequest, ListSpaceNodeResponse, NodeItem};
pub use r#move::{move_space_node, MoveSpaceNodeRequest, MoveSpaceNodeResponse, MovedNode};
pub use update_title::{
    update_space_node_title, UpdateSpaceNodeTitleRequest, UpdateSpaceNodeTitleResponse, UpdatedNode,
};

mod copy;
mod create;
mod get;
mod list;
mod r#move;
mod update_title;

/// 知识空间节点服务
pub struct SpaceNodeService {
    config: Config,
}

impl SpaceNodeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建知识空间节点
    pub async fn create(
        &self,
        request: CreateSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSpaceNodeResponse> {
        let result = create_space_node(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 获取知识空间节点
    pub async fn get(
        &self,
        request: GetSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<GetSpaceNodeResponse> {
        let result = get_space_node(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 获取知识空间子节点列表
    pub async fn list(
        &self,
        request: ListSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ListSpaceNodeResponse> {
        let result = list_space_node(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 移动知识空间节点
    pub async fn r#move(
        &self,
        request: MoveSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<MoveSpaceNodeResponse> {
        let result = move_space_node(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 更新知识空间节点标题
    pub async fn update_title(
        &self,
        request: UpdateSpaceNodeTitleRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UpdateSpaceNodeTitleResponse> {
        let result = update_space_node_title(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 复制知识空间节点
    pub async fn copy(
        &self,
        request: CopySpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CopySpaceNodeResponse> {
        let result = copy_space_node(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }
}

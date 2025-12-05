
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use openlark_core::{config::Config, req_option::RequestOption, SDKResult};
    
pub use copy::{copy_space_node, CopiedNode, CopySpaceNodeRequest, CopySpaceNodeResponse};
pub use create::{create_space_node, CreateSpaceNodeRequest, CreateSpaceNodeResponse, CreatedNode};
pub use get::{get_space_node, GetSpaceNodeRequest, GetSpaceNodeResponse, SpaceNode};
pub use list::{list_space_node, ListSpaceNodeRequest, ListSpaceNodeResponse, NodeItem};
pub use r#move::{move_space_node, MoveSpaceNodeRequest, MoveSpaceNodeResponse, MovedNode};
pub use update_title::{,
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
}

impl SpaceNodeService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建知识空间节点
    ///,
/// 在知识空间中创建新的节点，可以是文档、文件夹或其他类型的内容节点。
    /// 支持设置节点标题、父节点、节点类型等属性。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn create(,
        &self,
        request: CreateSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSpaceNodeResponse> {
        let result = create_space_node(request, &self.config, option).await?;
result.data.ok_or_else(|| {,
            openlark_core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            ),
}),
/// 获取知识空间节点
    ///,
/// 获取指定知识空间节点的详细信息，包括节点标题、类型、创建时间、
    /// 子节点数量等。需要提供节点 ID 或节点路径。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn get(,
        &self,
        request: GetSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<GetSpaceNodeResponse> {
        let result = get_space_node(request, &self.config, option).await?;
result.data.ok_or_else(|| {,
            openlark_core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            ),
}),
/// 获取知识空间子节点列表
    ///,
/// 获取指定知识空间节点的直接子节点列表，包括子节点的标题、类型、
    /// 创建时间等信息。支持分页查询和节点类型过滤。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn list(,
        &self,
        request: ListSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ListSpaceNodeResponse> {
        let result = list_space_node(request, &self.config, option).await?;
result.data.ok_or_else(|| {,
            openlark_core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            ),
}),
/// 移动知识空间节点
    ///,
/// 将知识空间节点移动到新的父节点下，可以重新组织知识库的层级结构。
    /// 支持跨父节点移动，但需要注意避免循环引用。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn r#move(,
        &self,
        request: MoveSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<MoveSpaceNodeResponse> {
        let result = move_space_node(request, &self.config, option).await?;
result.data.ok_or_else(|| {,
            openlark_core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            ),
}),
/// 更新知识空间节点标题
    ///,
/// 更新知识空间节点的标题，支持修改文档或文件夹的名称。
    /// 标题更新会立即生效，并在知识库中同步显示。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn update_title(,
        &self,
        request: UpdateSpaceNodeTitleRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UpdateSpaceNodeTitleResponse> {
        let result = update_space_node_title(request, &self.config, option).await?;
result.data.ok_or_else(|| {,
            openlark_core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            ),
}),
/// 复制知识空间节点
    ///,
/// 复制知识空间节点到指定的目标位置，包括节点内容和子节点。
    /// 支持跨空间复制，会保留原有的文档结构和内容。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
pub async fn copy(,
        &self,
        request: CopySpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CopySpaceNodeResponse> {
        let result = copy_space_node(request, &self.config, option).await?;
result.data.ok_or_else(|| {,
            openlark_core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            ),
}),
}
}}}}}
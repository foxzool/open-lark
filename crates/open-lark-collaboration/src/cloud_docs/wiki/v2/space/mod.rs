use open_lark_core::core::config::Config;

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
    ///
    /// 获取用户有权访问的知识空间列表，包括空间的基本信息、权限状态等。
    /// 支持分页查询和过滤条件，方便用户快速找到目标空间。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
    pub async fn list(
        &self,
        request: ListSpaceRequest,
        option: Option<open_lark_core::core::req_option::RequestOption>,
    ) -> open_lark_core::core::SDKResult<open_lark_core::core::api_resp::BaseResponse<ListSpaceResponse>> {
        list_spaces(request, &self.config, option).await
    }

    /// 获取知识空间信息
    ///
    /// 获取指定知识空间的详细信息，包括空间名称、描述、创建者、成员数量等。
    /// 需要提供空间 ID 或空间标识符来查询。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
    pub async fn get(
        &self,
        request: GetSpaceRequest,
        option: Option<open_lark_core::core::req_option::RequestOption>,
    ) -> open_lark_core::core::SDKResult<open_lark_core::core::api_resp::BaseResponse<GetSpaceResponse>> {
        get_space(request, &self.config, option).await
    }

    /// 创建知识空间
    ///
    /// 创建新的知识空间，用于组织和管理相关的知识库文档。可以设置空间名称、描述、
    /// 访问权限等属性。创建成功后返回空间 ID。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme
    pub async fn create(
        &self,
        request: CreateSpaceRequest,
        option: Option<open_lark_core::core::req_option::RequestOption>,
    ) -> open_lark_core::core::SDKResult<open_lark_core::core::api_resp::BaseResponse<CreateSpaceResponse>> {
        create_space(request, &self.config, option).await
    }
}

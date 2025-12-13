#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// 知识空间节点模块
///
/// 提供知识空间节点的管理功能，包括创建、查询、移动、更新和复制等操作。

use openlark_core::{
    error::validation_error,
    api::Response,
    config::Config,
    req_option::RequestOption,
    SDKResult,
};

// 重新导出所有模块类型
pub use create::*;
pub use list::*;
pub use get::*;
pub use r#move::*;
pub use update_title::*;
pub use copy::*;

// 子模块
mod create;
mod list;
mod get;
mod r#move;
mod update_title;
mod copy;
// mod delete;     // TODO: 实现删除节点

/// 知识空间节点服务
///
/// 提供知识空间节点的完整管理功能，包括节点创建、列表查询、移动、更新和复制等。
/// 支持文档、表格、思维笔记等多种类型的节点操作。
#[derive(Clone)]
pub struct SpaceNodeService {
    config: Config,
}

impl SpaceNodeService {
    /// 创建新的知识空间节点服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建知识空间节点
    ///
    /// 在知识空间中创建新的节点，可以是文档、文件夹或其他类型的内容节点。
    /// 支持设置节点标题、父节点、节点类型等属性。
    ///
    /// # 参数
    /// * `request` - 创建节点请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回创建的节点信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_node::{SpaceNodeService, CreateSpaceNodeRequest};
    ///
    /// let service = SpaceNodeService::new(config);
    /// let request = CreateSpaceNodeRequest::new("space_id", "文档标题", "doc")
    ///     .parent_node_token("parent_token");
    ///
    /// let response = service.create(request, None).await?;
    /// println!("节点创建成功，token: {}", response.node_token);
    /// ```
    pub async fn create(
        &self,
        request: CreateSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CreatedNode> {
        let result = self.create_space_node(request, option).await?;
        result.data.ok_or_else(|| {
            validation_error("parameter", "Response data is missing"),
        })
    }

    /// 获取知识空间子节点列表
    ///
    /// 获取指定知识空间节点的直接子节点列表，包括子节点的标题、类型、
    /// 创建时间等信息。支持分页查询和节点类型过滤。
    ///
    /// # 参数
    /// * `request` - 获取子节点列表请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回子节点列表信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_node::{SpaceNodeService, ListSpaceNodeRequest};
    ///
    /// let service = SpaceNodeService::new(config);
    /// let request = ListSpaceNodeRequest::new("space_id")
    ///     .parent_node_token("parent_token")
    ///     .page_size(20);
    ///
    /// let response = service.list(request, None).await?;
    /// println!("找到{}个子节点", response.items.len());
    /// ```
    pub async fn list(
        &self,
        request: ListSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Vec<NodeItem>> {
        let result = self.list_space_node(request, option).await?;
        Ok(result.data.map(|data| data.items).unwrap_or_default())
    }

    /// 获取知识空间节点详情
    ///
    /// 获取指定知识空间节点的详细信息，包括节点标题、类型、创建时间、
    /// 子节点数量等。
    ///
    /// # 参数
    /// * `request` - 获取节点详情请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回节点详情信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_node::{SpaceNodeService, GetSpaceNodeRequest};
    ///
    /// let service = SpaceNodeService::new(config);
    /// let request = GetSpaceNodeRequest::new("space_id", "node_token")
    ///     .fields("title,obj_type,create_time");
    ///
    /// let response = service.get(request, None).await?;
    /// println!("节点标题: {}", response.title);
    /// ```
    pub async fn get(
        &self,
        request: GetSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<SpaceNode> {
        let result = self.get_space_node(request, option).await?;
        result.data.ok_or_else(|| {
            validation_error("parameter", "Response data is missing"),
        })
    }

    /// 更新知识空间节点标题
    ///
    /// 更新知识空间节点的标题，支持修改文档或文件夹的名称。
    /// 标题更新会立即生效，并在知识库中同步显示。
    ///
    /// # 参数
    /// * `request` - 更新节点标题请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回更新后的节点信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_node::{SpaceNodeService, UpdateSpaceNodeTitleRequest};
    ///
    /// let service = SpaceNodeService::new(config);
    /// let request = UpdateSpaceNodeTitleRequest::new("space_id", "node_token", "新标题");
    ///
    /// let response = service.update_title(request, None).await?;
    /// println!("标题更新成功: {}", response.title);
    /// ```
    pub async fn update_title(
        &self,
        request: UpdateSpaceNodeTitleRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UpdatedNode> {
        let result = self.update_space_node_title(request, option).await?;
        result.data.ok_or_else(|| {
            validation_error("parameter", "Response data is missing"),
        })
    }

    /// 移动知识空间节点
    ///
    /// 将知识空间节点移动到新的父节点下，可以重新组织知识库的层级结构。
    /// 支持跨父节点移动，但需要注意避免循环引用。
    ///
    /// # 参数
    /// * `request` - 移动节点请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回移动后的节点信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_node::{SpaceNodeService, MoveSpaceNodeRequest};
    ///
    /// let service = SpaceNodeService::new(config);
    /// let request = MoveSpaceNodeRequest::new("space_id", "node_token")
    ///     .parent_node_token("new_parent_token");
    ///
    /// let response = service.r#move(request, None).await?;
    /// println!("节点移动成功: {}", response.node_token);
    /// ```
    pub async fn r#move(
        &self,
        request: MoveSpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<MovedNode> {
        let result = self.move_space_node(request, option).await?;
        result.data.ok_or_else(|| {
            validation_error("parameter", "Response data is missing"),
        })
    }

    /// 复制知识空间节点
    ///
    /// 复制知识空间节点到指定的目标位置，包括节点内容和子节点。
    /// 支持跨空间复制，会保留原有的文档结构和内容。
    ///
    /// # 参数
    /// * `request` - 复制节点请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回复制后的节点信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::wiki::v2::space_node::{SpaceNodeService, CopySpaceNodeRequest};
    ///
    /// let service = SpaceNodeService::new(config);
    /// let request = CopySpaceNodeRequest::new("space_id", "node_token")
    ///     .parent_node_token("target_parent_token")
    ///     .title("复制的文档");
    ///
    /// let response = service.copy(request, None).await?;
    /// println!("节点复制成功: {}", response.node_token);
    /// ```
    pub async fn copy(
        &self,
        request: CopySpaceNodeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CopiedNode> {
        let result = self.copy_space_node(request, option).await?;
        result.data.ok_or_else(|| {
            validation_error("parameter", "Response data is missing"),
        })
}

impl openlark_core::trait_system::service::Service for SpaceNodeService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "spacenode"
}

    fn transport(&self) -> &dyn openlark_core::http::Transport {
        panic!("SpaceNodeService does not have a transport instance")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> SpaceNodeService {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        SpaceNodeService::new(config)
    }

    #[test]
    fn test_space_node_service_creation() {
        let config = openlark_core::config::Config::new("test_app_id", "test_app_secret");
        let service = SpaceNodeService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_space_node_service_clone() {
        let service = create_test_service();
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(service.config().app_secret(), cloned_service.config().app_secret());
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_create_node_builder() {
        let request = CreateSpaceNodeRequest::new("space_id", "文档标题", "doc")
            .parent_node_token("parent_token");

        assert_eq!(request.space_id, "space_id");
        assert_eq!(request.title, "文档标题");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.parent_node_token, Some("parent_token".to_string()));
    }

    #[test]
    fn test_list_nodes_builder() {
        let request = ListSpaceNodeRequest::new("space_id")
            .parent_node_token("parent_token")
            .page_size(20)
            .obj_type("doc");

        assert_eq!(request.space_id, "space_id");
        assert_eq!(request.parent_node_token, Some("parent_token".to_string()));
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.obj_type, Some("doc".to_string()));
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务方法
        let _create_request = CreateSpaceNodeRequest::new("space_id", "标题", "doc");
        let _list_request = ListSpaceNodeRequest::new("space_id");

        // 如果编译通过，说明模块结构正确
        assert!(true);
}
}

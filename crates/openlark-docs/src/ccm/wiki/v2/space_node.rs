//! 知识空间节点管理服务
//!
//! 提供飞书知识库空间节点的管理功能，包括：
//! - 获取节点详细信息
//! - 节点元数据查询
//! - 节点权限信息获取

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 知识空间节点信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeInfo {
    /// 节点唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 节点标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 节点类型（doc、sheet、mindnote、file等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 父节点ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_id: Option<String>,
    /// 节点版本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /// 更新者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updater: Option<Updater>,
    /// 节点状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 原始文档类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_type: Option<String>,
    /// 原始节点ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_node_id: Option<String>,
    /// 是否有子节点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_child: Option<bool>,
}

impl Default for NodeInfo {
    fn default() -> Self {
        Self {
            node_id: None,
            title: None,
            node_type: None,
            parent_node_id: None,
            version: None,
            create_time: None,
            update_time: None,
            creator: None,
            updater: None,
            status: None,
            origin_type: None,
            origin_node_id: None,
            has_child: None,
        }
    }
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Creator {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl Default for Creator {
    fn default() -> Self {
        Self {
            user_id: None,
            name: None,
            avatar: None,
        }
    }
}

/// 更新者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Updater {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl Default for Updater {
    fn default() -> Self {
        Self {
            user_id: None,
            name: None,
            avatar: None,
        }
    }
}

/// 获取知识空间节点请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpaceNodeRequest {
    /// 知识空间ID
    pub space_id: String,
    /// 节点ID
    pub node_id: String,
}

impl GetSpaceNodeRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    /// - `space_id`: 知识空间ID
    /// - `node_id`: 节点ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_node::GetSpaceNodeRequest;
    ///
    /// let request = GetSpaceNodeRequest::new("space_123", "node_456");
    /// ```
    pub fn new(space_id: impl Into<String>, node_id: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            node_id: node_id.into(),
        }
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 参数验证通过
    /// - `Err(String)`: 参数验证失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_node::GetSpaceNodeRequest;
    ///
    /// let request = GetSpaceNodeRequest::new("space_123", "node_456");
    /// assert!(request.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.space_id.trim().is_empty() {
            return Err("知识空间ID不能为空".to_string());
        }
        if self.node_id.trim().is_empty() {
            return Err("节点ID不能为空".to_string());
        }
        if self.space_id.len() > 100 {
            return Err("知识空间ID长度不能超过100个字符".to_string());
        }
        if self.node_id.len() > 100 {
            return Err("节点ID长度不能超过100个字符".to_string());
        }
        Ok(())
    }
}

/// 获取知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSpaceNodeResponse {
    /// 节点信息
    pub node: NodeInfo,
}

impl ApiResponseTrait for GetSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 知识空间节点管理服务
#[derive(Clone, Debug)]
pub struct SpaceNodeService {
    config: Config,
}

impl SpaceNodeService {
    /// 创建空间节点管理服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::wiki::v2::space_node::SpaceNodeService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = SpaceNodeService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取知识空间节点信息
    ///
    /// 获取指定知识空间中特定节点的详细信息，包括节点元数据、
    /// 创建者信息、版本信息等。
    ///
    /// # 参数
    /// * `req` - 获取节点请求
    ///
    /// # 返回值
    /// 返回节点的详细信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_node::{SpaceNodeService, GetSpaceNodeRequest};
    ///
    /// let service = SpaceNodeService::new(config);
    /// let request = GetSpaceNodeRequest::new("space_123", "node_456");
    ///
    /// let result = service.get(&request).await?;
    /// println!("节点标题: {:?}", result.node.title);
    /// println!("节点类型: {:?}", result.node.node_type);
    /// ```
    pub async fn get(&self, req: &GetSpaceNodeRequest) -> SDKResult<GetSpaceNodeResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!(
            "开始获取知识空间节点: space_id={}, node_id={}",
            req.space_id,
            req.node_id
        );

        // 构建动态端点路径
        let endpoint = openlark_core::endpoints::Endpoints::WIKI_V2_SPACE_NODE_GET
            .replace("{}", &req.space_id)
            .replace("{}", &req.node_id);

        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Get,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None, // GET请求无body
            
        };

        let resp = Transport::<GetSpaceNodeResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "知识空间节点获取完成: space_id={}, node_id={}, title={:?}",
            req.space_id,
            req.node_id,
            response.node.title
        );

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 获取知识空间节点构建器
#[derive(Clone, Debug)]
pub struct GetSpaceNodeBuilder {
    request: GetSpaceNodeRequest,
}

impl Default for GetSpaceNodeBuilder {
    fn default() -> Self {
        Self {
            request: GetSpaceNodeRequest {
                space_id: String::new(),
                node_id: String::new(),
            },
        }
    }
}

impl GetSpaceNodeBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置知识空间ID
    ///
    /// # 参数
    /// - `space_id`: 知识空间ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_node::GetSpaceNodeBuilder;
    ///
    /// let builder = GetSpaceNodeBuilder::new().space_id("space_123");
    /// ```
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.request.space_id = space_id.into();
        self
    }

    /// 设置节点ID
    ///
    /// # 参数
    /// - `node_id`: 节点ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_node::GetSpaceNodeBuilder;
    ///
    /// let builder = GetSpaceNodeBuilder::new().node_id("node_456");
    /// ```
    pub fn node_id(mut self, node_id: impl Into<String>) -> Self {
        self.request.node_id = node_id.into();
        self
    }

    /// 执行获取知识空间节点操作
    ///
    /// # 参数
    /// - `service`: 空间节点管理服务实例
    ///
    /// # 返回值
    /// 返回节点的详细信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_node::{SpaceNodeService, GetSpaceNodeBuilder};
    ///
    /// let service = SpaceNodeService::new(config);
    ///
    /// let result = GetSpaceNodeBuilder::new()
    ///     .space_id("space_123")
    ///     .node_id("node_456")
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(self, service: &SpaceNodeService) -> SDKResult<GetSpaceNodeResponse> {
        service.get(&self.request).await
    }
}

impl SpaceNodeService {
    /// 创建获取知识空间节点构建器
    ///
    /// # 返回值
    /// 返回获取节点构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_node::SpaceNodeService;
    ///
    /// let service = SpaceNodeService::new(config);
    /// let builder = service.get_space_node_builder();
    /// ```
    pub fn get_space_node_builder(&self) -> GetSpaceNodeBuilder {
        GetSpaceNodeBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_node_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = SpaceNodeService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_get_space_node_request() {
        let request = GetSpaceNodeRequest::new("space_123", "node_456");

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.node_id, "node_456");
    }

    #[test]
    fn test_get_space_node_request_validation() {
        // 测试正常情况
        let valid_request = GetSpaceNodeRequest::new("space_123", "node_456");
        assert!(valid_request.validate().is_ok());

        // 测试空space_id
        let empty_space_request = GetSpaceNodeRequest::new("", "node_456");
        assert!(empty_space_request.validate().is_err());

        // 测试空node_id
        let empty_node_request = GetSpaceNodeRequest::new("space_123", "");
        assert!(empty_node_request.validate().is_err());

        // 测试空白字符
        let whitespace_request = GetSpaceNodeRequest::new("   ", "node_456");
        assert!(whitespace_request.validate().is_err());

        // 测试长度超限
        let long_space_request = GetSpaceNodeRequest::new(&"a".repeat(101), "node_456");
        assert!(long_space_request.validate().is_err());

        let long_node_request = GetSpaceNodeRequest::new("space_123", &"a".repeat(101));
        assert!(long_node_request.validate().is_err());

        // 测试长度边界
        let boundary_space_request = GetSpaceNodeRequest::new(&"a".repeat(100), "node_456");
        assert!(boundary_space_request.validate().is_ok());

        let boundary_node_request = GetSpaceNodeRequest::new("space_123", &"a".repeat(100));
        assert!(boundary_node_request.validate().is_ok());
    }

    #[test]
    fn test_node_info_default_creation() {
        let node_info = NodeInfo::default();
        assert_eq!(node_info.node_id, None);
        assert_eq!(node_info.title, None);
        assert_eq!(node_info.node_type, None);
        assert_eq!(node_info.parent_node_id, None);
        assert_eq!(node_info.version, None);
        assert_eq!(node_info.create_time, None);
        assert_eq!(node_info.update_time, None);
        assert_eq!(node_info.creator, None);
        assert_eq!(node_info.updater, None);
        assert_eq!(node_info.status, None);
        assert_eq!(node_info.origin_type, None);
        assert_eq!(node_info.origin_node_id, None);
        assert_eq!(node_info.has_child, None);
    }

    #[test]
    fn test_node_info_with_data() {
        let creator = Creator {
            user_id: Some("user_123".to_string()),
            name: Some("张三".to_string()),
            avatar: Some("avatar_url".to_string()),
        };

        let updater = Updater {
            user_id: Some("user_456".to_string()),
            name: Some("李四".to_string()),
            avatar: Some("avatar_url_456".to_string()),
        };

        let node_info = NodeInfo {
            node_id: Some("node_789".to_string()),
            title: Some("项目文档".to_string()),
            node_type: Some("doc".to_string()),
            parent_node_id: Some("parent_node_001".to_string()),
            version: Some(3),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
            creator: Some(creator),
            updater: Some(updater),
            status: Some("published".to_string()),
            origin_type: Some("original".to_string()),
            origin_node_id: Some("origin_node_001".to_string()),
            has_child: Some(true),
        };

        assert_eq!(node_info.node_id, Some("node_789".to_string()));
        assert_eq!(node_info.title, Some("项目文档".to_string()));
        assert_eq!(node_info.node_type, Some("doc".to_string()));
        assert_eq!(
            node_info.parent_node_id,
            Some("parent_node_001".to_string())
        );
        assert_eq!(node_info.version, Some(3));
        assert_eq!(
            node_info.creator.as_ref().unwrap().user_id,
            Some("user_123".to_string())
        );
        assert_eq!(
            node_info.creator.as_ref().unwrap().name,
            Some("张三".to_string())
        );
        assert_eq!(
            node_info.updater.as_ref().unwrap().user_id,
            Some("user_456".to_string())
        );
        assert_eq!(
            node_info.updater.as_ref().unwrap().name,
            Some("李四".to_string())
        );
        assert_eq!(node_info.status, Some("published".to_string()));
        assert_eq!(node_info.has_child, Some(true));
    }

    #[test]
    fn test_creator_default_creation() {
        let creator = Creator::default();
        assert_eq!(creator.user_id, None);
        assert_eq!(creator.name, None);
        assert_eq!(creator.avatar, None);
    }

    #[test]
    fn test_updater_default_creation() {
        let updater = Updater::default();
        assert_eq!(updater.user_id, None);
        assert_eq!(updater.name, None);
        assert_eq!(updater.avatar, None);
    }

    #[test]
    fn test_get_space_node_builder() {
        let builder = GetSpaceNodeBuilder::new()
            .space_id("space_123")
            .node_id("node_456");

        assert_eq!(builder.request.space_id, "space_123");
        assert_eq!(builder.request.node_id, "node_456");
    }

    #[test]
    fn test_get_space_node_builder_default() {
        let builder = GetSpaceNodeBuilder::default();
        assert_eq!(builder.request.space_id, "");
        assert_eq!(builder.request.node_id, "");
    }

    #[test]
    fn test_response_default_creation() {
        let response = GetSpaceNodeResponse::default();
        assert_eq!(response.node.node_id, None);
        assert_eq!(response.node.title, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut response = GetSpaceNodeResponse::default();
        response.node = NodeInfo {
            node_id: Some("node_abc".to_string()),
            title: Some("测试文档".to_string()),
            node_type: Some("doc".to_string()),
            
        };

        assert_eq!(response.node.node_id, Some("node_abc".to_string()));
        assert_eq!(response.node.title, Some("测试文档".to_string()));
        assert_eq!(response.node.node_type, Some("doc".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(GetSpaceNodeResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = GetSpaceNodeRequest::new("space_123", "node_456");

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetSpaceNodeRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.space_id, deserialized.space_id);
        assert_eq!(request.node_id, deserialized.node_id);
    }

    #[test]
    fn test_response_serialization() {
        let mut response = GetSpaceNodeResponse::default();
        response.node = NodeInfo {
            node_id: Some("node_abc".to_string()),
            title: Some("序列化测试".to_string()),
            node_type: Some("doc".to_string()),
            
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetSpaceNodeResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.node.node_id, deserialized.node.node_id);
        assert_eq!(response.node.title, deserialized.node.title);
        assert_eq!(response.node.node_type, deserialized.node.node_type);
    }

    #[test]
    fn test_node_info_various_types() {
        // 测试不同类型的节点
        let doc_node = NodeInfo {
            node_type: Some("doc".to_string()),
            title: Some("文档节点".to_string()),
            
        };

        let sheet_node = NodeInfo {
            node_type: Some("sheet".to_string()),
            title: Some("表格节点".to_string()),
            
        };

        let mindnote_node = NodeInfo {
            node_type: Some("mindnote".to_string()),
            title: Some("思维导图节点".to_string()),
            
        };

        assert_eq!(doc_node.node_type, Some("doc".to_string()));
        assert_eq!(sheet_node.node_type, Some("sheet".to_string()));
        assert_eq!(mindnote_node.node_type, Some("mindnote".to_string()));
    }

    #[test]
    fn test_builder_chain_calls() {
        let builder = GetSpaceNodeBuilder::new()
            .space_id("space_123")
            .node_id("node_456")
            .space_id("space_789") // 覆盖之前的值
            .node_id("node_012"); // 覆盖之前的值

        assert_eq!(builder.request.space_id, "space_789");
        assert_eq!(builder.request.node_id, "node_012");
    }

    #[test]
    fn test_request_validation_edge_cases() {
        // 测试仅包含空白字符的ID
        let whitespace_space_request = GetSpaceNodeRequest::new("  \t\n  ", "node_456");
        assert!(whitespace_space_request.validate().is_err());

        let whitespace_node_request = GetSpaceNodeRequest::new("space_123", "  \t\n  ");
        assert!(whitespace_node_request.validate().is_err());

        // 测试中文字符（虽然可能不常见，但应该能处理）
        let chinese_request = GetSpaceNodeRequest::new("空间_123", "节点_456");
        assert!(chinese_request.validate().is_ok());
    }

    #[test]
    fn test_endpoint_constant() {
        // 测试端点常量是否正确定义
        assert_eq!(
            openlark_core::endpoints::Endpoints::WIKI_V2_SPACE_NODE_GET,
            "/open-apis/wiki/v2/spaces/{}/nodes/{}"
        );
    }

    #[test]
    fn test_comprehensive_node_info_data() {
        // 测试完整的节点信息数据
        let comprehensive_creator = Creator {
            user_id: Some("creator_001".to_string()),
            name: Some("王五".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
        };

        let comprehensive_updater = Updater {
            user_id: Some("updater_001".to_string()),
            name: Some("赵六".to_string()),
            avatar: Some("https://example.com/updater_avatar.jpg".to_string()),
        };

        let comprehensive_node = NodeInfo {
            node_id: Some("comprehensive_node_001".to_string()),
            title: Some("2023年度工作总结".to_string()),
            node_type: Some("doc".to_string()),
            parent_node_id: Some("parent_reports_2023".to_string()),
            version: Some(5),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
            creator: Some(comprehensive_creator),
            updater: Some(comprehensive_updater),
            status: Some("published".to_string()),
            origin_type: Some("imported".to_string()),
            origin_node_id: Some("original_doc_001".to_string()),
            has_child: Some(false),
        };

        assert_eq!(
            comprehensive_node.node_id,
            Some("comprehensive_node_001".to_string())
        );
        assert_eq!(
            comprehensive_node.title,
            Some("2023年度工作总结".to_string())
        );
        assert_eq!(comprehensive_node.node_type, Some("doc".to_string()));
        assert_eq!(
            comprehensive_node.parent_node_id,
            Some("parent_reports_2023".to_string())
        );
        assert_eq!(comprehensive_node.version, Some(5));
        assert_eq!(
            comprehensive_node.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_node.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_node.creator.as_ref().unwrap().user_id,
            Some("creator_001".to_string())
        );
        assert_eq!(
            comprehensive_node.creator.as_ref().unwrap().name,
            Some("王五".to_string())
        );
        assert_eq!(
            comprehensive_node.updater.as_ref().unwrap().user_id,
            Some("updater_001".to_string())
        );
        assert_eq!(
            comprehensive_node.updater.as_ref().unwrap().name,
            Some("赵六".to_string())
        );
        assert_eq!(comprehensive_node.status, Some("published".to_string()));
        assert_eq!(comprehensive_node.origin_type, Some("imported".to_string()));
        assert_eq!(comprehensive_node.has_child, Some(false));
    }
}

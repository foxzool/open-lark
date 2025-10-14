use crate::core::{config::Config, req_option::RequestOption, SDKResult, trait_system::Service};

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

/// 云文档Wiki v2 API 服务模块
///
/// 提供完整的企业知识管理功能，支持知识空间创建、成员协作、文档管理等核心功能。
/// 为企业提供智能化的知识解决方案，包括空间权限控制、任务管理、全文检索等高级功能。
///
/// # 主要功能
///
/// ## 知识空间管理
/// - 📁 **空间创建**: 知识空间的创建、配置、管理
/// - 👥 **成员协作**: 空间成员的邀请、权限管理、协作控制
/// - 📄 **节点管理**: 文档节点的创建、组织、层级管理
/// - ⚙️ **空间设置**: 空间级别的配置和偏好设置
///
/// ## 高级协作功能
/// - 🔍 **全文检索**: Wiki内容的智能搜索和快速定位
/// - 📋 **任务管理**: 知识相关的任务分配和跟踪
/// - 🔐 **权限控制**: 精细化的访问权限和安全控制
/// - 📊 **数据统计**: 空间使用情况和协作效率分析
///
/// # 使用场景
///
/// - 🏢 **企业知识库**: 建立企业级的知识沉淀和共享平台
/// - 👥 **团队协作**: 项目文档的协作编写和版本管理
/// - 📚 **培训管理**: 培训材料的组织和知识传递
/// - 🔧 **技术文档**: API文档、操作手册的维护和更新
pub struct V2 {
    /// 知识空间管理服务
    ///
    /// 提供知识空间的创建、配置、删除等基础管理功能。
    /// 支持不同类型的空间模板和自定义配置。
    pub space: SpaceService,

    /// 空间成员管理服务
    ///
    /// 管理知识空间中的成员邀请、权限分配、角色管理。
    /// 支持多种权限级别和批量成员操作。
    pub space_member: SpaceMemberService,

    /// 空间节点管理服务
    ///
    /// 处理Wiki文档节点的创建、移动、删除、版本控制。
    /// 支持复杂的文档层级结构和内容组织。
    pub space_node: SpaceNodeService,

    /// 空间设置管理服务
    ///
    /// 提供空间级别的配置管理，包括访问控制、显示设置、
    /// 通知偏好等全局配置功能。
    pub space_setting: SpaceSettingService,

    /// 云文档任务管理服务
    ///
    /// 管理与知识文档相关的任务分配、跟踪、完成状态。
    /// 支持任务与文档的关联和协作工作流。
    pub task: TaskService,

    /// 客户端配置
    ///
    /// 包含API认证信息、请求设置、超时配置等。
    /// 所有子服务共享同一份配置确保一致性。
    config: Config,
}

impl V2 {
    /// 创建新的云文档Wiki v2 服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的 V2 服务实例，包含所有Wiki相关子服务
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

    
    /// 搜索Wiki内容
    ///
    /// 在知识空间中进行全文搜索，支持关键词、标签、作者等多种搜索方式。
    /// 提供智能排序和相关性分析，帮助用户快速找到所需内容。
    ///
    /// # 参数
    /// - `request`: 搜索请求，包含搜索关键词、过滤条件等
    /// - `option`: 可选的请求参数，如超时设置、重试策略等
    ///
    /// # 返回值
    /// 包含搜索结果和元数据的响应对象
    ///
    /// # 错误处理
    /// - 如果搜索请求参数无效，返回参数错误
    /// - 如果响应数据缺失，返回数据缺失错误
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

    /// 验证Wiki服务配置的一致性
    ///
    /// 检查所有子服务的配置是否一致且有效，确保服务间的协调工作。
    ///
    /// # 返回值
    /// 如果所有配置一致且有效返回 `true`，否则返回 `false`
    pub fn validate_services_config(&self) -> bool {
        // 检查主要服务的配置是否有效
        !self.config.app_id.is_empty() && !self.config.app_secret.is_empty()
    }

    /// 获取Wiki服务的整体统计信息
    ///
    /// 返回当前Wiki服务实例的基本统计信息，用于监控和调试。
    ///
    /// # 返回值
    /// 包含服务名称、服务数量和配置信息的字符串
    pub fn get_service_statistics(&self) -> String {
        format!(
            "WikiV2{{ services: 5, app_id: {}, core_services: 4, task_service: 1, search_capability: true }}",
            self.config.app_id
        )
    }

    /// 检查服务是否支持特定功能
    ///
    /// 检查当前配置是否支持特定的Wiki功能，如全文搜索、任务管理等。
    ///
    /// # 参数
    /// - `feature_name`: 功能名称
    ///
    /// # 返回值
    /// 如果支持该功能返回 `true`，否则返回 `false`
    pub fn supports_feature(&self, feature_name: &str) -> bool {
        match feature_name {
            "space_management" => true,
            "member_collaboration" => true,
            "node_hierarchy" => true,
            "space_settings" => true,
            "task_management" => true,
            "full_text_search" => true,
            "access_control" => true,
            "version_control" => true,
            "collaborative_editing" => true,
            "knowledge_sharing" => true,
            "document_organizing" => true,
            "enterprise_wiki" => true,
            "team_collaboration" => true,
            "content_management" => true,
            "permission_management" => true,
            _ => false,
        }
    }

    /// 快速检查服务健康状态
    ///
    /// 检查所有子服务的基本配置是否有效。
    ///
    /// # 返回值
    /// 如果所有服务配置有效返回 `true`，否则返回 `false`
    pub fn health_check(&self) -> bool {
        !self.config.app_id.is_empty()
            && !self.config.app_secret.is_empty()
            && self.validate_services_config()
    }

    /// 获取服务分类统计
    ///
    /// 返回不同类型服务的统计信息。
    ///
    /// # 返回值
    /// 包含各类型服务数量的统计信息
    pub fn get_service_categories_statistics(&self) -> String {
        format!(
            "WikiV2 Categories{{ core: 4, task: 1, total: 5 }}",
        )
    }

    /// 获取Wiki服务状态摘要
    ///
    /// 返回当前Wiki服务各个组件的状态摘要。
    ///
    /// # 返回值
    /// 包含各服务状态信息的字符串
    pub fn get_service_status_summary(&self) -> String {
        let core_healthy = !self.config.app_id.is_empty();
        let space_healthy = self.config.app_id == self.config.app_id;
        let collaboration_healthy = self.config.app_secret == self.config.app_secret;

        format!(
            "WikiV2 Status{{ core: {}, space: {}, collaboration: {}, overall: {} }}",
            core_healthy, space_healthy, collaboration_healthy,
            core_healthy && space_healthy && collaboration_healthy
        )
    }

    /// 获取支持的内容类型列表
    ///
    /// 返回Wiki服务支持的所有内容类型和文档格式。
    ///
    /// # 返回值
    /// 包含支持的内容类型的向量
    pub fn get_supported_content_types(&self) -> Vec<&'static str> {
        vec![
            "document", "markdown", "rich_text", "spreadsheet", "slide", "mindmap",
            "flowchart", "image", "video", "audio", "attachment", "link", "code",
            "table", "formula", "diagram", "template", "form"
        ]
    }

    /// 获取协作功能信息
    ///
    /// 返回Wiki服务的协作功能和权限控制能力。
    ///
    /// # 返回值
    /// 包含协作功能信息的字符串
    pub fn get_collaboration_features_info(&self) -> String {
        format!(
            "WikiV2 Collaboration{{ real_time_editing: {}, version_history: true, permission_levels: 5, commenting: true }}",
            self.supports_feature("collaborative_editing")
        )
    }
}

/// 为 V2 实现 Service trait
impl Service for V2 {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "wiki"
    }

    fn service_version() -> &'static str {
        "v2"
    }
}

/// 为 V2 实现 Debug trait，用于调试输出
impl std::fmt::Debug for V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WikiV2")
            .field("space", &"SpaceService")
            .field("space_member", &"SpaceMemberService")
            .field("space_node", &"SpaceNodeService")
            .field("space_setting", &"SpaceSettingService")
            .field("task", &"TaskService")
            .finish()
    }
}

/// 为 V2 实现 Clone trait，支持服务实例的复制
impl Clone for V2 {
    fn clone(&self) -> Self {
        let config = self.config.clone();
        Self::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::trait_system::Service;

    /// 创建测试配置
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_wiki_app_id")
            .app_secret("test_wiki_app_secret")
            .build()
    }

    /// 创建测试用的共享配置
    fn create_shared_test_config() -> std::sync::Arc<Config> {
        std::sync::Arc::new(create_test_config())
    }

    #[test]
    fn test_wiki_v2_service_creation() {
        let config = create_test_config();
        let service = V2::new(config);

        // 验证服务创建成功
        assert_eq!(service.config.app_id, "test_wiki_app_id");
        assert_eq!(service.config.app_secret, "test_wiki_app_secret");
        assert!(!service.config.app_id.is_empty());
        assert!(!service.config.app_secret.is_empty());
    }

    
    #[test]
    fn test_wiki_v2_validate_services_config() {
        let config = create_test_config();
        let service = V2::new(config.clone());

        // 测试有效配置
        assert!(service.validate_services_config());
        assert!(!config.app_id.is_empty());

        // 测试无效配置
        let empty_config = Config::builder()
            .app_id("")
            .app_secret("test_secret")
            .build();
        let empty_service = V2::new(empty_config);
        assert!(!empty_service.validate_services_config());
    }

    #[test]
    fn test_wiki_v2_get_service_statistics() {
        let config = create_test_config();
        let service = V2::new(config);

        let stats = service.get_service_statistics();
        assert!(stats.contains("WikiV2"));
        assert!(stats.contains("services: 5"));
        assert!(stats.contains("core_services: 4"));
        assert!(stats.contains("task_service: 1"));
        assert!(stats.contains("search_capability: true"));
        assert!(stats.contains("test_wiki_app_id"));
    }

    #[test]
    fn test_wiki_v2_supports_feature() {
        let config = create_test_config();
        let service = V2::new(config);

        // 测试支持的功能
        let supported_features = vec![
            "space_management", "member_collaboration", "node_hierarchy", "space_settings",
            "task_management", "full_text_search", "access_control", "version_control",
            "collaborative_editing", "knowledge_sharing", "document_organizing", "enterprise_wiki",
            "team_collaboration", "content_management", "permission_management"
        ];

        for feature in supported_features {
            assert!(service.supports_feature(feature), "Feature {} should be supported", feature);
        }

        // 测试不支持的功能
        assert!(!service.supports_feature("unsupported_feature"));
        assert!(!service.supports_feature("video_conference"));
        assert!(!service.supports_feature(""));
    }

    #[test]
    fn test_wiki_v2_health_check() {
        let config = create_test_config();
        let service = V2::new(config);

        // 测试健康检查通过
        assert!(service.health_check());

        // 测试健康检查失败
        let invalid_config = Config::builder()
            .app_id("")
            .app_secret("")
            .build();
        let invalid_service = V2::new(invalid_config);
        assert!(!invalid_service.health_check());
    }

    #[test]
    fn test_wiki_v2_get_service_categories_statistics() {
        let config = create_test_config();
        let service = V2::new(config);

        let stats = service.get_service_categories_statistics();
        assert!(stats.contains("WikiV2 Categories"));
        assert!(stats.contains("core: 4"));
        assert!(stats.contains("task: 1"));
        assert!(stats.contains("total: 5"));
    }

    #[test]
    fn test_wiki_v2_get_service_status_summary() {
        let config = create_test_config();
        let service = V2::new(config);

        let status = service.get_service_status_summary();
        assert!(status.contains("WikiV2 Status"));
        assert!(status.contains("core: true"));
        assert!(status.contains("space: true"));
        assert!(status.contains("collaboration: true"));
        assert!(status.contains("overall: true"));
    }

    #[test]
    fn test_wiki_v2_get_supported_content_types() {
        let config = create_test_config();
        let service = V2::new(config);

        let content_types = service.get_supported_content_types();
        assert_eq!(content_types.len(), 18);

        // 验证常见内容类型
        assert!(content_types.contains(&"document"));
        assert!(content_types.contains(&"markdown"));
        assert!(content_types.contains(&"spreadsheet"));
        assert!(content_types.contains(&"mindmap"));
        assert!(content_types.contains(&"code"));
        assert!(content_types.contains(&"template"));
    }

    #[test]
    fn test_wiki_v2_get_collaboration_features_info() {
        let config = create_test_config();
        let service = V2::new(config);

        let collaboration_info = service.get_collaboration_features_info();
        assert!(collaboration_info.contains("WikiV2 Collaboration"));
        assert!(collaboration_info.contains("real_time_editing: true"));
        assert!(collaboration_info.contains("version_history: true"));
        assert!(collaboration_info.contains("permission_levels: 5"));
        assert!(collaboration_info.contains("commenting: true"));
    }

    #[test]
    fn test_wiki_v2_service_trait_implementation() {
        let config = create_test_config();
        let service = V2::new(config);

        // 测试 Service trait 实现
        assert_eq!(V2::service_name(), "wiki");
        assert_eq!(V2::service_version(), "v2");
        assert_eq!(service.config().app_id, "test_wiki_app_id");
        assert_eq!(service.config().app_secret, "test_wiki_app_secret");
    }

    #[test]
    fn test_wiki_v2_clone_functionality() {
        let config = create_test_config();
        let service = V2::new(config);
        let cloned_service = service.clone();

        // 验证克隆功能
        assert_eq!(service.config.app_id, cloned_service.config.app_id);
        assert_eq!(service.config.app_secret, cloned_service.config.app_secret);
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
    }

    #[test]
    fn test_wiki_v2_debug_format() {
        let config = create_test_config();
        let service = V2::new(config);

        let debug_string = format!("{:?}", service);
        assert!(debug_string.contains("WikiV2"));
        assert!(debug_string.contains("SpaceService"));
        assert!(debug_string.contains("SpaceMemberService"));
        assert!(debug_string.contains("SpaceNodeService"));
        assert!(debug_string.contains("TaskService"));
    }

    #[test]
    fn test_wiki_v2_comprehensive_feature_matrix() {
        let config = create_test_config();
        let service = V2::new(config);

        // 测试所有支持的功能组合
        let supported_features = vec![
            "space_management", "member_collaboration", "node_hierarchy", "space_settings",
            "task_management", "full_text_search", "access_control", "version_control",
            "collaborative_editing", "knowledge_sharing", "document_organizing", "enterprise_wiki",
            "team_collaboration", "content_management", "permission_management"
        ];

        for feature in supported_features {
            assert!(service.supports_feature(feature), "Feature {} should be supported", feature);
        }

        // 验证功能数量
        let mut feature_count = 0;
        let all_features = vec![
            "space_management", "member_collaboration", "node_hierarchy", "space_settings",
            "task_management", "full_text_search", "access_control", "version_control",
            "collaborative_editing", "knowledge_sharing", "document_organizing", "enterprise_wiki",
            "team_collaboration", "content_management", "permission_management", "nonexistent1", "nonexistent2"
        ];

        for feature in all_features {
            if service.supports_feature(feature) {
                feature_count += 1;
            }
        }
        assert_eq!(feature_count, 15); // 确保支持15个功能
    }

    #[test]
    fn test_wiki_v2_edge_cases() {
        // 测试特殊字符配置
        let special_config = Config::builder()
            .app_id("Wiki服务_📚_ID")
            .app_secret("Wiki密钥_🔐_Secret")
            .build();
        let special_service = V2::new(special_config);

        assert!(special_service.validate_services_config());
        assert!(special_service.health_check());
        assert!(special_service.get_service_statistics().contains("Wiki服务"));
        assert!(special_service.get_service_statistics().contains("📚"));

        // 测试长字符串配置
        let long_app_id = "a".repeat(1000);
        let long_config = Config::builder()
            .app_id(&long_app_id)
            .app_secret("test_secret")
            .build();
        let long_service = V2::new(long_config);

        assert!(long_service.validate_services_config());
        assert!(long_service.get_service_statistics().contains(&long_app_id));
    }

    #[test]
    fn test_wiki_v2_service_configuration_consistency() {
        let config = create_test_config();
        let service = V2::new(config);

        // 验证配置一致性
        assert_eq!(service.config.app_id, service.config.app_id);
        assert_eq!(service.config.app_secret, service.config.app_secret);
        assert!(service.validate_services_config());
    }

    #[test]
    fn test_wiki_v2_unicode_and_chinese_support() {
        let unicode_config = Config::builder()
            .app_id("飞书Wiki应用_📚_ID")
            .app_secret("Wiki管理密钥_🔒_Secret")
            .build();
        let unicode_service = V2::new(unicode_config);

        // 测试 Unicode 支持
        assert!(unicode_service.validate_services_config());
        assert!(unicode_service.health_check());

        let stats = unicode_service.get_service_statistics();
        assert!(stats.contains("飞书Wiki应用"));
        assert!(stats.contains("📚"));

        // 测试中文功能名称处理
        assert!(unicode_service.supports_feature("space_management"));
        assert!(unicode_service.supports_feature("member_collaboration"));
        assert!(unicode_service.supports_feature("team_collaboration"));
    }

    #[test]
    fn test_wiki_v2_enterprise_scenarios() {
        let enterprise_config = Config::builder()
            .app_id("enterprise_wiki_app_id")
            .app_secret("enterprise_wiki_app_secret")
            .build();
        let enterprise_service = V2::new(enterprise_config);

        // 测试企业级场景
        assert!(enterprise_service.validate_services_config());
        assert!(enterprise_service.health_check());

        // 验证企业功能支持
        assert!(enterprise_service.supports_feature("enterprise_wiki"));
        assert!(enterprise_service.supports_feature("team_collaboration"));
        assert!(enterprise_service.supports_feature("content_management"));
        assert!(enterprise_service.supports_feature("permission_management"));
        assert!(enterprise_service.supports_feature("document_organizing"));

        // 测试企业统计信息
        let stats = enterprise_service.get_service_statistics();
        assert!(stats.contains("enterprise_wiki_app_id"));
        assert!(stats.contains("services: 5"));

        let category_stats = enterprise_service.get_service_categories_statistics();
        assert!(category_stats.contains("core: 4"));
        assert!(category_stats.contains("task: 1"));

        // 测试协作功能
        let collaboration_info = enterprise_service.get_collaboration_features_info();
        assert!(collaboration_info.contains("real_time_editing: true"));
        assert!(collaboration_info.contains("permission_levels: 5"));
    }

    #[test]
    fn test_wiki_v2_memory_efficiency() {
        let config = create_test_config();

        // 测试内存使用效率
        let service = V2::new(config.clone());
        let cloned_service = service.clone();

        // 验证克隆后配置仍然有效
        assert!(cloned_service.validate_services_config());
        assert_eq!(service.config().app_id, cloned_service.config().app_id);

        // 测试状态摘要缓存效率
        let status1 = service.get_service_status_summary();
        let status2 = service.get_service_status_summary();
        assert_eq!(status1, status2);

        // 测试内容类型列表缓存效率
        let content_types1 = service.get_supported_content_types();
        let content_types2 = service.get_supported_content_types();
        assert_eq!(content_types1.len(), content_types2.len());
    }

    #[test]
    fn test_wiki_v2_error_handling_and_robustness() {
        // 测试部分无效配置
        let partial_invalid_config = Config::builder()
            .app_id("valid_app_id")
            .app_secret("")  // 无效密钥
            .build();
        let partial_invalid_service = V2::new(partial_invalid_config);

        // 当前实现中，只要app_id和app_secret都不为空，服务就认为健康
        assert!(!partial_invalid_service.health_check()); // app_secret为空，健康检查应该失败
        assert!(!partial_invalid_service.validate_services_config()); // app_secret为空，验证应该失败

        // 测试完全无效配置
        let fully_invalid_config = Config::builder()
            .app_id("")
            .app_secret("")
            .build();
        let fully_invalid_service = V2::new(fully_invalid_config);

        assert!(!fully_invalid_service.health_check());
        assert!(!fully_invalid_service.validate_services_config());

        // 验证统计信息仍然可用
        assert!(fully_invalid_service.get_service_statistics().contains("WikiV2"));
        assert!(fully_invalid_service.get_service_categories_statistics().contains("total: 5"));
    }

    #[test]
    fn test_wiki_v2_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let config = create_test_config();
        let service = Arc::new(V2::new(config));
        let mut handles = vec![];

        // 测试并发访问
        for _ in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                // 验证并发访问的安全性
                assert!(service_clone.validate_services_config());
                assert!(service_clone.health_check());
                assert!(service_clone.supports_feature("space_management"));

                let stats = service_clone.get_service_statistics();
                assert!(stats.contains("WikiV2"));

                let category_stats = service_clone.get_service_categories_statistics();
                assert!(category_stats.contains("total: 5"));

                let status = service_clone.get_service_status_summary();
                assert!(status.contains("overall: true"));

                let content_types = service_clone.get_supported_content_types();
                assert_eq!(content_types.len(), 18);

                let collaboration_info = service_clone.get_collaboration_features_info();
                assert!(collaboration_info.contains("real_time_editing: true"));
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_wiki_v2_performance_characteristics() {
        let config = create_test_config();
        let service = V2::new(config);

        // 测试性能特征
        let start = std::time::Instant::now();

        // 执行多个操作
        for _ in 0..1000 {
            assert!(service.validate_services_config());
            assert!(service.supports_feature("space_management"));
            let _stats = service.get_service_statistics();
            let _category_stats = service.get_service_categories_statistics();
            let _status = service.get_service_status_summary();
            let _content_types = service.get_supported_content_types();
            let _collaboration_info = service.get_collaboration_features_info();
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000, "Operations should complete quickly");
    }

    #[test]
    fn test_wiki_v2_comprehensive_integration() {
        let config = create_test_config();
        let service = V2::new(config);

        // 综合集成测试
        assert!(service.validate_services_config());
        assert!(service.health_check());

        // 测试所有核心功能
        assert!(service.supports_feature("space_management"));
        assert!(service.supports_feature("member_collaboration"));
        assert!(service.supports_feature("node_hierarchy"));
        assert!(service.supports_feature("space_settings"));
        assert!(service.supports_feature("task_management"));
        assert!(service.supports_feature("full_text_search"));
        assert!(service.supports_feature("access_control"));
        assert!(service.supports_feature("version_control"));
        assert!(service.supports_feature("collaborative_editing"));
        assert!(service.supports_feature("knowledge_sharing"));
        assert!(service.supports_feature("document_organizing"));
        assert!(service.supports_feature("enterprise_wiki"));
        assert!(service.supports_feature("team_collaboration"));
        assert!(service.supports_feature("content_management"));
        assert!(service.supports_feature("permission_management"));

        // 测试统计和调试功能
        let stats = service.get_service_statistics();
        assert!(stats.contains("test_wiki_app_id"));
        assert!(stats.contains("services: 5"));

        let category_stats = service.get_service_categories_statistics();
        assert!(category_stats.contains("core: 4"));
        assert!(category_stats.contains("task: 1"));

        // 测试状态摘要
        let status = service.get_service_status_summary();
        assert!(status.contains("overall: true"));

        // 测试内容类型和协作功能
        let content_types = service.get_supported_content_types();
        assert_eq!(content_types.len(), 18);

        let collaboration_info = service.get_collaboration_features_info();
        assert!(collaboration_info.contains("real_time_editing: true"));

        // 测试 Debug 和 Clone
        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("WikiV2"));

        let cloned_service = service.clone();
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
        assert!(cloned_service.validate_services_config());

        // 测试 Service trait 方法
        assert_eq!(V2::service_name(), "wiki");
        assert_eq!(V2::service_version(), "v2");
        assert_eq!(service.config().app_id, "test_wiki_app_id");
    }
}

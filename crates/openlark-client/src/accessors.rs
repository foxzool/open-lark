//! 便利服务访问器
//!
//! 提供向后兼容的服务访问方法，使迁移更加平滑

use crate::traits::{LarkClientTrait, ServiceRegistry};

// Map features to available services from openlark-docs
// ccm-doc, bitable -> BaseService
// ccm-sheets, ccm-wiki, ccm-drive, ccm -> CcmService
#[cfg(any(feature = "ccm-doc", feature = "bitable"))]
use openlark_docs::BaseService;

#[cfg(any(
    feature = "ccm-sheets",
    feature = "ccm-wiki",
    feature = "ccm-drive",
    feature = "ccm"
))]
use openlark_docs::CcmService;

#[cfg(feature = "communication")]
use openlark_communication::contact::ContactService;

/// 便利服务访问器 trait
///
/// 为服务注册表添加便利的服务访问方法
pub trait ServiceAccessorsExt {
    // 云文档服务访问器 - 使用可用的服务类型
    #[cfg(feature = "ccm-doc")]
    fn docs_ext(&self) -> Option<&BaseService>;

    #[cfg(feature = "ccm-sheets")]
    fn sheet_ext(&self) -> Option<&CcmService>;

    #[cfg(feature = "docs")]  // bitable 功能包含在 docs 模块中
    fn bitable_ext(&self) -> Option<&BaseService>;

    #[cfg(feature = "ccm-wiki")]
    fn wiki_ext(&self) -> Option<&CcmService>;

    #[cfg(feature = "ccm-drive")]
    fn drive_ext(&self) -> Option<&CcmService>;

    #[cfg(feature = "ccm")]
    fn ccm_ext(&self) -> Option<&CcmService>;

    // 通信服务访问器
    #[cfg(feature = "communication")]
    fn contact_ext(&self) -> Option<&ContactService>;
}

// 为 DefaultServiceRegistry 实现便利访问器
impl ServiceAccessorsExt for crate::registry::DefaultServiceRegistry {
    #[cfg(feature = "ccm-doc")]
    fn docs_ext(&self) -> Option<&BaseService> {
        self.get_service("docs")
    }

    #[cfg(feature = "ccm-sheets")]
    fn sheet_ext(&self) -> Option<&CcmService> {
        self.get_service("sheet")
    }

    #[cfg(feature = "docs")]  // bitable 功能包含在 docs 模块中
    fn bitable_ext(&self) -> Option<&BaseService> {
        self.get_service("bitable")
    }

    #[cfg(feature = "ccm-wiki")]
    fn wiki_ext(&self) -> Option<&CcmService> {
        self.get_service("wiki")
    }

    #[cfg(feature = "ccm-drive")]
    fn drive_ext(&self) -> Option<&CcmService> {
        self.get_service("drive")
    }

    #[cfg(feature = "ccm")]
    fn ccm_ext(&self) -> Option<&CcmService> {
        self.get_service("ccm")
    }

    #[cfg(feature = "communication")]
    fn contact_ext(&self) -> Option<&ContactService> {
        self.get_service("contact")
    }
}

/// 构建器扩展
///
/// 提供与原有 LarkClientBuilder 兼容的构建方法
#[derive(Debug)]
pub struct CompatibleClientBuilder {
    app_id: String,
    app_secret: String,
    app_type: Option<openlark_core::constants::AppType>,
    enable_token_cache: Option<bool>,
}

impl CompatibleClientBuilder {
    /// 创建新的兼容构建器
    pub fn new(app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            app_secret: app_secret.into(),
            app_type: None,
            enable_token_cache: None,
        }
    }

    /// 设置应用类型
    pub fn with_app_type(mut self, app_type: openlark_core::constants::AppType) -> Self {
        self.app_type = Some(app_type);
        self
    }

    /// 设置是否启用令牌缓存
    pub fn with_enable_token_cache(mut self, enable: bool) -> Self {
        self.enable_token_cache = Some(enable);
        self
    }

    /// 构建客户端实例
    pub fn build(self) -> crate::DefaultLarkClient {
        let mut config_builder = openlark_core::config::Config::builder()
            .app_id(self.app_id)
            .app_secret(self.app_secret);

        if let Some(app_type) = self.app_type {
            config_builder = config_builder.app_type(app_type);
        }

        if let Some(_enable) = self.enable_token_cache {
            // TODO: 实现令牌缓存设置
        }

        let config = config_builder.build();
        crate::DefaultLarkClient::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compatible_builder() {
        let builder = CompatibleClientBuilder::new("test_app_id", "test_app_secret");
        let _client = builder
            .with_app_type(openlark_core::constants::AppType::SelfBuild)
            .build();
    }

    #[test]
    fn test_service_extensions() {
        let registry = crate::registry::DefaultServiceRegistry::new();

        // 测试扩展方法
        #[cfg(feature = "docs")]
        assert!(registry.docs_ext().is_none());

        assert_eq!(registry.list_services().len(), 0);
    }
}

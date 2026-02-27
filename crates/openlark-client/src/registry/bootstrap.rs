//! 内置服务注册（meta 单入口）
//!
//! openlark-client 不在本 crate 内重复实现业务 API 包装层，但仍需要一份轻量的“服务元信息”
//! 以支持 registry 的可观测、依赖关系展示等用途。
//!
//! 这里集中管理 **按 feature 编译进来的服务**，避免在多个位置重复注册（DRY）。

use super::{DefaultServiceRegistry, ServiceMetadata, ServiceRegistry};
#[cfg(any(feature = "auth", feature = "communication", feature = "docs", feature = "cardkit", feature = "meeting", feature = "security", feature = "hr", feature = "ai"))]
use super::ServiceStatus;
use crate::{error, Result};

#[allow(unused_variables, dead_code, unused_imports)]
pub(crate) fn register_compiled_services(registry: &mut DefaultServiceRegistry) -> Result<()> {
    #[cfg(feature = "auth")]
    register_auth(registry)?;

    #[cfg(feature = "communication")]
    register_communication(registry)?;

    #[cfg(feature = "docs")]
    register_docs(registry)?;

    #[cfg(feature = "cardkit")]
    register_cardkit(registry)?;

    #[cfg(feature = "meeting")]
    register_meeting(registry)?;

    #[cfg(feature = "security")]
    register_security(registry)?;

    #[cfg(feature = "hr")]
    register_hr(registry)?;

    #[cfg(feature = "ai")]
    register_ai(registry)?;

    Ok(())
}

#[allow(dead_code)]
fn register(registry: &mut DefaultServiceRegistry, metadata: ServiceMetadata) -> Result<()> {
    registry
        .register_service(metadata)
        .map_err(error::registry_error)
}

#[cfg(feature = "auth")]
fn register_auth(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let metadata = ServiceMetadata {
        name: "auth".to_string(),
        version: "1.0.0".to_string(),
        description: Some("飞书认证服务，提供令牌管理、身份验证等功能".to_string()),
        dependencies: vec![],
        provides: vec![
            "token-management".to_string(),
            "permission-control".to_string(),
        ],
        status: ServiceStatus::Uninitialized,
        priority: 1,
    };
    register(registry, metadata)
}

#[cfg(feature = "communication")]
fn register_communication(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let metadata = ServiceMetadata {
        name: "communication".to_string(),
        version: "1.0.0".to_string(),
        description: Some("飞书通讯服务，提供消息、联系人、群组等功能".to_string()),
        dependencies: vec!["auth".to_string()],
        provides: vec![
            "im".to_string(),
            "contacts".to_string(),
            "groups".to_string(),
        ],
        status: ServiceStatus::Uninitialized,
        priority: 2,
    };
    register(registry, metadata)
}

#[cfg(feature = "docs")]
fn register_docs(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let metadata = ServiceMetadata {
        name: "docs".to_string(),
        version: "1.0.0".to_string(),
        description: Some("飞书文档服务，提供云文档、表格、知识库等功能".to_string()),
        dependencies: vec!["auth".to_string()],
        provides: vec![
            "cloud-docs".to_string(),
            "sheets".to_string(),
            "wiki".to_string(),
        ],
        status: ServiceStatus::Uninitialized,
        priority: 2,
    };
    register(registry, metadata)
}

#[cfg(feature = "cardkit")]
fn register_cardkit(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let metadata = ServiceMetadata {
        name: "cardkit".to_string(),
        version: "1.0.0".to_string(),
        description: Some("飞书卡片服务，提供卡片渲染与交互能力".to_string()),
        dependencies: vec!["auth".to_string()],
        provides: vec!["card".to_string()],
        status: ServiceStatus::Uninitialized,
        priority: 3,
    };
    register(registry, metadata)
}

#[cfg(feature = "meeting")]
fn register_meeting(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let metadata = ServiceMetadata {
        name: "meeting".to_string(),
        version: "1.0.0".to_string(),
        description: Some("飞书会议服务，提供视频会议与会议室管理能力".to_string()),
        dependencies: vec!["auth".to_string()],
        provides: vec!["vc".to_string()],
        status: ServiceStatus::Uninitialized,
        priority: 3,
    };
    register(registry, metadata)
}

#[cfg(feature = "security")]
fn register_security(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let metadata = ServiceMetadata {
        name: "security".to_string(),
        version: "1.0.0".to_string(),
        description: Some("飞书安全服务，提供安全审计与风控相关能力".to_string()),
        dependencies: vec!["auth".to_string()],
        provides: vec!["security".to_string()],
        status: ServiceStatus::Uninitialized,
        priority: 3,
    };
    register(registry, metadata)
}

#[cfg(feature = "hr")]
fn register_hr(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let metadata = ServiceMetadata {
        name: "hr".to_string(),
        version: "1.0.0".to_string(),
        description: Some("飞书人力资源服务，提供员工、考勤、薪酬等功能".to_string()),
        dependencies: vec!["auth".to_string()],
        provides: vec![
            "attendance".to_string(),
            "corehr".to_string(),
            "ehr".to_string(),
        ],
        status: ServiceStatus::Uninitialized,
        priority: 4,
    };
    register(registry, metadata)
}

#[cfg(feature = "ai")]
fn register_ai(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let metadata = ServiceMetadata {
        name: "ai".to_string(),
        version: "1.0.0".to_string(),
        description: Some("飞书AI服务，提供智能助手、AI分析等功能".to_string()),
        dependencies: vec!["auth".to_string(), "communication".to_string()],
        provides: vec!["chatbot".to_string(), "smart-analysis".to_string()],
        status: ServiceStatus::Uninitialized,
        priority: 4,
    };
    register(registry, metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_metadata_creation() {
        let metadata = ServiceMetadata {
            name: "test_service".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test service description".to_string()),
            dependencies: vec!["auth".to_string()],
            provides: vec!["test".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 1,
        };
        assert_eq!(metadata.name, "test_service");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.priority, 1);
    }

    #[test]
    fn test_service_status_debug() {
        let status = ServiceStatus::Uninitialized;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("Uninitialized"));
    }

    #[test]
    fn test_service_status_clone() {
        let status = ServiceStatus::Uninitialized;
        let cloned = status.clone();
        assert!(matches!(cloned, ServiceStatus::Uninitialized));
    }

    #[test]
    fn test_register_compiled_services() {
        let mut registry = DefaultServiceRegistry::new();
        let result = register_compiled_services(&mut registry);
        assert!(result.is_ok());
    }

    #[test]
    fn test_service_metadata_with_empty_dependencies() {
        let metadata = ServiceMetadata {
            name: "standalone_service".to_string(),
            version: "2.0.0".to_string(),
            description: None,
            dependencies: vec![],
            provides: vec![],
            status: ServiceStatus::Uninitialized,
            priority: 0,
        };
        assert!(metadata.dependencies.is_empty());
        assert!(metadata.provides.is_empty());
    }

    #[test]
    fn test_service_metadata_priority_ordering() {
        let low_priority = ServiceMetadata {
            name: "low".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            dependencies: vec![],
            provides: vec![],
            status: ServiceStatus::Uninitialized,
            priority: 1,
        };
        let high_priority = ServiceMetadata {
            name: "high".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            dependencies: vec![],
            provides: vec![],
            status: ServiceStatus::Uninitialized,
            priority: 10,
        };
        assert!(high_priority.priority > low_priority.priority);
    }
}

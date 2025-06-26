use crate::core::config::Config;

pub mod v1;

/// 组织架构服务
///
/// 提供组织架构相关功能，包括员工管理、部门管理等
///
/// # 功能模块
/// - v1: 组织架构 v1 API
///
/// # 示例
/// ```rust,ignore
/// use open_lark::LarkClient;
///
/// let client = LarkClient::builder("app_id", "app_secret").build();
///
/// // 创建员工
/// let response = client.directory.v1.employee.create(request, None).await?;
///
/// // 创建部门
/// let response = client.directory.v1.department.create(request, None).await?;
/// ```
pub struct DirectoryService {
    /// v1 API
    pub v1: v1::V1,
}

impl DirectoryService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}

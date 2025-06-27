pub mod attachment;
pub mod employee;
pub mod models;

use crate::core::config::Config;

use attachment::AttachmentService;
use employee::EmployeeService;

/// 飞书人事(标准版)服务
///
/// 飞书人事(标准版)为企业提供了完整的人力资源管理功能，包括员工档案管理、
/// 附件存储等核心功能。本服务封装了相关API接口，支持：
///
/// ## 主要功能
///
/// ### 员工花名册管理
/// - 批量获取员工信息
/// - 支持多维度筛选（部门、状态、自定义字段等）
/// - 分页查询大量员工数据
/// - 完整的员工档案信息（基本信息、职位、部门、个人信息等）
///
/// ### 人员附件管理
/// - 下载员工相关附件文件
/// - 支持多种文件格式
/// - 安全的文件访问控制
///
/// ## 使用场景
///
/// - **人事系统集成**: 与第三方人事系统同步员工数据
/// - **报表生成**: 基于员工数据生成各类人事报表
/// - **组织架构管理**: 获取部门和员工的层级关系
/// - **员工档案查询**: 查询员工的详细信息和历史记录
/// - **附件管理**: 下载和管理员工的证件、合同等文件
///
/// ## 权限要求
///
/// 使用本服务需要相应的应用权限：
/// - `ehr:employee`: 员工信息读取权限
/// - `ehr:attachment`: 附件下载权限
///
/// ## 示例用法
///
/// ```rust
/// use open_lark::prelude::*;
/// use open_lark::service::ehr::models::*;
///
/// // 创建客户端
/// let client = LarkClient::builder(app_id, app_secret)
///     .with_app_type(AppType::SelfBuild)
///     .build();
///
/// // 获取员工列表
/// let employee_request = EmployeeListRequest {
///     page_size: Some(50),
///     status: Some("active".to_string()),
///     department_id: Some("dept_123".to_string()),
///     fields: Some(vec![
///         "name".to_string(),
///         "employee_number".to_string(),
///         "email".to_string(),
///     ]),
///     ..Default::default()
/// };
///
/// let employees = client.ehr.employee.list_employees(employee_request, None).await?;
///
/// // 下载员工附件
/// let attachment_request = EmployeeAttachmentRequest {
///     employee_id: "emp_123".to_string(),
///     attachment_id: "attach_456".to_string(),
///     user_id_type: Some("open_id".to_string()),
/// };
///
/// let attachment = client.ehr.attachment.download_attachment(attachment_request, None).await?;
/// ```
pub struct EhrService {
    /// 员工花名册服务
    pub employee: EmployeeService,
    /// 人员附件服务
    pub attachment: AttachmentService,
}

impl EhrService {
    pub fn new(config: Config) -> Self {
        Self {
            employee: EmployeeService::new(config.clone()),
            attachment: AttachmentService::new(config),
        }
    }
}

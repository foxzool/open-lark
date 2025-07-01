//! 飞书人事标准版（EHR）服务
//!
//! 提供飞书人事标准版的完整功能集，支持员工花名册管理、人员附件管理等
//! 标准化人力资源管理能力。是中小型企业人事系统的核心基础。
//!
//! # 核心功能
//!
//! ## 员工花名册管理
//! - 👥 员工信息批量获取
//! - 🔍 多维度员工搜索筛选
//! - 📊 员工数据分页查询
//! - 📋 完整员工档案信息
//! - 🏢 部门和职位信息管理
//!
//! ## 人员附件管理
//! - 📁 员工附件文件下载
//! - 🔒 安全文件访问控制
//! - 📄 多种文件格式支持
//! - 📝 附件元信息查询
//! - 🗃️ 文件存储和管理
//!
//! ## 数据管理
//! - 📈 员工数据统计分析
//! - 🔄 数据实时同步更新
//! - 📊 人事报表生成支持
//! - 🔍 灵活查询条件设置
//! - 📋 批量数据处理能力
//!
//! ## 权限控制
//! - 🔐 细粒度权限管理
//! - 👑 角色访问控制
//! - 📊 操作日志记录
//! - 🛡️ 数据安全保护
//! - 🔒 敏感信息脱敏
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取人事标准版服务
//! let ehr = &client.ehr;
//!
//! // 获取员工列表
//! // let employee_request = ListEmployeeRequest::builder()
//! //     .page_size(50)
//! //     .status("active")
//! //     .department_id("dept_123")
//! //     .fields(vec!["name", "employee_number", "email"])
//! //     .build();
//! // let employees = ehr.employee.list(employee_request, None).await?;
//!
//! // 搜索员工信息
//! // let search_request = SearchEmployeeRequest::builder()
//! //     .keyword("张三")
//! //     .department_ids(vec!["dept_123", "dept_456"])
//! //     .employment_status(vec!["active", "inactive"])
//! //     .build();
//! // let search_results = ehr.employee.search(search_request, None).await?;
//!
//! // 下载员工附件
//! // let attachment_request = DownloadAttachmentRequest::builder()
//! //     .employee_id("emp_123")
//! //     .attachment_id("attach_456")
//! //     .user_id_type("open_id")
//! //     .build();
//! // let attachment = ehr.attachment.download(attachment_request, None).await?;
//!
//! // 获取员工附件列表
//! // let list_request = ListAttachmentRequest::builder()
//! //     .employee_id("emp_123")
//! //     .attachment_type("ID_CARD")
//! //     .build();
//! // let attachments = ehr.attachment.list(list_request, None).await?;
//! ```
//!
//! # 标准版特性
//!
//! - 🏢 适合中小型企业使用
//! - 📊 核心人事数据管理
//! - 🔗 简单高效的API接口
//! - 📋 标准化数据格式
//! - 💼 基础人事管理功能
//!
//! # 人事管理
//!
//! - 👥 员工全生命周期管理
//! - 📊 人事数据统计分析
//! - 📋 标准化流程支持
//! - 🔍 快速查询和检索
//! - 📁 文档管理和存储

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
/// ```ignore
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

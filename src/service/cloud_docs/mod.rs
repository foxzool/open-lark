//! 云文档（Cloud Docs）服务
//!
//! 提供飞书云文档的完整功能集，支持云空间、知识库、文档、电子表格、
//! 多维表格、画板、权限管理、评论系统等企业级文档协作能力。
//!
//! # 核心功能
//!
//! ## 云空间管理
//! - 📁 云盘文件存储管理
//! - 🔍 文件搜索和检索
//! - 📊 文件版本控制
//! - 🔗 文件分享和链接
//! - 📈 存储空间统计
//!
//! ## 知识库管理
//! - 📚 知识库创建和管理
//! - 📝 知识文档编写发布
//! - 🔍 知识内容搜索
//! - 👥 知识协作和共享
//! - 📊 知识使用统计
//!
//! ## 文档编辑
//! - 📝 富文本文档创建编辑
//! - 🎨 文档格式样式设置
//! - 👥 多人实时协作编辑
//! - 💬 文档评论和讨论
//! - 📱 跨平台同步支持
//!
//! ## 电子表格
//! - 📊 表格数据管理操作
//! - 📈 图表可视化展示
//! - 🔢 公式计算和函数
//! - 🔄 数据导入导出
//! - 👥 协作编辑和分享
//!
//! ## 多维表格
//! - 🗃️ 结构化数据管理
//! - 🔍 多维度数据筛选
//! - 📊 数据统计和分析
//! - 🔗 数据关联和引用
//! - 📋 表单数据收集
//!
//! ## 画板协作
//! - 🎨 创意画板绘制设计
//! - 🧩 模板和组件库
//! - 👥 团队协作创作
//! - 💡 头脑风暴和规划
//! - 📱 多设备同步支持
//!
//! ## 权限管理
//! - 🔐 细粒度权限控制
//! - 👥 用户角色管理
//! - 🔗 外部链接权限
//! - 📊 权限审计日志
//! - 🛡️ 数据安全保护
//!
//! ## 评论系统
//! - 💬 文档评论和回复
//! - 🎯 精确位置评论
//! - 🔔 评论通知提醒
//! - 📊 评论统计分析
//! - 🔄 评论状态管理
//!
//! ## 智能助手
//! - 🤖 AI写作辅助支持
//! - 📝 智能内容生成
//! - 🔍 智能信息检索
//! - 📊 文档质量分析
//! - 💡 创作建议推荐
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuilt)
//!     .build();
//!
//! // 获取云文档服务
//! let cloud_docs = &client.cloud_docs;
//!
//! // 云空间操作
//! // let file_request = CreateFileRequest::builder()
//! //     .name("项目计划书.docx")
//! //     .parent_token("folder_123")
//! //     .file_type("docx")
//! //     .build();
//! // let file = cloud_docs.drive.v1.file.create(file_request, None).await?;
//!
//! // 创建知识库
//! // let wiki_request = CreateWikiRequest::builder()
//! //     .name("产品知识库")
//! //     .description("产品相关文档和资料")
//! //     .build();
//! // let wiki = cloud_docs.wiki.v2.space.create(wiki_request, None).await?;
//!
//! // 操作文档
//! // let docx_request = CreateDocumentRequest::builder()
//! //     .title("会议纪要")
//! //     .folder_token("folder_456")
//! //     .build();
//! // let document = cloud_docs.docx.v1.document.create(docx_request, None).await?;
//!
//! // 操作电子表格
//! // let sheets_request = CreateSpreadsheetRequest::builder()
//! //     .title("销售数据")
//! //     .folder_token("folder_789")
//! //     .build();
//! // let spreadsheet = cloud_docs.sheets.v3.spreadsheet.create(sheets_request, None).await?;
//!
//! // 操作多维表格
//! // let bitable_request = CreateBitableRequest::builder()
//! //     .name("项目管理表")
//! //     .folder_token("folder_abc")
//! //     .build();
//! // let bitable = cloud_docs.bitable.v1.app.create(bitable_request, None).await?;
//! ```
//!
//! # 协作特性
//!
//! - 👥 多人实时协作编辑
//! - 💬 全面的评论讨论系统
//! - 📱 跨平台设备同步
//! - 🔔 智能通知提醒
//! - 📊 协作数据分析
//!
//! # 企业应用
//!
//! - 📋 文档管理和归档
//! - 👥 团队知识共享
//! - 📊 数据分析和报告
//! - 🎨 创意设计协作
//! - 📈 项目管理和跟踪

use crate::core::config::Config;
use std::sync::Arc;

// 子模块声明
pub mod assistant;
pub mod bitable;
pub mod board;
pub mod comments;
pub mod docx;
pub mod drive;
pub mod permission;
pub mod sheets;
pub mod wiki;

// 重新导出服务类型
pub use assistant::AssistantService;
pub use bitable::BitableService;
pub use board::BoardService;
pub use comments::CommentsService;
pub use docx::DocxService;
pub use drive::DriveService;
pub use permission::PermissionService;
pub use sheets::SheetsService;
pub use wiki::WikiService;

/// 云文档服务聚合器
///
/// 提供统一的云文档相关功能访问接口，包括：
/// - 云空间 (drive)
/// - 知识库 (wiki)  
/// - 文档 (docx)
/// - 电子表格 (sheets)
/// - 多维表格 (bitable)
/// - 画板 (board)
/// - 权限 (permission)
/// - 评论 (comments)
/// - 云文档助手 (assistant)
pub struct CloudDocsService {
    pub drive: DriveService,
    pub wiki: WikiService,
    pub docx: DocxService,
    pub sheets: SheetsService,
    pub bitable: BitableService,
    pub board: BoardService,
    pub permission: PermissionService,
    pub comments: CommentsService,
    pub assistant: AssistantService,
}

impl CloudDocsService {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            drive: DriveService::new(Arc::clone(&config)),
            wiki: WikiService::new(Arc::clone(&config)),
            docx: DocxService::new(Arc::clone(&config)),
            sheets: SheetsService::new(Arc::clone(&config)),
            bitable: BitableService::new(Arc::clone(&config)),
            board: BoardService::new(Arc::clone(&config)),
            permission: PermissionService::new(Arc::clone(&config)),
            comments: CommentsService::new(Arc::clone(&config)),
            assistant: AssistantService::new(Arc::clone(&config)),
        }
    }
}

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

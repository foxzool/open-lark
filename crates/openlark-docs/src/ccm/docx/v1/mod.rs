//! ccm/docx v1版本API
//!
//! 按照bizTag/project/version/resource/name.rs模式组织
//! 包含chat公告和document操作的相关API

use openlark_core::config::Config;

/// 聊天公告模块
pub mod chat;

/// 文档操作服务
pub mod document;


/// Document Service
pub struct DocxService {
    #[allow(dead_code)] // 配置保留供将来使用
    config: Config,
}

impl DocxService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

// 导出document模块
pub use document::*;

// 导出chat模块，添加前缀以避免与document模块冲突
pub use chat::announcement::{
    GetChatAnnouncementRequest, GetChatAnnouncementParams, GetChatAnnouncementResponse,
    AnnouncementData, UserInfo as ChatUserInfo
};

// 聊天公告block相关API
pub use chat::announcement::block::{
    GetChatAnnouncementBlockRequest, GetChatAnnouncementBlockParams, GetChatAnnouncementBlockResponse,
    BlockData as ChatBlockData, BlockContent as ChatBlockContent
};

// 聊天公告block children相关API
pub use chat::announcement::block::children::{
    GetChatAnnouncementBlockChildrenRequest, GetChatAnnouncementBlockChildrenParams, GetChatAnnouncementBlockChildrenResponse,
    ChildrenListData as ChatChildrenListData, ChildBlockItem as ChatChildBlockItem,
    RichText as ChatRichText, Paragraph as ChatParagraph, TextElement as ChatTextElement,
    TextRun as ChatTextRun, TextStyle as ChatTextStyle
};

// 聊天公告block batch_update相关API
pub use chat::announcement::block::batch_update::{
    BatchUpdateChatAnnouncementBlocksRequest, BatchUpdateChatAnnouncementBlocksParams, BatchUpdateChatAnnouncementBlocksResponse
};

// 聊天公告block children batch_delete相关API
pub use chat::announcement::block::children::batch_delete::{
    BatchDeleteChatAnnouncementBlockChildrenRequest, BatchDeleteChatAnnouncementBlockChildrenParams, BatchDeleteChatAnnouncementBlockChildrenResponse
};

// 聊天公告block children create相关API
pub use chat::announcement::block::children::create::{
    CreateChatAnnouncementBlockChildrenRequest, CreateChatAnnouncementBlockChildrenParams, CreateChatAnnouncementBlockChildrenResponse,
    CreateResult as ChatCreateResult, CreatedBlock as ChatCreatedBlock,
    NewBlock as ChatNewBlock, BlockLocation as ChatBlockLocation
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docx_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the DocxService functionality
    }
}

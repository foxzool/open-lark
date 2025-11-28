pub mod chat;
pub mod document;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct DocxV1 {
    service: Arc<DocsService>,
}

impl DocxV1 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn chat_announcement(&self) -> chat::announcement::ChatAnnouncement {
        chat::announcement::ChatAnnouncement::new(self.service.clone())
    }

    pub fn chat_announcement_block(&self) -> chat::announcement::block::ChatAnnouncementBlock {
        chat::announcement::block::ChatAnnouncementBlock::new(self.service.clone())
    }

    pub fn chat_announcement_block_children(&self) -> chat::announcement::block::children::ChatAnnouncementBlockChildren {
        chat::announcement::block::children::ChatAnnouncementBlockChildren::new(self.service.clone())
    }

    pub fn document(&self) -> document::Document {
        document::Document::new(self.service.clone())
    }

    pub fn document_block(&self) -> document::block::DocumentBlock {
        document::block::DocumentBlock::new(self.service.clone())
    }

    pub fn document_block_children(&self) -> document::block::children::DocumentBlockChildren {
        document::block::children::DocumentBlockChildren::new(self.service.clone())
    }

    pub fn document_block_descendant(&self) -> document::block::descendant::DocumentBlockDescendant {
        document::block::descendant::DocumentBlockDescendant::new(self.service.clone())
    }
}

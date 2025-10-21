use crate::core::config::Config;

pub mod document;
pub mod document_block;

pub struct V1 {
    pub document: document::DocumentService,
    pub document_block: document_block::DocumentBlockService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            document: document::DocumentService::new(config.clone()),
            document_block: document_block::DocumentBlockService::new(config),
        }
    }
}

/// Documents operation API for DOCX
///
/// This module provides comprehensive document operations functionality,
/// including document creation, reading, updating, and deletion.
use openlark_core::config::Config;

/// Documents operation service
pub struct DocumentsService {
    config: Config,
}

impl DocumentsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::DocumentsService;
    use openlark_core::config::Config;

    #[test]
    fn test_documents_service_creation() {
        let service = DocumentsService::new(Config::default());
        assert!(!service.config().base_url.is_empty());
    }
}

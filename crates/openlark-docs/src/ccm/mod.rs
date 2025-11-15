//! Content Collaboration Management (CCM) module
//!
//! This module provides comprehensive document collaboration functionality
//! including docs, sheets, wiki, drive, and related content management services.

use crate::prelude::*;

// Import submodules
pub mod doc;
pub mod docx;
pub mod drive;
pub mod export_tasks;
pub mod models;
pub mod sheets;
pub mod wiki;
pub mod ccm_sheet;

// Re-export services for convenience
#[cfg(feature = "ccm-doc")]
pub use doc::DocService;

#[cfg(feature = "ccm-docx")]
pub use docx::DocxService;

#[cfg(feature = "ccm-drive")]
pub use drive::DriveService;

/// Export tasks service (included in ccm feature)
pub use export_tasks::ExportTasksService;

/// CCM Sheet service (included in ccm feature)
pub use ccm_sheet::CcmSheetService;

/// Drive Explorer service (included in ccm feature)
pub use drive::ExplorerService;

/// Drive Permission service (included in ccm feature)
pub use drive::PermissionService;

#[cfg(feature = "ccm-sheets")]
pub use sheets::SheetsService;

#[cfg(feature = "ccm-wiki")]
pub use wiki::WikiService;

/// Main CCM Service providing access to all content collaboration features
#[derive(Clone)]
pub struct CcmService {
    client: std::sync::Arc<LarkClient>,
}

impl CcmService {
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self { client }
    }

    /// Get access to document services
    #[cfg(feature = "ccm-doc")]
    pub fn doc(&self) -> DocService {
        DocService::new(self.client.clone())
    }

    /// Get access to document services
    #[cfg(feature = "ccm-docx")]
    pub fn docx(&self) -> DocxService {
        DocxService::new(self.client.clone())
    }

    /// Get access to drive services
    #[cfg(feature = "ccm-drive")]
    pub fn drive(&self) -> DriveService {
        DriveService::new(self.client.clone())
    }

    /// Get access to sheets services
    #[cfg(feature = "ccm-sheets")]
    pub fn sheets(&self) -> SheetsService {
        SheetsService::new(self.client.clone())
    }

    /// Get access to export tasks services
    pub fn export_tasks(&self) -> ExportTasksService {
        ExportTasksService::new(openlark_core::config::Config::default())
    }

    /// Get access to CCM sheet services
    pub fn ccm_sheet(&self) -> CcmSheetService {
        CcmSheetService::new(openlark_core::config::Config::default())
    }

    /// Get access to wiki services
    #[cfg(feature = "ccm-wiki")]
    pub fn wiki(&self) -> WikiService {
        WikiService::new(self.client.clone())
    }
}

impl std::ops::Deref for CcmService {
    type Target = LarkClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ccm_service_creation() {
        // This is a placeholder test
        // In a real implementation, you would create a mock client
        // and test the CcmService functionality
    }
}

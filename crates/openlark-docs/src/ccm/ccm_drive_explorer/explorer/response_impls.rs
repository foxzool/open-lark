/// ApiResponseTrait implementations for response types
use openlark_core::api::{ApiResponseTrait, ResponseFormat};

use super::responses::*;

// Implement ApiResponseTrait for all response types
impl ApiResponseTrait for FileData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for FileCopyData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for FileDocsData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for FileSpreadsheetsData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for FolderData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for FolderChildrenData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for FolderMetaData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for RootFolderMetaData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
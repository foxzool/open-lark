/// Old API 模块已废弃
pub struct CcmSheetOldService;

impl CcmSheetOldService {
    pub fn new(_config: openlark_core::config::Config) -> Self {
        Self
    }
}

pub mod v2 {
    /// 占位类型
    pub struct CcmSheetOldV2;
}

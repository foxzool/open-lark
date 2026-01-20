#![allow(deprecated)]

/// Old V2 API 模块已废弃
#[deprecated(note = "使用新架构替代")]
pub struct CcmSheetOldV2Placeholder;

/// Old V2 API 访问器 - 已废弃
#[deprecated(note = "使用 CcmSheetService::v2() 替代")]
pub struct CcmSheetOldV2 {
    _private: (),
}

impl CcmSheetOldV2 {
    #[deprecated(note = "此 API 已废弃")]
    pub fn new(_config: openlark_core::config::Config) -> Self {
        Self { _private: () }
    }
}

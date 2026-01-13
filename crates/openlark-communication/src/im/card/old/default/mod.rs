//! default

pub mod v1;

// 兼容旧模块路径：历史上曾存在 `im/card/old/default/v1_card_update.rs`，
// 目前改为通过模块别名导出同名路径，避免单独维护重复文件。
pub mod v1_card_update {
    pub use super::v1::card::update::*;
}

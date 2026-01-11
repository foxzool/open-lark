//! default

pub mod v1;

// 兼容旧模块路径：im/card/old/default/v1_card_update.rs
pub mod v1_card_update {
    pub use super::v1::card::update::*;
}

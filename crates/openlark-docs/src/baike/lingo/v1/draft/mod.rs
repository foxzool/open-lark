/// Lingo草稿管理模块
pub mod create;
pub mod update;

pub use create::{CreateDraftRequest, CreateDraftResp};
pub use update::{UpdateDraftRequest, UpdateDraftResp};

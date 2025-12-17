/// Baike草稿管理模块
///
/// 提供词条草稿的创建和更新功能
pub mod create;
pub mod update;

// 重新导出构建器
pub use create::CreateDraftBuilder;
pub use update::UpdateDraftBuilder;

/// Baike词条管理模块
///
/// 提供词条的创建、更新、查询、搜索和提取功能
pub mod create;
pub mod get;
pub mod list;
pub mod update;
// pub mod entity_match; // Generated: Module file not found
pub mod extract;
pub mod highlight;
pub mod search;

// 重新导出构建器
pub use create::CreateEntityBuilder;
pub use get::GetEntityBuilder;
pub use list::ListEntityBuilder;
pub use update::UpdateEntityBuilder;
// pub use entity_match::MatchEntityBuilder; // Generated: Module use not found
pub use extract::ExtractEntityBuilder;
pub use highlight::HighlightEntityBuilder;
pub use search::SearchEntityBuilder;

/// Lingo词条管理模块
///
/// 提供词条的创建、更新、删除、查询和搜索功能

pub mod create;
pub mod update;
pub mod delete;
pub mod get;
pub mod list;
// pub mod entity_match; // Generated: Module file not found
pub mod search;
pub mod highlight;

// 重新导出构建器
pub use create::CreateEntityBuilder;
pub use update::UpdateEntityBuilder;
pub use delete::DeleteEntityBuilder;
pub use get::GetEntityBuilder;
pub use list::ListEntityBuilder;
// pub use entity_match::MatchEntityBuilder; // Generated: Module use not found
pub use search::SearchEntityBuilder;
pub use highlight::HighlightEntityBuilder;
/// Baike词条管理模块
///
/// 提供词条的创建、更新、查询、搜索和提取功能

pub mod create;
pub mod update;
pub mod get;
pub mod list;
pub mod entity_match;
pub mod search;
pub mod highlight;
pub mod extract;

// 重新导出构建器
pub use create::CreateEntityBuilder;
pub use update::UpdateEntityBuilder;
pub use get::GetEntityBuilder;
pub use list::ListEntityBuilder;
pub use entity_match::MatchEntityBuilder;
pub use search::SearchEntityBuilder;
pub use highlight::HighlightEntityBuilder;
pub use extract::ExtractEntityBuilder;
/// 知识库服务 v1 模块
///
/// 包含 baike 和 lingo 两个项目的 v1 版本API实现

pub mod lingo;
pub mod baike;

// 重新导出服务
pub use lingo::LingoV1Service;
pub use baike::BaikeV1Service;
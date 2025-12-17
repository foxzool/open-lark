/// Wiki 知识库服务模块
///
/// 提供企业知识库和Wiki管理功能。
pub mod v1;
pub mod v2;

// 导出V2版本的主要服务
pub use v2::WikiService;

// 重新导出V1搜索API
pub use v1::node::search::{SearchWikiParams, SearchWikiRequest, SearchWikiResponse};

// 重新导出V2版本的所有内容
pub use v2::*;

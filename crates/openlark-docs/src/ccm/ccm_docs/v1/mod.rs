/// CCM Docs V1 API 模块
///
/// 云文档内容管理API实现，包含2个API：
/// - docs_api/search_object: 搜索云文档
/// - docs_api/meta: 获取元数据

// 导出docs_api模块
// pub mod docs_api; // Generated: Module file not found

// 导出模型定义
pub mod models;
// pub mod requests; // Generated: Module file not found
// pub mod responses; // Generated: Module file not found

// 重新导出主要的API函数，方便外部使用
// pub use docs_api::{search_object, get_meta}; // Generated: Module use not found

// 重新导出模型和类型
// models 模块显式导出
pub use models::{
    DocumentItem,
    GetMetaParams,
    GetMetaResponse,
    MetaData,
    MetaItem,
    PermissionInfo,
    SearchData,
    SearchObjectParams,
    SearchObjectResponse,
    UserInfo,
};
// pub use requests::*; // Generated: Module use not found
// pub use responses::*; // Generated: Module use not found
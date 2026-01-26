/// children模块 - 文档块子块操作API
///
/// 按照bizTag/project/version/resource/name.rs模式组织
pub mod batch_delete;
pub mod create;
pub mod get;

// 使用通配符导出所有子模块
pub use batch_delete::*;
pub use create::*;
pub use get::*;

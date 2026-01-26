/// Wiki V2 空间管理模块
pub mod create;
pub mod get;
pub mod get_node;
pub mod list;
pub mod member;
pub mod node;
pub mod setting;

// 使用通配符导出所有子模块
#[allow(deprecated)]
pub use create::*;
pub use get::*;
pub use get_node::*;
#[allow(deprecated)]
pub use list::*;
pub use member::*;
pub use node::*;
pub use setting::*;

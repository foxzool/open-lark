pub mod copy;
pub mod create;
pub mod list;
pub mod r#move;
pub mod move_docs_to_wiki;
pub mod update_title;

// 使用通配符导出所有子模块
pub use copy::*;
pub use create::*;
pub use list::*;
#[allow(deprecated)]
pub use move_docs_to_wiki::*;
pub use r#move::*;
pub use update_title::*;

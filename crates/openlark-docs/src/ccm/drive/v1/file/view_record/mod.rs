pub mod list;

// 使用通配符导出所有子模块
// list 模块显式导出
pub use list::{GetFileViewRecordsRequest, GetFileViewRecordsResponse, ViewRecord};

/// 妙记统计（minute.statistics）相关接口
pub mod get;

// 使用通配符导出所有子模块
// get 模块显式导出
pub use get::{
    GetMinuteStatisticsRequest,
    GetMinuteStatisticsResponse,
    MinuteStatistics,
    UserViewDetail,
};

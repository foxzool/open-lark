/// 妙记统计（minute.statistics）相关接口
pub mod get;

// get 模块显式导出
pub use get::{
    GetMinuteStatisticsRequest,
    GetMinuteStatisticsResponse,
    MinuteStatistics,
    UserViewDetail,
    execute,
    execute_with_options,
    minute_token,
    new,
    user_id_type,
};

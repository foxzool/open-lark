/// 获取会议纪要详情接口。
pub mod get;
/// 获取会议纪要媒体接口。
pub mod media;
/// 会议纪要模型模块。
pub mod models;
/// 会议纪要统计接口。
pub mod statistics;
/// 会议纪要转写接口。
pub mod transcript;

/// 重新导出会议纪要详情类型。
pub use get::{GetMinuteRequest, GetMinuteResponse, MinuteInfo};
/// 重新导出会议纪要媒体类型。
pub use media::{GetMinuteMediaRequest, GetMinuteMediaResponse};
/// 重新导出会议纪要模型。
pub use models::{
    MinuteInfo as ModelMinuteInfo, MinuteMediaInfo, MinuteStatistics, UserIdType, UserViewDetail,
};
/// 重新导出会议纪要统计类型。
pub use statistics::{
    GetMinuteStatisticsRequest, GetMinuteStatisticsResponse,
    MinuteStatistics as StatMinuteStatistics, UserViewDetail as StatUserViewDetail,
};
/// 重新导出会议纪要转写请求类型。
pub use transcript::GetMinuteTranscriptRequest;

/// 会议纪要模块
pub mod get;
pub mod media;
pub mod models;
pub mod statistics;
pub mod transcript;

pub use get::{GetMinuteRequest, GetMinuteResponse, MinuteInfo};
pub use media::{GetMinuteMediaRequest, GetMinuteMediaResponse};
pub use models::{
    MinuteInfo as ModelMinuteInfo, MinuteMediaInfo, MinuteStatistics, UserIdType, UserViewDetail,
};
pub use statistics::{
    GetMinuteStatisticsRequest, GetMinuteStatisticsResponse,
    MinuteStatistics as StatMinuteStatistics, UserViewDetail as StatUserViewDetail,
};
pub use transcript::GetMinuteTranscriptRequest;

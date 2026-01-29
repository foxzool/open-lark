/// Minutes API 模块
pub mod v1;

pub use v1::{
    GetMinuteMediaRequest, GetMinuteMediaResponse, GetMinuteRequest, GetMinuteResponse,
    GetMinuteStatisticsRequest, GetMinuteStatisticsResponse, GetMinuteTranscriptRequest,
    MinuteInfo, MinuteMediaInfo, MinuteStatistics, ModelMinuteInfo, StatMinuteStatistics,
    StatUserViewDetail, UserIdType, UserViewDetail,
};

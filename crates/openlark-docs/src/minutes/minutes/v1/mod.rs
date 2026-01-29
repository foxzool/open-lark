/// Minutes V1 API 模块
pub mod minute;

pub use minute::{
    GetMinuteMediaRequest, GetMinuteMediaResponse, GetMinuteRequest, GetMinuteResponse,
    GetMinuteStatisticsRequest, GetMinuteStatisticsResponse, GetMinuteTranscriptRequest,
    MinuteInfo, MinuteMediaInfo, MinuteStatistics, ModelMinuteInfo, StatMinuteStatistics,
    StatUserViewDetail, UserIdType, UserViewDetail,
};

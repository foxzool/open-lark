#![allow(clippy::module_inception)]

/// 妙记服务模块
///
/// 提供飞书妙记的创建、查询、管理等功能。
/// docPath: https://open.feishu.cn/document/server-docs/minutes-v1/minute/get
use openlark_core::config::Config;

pub mod minutes;

pub use minutes::{
    GetMinuteMediaRequest, GetMinuteMediaResponse, GetMinuteRequest, GetMinuteResponse,
    GetMinuteStatisticsRequest, GetMinuteStatisticsResponse, GetMinuteTranscriptRequest, MinuteInfo,
    ModelMinuteInfo, MinuteMediaInfo, MinuteStatistics, StatMinuteStatistics, StatUserViewDetail,
    UserIdType, UserViewDetail,
};

/// Minutes 服务
#[derive(Debug, Clone)]
pub struct MinutesService {
    config: Config,
}

impl MinutesService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

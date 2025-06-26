use crate::core::config::Config;

pub mod media;
pub mod minute;
pub mod statistics;
pub mod transcript;

use media::MediaService;
use minute::MinuteService;
use statistics::StatisticsService;
use transcript::TranscriptService;

/// Minutes v1服务
pub struct V1 {
    /// 音视频文件服务
    pub media: MediaService,
    /// 妙记信息服务
    pub minute: MinuteService,
    /// 统计数据服务
    pub statistics: StatisticsService,
    /// 文字记录服务
    pub transcript: TranscriptService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            media: MediaService::new(config.clone()),
            minute: MinuteService::new(config.clone()),
            statistics: StatisticsService::new(config.clone()),
            transcript: TranscriptService::new(config),
        }
    }
}

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
}

impl V1 {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
impl V1 {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}
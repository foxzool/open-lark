pub mod minute;

use std::sync::Arc;
use crate::service::DocsService;

#[derive(Clone)]
pub struct MinutesV1 {
    service: Arc<DocsService>,
}

impl MinutesV1 {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    pub fn minute(&self) -> minute::Minute {
        minute::Minute::new(self.service.clone())
    }

    pub fn minute_media(&self) -> minute::media::MinuteMedia {
        minute::media::MinuteMedia::new(self.service.clone())
    }

    pub fn minute_statistics(&self) -> minute::statistics::MinuteStatistics {
        minute::statistics::MinuteStatistics::new(self.service.clone())
    }

    pub fn minute_transcript(&self) -> minute::transcript::MinuteTranscript {
        minute::transcript::MinuteTranscript::new(self.service.clone())
    }
}
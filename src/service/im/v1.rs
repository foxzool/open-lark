use crate::core::config::Config;
use crate::service::im::v1::message::Message;

pub mod message;

pub struct V1 {
    pub message: Message,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            message: Message { config },
        }
    }
}

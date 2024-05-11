use crate::core::config::Config;
use crate::service::im::v1::chats::Chats;
use crate::service::im::v1::message::Message;

pub mod chats;
pub mod message;

pub struct V1 {
    pub chats: Chats,
    pub message: Message,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            chats: Chats {
                config: config.clone(),
            },
            message: Message { config },
        }
    }
}

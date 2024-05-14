use crate::core::config::Config;
use crate::service::im::v1::chats::ChatsService;
use crate::service::im::v1::message::MessageService;

pub mod chats;
pub mod message;

pub struct V1 {
    pub chats: ChatsService,
    pub message: MessageService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            chats: ChatsService {
                config: config.clone(),
            },
            message: MessageService { config },
        }
    }
}
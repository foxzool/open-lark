use std::sync::Arc;

use crate::{
    core::config::Config,
    service::im::v1::{
        batch_message::BatchMessageService, buzz_messages::BuzzMessagesService,
        chats::ChatsService, file::FileService, image::ImageService, message::MessageService,
        message_card::MessageCardService, message_reaction::MessageReactionService,
        pin::PinService, url_preview::UrlPreviewService,
    },
};

// 现有模块
pub mod chats;
pub mod message;
pub mod message_service;
pub mod p2_im_message_read_v1;
pub mod p2_im_message_receive_v1;

// 新增模块
pub mod batch_message;
pub mod buzz_messages;
pub mod file;
pub mod image;
pub mod message_card;
pub mod message_reaction;
pub mod models;
pub mod pin;
pub mod url_preview;

pub struct V1 {
    // 现有服务
    pub chats: ChatsService,
    pub message: MessageService,

    // 新增服务
    pub batch_message: BatchMessageService,
    pub image: ImageService,
    pub file: FileService,
    pub message_reaction: MessageReactionService,
    pub pin: PinService,
    pub message_card: MessageCardService,
    pub buzz_messages: BuzzMessagesService,
    pub url_preview: UrlPreviewService,
}

impl V1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            // 现有服务
            chats: ChatsService {
                config: Arc::clone(&config),
            },
            message: MessageService {
                config: Arc::clone(&config),
            },

            // 新增服务
            batch_message: BatchMessageService::new(Arc::clone(&config)),
            image: ImageService::new(Arc::clone(&config)),
            file: FileService::new(Arc::clone(&config)),
            message_reaction: MessageReactionService::new(Arc::clone(&config)),
            pin: PinService::new(Arc::clone(&config)),
            message_card: MessageCardService::new(Arc::clone(&config)),
            buzz_messages: BuzzMessagesService::new(Arc::clone(&config)),
            url_preview: UrlPreviewService::new(config),
        }
    }
}

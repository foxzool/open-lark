pub mod batch_message;
pub mod chat;
pub mod file;
pub mod image;
pub mod message;
pub mod pin;
pub mod thread;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct ImV1 {
    service: Arc<CommunicationService>,
}

impl ImV1 {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn batch_message(&self) -> batch_message::BatchMessage {
        batch_message::BatchMessage::new(self.service.clone())
    }

    pub fn chat(&self) -> chat::Chat {
        chat::Chat::new(self.service.clone())
    }

    pub fn chat_announcement(&self) -> chat::announcement::ChatAnnouncement {
        chat::announcement::ChatAnnouncement::new(self.service.clone())
    }

    pub fn chat_managers(&self) -> chat::managers::ChatManagers {
        chat::managers::ChatManagers::new(self.service.clone())
    }

    pub fn chat_members(&self) -> chat::members::ChatMembers {
        chat::members::ChatMembers::new(self.service.clone())
    }

    pub fn chat_menu_item(&self) -> chat::menu_item::ChatMenuItem {
        chat::menu_item::ChatMenuItem::new(self.service.clone())
    }

    pub fn chat_menu_tree(&self) -> chat::menu_tree::ChatMenuTree {
        chat::menu_tree::ChatMenuTree::new(self.service.clone())
    }

    pub fn chat_moderation(&self) -> chat::moderation::ChatModeration {
        chat::moderation::ChatModeration::new(self.service.clone())
    }

    pub fn chat_tab(&self) -> chat::tab::ChatTab {
        chat::tab::ChatTab::new(self.service.clone())
    }

    pub fn chat_top_notice(&self) -> chat::top_notice::ChatTopNotice {
        chat::top_notice::ChatTopNotice::new(self.service.clone())
    }

    pub fn file(&self) -> file::File {
        file::File::new(self.service.clone())
    }

    pub fn image(&self) -> image::Image {
        image::Image::new(self.service.clone())
    }

    pub fn message(&self) -> message::Message {
        message::Message::new(self.service.clone())
    }

    pub fn message_reaction(&self) -> message::reaction::MessageReaction {
        message::reaction::MessageReaction::new(self.service.clone())
    }

    pub fn message_resource(&self) -> message::resource::MessageResource {
        message::resource::MessageResource::new(self.service.clone())
    }

    pub fn pin(&self) -> pin::Pin {
        pin::Pin::new(self.service.clone())
    }

    pub fn thread(&self) -> thread::Thread {
        thread::Thread::new(self.service.clone())
    }
}
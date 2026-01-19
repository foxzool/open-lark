pub mod app_feed_card;
pub mod biz_entity_tag_relation;
pub mod chat_button;
pub mod feed_card;
pub mod tag;
pub mod url_preview;

use std::sync::Arc;
use crate::service::CommunicationService;

#[derive(Clone)]
pub struct ImV2 {
    service: Arc<CommunicationService>,
}

impl ImV2 {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    pub fn app_feed_card(&self) -> app_feed_card::AppFeedCard {
        app_feed_card::AppFeedCard::new(self.service.clone())
    }

    pub fn app_feed_card_batch(&self) -> app_feed_card::batch::AppFeedCardBatch {
        app_feed_card::batch::AppFeedCardBatch::new(self.service.clone())
    }

    pub fn biz_entity_tag_relation(&self) -> biz_entity_tag_relation::BizEntityTagRelation {
        biz_entity_tag_relation::BizEntityTagRelation::new(self.service.clone())
    }

    pub fn chat_button(&self) -> chat_button::ChatButton {
        chat_button::ChatButton::new(self.service.clone())
    }

    pub fn feed_card(&self) -> feed_card::FeedCard {
        feed_card::FeedCard::new(self.service.clone())
    }

    pub fn tag(&self) -> tag::Tag {
        tag::Tag::new(self.service.clone())
    }

    pub fn url_preview(&self) -> url_preview::UrlPreview {
        url_preview::UrlPreview::new(self.service.clone())
    }
}

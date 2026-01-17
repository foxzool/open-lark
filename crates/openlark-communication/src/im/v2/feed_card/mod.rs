use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct FeedCard {
    service: Arc<CommunicationService>,
}

impl FeedCard {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/im-v2/groups-bots/bot_time_sentive
    pub async fn patch_open_apis_im_v2_feed_cards_bot_time_sentive(&self, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v2/feed_cards/bot_time_sentive".to_string();
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/im-v2/groups-bots/patch
    pub async fn patch_open_apis_im_v2_feed_cards_by_feed_card_id(&self, feed_card_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v2/feed_cards/:feed_card_id".to_string();
        path = path.replace(":feed_card_id", feed_card_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}

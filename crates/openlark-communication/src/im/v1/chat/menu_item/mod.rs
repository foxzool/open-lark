use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::CommunicationService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ChatMenuItem {
    service: Arc<CommunicationService>,
}

impl ChatMenuItem {
    pub fn new(service: Arc<CommunicationService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/server-docs/group/chat-menu_tree/patch
    pub async fn patch_open_apis_im_v1_chats_by_chat_id_menu_items_by_menu_item_id(&self, chat_id: impl AsRef<str>, menu_item_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/im/v1/chats/:chat_id/menu_items/:menu_item_id".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        path = path.replace(":menu_item_id", menu_item_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}

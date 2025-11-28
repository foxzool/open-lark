pub mod children;

use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::DocsService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct ChatAnnouncementBlock {
    service: Arc<DocsService>,
}

impl ChatAnnouncementBlock {
    pub fn new(service: Arc<DocsService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement/list
    pub async fn get_open_apis_docx_v1_chats_by_chat_id_announcement_blocks(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docx/v1/chats/:chat_id/announcement/blocks".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/batch_update
    pub async fn patch_open_apis_docx_v1_chats_by_chat_id_announcement_blocks_batch_update(&self, chat_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docx/v1/chats/:chat_id/announcement/blocks/batch_update".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        let method = Method::PATCH;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/group/upgraded-group-announcement/chat-announcement-block/get
    pub async fn get_open_apis_docx_v1_chats_by_chat_id_announcement_blocks_by_block_id(&self, chat_id: impl AsRef<str>, block_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id".to_string();
        path = path.replace(":chat_id", chat_id.as_ref());
        path = path.replace(":block_id", block_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}

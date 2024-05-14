#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct RequestOption {
    pub(crate) tenant_key: String,
    pub(crate) user_access_token: String,
    pub(crate) app_access_token: String,
    pub(crate) tenant_access_token: String,
    need_helpdesk_auth: bool,
    pub(crate) request_id: String,
    pub(crate) app_ticket: String,
    pub(crate) file_upload: bool,
    file_download: bool,
    pub(crate) header: HashMap<String, String>,
}
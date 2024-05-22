#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct RequestOption {
    pub tenant_key: String,
    pub user_access_token: String,
    pub app_access_token: String,
    pub tenant_access_token: String,
    pub need_helpdesk_auth: bool,
    pub request_id: String,
    pub app_ticket: String,
    pub file_upload: bool,
    pub file_download: bool,
    pub header: HashMap<String, String>,
}

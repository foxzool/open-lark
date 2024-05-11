#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;

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

pub type RequestOptionFunc = Box<dyn Fn(&mut RequestOption)>;

fn with_need_helpdesk_auth() -> RequestOptionFunc {
    Box::new(|option: &mut RequestOption| {
        option.need_helpdesk_auth = true;
    })
}

fn with_request_id(request_id: String) -> RequestOptionFunc {
    let request_id = Arc::new(request_id);
    Box::new(move |option: &mut RequestOption| {
        option.request_id = request_id.to_string();
    })
}

fn with_tenant_key(tenant_key: String) -> RequestOptionFunc {
    let tenant_key = Arc::new(tenant_key);
    Box::new(move |option: &mut RequestOption| {
        option.tenant_key = tenant_key.to_string();
    })
}

fn with_app_ticket(app_ticket: String) -> RequestOptionFunc {
    let app_ticket = Arc::new(app_ticket);
    Box::new(move |option: &mut RequestOption| {
        option.app_ticket = app_ticket.to_string();
    })
}

fn with_file_upload() -> RequestOptionFunc {
    Box::new(|option: &mut RequestOption| {
        option.file_upload = true;
    })
}

fn with_file_download() -> RequestOptionFunc {
    Box::new(|option: &mut RequestOption| {
        option.file_download = true;
    })
}

fn with_headers(header: HashMap<String, String>) -> RequestOptionFunc {
    Box::new(move |option: &mut RequestOption| {
        option.header.clone_from(&header);
    })
}

fn with_user_access_token(user_access_token: String) -> RequestOptionFunc {
    let user_access_token = Arc::new(user_access_token);
    Box::new(move |option: &mut RequestOption| {
        option.user_access_token = user_access_token.to_string();
    })
}

fn with_tenant_access_token(tenant_access_token: String) -> RequestOptionFunc {
    let tenant_access_token = Arc::new(tenant_access_token);
    Box::new(move |option: &mut RequestOption| {
        option.tenant_access_token = tenant_access_token.to_string();
    })
}

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct RequestOption {
    pub(crate) tenant_key: String,
    pub(crate) user_access_token: String,
    pub(crate) app_access_token: String,
    pub(crate) tenant_access_token: String,
    need_helpdesk_auth: bool,
    request_id: String,
    app_ticket: String,
    file_upload: bool,
    file_download: bool,
    pub(crate) header: HashMap<String, String>,
}

pub type RequestOptionFunc = Box<dyn FnOnce(&mut RequestOption)>;


fn with_need_helpdesk_auth() -> RequestOptionFunc {
    Box::new(|option: &mut RequestOption| {
        option.need_helpdesk_auth = true;
    })
}

fn with_request_id(request_id: String) -> RequestOptionFunc {
    Box::new(move |option: &mut RequestOption| {
        option.request_id = request_id;
    })
}

fn with_tenant_key(tenant_key: String) -> RequestOptionFunc {
    Box::new(move |option: &mut RequestOption| {
        option.tenant_key = tenant_key;
    })
}

fn with_app_ticket(app_ticket: String) -> RequestOptionFunc {
    Box::new(move |option: &mut RequestOption| {
        option.app_ticket = app_ticket;
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
        option.header = header;
    })
}

fn with_user_access_token(user_access_token: String) -> RequestOptionFunc {
    Box::new(move |option: &mut RequestOption| {
        option.user_access_token = user_access_token;
    })
}

fn with_tenant_access_token(tenant_access_token: String) -> RequestOptionFunc {
    Box::new(move |option: &mut RequestOption| {
        option.tenant_access_token = tenant_access_token;
    })
}
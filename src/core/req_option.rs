#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct RequestOption {
    pub(crate) tenant_key: String,
    pub(crate) user_access_token: String,
    pub(crate) app_access_token: String,
    pub(crate) tenant_access_token: String,
    pub(crate) need_helpdesk_auth: bool,
    pub(crate) request_id: String,
    pub(crate) app_ticket: String,
    pub(crate) file_upload: bool,
    pub(crate) file_download: bool,
    pub(crate) header: HashMap<String, String>,
}

impl RequestOption {
    pub fn builder() -> RequestOptionBuilder {
        RequestOptionBuilder::default()
    }
}

#[derive(Default)]
pub struct RequestOptionBuilder {
    option: RequestOption,
}

impl RequestOptionBuilder {
    pub fn tenant_key(mut self, tenant_key: impl ToString) -> Self {
        self.option.tenant_key = tenant_key.to_string();
        self
    }

    pub fn user_access_token(mut self, user_access_token: impl ToString) -> Self {
        self.option.user_access_token = user_access_token.to_string();
        self
    }

    pub fn app_access_token(mut self, app_access_token: impl ToString) -> Self {
        self.option.app_access_token = app_access_token.to_string();
        self
    }

    pub fn tenant_access_token(mut self, tenant_access_token: impl ToString) -> Self {
        self.option.tenant_access_token = tenant_access_token.to_string();
        self
    }

    pub fn need_helpdesk_auth(mut self, need_helpdesk_auth: bool) -> Self {
        self.option.need_helpdesk_auth = need_helpdesk_auth;
        self
    }

    pub fn request_id(mut self, request_id: impl ToString) -> Self {
        self.option.request_id = request_id.to_string();
        self
    }

    pub fn app_ticket(mut self, app_ticket: impl ToString) -> Self {
        self.option.app_ticket = app_ticket.to_string();
        self
    }

    pub fn file_upload(mut self, file_upload: bool) -> Self {
        self.option.file_upload = file_upload;
        self
    }

    pub fn file_download(mut self, file_download: bool) -> Self {
        self.option.file_download = file_download;
        self
    }

    pub fn header(mut self, header: HashMap<String, String>) -> Self {
        self.option.header = header;
        self
    }

    /// 添加请求头
    pub fn add_header(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.option
            .header
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> RequestOption {
        self.option
    }
}

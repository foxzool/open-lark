use std::collections::HashMap;

#[derive(Default)]
pub struct RequestOption {
    pub(crate) tenant_key: Option<String>,
    pub(crate) user_access_token: Option<String>,
    pub(crate) tenant_access_token: Option<String>,
    pub(crate) app_access_token: Option<String>,
    pub(crate) headers: HashMap<String, String>,
}

pub struct RequestOptionBuilder {
    request_option: RequestOption,
}

impl RequestOptionBuilder {
    pub fn tenant_key(mut self, tenant_key: String) -> Self {
        self.request_option.tenant_key = Some(tenant_key);
        self
    }

    pub fn user_access_token(mut self, user_access_token: String) -> Self {
        self.request_option.user_access_token = Some(user_access_token);
        self
    }

    pub fn tenant_access_token(mut self, tenant_access_token: String) -> Self {
        self.request_option.tenant_access_token = Some(tenant_access_token);
        self
    }

    pub fn app_access_token(mut self, app_access_token: String) -> Self {
        self.request_option.app_access_token = Some(app_access_token);
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.request_option.headers = headers;
        self
    }

    pub fn build(self) -> RequestOption {
        self.request_option
    }
}

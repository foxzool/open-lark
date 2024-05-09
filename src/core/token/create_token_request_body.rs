use serde::Serialize;

#[derive(Default, Serialize, Clone, Debug)]
pub struct CreateTokenRequestBody {
    app_id: Option<String>,
    app_secret: Option<String>,
    app_ticket: Option<String>,
    app_access_token: Option<String>,
    tenant_key: Option<String>,
}

impl CreateTokenRequestBody {
    pub fn builder() -> CreateTokenRequestBodyBuilder {
        CreateTokenRequestBodyBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateTokenRequestBodyBuilder {
    create_token_request_body: CreateTokenRequestBody,
}

impl CreateTokenRequestBodyBuilder {
    pub fn app_id(mut self, app_id: String) -> Self {
        self.create_token_request_body.app_id = Some(app_id);
        self
    }

    pub fn app_secret(mut self, app_secret: String) -> Self {
        self.create_token_request_body.app_secret = Some(app_secret);
        self
    }

    pub fn app_ticket(mut self, app_ticket: String) -> Self {
        self.create_token_request_body.app_ticket = Some(app_ticket);
        self
    }

    pub fn app_access_token(mut self, app_access_token: String) -> Self {
        self.create_token_request_body.app_access_token = Some(app_access_token);
        self
    }

    pub fn tenant_key(mut self, tenant_key: String) -> Self {
        self.create_token_request_body.tenant_key = Some(tenant_key);
        self
    }

    pub fn build(self) -> CreateTokenRequestBody {
        self.create_token_request_body
    }
}
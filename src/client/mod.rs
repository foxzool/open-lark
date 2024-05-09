use crate::core::model::Config;

pub struct Client {
    config: Option<Config>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}

#[derive(Default)]
pub struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    pub fn app_id(mut self, app_id: String) -> Self {
        self.config.app_id = Some(app_id);
        self
    }

    pub fn app_secret(mut self, app_secret: String) -> Self {
        self.config.app_secret = Some(app_secret);
        self
    }

    pub fn domain(mut self, domain: String) -> Self {
        self.config.domain = domain;
        self
    }

    pub fn timeout(mut self, timeout: f32) -> Self {
        self.config.timeout = Some(timeout);
        self
    }

    pub fn app_type(mut self, app_type: AppType) -> Self {
        self.config.app_type = app_type;
        self
    }

    pub fn app_ticket(mut self, app_ticket: String) -> Self {
        self.config.app_ticket = Some(app_ticket);
        self
    }

    pub fn enable_set_token(mut self, enable_set_token: bool) -> Self {
        self.config.enable_set_token = enable_set_token;
        self
    }

    pub fn build(self) -> Client {
        Client {
            config: Some(self.config),
        }
    }
}

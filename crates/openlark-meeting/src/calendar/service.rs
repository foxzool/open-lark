use openlark_core::config::Config;

pub struct CalendarService {
    config: Config,
}

impl CalendarService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn v4(&self) -> CalendarV4Service {
        CalendarV4Service::new(self.config.clone())
    }
}

pub struct CalendarV4Service {
    config: Config,
}

impl CalendarV4Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn calendar(&self) -> CalendarResource {
        CalendarResource::new(self.config.clone())
    }
}

pub struct CalendarResource {
    config: Config,
}

impl CalendarResource {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

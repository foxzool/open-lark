use crate::{core::config::Config, service::search::v1::user::UserService};

pub mod user;

pub struct V1 {
    pub user: UserService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            user: UserService::new(config),
        }
    }
}

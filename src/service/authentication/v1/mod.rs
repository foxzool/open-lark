pub use auth::*;
mod auth;
pub struct V1 {
}

impl V1 {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
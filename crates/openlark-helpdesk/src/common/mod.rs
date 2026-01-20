pub mod api_endpoints;
pub mod api_utils;

pub use api_endpoints::HelpdeskApiV1;

pub mod constants {
    pub const DEFAULT_PAGE_SIZE: i32 = 20;
    pub const MAX_PAGE_SIZE: i32 = 100;
}

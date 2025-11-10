use crate::error::LarkAPIError;

// Re-export ApiRequest from open-lark-core
pub use open_lark_core::core::api_req::ApiRequest;
pub mod api_resp;
pub mod app_ticket_manager;
pub mod cache;
pub mod config;
pub mod constants;
pub mod endpoints;
pub mod endpoints_original;
pub mod endpoints_unified;
pub mod error;
pub mod error_codes;
pub mod error_helper;
pub mod error_logger;
pub mod error_metrics;
pub mod http;
pub mod improved_response_handler;
pub mod observability;
pub mod performance;
pub mod query_params;
pub mod req_option;
pub mod req_translator;
pub mod request_builder;
pub mod request_executor;
pub mod retry_middleware;
pub mod standard_response;
#[cfg(test)]
pub mod test_utils;
pub mod token_manager;
pub mod trait_system;
pub mod utils;
pub mod validation;

pub type SDKResult<T> = Result<T, LarkAPIError>;

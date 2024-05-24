use crate::core::error::LarkAPIError;

pub mod api_req;
pub mod api_resp;
pub mod app_ticket_manager;
pub mod cache;
pub mod config;
pub mod constants;
pub mod error;
pub mod http;
pub mod req_option;
pub mod req_translator;
pub mod token_manager;
pub mod utils;
// pub mod multi_part;

pub type SDKResult<T> = Result<T, LarkAPIError>;


/// 异步迭代器
/// `async fn next` using AFIT
pub trait AfitAsyncIter {
    type Item;

    async fn next(&mut self) -> Option<Self::Item>;
}
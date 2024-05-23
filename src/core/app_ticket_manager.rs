use std::time::Duration;

use lazy_static::lazy_static;
use log::error;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponse, RawResponse},
    cache::{Cache, LocalCache},
    config::Config,
    constants::{AccessTokenType, APP_TICKET_KEY_PREFIX, APPLY_APP_TICKET_PATH},
    http::Transport,
    SDKResult,
};

lazy_static! {
    pub static ref APP_TICKET_MANAGER: AppTicketManager = AppTicketManager::new();
}

pub struct AppTicketManager {
    cache: LocalCache,
}

impl Default for AppTicketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AppTicketManager {
    pub fn new() -> Self {
        Self {
            cache: LocalCache::new(),
        }
    }

    pub fn set(&mut self, app_id: &str, value: &str, expire_time: Duration) {
        let key = app_ticket_key(app_id);
        self.cache.set(&key, value, expire_time);
    }

    pub fn get(&self, config: &Config) -> Option<String> {
        let key = app_ticket_key(&config.app_id);
        match self.cache.get(&key) {
            None => None,
            Some(ticket) => {
                if ticket.is_empty() {
                    apply_app_ticket(config).ok();
                }

                Some(ticket)
            }
        }
    }
}

fn app_ticket_key(app_id: &str) -> String {
    format!("{}-{}", APP_TICKET_KEY_PREFIX, app_id)
}

pub fn apply_app_ticket(config: &Config) -> SDKResult<()> {
    let resp: ApiResponse<RawResponse> = Transport::request(
        ApiRequest {
            http_method: "POST".to_string(),
            api_path: APPLY_APP_TICKET_PATH.to_string(),
            supported_access_token_types: vec![AccessTokenType::App],
            ..Default::default()
        },
        config,
        None,
    )?;

    if let ApiResponse::Error(error_msg) = resp {
        error!("apply_app_ticket failed: {}", error_msg);
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct ResendAppTicketReq {
    app_id: String,
    app_secret: String,
}

// #[derive(Serialize, Deserialize)]
// struct ResendAppTicketResp {
//     #[serde(skip)]
//     api_resp: ApiResponse,
//     #[serde(flatten)]
//     code_error: ErrorResponse,
// }

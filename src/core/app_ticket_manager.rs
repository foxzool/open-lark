use std::time::Duration;

use lazy_static::lazy_static;
use log::error;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::api_req::ApiReq;
use crate::core::api_resp::{ApiResp, CodeMsg};
use crate::core::cache::{Cache, LocalCache};
use crate::core::config::Config;
use crate::core::constants::{AccessTokenType, APP_TICKET_KEY_PREFIX, APPLY_APP_TICKET_PATH};
use crate::core::http::Transport;
use crate::core::SDKResult;

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
    let resp = Transport::request(
        ApiReq {
            http_method: Method::POST,
            api_path: APPLY_APP_TICKET_PATH.to_string(),
            body: Default::default(),
            query_params: Default::default(),
            path_params: Default::default(),
            supported_access_token_types: vec![AccessTokenType::App],
        },
        config,
        vec![],
    )?;

    let code_error: CodeMsg = serde_json::from_slice(&resp.raw_body)?;
    if code_error.code != 0 {
        error!("apply_app_ticket failed: {}", code_error);
    }

    Ok(())
}
#[derive(Serialize, Deserialize)]
struct ResendAppTicketReq {
    app_id: String,
    app_secret: String,
}

#[derive(Serialize, Deserialize)]
struct ResendAppTicketResp {
    #[serde(skip)]
    api_resp: ApiResp,
    #[serde(flatten)]
    code_error: CodeMsg,
}

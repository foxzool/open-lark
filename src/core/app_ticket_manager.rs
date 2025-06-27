use serde::{Deserialize, Serialize};

use crate::core::{
    cache::QuickCache,
    config::Config,
    constants::{APPLY_APP_TICKET_PATH, APP_TICKET_KEY_PREFIX},
    SDKResult,
};

#[derive(Debug)]
pub struct AppTicketManager {
    pub cache: QuickCache<String>,
}

impl Default for AppTicketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AppTicketManager {
    pub fn new() -> Self {
        Self {
            cache: QuickCache::new(),
        }
    }

    pub fn set(&mut self, app_id: &str, value: &str, expire_time: i32) {
        let key = app_ticket_key(app_id);
        self.cache.set(&key, value.to_string(), expire_time);
    }

    pub async fn get(&self, config: &Config) -> Option<String> {
        let key = app_ticket_key(&config.app_id);
        match self.cache.get(&key) {
            None => None,
            Some(ticket) => {
                if ticket.is_empty() {
                    apply_app_ticket(config).await.ok();
                }

                Some(ticket)
            }
        }
    }
}

fn app_ticket_key(app_id: &str) -> String {
    format!("{APP_TICKET_KEY_PREFIX}-{app_id}")
}

pub async fn apply_app_ticket(config: &Config) -> SDKResult<()> {
    let url = format!("{}{}", config.base_url, APPLY_APP_TICKET_PATH);

    let body = ResendAppTicketReq {
        app_id: config.app_id.clone(),
        app_secret: config.app_secret.clone(),
    };

    let _response = config.http_client.post(&url).json(&body).send().await?;

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
//     api_resp:
//     #[serde(flatten)]
//     code_error: ErrorResponse,
// }

use log::warn;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::core::{
    api_resp::{ApiResponseTrait, RawResponse, ResponseFormat},
    app_ticket_manager::AppTicketManager,
    cache::QuickCache,
    config::Config,
    constants::{
        AppType, APP_ACCESS_TOKEN_INTERNAL_URL_PATH, APP_ACCESS_TOKEN_KEY_PREFIX,
        APP_ACCESS_TOKEN_URL_PATH, EXPIRY_DELTA, TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH, 
        TENANT_ACCESS_TOKEN_URL_PATH,
    },
    error::LarkAPIError,
    SDKResult,
};

#[derive(Debug)]
pub struct TokenManager {
    cache: QuickCache<String>,
}

impl Default for TokenManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenManager {
    pub fn new() -> Self {
        Self {
            cache: QuickCache::new(),
        }
    }
    pub async fn get_app_access_token(
        &mut self,
        config: &Config,
        app_ticket: &str,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<String> {
        let mut token = self
            .cache
            .get(&app_access_token_key(&config.app_id))
            .ok_or_else(|| LarkAPIError::illegal_param("cache error"))?;

        let app_type = config.app_type;
        if token.is_empty() {
            if app_type == AppType::SelfBuild {
                token = self.get_custom_app_access_token_then_cache(config).await?;
            } else {
                token = self
                    .get_marketplace_app_access_token_then_cache(config, app_ticket, app_ticket_manager)
                    .await?;
            }
        }

        Ok(token)
    }

    async fn get_custom_app_access_token_then_cache(
        &mut self,
        config: &Config,
    ) -> SDKResult<String> {
        let url = format!("{}{}", config.base_url, APP_ACCESS_TOKEN_INTERNAL_URL_PATH);
        
        let body = SelfBuiltAppAccessTokenReq {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
        };

        let response = config.http_client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        let resp: AppAccessTokenResp = response.json().await?;
        if resp.raw_response.code == 0 {
            let expire = resp.expire - EXPIRY_DELTA;
            self.cache.set(
                &app_access_token_key(&config.app_id),
                resp.app_access_token.clone(),
                expire,
            );

            Ok(resp.app_access_token)
        } else {
            warn!("custom app appAccessToken cache {:#?}", resp.raw_response);
            Err(LarkAPIError::illegal_param(resp.raw_response.msg.clone()))
        }
    }
    async fn get_marketplace_app_access_token_then_cache(
        &mut self,
        config: &Config,
        app_ticket: &str,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<String> {
        let mut app_ticket = app_ticket.to_string();
        if app_ticket.is_empty() {
            match app_ticket_manager.lock().await.get(config).await {
                None => return Err(LarkAPIError::illegal_param("App ticket is empty")),
                Some(ticket) => {
                    app_ticket = ticket;
                }
            }
        }

        let url = format!("{}{}", config.base_url, APP_ACCESS_TOKEN_URL_PATH);
        
        let body = MarketplaceAppAccessTokenReq {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
            app_ticket,
        };

        let response = config.http_client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        let resp: AppAccessTokenResp = response.json().await?;

        if resp.raw_response.code == 0 {
            let expire = resp.expire - EXPIRY_DELTA;

            self.cache.set(
                &app_access_token_key(&config.app_id),
                resp.app_access_token.clone(),
                expire,
            );

            Ok(resp.app_access_token)
        } else {
            warn!(
                "marketplace app appAccessToken cache {:#?}",
                resp.raw_response
            );
            Err(LarkAPIError::illegal_param(resp.raw_response.msg.clone()))
        }
    }

    pub async fn get_tenant_access_token(
        &mut self,
        config: &Config,
        tenant_key: &str,
        app_ticket: &str,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<String> {
        let mut token = self
            .cache
            .get(&tenant_access_token_key(&config.app_id, tenant_key))
            .unwrap_or_default();
        if token.is_empty() {
            if config.app_type == AppType::SelfBuild {
                token = self
                    .get_custom_tenant_access_token_then_cache(config, tenant_key)
                    .await?;
            } else {
                token = self
                    .get_marketplace_tenant_access_token_then_cache(config, tenant_key, app_ticket, app_ticket_manager)
                    .await?;
            }
        }

        Ok(token)
    }

    async fn get_custom_tenant_access_token_then_cache(
        &mut self,
        config: &Config,
        tenant_key: &str,
    ) -> SDKResult<String> {
        let url = format!("{}{}", config.base_url, TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH);
        
        let body = SelfBuiltTenantAccessTokenReq {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
        };

        let response = config.http_client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        let resp: TenantAccessTokenResp = response.json().await?;

        if resp.raw_response.code == 0 {
            let expire = resp.expire - EXPIRY_DELTA;

            self.cache.set(
                &tenant_access_token_key(&config.app_id, tenant_key),
                resp.tenant_access_token.clone(),
                expire,
            );

            Ok(resp.tenant_access_token)
        } else {
            warn!(
                "custom app tenantAccessToken cache {:#?}",
                resp.raw_response
            );
            Err(LarkAPIError::illegal_param(resp.raw_response.msg.clone()))
        }
    }

    async fn get_marketplace_tenant_access_token_then_cache(
        &mut self,
        config: &Config,
        tenant_key: &str,
        app_ticket: &str,
        app_ticket_manager: &Arc<Mutex<AppTicketManager>>,
    ) -> SDKResult<String> {
        let app_access_token = self
            .get_marketplace_app_access_token_then_cache(config, app_ticket, app_ticket_manager)
            .await?;

        let url = format!("{}{}", config.base_url, TENANT_ACCESS_TOKEN_URL_PATH);
        
        let body = MarketplaceTenantAccessTokenReq {
            app_access_token,
            tenant_key: tenant_key.to_string(),
        };

        let response = config.http_client
            .post(&url)
            .json(&body)
            .header("Authorization", &format!("Bearer {}", &body.app_access_token))
            .send()
            .await?;

        let resp: TenantAccessTokenResp = response.json().await?;

        if resp.raw_response.code == 0 {
            let expire = resp.expire - EXPIRY_DELTA;

            self.cache.set(
                &tenant_access_token_key(&config.app_id, tenant_key),
                resp.tenant_access_token.clone(),
                expire,
            );

            Ok(resp.tenant_access_token)
        } else {
            warn!(
                "marketplace app tenantAccessToken cache {:#?}",
                resp.raw_response
            );
            Err(LarkAPIError::illegal_param(resp.raw_response.msg.clone()))
        }
    }
}

fn app_access_token_key(app_id: &str) -> String {
    format!("{}-{}", APP_ACCESS_TOKEN_KEY_PREFIX, app_id)
}

fn tenant_access_token_key(app_id: &str, tenant_key: &str) -> String {
    format!("{}-{}-{}", APP_ACCESS_TOKEN_KEY_PREFIX, app_id, tenant_key)
}

#[derive(Debug, Serialize, Deserialize)]
struct SelfBuiltAppAccessTokenReq {
    app_id: String,
    app_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SelfBuiltTenantAccessTokenReq {
    app_id: String,
    app_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppAccessTokenResp {
    #[serde(flatten)]
    raw_response: RawResponse,
    expire: i32,
    app_access_token: String,
}

impl ApiResponseTrait for AppAccessTokenResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

#[derive(Serialize, Deserialize)]
struct MarketplaceAppAccessTokenReq {
    app_id: String,
    app_secret: String,
    app_ticket: String,
}

#[derive(Serialize, Deserialize)]
struct MarketplaceTenantAccessTokenReq {
    app_access_token: String,
    tenant_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TenantAccessTokenResp {
    #[serde(flatten)]
    raw_response: RawResponse,
    expire: i32,
    tenant_access_token: String,
}

impl ApiResponseTrait for TenantAccessTokenResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

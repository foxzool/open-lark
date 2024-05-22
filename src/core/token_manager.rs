use std::{sync::Mutex, time::Duration};

use lazy_static::lazy_static;
use log::warn;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiReq,
    api_resp::{ApiResponse, ApiResponseTrait, RawResponse},
    app_ticket_manager::APP_TICKET_MANAGER,
    cache::{Cache, LocalCache},
    config::Config,
    constants::{
        AccessTokenType, APP_ACCESS_TOKEN_INTERNAL_URL_PATH, APP_ACCESS_TOKEN_KEY_PREFIX, AppType,
        EXPIRY_DELTA, TENANT_ACCESS_TOKEN_URL_PATH,
    },
    error::LarkAPIError,
    http::Transport,
    SDKResult,
};

lazy_static! {
    pub static ref TOKEN_MANAGER: Mutex<TokenManager> = Mutex::new(TokenManager::new());
}

pub struct TokenManager {
    cache: LocalCache,
}

impl Default for TokenManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenManager {
    pub fn new() -> Self {
        let cache = LocalCache::new();
        Self { cache } //
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.cache.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str, expire_time: Duration) {
        self.cache.set(key, value, expire_time);
    }

    pub fn get_app_access_token(&mut self, config: &Config, app_ticket: &str) -> SDKResult<String> {
        let mut token = self
            .get(&app_access_token_key(&config.app_id))
            .unwrap_or_default();

        let app_type = config.app_type;
        if app_type == AppType::SelfBuild {
            token = self.get_custom_app_access_token_then_cache(config)?;
        } else {
            token = self.get_marketplace_app_access_token_then_cache(config, app_ticket)?;
        }

        Ok(token)
    }

    fn get_custom_app_access_token_then_cache(&mut self, config: &Config) -> SDKResult<String> {
        let req = ApiReq {
            http_method: "POST".to_string(),
            api_path: APP_ACCESS_TOKEN_INTERNAL_URL_PATH.to_string(),
            supported_access_token_types: vec![AccessTokenType::None],
            ..Default::default()
        };
        let resp: ApiResponse<AppAccessTokenResp> = Transport::request(req, config, None)?;
        match resp {
            ApiResponse::Success { data, .. } => {
                let expire = Duration::from_secs(data.expire as u64) - EXPIRY_DELTA;
                self.set(
                    &app_access_token_key(&config.app_id),
                    &data.app_access_token,
                    expire,
                );

                Ok(data.app_access_token)
            }
            ApiResponse::Error(error_resp) => {
                warn!("custom app appAccessToken cache {:#?}", error_resp);
                Err(LarkAPIError::CodeError(error_resp))
            }
        }
    }
    fn get_marketplace_app_access_token_then_cache(
        &mut self,
        config: &Config,
        app_ticket: &str,
    ) -> SDKResult<String> {
        let mut app_ticket = app_ticket.to_string();
        if app_ticket.is_empty() {
            match APP_TICKET_MANAGER.get(config) {
                None => {
                    return Err(LarkAPIError::IllegalParamError(
                        "App ticket is empty".to_string(),
                    ))
                }
                Some(ticket) => {
                    app_ticket = ticket;
                }
            }
        }

        let body = serde_json::to_vec(&MarketplaceAppAccessTokenReq {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
            app_ticket,
        })?;
        let body = body.into();

        let req = ApiReq {
            http_method: "POST".to_string(),
            api_path: APP_ACCESS_TOKEN_INTERNAL_URL_PATH.to_string(),
            body,
            supported_access_token_types: vec![AccessTokenType::None],
            ..Default::default()
        };
        let raw_resp: ApiResponse<AppAccessTokenResp> = Transport::request(req, config, None)?;
        match raw_resp {
            ApiResponse::Success { data, .. } => {
                let expire = Duration::from_secs(data.expire as u64) - EXPIRY_DELTA;

                self.set(
                    &app_access_token_key(&config.app_id),
                    &data.app_access_token,
                    expire,
                );

                Ok(data.app_access_token)
            }
            ApiResponse::Error(error_resp) => {
                warn!("marketplace app appAccessToken cache {:#?}", error_resp);
                Err(LarkAPIError::CodeError(error_resp))
            }
        }
    }

    pub fn get_tenant_access_token(
        &mut self,
        config: &Config,
        tenant_key: &str,
        app_ticket: &str,
    ) -> SDKResult<String> {
        let mut token = self
            .get(&tenant_access_token_key(&config.app_id, tenant_key))
            .unwrap_or_default();
        if token.is_empty() {
            if config.app_type == AppType::SelfBuild {
                token = self.get_custom_tenant_access_token_then_cache(config, tenant_key)?;
            } else {
                token = self.get_marketplace_tenant_access_token_then_cache(
                    config, tenant_key, app_ticket,
                )?;
            }
        }

        Ok(token)
    }

    fn get_custom_tenant_access_token_then_cache(
        &mut self,
        config: &Config,
        tenant_key: &str,
    ) -> SDKResult<String> {
        let body = serde_json::to_vec(&SelfBuiltAppAccessTokenReq {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
        })?;
        let body = body.into();
        let req = ApiReq {
            http_method: "POST".to_string(),
            api_path: APP_ACCESS_TOKEN_INTERNAL_URL_PATH.to_string(),
            body,
            supported_access_token_types: vec![AccessTokenType::None],
            ..Default::default()
        };
        let raw_resp: ApiResponse<TenantAccessTokenResp> = Transport::request(req, config, None)?;

        match raw_resp {
            ApiResponse::Success { data, .. } => {
                let expire = Duration::from_secs(data.expire as u64) - EXPIRY_DELTA;

                self.set(
                    &tenant_access_token_key(&config.app_id, tenant_key),
                    &data.tenant_access_token,
                    expire,
                );

                Ok(data.tenant_access_token)
            }
            ApiResponse::Error(error_resp) => {
                warn!("custom app tenantAccessToken cache {:#?}", error_resp);
                Err(LarkAPIError::CodeError(error_resp))
            }
        }
    }

    fn get_marketplace_tenant_access_token_then_cache(
        &mut self,
        config: &Config,
        tenant_key: &str,
        app_ticket: &str,
    ) -> SDKResult<String> {
        let app_access_token =
            self.get_marketplace_app_access_token_then_cache(config, app_ticket)?;

        let body = serde_json::to_vec(&MarketplaceTenantAccessTokenReq {
            app_access_token,
            tenant_key: tenant_key.to_string(),
        })?;
        let body = body.into();
        let req = ApiReq {
            http_method: "POST".to_string(),
            api_path: TENANT_ACCESS_TOKEN_URL_PATH.to_string(),
            body,
            supported_access_token_types: vec![AccessTokenType::None],
            ..Default::default()
        };
        let raw_resp: ApiResponse<TenantAccessTokenResp> = Transport::request(req, config, None)?;
        match raw_resp {
            ApiResponse::Success { data, .. } => {
                let expire = Duration::from_secs(data.expire as u64) - EXPIRY_DELTA;

                self.set(
                    &tenant_access_token_key(&config.app_id, tenant_key),
                    &data.tenant_access_token,
                    expire,
                );

                Ok(data.tenant_access_token)
            }
            ApiResponse::Error(error_resp) => {
                warn!("marketplace app tenantAccessToken cache {:#?}", error_resp);
                Err(LarkAPIError::CodeError(error_resp))
            }
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
    fn standard_data_format() -> bool {
        false
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
    fn standard_data_format() -> bool {
        false
    }
}

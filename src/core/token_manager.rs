use lazy_static::lazy_static;
use log::warn;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use tokio::{sync::Mutex, time::Instant};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, RawResponse, ResponseFormat},
    app_ticket_manager::APP_TICKET_MANAGER,
    cache::QuickCache,
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
    cache: QuickCache,
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
    ) -> SDKResult<String> {
        let mut token = self
            .cache
            .get(&app_access_token_key(&config.app_id))
            .ok_or(LarkAPIError::IllegalParamError("cache error".to_string()))?;

        let app_type = config.app_type;
        if token.is_empty() {
            if app_type == AppType::SelfBuild {
                token = self.get_custom_app_access_token_then_cache(config).await?;
            } else {
                token = self
                    .get_marketplace_app_access_token_then_cache(config, app_ticket)
                    .await?;
            }
        }

        Ok(token)
    }

    async fn get_custom_app_access_token_then_cache(
        &mut self,
        config: &Config,
    ) -> SDKResult<String> {
        let req = ApiRequest {
            http_method: Method::POST,
            api_path: APP_ACCESS_TOKEN_INTERNAL_URL_PATH.to_string(),
            supported_access_token_types: vec![AccessTokenType::None],
            ..Default::default()
        };
        let resp: BaseResponse<AppAccessTokenResp> = Transport::request(req, config, None).await?;
        if resp.success() {
            let data = resp.data.unwrap();
            let expire = data.expire - EXPIRY_DELTA;
            self.cache.set(
                &app_access_token_key(&config.app_id),
                &data.app_access_token,
                expire,
            );

            Ok(data.app_access_token)
        } else {
            warn!("custom app appAccessToken cache {:#?}", resp.raw_response);
            Err(LarkAPIError::IllegalParamError(resp.msg().to_string()))
        }
    }
    async fn get_marketplace_app_access_token_then_cache(
        &mut self,
        config: &Config,
        app_ticket: &str,
    ) -> SDKResult<String> {
        let mut app_ticket = app_ticket.to_string();
        if app_ticket.is_empty() {
            match APP_TICKET_MANAGER.get(config).await {
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

        let req = ApiRequest {
            http_method: Method::POST,
            api_path: APP_ACCESS_TOKEN_INTERNAL_URL_PATH.to_string(),
            body,
            supported_access_token_types: vec![AccessTokenType::None],
            ..Default::default()
        };
        let resp: BaseResponse<AppAccessTokenResp> = Transport::request(req, config, None).await?;

        if resp.success() {
            let data = resp.data.unwrap();
            let expire = data.expire - EXPIRY_DELTA;

            self.cache.set(
                &app_access_token_key(&config.app_id),
                &data.app_access_token,
                expire,
            );

            Ok(data.app_access_token)
        } else {
            warn!(
                "marketplace app appAccessToken cache {:#?}",
                resp.raw_response
            );
            Err(LarkAPIError::IllegalParamError(resp.msg().to_string()))
        }
    }

    pub async fn get_tenant_access_token(
        &mut self,
        config: &Config,
        tenant_key: &str,
        app_ticket: &str,
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
                    .get_marketplace_tenant_access_token_then_cache(config, tenant_key, app_ticket)
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
        let body = serde_json::to_vec(&SelfBuiltAppAccessTokenReq {
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
        })?;

        let req = ApiRequest {
            http_method: Method::POST,
            api_path: APP_ACCESS_TOKEN_INTERNAL_URL_PATH.to_string(),
            body,
            supported_access_token_types: vec![AccessTokenType::None],
            ..Default::default()
        };
        let resp: BaseResponse<TenantAccessTokenResp> =
            Transport::request(req, config, None).await?;

        if resp.success() {
            let data = resp.data.unwrap();
            let expire = data.expire - EXPIRY_DELTA;

            self.cache.set(
                &tenant_access_token_key(&config.app_id, tenant_key),
                &data.tenant_access_token,
                expire,
            );

            Ok(data.tenant_access_token)
        } else {
            warn!(
                "custom app tenantAccessToken cache {:#?}",
                resp.raw_response
            );
            Err(LarkAPIError::IllegalParamError(resp.msg().to_string()))
        }
    }

    async fn get_marketplace_tenant_access_token_then_cache(
        &mut self,
        config: &Config,
        tenant_key: &str,
        app_ticket: &str,
    ) -> SDKResult<String> {
        let app_access_token = self
            .get_marketplace_app_access_token_then_cache(config, app_ticket)
            .await?;

        let body = serde_json::to_vec(&MarketplaceTenantAccessTokenReq {
            app_access_token,
            tenant_key: tenant_key.to_string(),
        })?;

        let req = ApiRequest {
            http_method: Method::POST,
            api_path: TENANT_ACCESS_TOKEN_URL_PATH.to_string(),
            body,
            supported_access_token_types: vec![AccessTokenType::None],
            ..Default::default()
        };
        let resp: BaseResponse<TenantAccessTokenResp> =
            Transport::request(req, config, None).await?;

        if resp.success() {
            let data = resp.data.unwrap();
            let expire = data.expire - EXPIRY_DELTA;

            self.cache.set(
                &tenant_access_token_key(&config.app_id, tenant_key),
                &data.tenant_access_token,
                expire,
            );

            Ok(data.tenant_access_token)
        } else {
            warn!(
                "marketplace app tenantAccessToken cache {:#?}",
                resp.raw_response
            );
            Err(LarkAPIError::IllegalParamError(resp.msg().to_string()))
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

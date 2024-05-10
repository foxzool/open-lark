use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use moka::Expiry;
use moka::sync::Cache;
use crate::core::config::Config;

use crate::core::error::LarkAPIError;
use crate::core::http::Transport;
use crate::core::model::{BaseResponse};
use crate::core::token::{
    AccessTokenResponse, CreateIsvAppTokenRequest, CreateSelfAppTokenRequest,
    CreateTokenRequestBody,
};
use crate::core::token::create_isv_tenant_token_request::CreateIsvTenantTokenRequest;
use crate::core::token::create_self_tenant_token_request::CreateSelfTenantTokenRequest;

lazy_static! {
    pub static ref TOKEN_MANAGER: TokenManager = TokenManager::new();
}
pub struct TokenManager {
    cache: Cache<String, (Duration, String)>,
}

pub struct TokenExpiry;

impl Expiry<String, (Duration, String)> for TokenExpiry {
    fn expire_after_update(
        &self,
        _key: &String,
        value: &(Duration, String),
        _update_at: Instant,
        _duration: Option<Duration>,
    ) -> Option<Duration> {
        Some(value.0)
    }
}

impl TokenManager {
    pub fn new() -> Self {
        let cache = Cache::builder()
            .max_capacity(1000)
            .expire_after(TokenExpiry)
            .build();
        Self { cache }
    }

    ///
    pub fn get_self_app_token(&self, config: &Config) -> Result<String, LarkAPIError> {
        let cache_key = format!("self_app_token:{}", config.app_id.as_ref().unwrap());
        return match self.cache.get(&cache_key) {
            Some(token) => Ok(token.1),
            // 缓存不存在则发起请求获取token
            None => {
                let req = CreateSelfAppTokenRequest::builder()
                    .request_body(
                        CreateTokenRequestBody::builder()
                            .app_id(config.app_id.as_ref().unwrap().to_string())
                            .app_secret(config.app_secret.as_ref().unwrap().to_string())
                            .build(),
                    )
                    .build();

                let raw = Transport::execute(config, &req.base_request, None)?;
                let resp = BaseResponse::<AccessTokenResponse>::from_response(raw)?;
                if !resp.success() {
                    return Err(LarkAPIError::ObtainAccessTokenException(
                        "obtain self app access token failed".to_string(),
                        resp.code,
                        resp.msg,
                    ));
                }
                let access_token_resp = resp.content.unwrap();
                // 写缓存
                let token = access_token_resp.tenant_access_token;
                // 提前10分钟过期
                let expire = Duration::from_secs((access_token_resp.expire - 10 * 60) as u64);
                self.cache.get_with(cache_key, || (expire, token.clone()));
                Ok(token)
            }
        };
    }

    /// 获取自建应用的租户级访问令牌
    pub fn get_self_tenant_token(&self, config: &Config) -> Result<String, LarkAPIError> {
        let cache_key = format!("self_tenant_token:{}", config.app_id.as_ref().unwrap());
        return match self.cache.get(&cache_key) {
            Some(token) => Ok(token.1),
            // 缓存不存在则发起请求获取token
            None => {
                let req = CreateSelfTenantTokenRequest::builder()
                    .request_body(
                        CreateTokenRequestBody::builder()
                            .app_id(config.app_id.as_ref().unwrap().to_string())
                            .app_secret(config.app_secret.as_ref().unwrap().to_string())
                            .build(),
                    )
                    .build();

                let raw = Transport::execute(config, &req.base_request, None)?;
                let resp = BaseResponse::<AccessTokenResponse>::from_response(raw)?;
                if !resp.success() {
                    return Err(LarkAPIError::ObtainAccessTokenException(
                        "obtain self tenant access token failed".to_string(),
                        resp.code,
                        resp.msg,
                    ));
                }
                let access_token_resp = resp.content.unwrap();
                // 写缓存
                let token = access_token_resp.tenant_access_token;
                // 提前10分钟过期
                let expire = Duration::from_secs((access_token_resp.expire - 10 * 60) as u64);
                self.cache.get_with(cache_key, || (expire, token.clone()));

                Ok(token)
            }
        };
    }

    pub fn get_isv_app_token(&self, config: &Config) -> Result<String, LarkAPIError> {
        let cache_key = format!("isv_app_token:{}", config.app_id.as_ref().unwrap(),);
        return match self.cache.get(&cache_key) {
            Some(token) => Ok(token.1),
            // 缓存不存在则发起请求获取token
            None => {
                let req = CreateIsvAppTokenRequest::builder()
                    .request_body(
                        CreateTokenRequestBody::builder()
                            .app_id(config.app_id.as_ref().unwrap().to_string())
                            .app_secret(config.app_secret.as_ref().unwrap().to_string())
                            .build(),
                    )
                    .build();
                let raw = Transport::execute(config, &req.base_request, None)?;
                let resp = BaseResponse::<AccessTokenResponse>::from_response(raw)?;
                if !resp.success() {
                    return Err(LarkAPIError::ObtainAccessTokenException(
                        "obtain isv app access token failed".to_string(),
                        resp.code,
                        resp.msg,
                    ));
                }
                let access_token_resp = resp.content.unwrap();
                // 写缓存
                let token = access_token_resp.tenant_access_token;
                // 提前10分钟过期
                let expire = Duration::from_secs((access_token_resp.expire - 10 * 60) as u64);
                self.cache.get_with(cache_key, || (expire, token.clone()));
                Ok(token)
            }
        };
    }
    pub fn get_isv_tenants_token(
        &self,
        config: &Config,
        tenant_key: &str,
    ) -> Result<String, LarkAPIError> {
        let cache_key = format!(
            "isv_tenant_token:{}:{}",
            config.app_id.as_ref().unwrap(),
            tenant_key
        );
        return match self.cache.get(&cache_key) {
            Some(token) => Ok(token.1),
            // 缓存不存在则发起请求获取token
            None => {
                let req = CreateIsvTenantTokenRequest::builder()
                    .request_body(
                        CreateTokenRequestBody::builder()
                            .app_access_token(config.app_id.as_ref().unwrap().to_string())
                            .tenant_key(tenant_key.to_string())
                            .build(),
                    )
                    .build();
                let raw = Transport::execute(config, &req.base_request, None)?;
                let resp = BaseResponse::<AccessTokenResponse>::from_response(raw)?;
                if !resp.success() {
                    return Err(LarkAPIError::ObtainAccessTokenException(
                        "obtain  isv tenant access token failed".to_string(),
                        resp.code,
                        resp.msg,
                    ));
                }
                let access_token_resp = resp.content.unwrap();
                // 写缓存
                let token = access_token_resp.tenant_access_token;
                // 提前10分钟过期
                let expire = Duration::from_secs((access_token_resp.expire - 10 * 60) as u64);
                self.cache.get_with(cache_key, || (expire, token.clone()));
                Ok(token)
            }
        };
    }
}

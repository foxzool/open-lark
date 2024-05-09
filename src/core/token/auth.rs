use std::collections::HashSet;

use crate::core::enum_type::{AccessTokenType, AppType};
use crate::core::error::LarkAPIError;
use crate::core::model::{BaseRequest, Config, RequestOption};
use crate::core::token::TOKEN_MANAGER;

pub fn verify(
    config: &Config,
    request: &mut BaseRequest,
    option: &mut RequestOption,
) -> Result<(), LarkAPIError> {
    // 接口无需token
    if request.token_types.is_empty() {
        return Ok(());
    }

    // 如开启token配置，需手动传入token
    if config.enable_set_token {
        if option.tenant_access_token.is_some()
            && request.token_types.contains(&AccessTokenType::TENANT)
        {
            request.token_types = HashSet::from([AccessTokenType::TENANT]);
            return Ok(());
        }
        if option.app_access_token.is_some() && request.token_types.contains(&AccessTokenType::APP)
        {
            request.token_types = HashSet::from([AccessTokenType::APP]);
            return Ok(());
        }
        if option.user_access_token.is_some()
            && request.token_types.contains(&AccessTokenType::USER)
        {
            request.token_types = HashSet::from([AccessTokenType::USER]);
            return Ok(());
        }
    }

    // 未开启token配置，根据app_id/app_secret获取token
    if config.app_id.is_none() || config.app_secret.is_none() {
        return Err(LarkAPIError::NoAuthorization(
            "app_id or app_secret is None".to_string(),
        ));
    }

    if request.token_types.contains(&AccessTokenType::TENANT) {
        let mut tenant_access_token = "".to_string();
        if AppType::SELF == config.app_type {
            tenant_access_token = TOKEN_MANAGER.get_self_tenant_token(&config)?;
        } else {
            if option.tenant_key.is_none() {
                return Err(LarkAPIError::NoAuthorization(
                    "tenant_key is None".to_string(),
                ));
            }
            tenant_access_token = TOKEN_MANAGER
                .get_isv_tenants_token(&config, option.tenant_key.as_ref().unwrap())?;
        }

        option.tenant_access_token = Some(tenant_access_token.clone());
        request.token_types = HashSet::from([AccessTokenType::TENANT]);
        return Ok(());
    }

    if request.token_types.contains(&AccessTokenType::APP) {
        let mut app_access_token = "".to_string();
        if AppType::SELF == config.app_type {
            app_access_token = TOKEN_MANAGER.get_self_app_token(&config)?;
        } else {
            app_access_token = TOKEN_MANAGER.get_isv_app_token(&config)?;
        }
        option.app_access_token = Some(app_access_token.clone());
        request.token_types = HashSet::from([AccessTokenType::APP]);
        return Ok(());
    }

    return Ok(());
}

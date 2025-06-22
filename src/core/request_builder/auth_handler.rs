use crate::core::{
    config::Config, constants::AccessTokenType, error::LarkAPIError, req_option::RequestOption,
};
use reqwest::RequestBuilder;

/// 处理不同类型的 AccessToken 认证
pub struct AuthHandler;

impl AuthHandler {
    /// 根据认证类型为请求添加相应的认证头
    pub async fn apply_auth(
        req_builder: RequestBuilder,
        access_token_type: AccessTokenType,
        config: &Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        match access_token_type {
            AccessTokenType::None => Ok(req_builder),
            AccessTokenType::App => Self::apply_app_auth(req_builder, config, option).await,
            AccessTokenType::Tenant => Self::apply_tenant_auth(req_builder, config, option).await,
            AccessTokenType::User => Ok(Self::apply_user_auth(req_builder, option)),
        }
    }

    /// 应用应用级认证
    async fn apply_app_auth(
        req_builder: RequestBuilder,
        config: &Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let app_access_token = if !option.app_access_token.is_empty() {
            option.app_access_token.clone()
        } else if config.enable_token_cache {
            let mut token_manager = config.token_manager.lock().await;
            token_manager
                .get_app_access_token(config, &option.app_ticket, &config.app_ticket_manager)
                .await?
        } else {
            return Err(LarkAPIError::MissingAccessToken);
        };

        Ok(Self::add_auth_header(req_builder, &app_access_token))
    }

    /// 应用租户级认证
    async fn apply_tenant_auth(
        req_builder: RequestBuilder,
        config: &Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let tenant_access_token = if !option.tenant_access_token.is_empty() {
            option.tenant_access_token.clone()
        } else if config.enable_token_cache {
            let mut token_manager = config.token_manager.lock().await;
            token_manager
                .get_tenant_access_token(
                    config,
                    &option.tenant_key,
                    &option.app_ticket,
                    &config.app_ticket_manager,
                )
                .await?
        } else {
            return Err(LarkAPIError::MissingAccessToken);
        };

        Ok(Self::add_auth_header(req_builder, &tenant_access_token))
    }

    /// 应用用户级认证
    fn apply_user_auth(req_builder: RequestBuilder, option: &RequestOption) -> RequestBuilder {
        Self::add_auth_header(req_builder, &option.user_access_token)
    }

    /// 添加 Authorization 头
    fn add_auth_header(req_builder: RequestBuilder, token: &str) -> RequestBuilder {
        req_builder.header("Authorization", format!("Bearer {}", token))
    }
}

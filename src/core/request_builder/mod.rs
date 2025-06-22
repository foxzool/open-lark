mod auth_handler;
mod header_builder;
mod multipart_builder;

pub use auth_handler::AuthHandler;
pub use header_builder::HeaderBuilder;
pub use multipart_builder::MultipartBuilder;

use crate::core::{
    api_req::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    req_option::RequestOption,
};
use reqwest::RequestBuilder;
use std::{future::Future, pin::Pin};

/// 统一的请求构建器，负责协调各个子构建器
pub struct UnifiedRequestBuilder;

impl UnifiedRequestBuilder {
    pub fn build<'a>(
        req: &'a mut ApiRequest,
        access_token_type: AccessTokenType,
        config: &'a Config,
        option: &'a RequestOption,
    ) -> Pin<Box<dyn Future<Output = Result<RequestBuilder, LarkAPIError>> + Send + 'a>> {
        Box::pin(async move {
            // 1. 构建基础请求
            let url = Self::build_url(config, req)?;
            let mut req_builder = config
                .http_client
                .request(req.http_method.clone(), url.as_ref());

            // 2. 构建请求头
            req_builder = HeaderBuilder::build_headers(req_builder, config, option);

            // 3. 处理认证
            req_builder =
                AuthHandler::apply_auth(req_builder, access_token_type, config, option).await?;

            // 4. 处理请求体
            if !req.file.is_empty() {
                req_builder = MultipartBuilder::build_multipart(req_builder, &req.body, &req.file)?;
            } else if !req.body.is_empty() {
                req_builder = req_builder.body(req.body.clone());
                req_builder = req_builder.header(
                    crate::core::constants::CONTENT_TYPE_HEADER,
                    crate::core::constants::DEFAULT_CONTENT_TYPE,
                );
            }

            Ok(req_builder)
        })
    }

    fn build_url(config: &Config, req: &ApiRequest) -> Result<url::Url, LarkAPIError> {
        let path = format!("{}{}", config.base_url, req.api_path);
        let query_params = req
            .query_params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect::<Vec<_>>();
        Ok(url::Url::parse_with_params(&path, query_params)?)
    }
}

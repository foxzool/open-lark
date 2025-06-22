use std::{future::Future, pin::Pin};

use reqwest::RequestBuilder;

use crate::core::{
    api_req::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    req_option::RequestOption, request_builder::UnifiedRequestBuilder,
};

pub struct ReqTranslator;

impl ReqTranslator {
    pub fn translate<'a>(
        req: &'a mut ApiRequest,
        access_token_type: AccessTokenType,
        config: &'a Config,
        option: &'a RequestOption,
    ) -> Pin<Box<dyn Future<Output = Result<RequestBuilder, LarkAPIError>> + Send + 'a>> {
        // 委托给新的统一请求构建器
        UnifiedRequestBuilder::build(req, access_token_type, config, option)
    }
}

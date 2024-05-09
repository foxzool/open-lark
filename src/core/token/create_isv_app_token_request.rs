use reqwest::Method;
use crate::core::model::BaseRequest;
use crate::core::token::CreateTokenRequestBody;

#[derive(Default)]
pub struct CreateIsvAppTokenRequest {
    pub(crate) base_request: BaseRequest,
    request_body: Option<CreateTokenRequestBody>
}



impl CreateIsvAppTokenRequest {
    pub fn builder() -> CreateIsvAppTokenRequestBuilder {
        CreateIsvAppTokenRequestBuilder::default()
    }
}

pub struct CreateIsvAppTokenRequestBuilder {
    create_isv_app_token_request: CreateIsvAppTokenRequest
}

impl Default for CreateIsvAppTokenRequestBuilder {
    fn default() -> Self {
        let mut request = CreateIsvAppTokenRequest::default();
        request.base_request.http_method = Some(Method::POST);
        request.base_request.uri = Some("/open-apis/auth/v3/app_access_token".to_string());
        Self {
            create_isv_app_token_request: request
        }
    }
}

impl CreateIsvAppTokenRequestBuilder {
    pub fn request_body(mut self, request_body: CreateTokenRequestBody) -> Self {
        self.create_isv_app_token_request.request_body = Some(request_body.clone());
        self.create_isv_app_token_request.base_request.body = Some(serde_json::to_value(request_body).unwrap());
        self
    }

    pub fn build(self) -> CreateIsvAppTokenRequest {
        self.create_isv_app_token_request
    }
}
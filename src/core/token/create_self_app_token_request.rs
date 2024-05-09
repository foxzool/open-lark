use reqwest::Method;
use crate::core::model::BaseRequest;
use crate::core::token::CreateTokenRequestBody;

#[derive(Default)]
pub struct CreateSelfAppTokenRequest {
    pub(crate) base_request: BaseRequest,
    request_body: Option<CreateTokenRequestBody>
}



impl CreateSelfAppTokenRequest {
    pub fn builder() -> CreateSelfAppTokenRequestBuilder {
        CreateSelfAppTokenRequestBuilder::default()
    }
}

pub struct CreateSelfAppTokenRequestBuilder {
    create_self_app_token_request: CreateSelfAppTokenRequest
}

impl Default for CreateSelfAppTokenRequestBuilder {
    fn default() -> Self {
        let mut request = CreateSelfAppTokenRequest::default();
        request.base_request.http_method = Some(Method::POST);
        request.base_request.uri = Some("/open-apis/auth/v3/app_access_token/internal".to_string());
        Self {
            create_self_app_token_request: request
        }
    }
}

impl CreateSelfAppTokenRequestBuilder {
    pub fn request_body(mut self, request_body: CreateTokenRequestBody) -> Self {
        self.create_self_app_token_request.request_body = Some(request_body.clone());
        self.create_self_app_token_request.base_request.body = Some(serde_json::to_value(request_body).unwrap());
        self
    }

    pub fn build(self) -> CreateSelfAppTokenRequest {
        self.create_self_app_token_request
    }
}
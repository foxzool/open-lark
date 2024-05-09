use crate::core::model::BaseRequest;
use crate::core::token::CreateTokenRequestBody;

#[derive(Default)]
pub struct CreateIsvTenantTokenRequest {
    pub(crate) base_request: BaseRequest,
    request_body: Option<CreateTokenRequestBody>
}

impl CreateIsvTenantTokenRequest {
    pub fn builder() -> CreateIsvTenantTokenRequestBuilder {
        CreateIsvTenantTokenRequestBuilder::default()
    }
}


pub struct CreateIsvTenantTokenRequestBuilder {
    create_isv_tenant_token_request: CreateIsvTenantTokenRequest
}

impl Default for CreateIsvTenantTokenRequestBuilder {
    fn default() -> Self {
        let mut request = CreateIsvTenantTokenRequest::default();
        request.base_request.http_method = Some(reqwest::Method::POST);
        request.base_request.uri = Some("/auth/v3/tenant_access_token".to_string());
        Self {
            create_isv_tenant_token_request: request
        }
    }
}

impl CreateIsvTenantTokenRequestBuilder {
    pub fn request_body(mut self, request_body: CreateTokenRequestBody) -> Self {
        self.create_isv_tenant_token_request.request_body = Some(request_body.clone());
        self.create_isv_tenant_token_request.base_request.body = Some(serde_json::to_value(request_body).unwrap());
        self
    }

    pub fn build(self) -> CreateIsvTenantTokenRequest {
        self.create_isv_tenant_token_request
    }
}
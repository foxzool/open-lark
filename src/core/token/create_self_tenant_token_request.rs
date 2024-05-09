use reqwest::Method;
use crate::core::model::BaseRequest;
use crate::core::token::CreateTokenRequestBody;

#[derive(Default)]
pub struct CreateSelfTenantTokenRequest {
    pub(crate) base_request: BaseRequest,
    request_body: Option<CreateTokenRequestBody>
}



impl CreateSelfTenantTokenRequest {
    pub fn builder() -> CreateSelfTenantTokenRequestBuilder {
        CreateSelfTenantTokenRequestBuilder::default()
    }
}

pub struct CreateSelfTenantTokenRequestBuilder {
    create_self_tenant_token_request: CreateSelfTenantTokenRequest
}

impl Default for CreateSelfTenantTokenRequestBuilder {
    fn default() -> Self {
        let mut request = CreateSelfTenantTokenRequest::default();
        request.base_request.http_method = Some(Method::POST);
        request.base_request.uri = Some("/open-apis/auth/v3/tenant_access_token/internal".to_string());
        Self {
            create_self_tenant_token_request: request
        }
    }
}

impl CreateSelfTenantTokenRequestBuilder {
    pub fn request_body(mut self, request_body: CreateTokenRequestBody) -> Self {
        self.create_self_tenant_token_request.request_body = Some(request_body.clone());
        self.create_self_tenant_token_request.base_request.body = Some(serde_json::to_value(request_body).unwrap());
        self
    }

    pub fn build(self) -> CreateSelfTenantTokenRequest {
        self.create_self_tenant_token_request
    }
}
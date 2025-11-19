use crate::{
    constants::AccessTokenType, http::Transport,
    api::ApiRequest, api::{ApiResponseTrait, BaseResponse}, config::Config,
};
use serde::{Deserialize, Serialize};

/// 自定义用户字段服务
pub struct CustomAttrService {
    config: Config,
}

impl CustomAttrService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取企业自定义用户字段
    pub async fn list(
        &self,
        request: openlark_core::contact::v3::v3::custom_attr::ListCustomAttrRequest,
        option: Option<crate::req_option::RequestOption>,
    ) -> crate::SDKResult<crate::api::Response<openlark_core::contact::v3::v3::custom_attr::ListCustomAttrResponse>> {
        let mut api_req = ApiRequest::default();
        api_req.set_method(reqwest::Method::GET);
        api_req.set_api_path(
            crate::endpoints::contact::CONTACT_V3_CUSTOM_ATTRS.to_string()
        );
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User,
        ]);
        
        api_req.query_params = request.query_params;
        api_req.body = serde_json::to_vec(&request)?;
        
        let api_resp = Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

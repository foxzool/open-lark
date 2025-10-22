use open_lark_core::core::{
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
    constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
};
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 工作城市服务
pub struct WorkCityService {
    config: Config,
}

impl WorkCityService {
    pub fn new(config: Config) -> Self {
    Self { config }
    }
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact/get

    /// 获取单个工作城市信息
    pub async fn get(&self, work_city_id: &str) -> open_lark_core::core::SDKResult<GetWorkCityResponse> {
            let api_req = ApiRequest {
                http_method: reqwest::Method::GET,
                api_path: EndpointBuilder::replace_param(
                    open_lark_core::core::endpoints::contact::CONTACT_V3_WORK_CITIES_GET,
                    "work_city_id",
                    work_city_id,
                ),
                supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
                body: Vec::new(),
                query_params: std::collections::HashMap::new(),
                ..Default::default()
            };

    let resp = Transport::<GetWorkCityResponse>::request(api_req, &self.config, None).await?;
    Ok(resp.data.unwrap_or_default());
    }

    /// 获取租户工作城市列表
    pub async fn list(
    &self,
    _req: &ListWorkCitiesRequest,
    ) -> open_lark_core::core::SDKResult<ListWorkCitiesResponse> {
            let api_req = ApiRequest {
                http_method: reqwest::Method::GET,
                api_path: open_lark_core::core::endpoints::contact::CONTACT_V3_WORK_CITIES.to_string(),
                supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
                body: Vec::new(),
                query_params: std::collections::HashMap::new(),
                ..Default::default()
            };

    let resp = Transport::<ListWorkCitiesResponse>::request(api_req, &self.config, None).await?;
    Ok(resp.data.unwrap_or_default());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetWorkCityResponse {
    pub work_city: WorkCity,
}

impl ApiResponseTrait for GetWorkCityResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
    open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListWorkCitiesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListWorkCitiesResponse {
    pub items: Vec<WorkCity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListWorkCitiesResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
    open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

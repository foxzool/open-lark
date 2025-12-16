use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeRequest {
    pub file_token: String,
    pub event_type: Option<String>,
}

impl GetSubscribeRequest {
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            event_type: None,
        }
    }

    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = Some(event_type.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeResponse {
    pub is_subscribe: bool,
}

impl ApiResponseTrait for GetSubscribeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn get_subscribe(
    request: GetSubscribeRequest,
    config: &Config,
) -> SDKResult<Response<GetSubscribeResponse>> {
    let mut req = ApiRequest::<GetSubscribeResponse>::get(format!("/open-apis/drive/v1/files/{}/get_subscribe", request.file_token));
    if let Some(et) = request.event_type {
        req = req.query_param("event_type", &et);
    }
    Transport::request(req, config, None).await
}

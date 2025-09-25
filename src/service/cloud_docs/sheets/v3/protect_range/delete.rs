use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::sheets::v3::SpreadsheetService,
};

impl SpreadsheetService {
    /// 删除保护范围
    pub async fn delete_protect_range(
        &self,
        request: DeleteProtectRangeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteProtectRangeResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::DELETE;
        api_req.api_path = SHEETS_V3_SPREADSHEET_PROTECT_RANGE_GET
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.protect_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 删除保护范围请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteProtectRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 保护范围 ID
    protect_id: String,
}

impl DeleteProtectRangeRequest {
    pub fn builder() -> DeleteProtectRangeRequestBuilder {
        DeleteProtectRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteProtectRangeRequestBuilder {
    request: DeleteProtectRangeRequest,
}

impl DeleteProtectRangeRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn protect_id(mut self, protect_id: impl ToString) -> Self {
        self.request.protect_id = protect_id.to_string();
        self
    }

    pub fn build(mut self) -> DeleteProtectRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 删除保护范围响应体最外层
#[derive(Deserialize, Debug)]
pub struct DeleteProtectRangeResponseData {
    /// 删除操作是否成功
    #[serde(default)]
    pub success: bool,
    /// 删除的保护范围 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_id: Option<String>,
}

impl ApiResponseTrait for DeleteProtectRangeResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_delete_protect_range_response() {
        let json = json!({
            "success": true,
            "protect_id": "protect_001"
        });

        let response: DeleteProtectRangeResponseData = serde_json::from_value(json).unwrap();
        assert!(response.success);
        assert_eq!(response.protect_id, Some("protect_001".to_string()));
    }
}

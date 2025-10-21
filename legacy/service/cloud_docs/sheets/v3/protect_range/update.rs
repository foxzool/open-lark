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

use super::create::ProtectRangeData;

impl SpreadsheetService {
    /// 修改保护范围
    pub async fn update_protect_range(
        &self,
        request: UpdateProtectRangeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateProtectRangeResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PUT;
        api_req.api_path = SHEETS_V3_SPREADSHEET_PROTECT_RANGE_GET
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.protect_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 修改保护范围请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UpdateProtectRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 保护范围 ID
    protect_id: String,
    /// 新的保护范围设置
    protect_range: ProtectRangeData,
}

impl UpdateProtectRangeRequest {
    pub fn builder() -> UpdateProtectRangeRequestBuilder {
        UpdateProtectRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateProtectRangeRequestBuilder {
    request: UpdateProtectRangeRequest,
}

impl UpdateProtectRangeRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn protect_id(mut self, protect_id: impl ToString) -> Self {
        self.request.protect_id = protect_id.to_string();
        self
    }

    pub fn protect_range(mut self, protect_range: ProtectRangeData) -> Self {
        self.request.protect_range = protect_range;
        self
    }

    pub fn build(mut self) -> UpdateProtectRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 修改保护范围响应体最外层
#[derive(Deserialize, Debug)]
pub struct UpdateProtectRangeResponseData {
    /// 保护范围 ID
    pub protect_id: String,
    /// 更新后的保护范围信息
    #[serde(flatten)]
    pub protect_range: ProtectRangeData,
}

impl ApiResponseTrait for UpdateProtectRangeResponseData {
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
    fn test_update_protect_range_response() {
        let json = json!({
            "protect_id": "protect_001",
            "dimension": "ROWS",
            "sheet_id": "Sheet1",
            "start_index": 5,
            "end_index": 15,
            "lock_info": "user@example.com"
        });

        let response: UpdateProtectRangeResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.protect_id, "protect_001");
        assert_eq!(response.protect_range.start_index, 5);
        assert_eq!(response.protect_range.end_index, 15);
    }
}

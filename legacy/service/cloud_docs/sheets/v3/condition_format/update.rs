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
    service::sheets::v3::SpreadsheetSheetService,
};

use super::create::{ConditionFormatInfo, ConditionFormatRule};

impl SpreadsheetSheetService {
    /// 批量更新条件格式
    pub async fn update_condition_formats(
        &self,
        request: UpdateConditionFormatsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateConditionFormatsResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PUT;
        api_req.api_path = SHEETS_V3_SPREADSHEET_CONDITION_FORMAT
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 批量更新条件格式请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UpdateConditionFormatsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 要更新的条件格式规则列表
    condition_formats: Vec<UpdateConditionFormatRule>,
}

impl UpdateConditionFormatsRequest {
    pub fn builder() -> UpdateConditionFormatsRequestBuilder {
        UpdateConditionFormatsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateConditionFormatsRequestBuilder {
    request: UpdateConditionFormatsRequest,
}

impl UpdateConditionFormatsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn condition_formats(mut self, condition_formats: Vec<UpdateConditionFormatRule>) -> Self {
        self.request.condition_formats = condition_formats;
        self
    }

    pub fn add_condition_format(mut self, condition_format: UpdateConditionFormatRule) -> Self {
        self.request.condition_formats.push(condition_format);
        self
    }

    pub fn build(mut self) -> UpdateConditionFormatsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 更新条件格式规则
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConditionFormatRule {
    /// 条件格式 ID
    pub cf_id: String,
    /// 条件格式规则
    #[serde(flatten)]
    pub rule: ConditionFormatRule,
}

impl UpdateConditionFormatRule {
    pub fn new(cf_id: impl ToString, rule: ConditionFormatRule) -> Self {
        Self {
            cf_id: cf_id.to_string(),
            rule,
        }
    }
}

/// 批量更新条件格式响应体最外层
#[derive(Deserialize, Debug)]
pub struct UpdateConditionFormatsResponseData {
    /// 更新后的条件格式列表
    pub items: Vec<ConditionFormatInfo>,
    /// 更新成功的数量
    #[serde(default)]
    pub updated_count: u32,
}

impl ApiResponseTrait for UpdateConditionFormatsResponseData {
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
    fn test_update_condition_formats_response() {
        let json = json!({
            "items": [
                {
                    "cf_id": "cf_001",
                    "range": "A1:A15",
                    "condition_type": "NUMBER_GREATER",
                    "condition_values": ["200"],
                    "format": {
                        "background_color": "#00FF00",
                        "text_color": "#000000",
                        "bold": false
                    }
                }
            ],
            "updated_count": 1
        });

        let response: UpdateConditionFormatsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].cf_id, "cf_001");
        assert_eq!(response.updated_count, 1);
    }
}

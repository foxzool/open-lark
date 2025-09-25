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

use super::create::ConditionFormatInfo;

impl SpreadsheetSheetService {
    /// 批量获取条件格式
    pub async fn get_condition_formats(
        &self,
        request: GetConditionFormatsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetConditionFormatsResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = SHEETS_V3_SPREADSHEET_CONDITION_FORMAT
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        // 添加查询参数
        if let Some(range) = &request.range {
            api_req.query_params.insert("range", range.clone());
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 批量获取条件格式请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GetConditionFormatsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 可选：查询范围，如果不提供则返回整个工作表的条件格式
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<String>,
}

impl GetConditionFormatsRequest {
    pub fn builder() -> GetConditionFormatsRequestBuilder {
        GetConditionFormatsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetConditionFormatsRequestBuilder {
    request: GetConditionFormatsRequest,
}

impl GetConditionFormatsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = Some(range.to_string());
        self
    }

    pub fn build(mut self) -> GetConditionFormatsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 批量获取条件格式响应体最外层
#[derive(Deserialize, Debug)]
pub struct GetConditionFormatsResponseData {
    /// 条件格式列表
    pub items: Vec<ConditionFormatInfo>,
    /// 是否还有更多数据
    #[serde(default)]
    pub has_more: bool,
    /// 下次请求的页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetConditionFormatsResponseData {
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
    fn test_get_condition_formats_response() {
        let json = json!({
            "items": [
                {
                    "cf_id": "cf_001",
                    "range": "A1:A10",
                    "condition_type": "NUMBER_GREATER",
                    "condition_values": ["100"],
                    "format": {
                        "background_color": "#FF0000",
                        "text_color": "#FFFFFF",
                        "bold": true
                    }
                },
                {
                    "cf_id": "cf_002",
                    "range": "B1:B10",
                    "condition_type": "TEXT_CONTAINS",
                    "condition_values": ["重要"],
                    "format": {
                        "background_color": "#FFFF00",
                        "text_color": "#000000"
                    }
                }
            ],
            "has_more": false
        });

        let response: GetConditionFormatsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].cf_id, "cf_001");
        assert_eq!(
            response.items[1].condition_format.condition_type,
            "TEXT_CONTAINS"
        );
        assert!(!response.has_more);
    }
}

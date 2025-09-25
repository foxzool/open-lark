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

impl SpreadsheetSheetService {
    /// 批量删除条件格式
    pub async fn delete_condition_formats(
        &self,
        request: DeleteConditionFormatsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteConditionFormatsResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::DELETE;
        api_req.api_path = SHEETS_V3_SPREADSHEET_CONDITION_FORMAT
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 批量删除条件格式请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteConditionFormatsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 要删除的条件格式 ID 列表
    cf_ids: Vec<String>,
}

impl DeleteConditionFormatsRequest {
    pub fn builder() -> DeleteConditionFormatsRequestBuilder {
        DeleteConditionFormatsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteConditionFormatsRequestBuilder {
    request: DeleteConditionFormatsRequest,
}

impl DeleteConditionFormatsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn cf_ids(mut self, cf_ids: Vec<String>) -> Self {
        self.request.cf_ids = cf_ids;
        self
    }

    pub fn add_cf_id(mut self, cf_id: impl ToString) -> Self {
        self.request.cf_ids.push(cf_id.to_string());
        self
    }

    pub fn build(mut self) -> DeleteConditionFormatsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 删除结果
#[derive(Deserialize, Debug)]
pub struct DeleteResult {
    /// 条件格式 ID
    pub cf_id: String,
    /// 删除是否成功
    #[serde(default)]
    pub success: bool,
    /// 错误信息（如果删除失败）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// 批量删除条件格式响应体最外层
#[derive(Deserialize, Debug)]
pub struct DeleteConditionFormatsResponseData {
    /// 删除结果列表
    pub items: Vec<DeleteResult>,
    /// 删除成功的数量
    #[serde(default)]
    pub deleted_count: u32,
    /// 删除失败的数量
    #[serde(default)]
    pub failed_count: u32,
}

impl ApiResponseTrait for DeleteConditionFormatsResponseData {
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
    fn test_delete_condition_formats_response() {
        let json = json!({
            "items": [
                {
                    "cf_id": "cf_001",
                    "success": true
                },
                {
                    "cf_id": "cf_002",
                    "success": false,
                    "error_message": "条件格式不存在"
                }
            ],
            "deleted_count": 1,
            "failed_count": 1
        });

        let response: DeleteConditionFormatsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.items.len(), 2);
        assert!(response.items[0].success);
        assert!(!response.items[1].success);
        assert_eq!(response.deleted_count, 1);
        assert_eq!(response.failed_count, 1);
    }
}

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
    impl_executable_builder_owned,
    service::sheets::v3::SpreadsheetService,
};

impl SpreadsheetService {
    /// 增加保护范围
    pub async fn add_protect_range(
        &self,
        request: AddProtectRangeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AddProtectRangeResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path =
            SHEETS_V3_SPREADSHEET_PROTECT_RANGE.replace("{}", &request.spreadsheet_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 增加保护范围请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AddProtectRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 保护范围设置
    protect_range: ProtectRangeData,
}

impl AddProtectRangeRequest {
    pub fn builder() -> AddProtectRangeRequestBuilder {
        AddProtectRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct AddProtectRangeRequestBuilder {
    request: AddProtectRangeRequest,
}

impl AddProtectRangeRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn protect_range(mut self, protect_range: ProtectRangeData) -> Self {
        self.request.protect_range = protect_range;
        self
    }

    pub fn build(mut self) -> AddProtectRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    AddProtectRangeRequestBuilder,
    SpreadsheetService,
    AddProtectRangeRequest,
    BaseResponse<AddProtectRangeResponseData>,
    add_protect_range
);

/// 保护范围数据
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ProtectRangeData {
    /// 保护范围的维度
    pub dimension: String,
    /// 保护工作表 ID
    pub sheet_id: String,
    /// 保护起始位置
    pub start_index: i32,
    /// 保护结束位置
    pub end_index: i32,
    /// 保护范围 ID（仅在获取时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_id: Option<String>,
    /// 锁定用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_info: Option<String>,
}

impl ProtectRangeData {
    pub fn new(
        dimension: impl ToString,
        sheet_id: impl ToString,
        start_index: i32,
        end_index: i32,
    ) -> Self {
        Self {
            dimension: dimension.to_string(),
            sheet_id: sheet_id.to_string(),
            start_index,
            end_index,
            protect_id: None,
            lock_info: None,
        }
    }

    /// 创建行保护范围
    pub fn row_range(sheet_id: impl ToString, start_row: i32, end_row: i32) -> Self {
        Self::new("ROWS", sheet_id, start_row, end_row)
    }

    /// 创建列保护范围
    pub fn column_range(sheet_id: impl ToString, start_col: i32, end_col: i32) -> Self {
        Self::new("COLUMNS", sheet_id, start_col, end_col)
    }
}

/// 增加保护范围响应体最外层
#[derive(Deserialize, Debug)]
pub struct AddProtectRangeResponseData {
    /// 保护范围 ID
    pub protect_id: String,
    /// 保护范围信息
    #[serde(flatten)]
    pub protect_range: ProtectRangeData,
}

impl ApiResponseTrait for AddProtectRangeResponseData {
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
    fn test_protect_range_data_creation() {
        let protect_range = ProtectRangeData::row_range("Sheet1", 1, 10);
        assert_eq!(protect_range.dimension, "ROWS");
        assert_eq!(protect_range.sheet_id, "Sheet1");
        assert_eq!(protect_range.start_index, 1);
        assert_eq!(protect_range.end_index, 10);
    }

    #[test]
    fn test_add_protect_range_response() {
        let json = json!({
            "protect_id": "protect_001",
            "dimension": "COLUMNS",
            "sheet_id": "Sheet1",
            "start_index": 1,
            "end_index": 5,
            "lock_info": "user@example.com"
        });

        let response: AddProtectRangeResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.protect_id, "protect_001");
        assert_eq!(response.protect_range.dimension, "COLUMNS");
    }
}

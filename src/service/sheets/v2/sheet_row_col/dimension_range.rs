use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        req_option, SDKResult,
    },
    service::sheets::v2::SpreadsheetService,
};

#[derive(Serialize, Debug, Default)]
pub struct DimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要增加行列的维度信息
    dimension: DimensionRequest,
}

#[derive(Serialize, Debug, Default)]
struct DimensionRequest {
    /// 电子表格工作表的 ID。调用获取工作表获取 ID
    #[serde(rename = "sheetId")]
    sheet_id: String,
    /// 更新的维度。可选值：
    /// - ROWS：行
    /// - COLUMNS：列
    #[serde(rename = "majorDimension")]
    major_dimension: String,
    /// 要增加的行数或列数。取值范围为 (0,5000]
    length: i32,
}

impl DimensionRangeRequest {
    pub fn builder() -> DimensionRangeRequestBuilder {
        DimensionRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DimensionRangeRequestBuilder {
    request: DimensionRangeRequest,
}

impl DimensionRangeRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 电子表格工作表的 ID。调用获取工作表获取 ID
    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.dimension.sheet_id = sheet_id.to_string();
        self
    }

    /// 更新的维度。可选值：
    /// - ROWS：行
    /// - COLUMNS：列
    pub fn major_dimension(mut self, major_dimension: impl ToString) -> Self {
        self.request.dimension.major_dimension = major_dimension.to_string();
        self
    }

    /// 要增加的行数或列数。取值范围为 (0,5000]
    pub fn length(mut self, length: i32) -> Self {
        self.request.dimension.length = length;
        self
    }

    pub fn build(mut self) -> DimensionRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl SpreadsheetService {
    /// 该接口用于在电子表格中增加空白行或列。
    pub async fn dimension_range(
        &self,
        request: DimensionRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<DimensionRangeResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dimension_range",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 增加行列响应体
#[derive(Deserialize, Debug)]
pub struct DimensionRangeResponse {
    #[serde(rename = "addCount")]
    pub add_count: i32,
    #[serde(rename = "majorDimension")]
    pub major_dimension: String,
}

impl ApiResponseTrait for DimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

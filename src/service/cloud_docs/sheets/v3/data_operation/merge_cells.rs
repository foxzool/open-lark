use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};

impl DataOperationService {
    /// 合并单元格
    pub async fn merge_cells(
        &self,
        request: MergeCellsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MergeCellsResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/merge_cells",
            request.spreadsheet_token, request.sheet_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 合并单元格请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MergeCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 合并范围
    range: String,
    /// 合并类型
    merge_type: String,
}

impl MergeCellsRequest {
    pub fn builder() -> MergeCellsRequestBuilder {
        MergeCellsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct MergeCellsRequestBuilder {
    request: MergeCellsRequest,
}

impl MergeCellsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
        self
    }

    /// 设置合并类型
    /// - MERGE_ALL: 合并所有单元格
    /// - MERGE_COLUMNS: 按列合并
    /// - MERGE_ROWS: 按行合并
    pub fn merge_type(mut self, merge_type: impl ToString) -> Self {
        self.request.merge_type = merge_type.to_string();
        self
    }

    pub fn build(mut self) -> MergeCellsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// Trait implementation
impl_executable_builder_owned!(
    MergeCellsRequestBuilder,
    DataOperationService,
    MergeCellsRequest,
    BaseResponse<MergeCellsResponseData>,
    merge_cells
);

/// 合并单元格响应体最外层
#[derive(Deserialize, Debug)]
pub struct MergeCellsResponseData {
    /// 合并后的范围
    pub merged_range: String,
}

impl ApiResponseTrait for MergeCellsResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::MergeCellsResponseData;

    #[test]
    fn test_merge_cells_response() {
        let json = json!({
            "merged_range": "A1:C3"
        });

        let response: MergeCellsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.merged_range, "A1:C3");
    }
}

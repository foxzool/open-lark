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
        standard_response::StandardResponse,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};

impl DataOperationService {
    /// 拆分单元格
    pub async fn split_cells(
        &self,
        request: SplitCellsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<SplitCellsResponseData> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = SHEETS_V3_SPREADSHEET_UNMERGE_CELLS
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<SplitCellsResponseData> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

/// 拆分单元格请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SplitCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 拆分范围
    range: String,
}

impl SplitCellsRequest {
    pub fn builder() -> SplitCellsRequestBuilder {
        SplitCellsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct SplitCellsRequestBuilder {
    request: SplitCellsRequest,
}

impl SplitCellsRequestBuilder {
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

    pub fn build(mut self) -> SplitCellsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 拆分单元格响应体最外层
#[derive(Deserialize, Debug)]
pub struct SplitCellsResponseData {
    /// 拆分后的范围
    pub unmerged_range: String,
}

impl ApiResponseTrait for SplitCellsResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl_executable_builder_owned!(
    SplitCellsRequestBuilder,
    DataOperationService,
    SplitCellsRequest,
    SplitCellsResponseData,
    split_cells
);

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use super::SplitCellsResponseData;

    #[test]
    fn test_split_cells_response() {
        let json = json!({
            "unmerged_range": "A1:C3"
        });

        let response: SplitCellsResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.unmerged_range, "A1:C3");
    }
}

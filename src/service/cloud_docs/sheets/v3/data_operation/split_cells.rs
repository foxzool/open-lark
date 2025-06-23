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
    service::sheets::v3::DataOperationService,
};

impl DataOperationService {
    /// 拆分单元格
    pub async fn split_cells(
        &self,
        request: SplitCellsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SplitCellsResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/unmerge_cells",
            request.spreadsheet_token, request.sheet_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
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

    /// 直接执行拆分单元格请求
    ///
    /// 这是一个便捷方法，相当于 `builder.build()` 然后调用 `service.split_cells()`
    pub async fn execute(
        self,
        service: &crate::service::sheets::v3::DataOperationService,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<SplitCellsResponseData>> {
        service.split_cells(self.build(), None).await
    }

    /// 直接执行拆分单元格请求（带选项）
    ///
    /// 这是一个便捷方法，相当于 `builder.build()` 然后调用 `service.split_cells()`
    pub async fn execute_with_options(
        self,
        service: &crate::service::sheets::v3::DataOperationService,
        option: crate::core::req_option::RequestOption,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<SplitCellsResponseData>> {
        service.split_cells(self.build(), Some(option)).await
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

#[cfg(test)]
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

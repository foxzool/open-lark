use reqwest::Method;
use serde::Serialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{BaseResponse, RawResponse},
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
    /// 修改电子表格属性
    pub async fn patch(
        &self,
        request: PatchSpreadSheetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RawResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PATCH;
        api_req.api_path = SHEETS_V3_SPREADSHEET_GET.replace("{}", &request.spreadsheet_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 修改电子表格属性 请求体
#[derive(Default, Debug, Serialize)]
pub struct PatchSpreadSheetRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 表格的token
    #[serde(skip)]
    spreadsheet_token: String,
    /// 表格标题
    title: String,
}

impl PatchSpreadSheetRequest {
    pub fn builder() -> PatchSpreadSheetRequestBuilder {
        PatchSpreadSheetRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct PatchSpreadSheetRequestBuilder {
    request: PatchSpreadSheetRequest,
}

impl PatchSpreadSheetRequestBuilder {
    /// 表格标题
    pub fn title(mut self, title: impl ToString) -> Self {
        self.request.title = title.to_string();
        self
    }

    /// 表格的token
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn build(mut self) -> PatchSpreadSheetRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// Trait implementation
impl_executable_builder_owned!(
    PatchSpreadSheetRequestBuilder,
    SpreadsheetService,
    PatchSpreadSheetRequest,
    BaseResponse<RawResponse>,
    patch
);

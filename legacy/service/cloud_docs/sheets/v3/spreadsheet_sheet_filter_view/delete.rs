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
    service::sheets::v3::SpreadsheetSheetFilterViewService,
};

impl SpreadsheetSheetFilterViewService {
    /// 删除筛选视图
    pub async fn delete(
        &self,
        request: DeleteFilterViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteFilterViewResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::DELETE;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FILTER_VIEW_GET
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id)
            .replace("{}", &request.filter_view_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 删除筛选视图请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DeleteFilterViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图 ID
    filter_view_id: String,
}

impl DeleteFilterViewRequest {
    pub fn builder() -> DeleteFilterViewRequestBuilder {
        DeleteFilterViewRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteFilterViewRequestBuilder {
    request: DeleteFilterViewRequest,
}

impl DeleteFilterViewRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn filter_view_id(mut self, filter_view_id: impl ToString) -> Self {
        self.request.filter_view_id = filter_view_id.to_string();
        self
    }

    pub fn build(mut self) -> DeleteFilterViewRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 删除筛选视图响应体最外层
#[derive(Deserialize, Debug)]
pub struct DeleteFilterViewResponseData {
    /// 操作结果
    pub success: bool,
}

impl ApiResponseTrait for DeleteFilterViewResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

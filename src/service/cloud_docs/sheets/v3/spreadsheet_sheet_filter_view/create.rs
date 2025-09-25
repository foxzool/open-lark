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
    service::sheets::v3::SpreadsheetSheetFilterViewService,
};

impl SpreadsheetSheetFilterViewService {
    /// 创建筛选视图
    pub async fn create(
        &self,
        request: CreateFilterViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateFilterViewResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = SHEETS_V3_SPREADSHEET_FILTER_VIEWS
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 创建筛选视图请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CreateFilterViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图的名称
    filter_view_name: String,
    /// 筛选视图的范围
    range: String,
}

impl CreateFilterViewRequest {
    pub fn builder() -> CreateFilterViewRequestBuilder {
        CreateFilterViewRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateFilterViewRequestBuilder {
    request: CreateFilterViewRequest,
}

impl CreateFilterViewRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn filter_view_name(mut self, filter_view_name: impl ToString) -> Self {
        self.request.filter_view_name = filter_view_name.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
        self
    }

    pub fn build(mut self) -> CreateFilterViewRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 创建筛选视图响应体最外层
#[derive(Deserialize, Debug)]
pub struct CreateFilterViewResponseData {
    /// 筛选视图 ID
    pub filter_view_id: String,
    /// 筛选视图名称
    pub filter_view_name: String,
    /// 筛选范围
    pub range: String,
}

impl ApiResponseTrait for CreateFilterViewResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {
    use serde_json::json;

    use super::CreateFilterViewResponseData;

    #[test]
    fn test_create_filter_view_response() {
        let json = json!({
            "filter_view_id": "fltr_vw_001",
            "filter_view_name": "销售数据筛选",
            "range": "A1:E100"
        });

        let response: CreateFilterViewResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.filter_view_id, "fltr_vw_001");
        assert_eq!(response.filter_view_name, "销售数据筛选");
        assert_eq!(response.range, "A1:E100");
    }
}

// 实现ExecutableBuilder trait
impl_executable_builder_owned!(
    CreateFilterViewRequestBuilder,
    SpreadsheetSheetFilterViewService,
    CreateFilterViewRequest,
    BaseResponse<CreateFilterViewResponseData>,
    create
);

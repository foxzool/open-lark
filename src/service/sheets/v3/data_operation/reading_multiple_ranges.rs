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

use super::ValueRange;

impl DataOperationService {
    /// 读取多个范围
    pub async fn reading_multiple_ranges(
        &self,
        request: ReadingMultipleRangesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ReadingMultipleRangesResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/values/batch_get",
            request.spreadsheet_token
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 读取多个范围请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ReadingMultipleRangesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 查询范围列表
    ranges: Vec<String>,
    /// 指定单元格数据的格式
    #[serde(rename = "valueRenderOption")]
    value_render_option: Option<String>,
    /// 指定数据类型为日期、时间、或时间日期的单元格数据的格式
    #[serde(rename = "dateTimeRenderOption")]
    date_time_render_option: Option<String>,
    /// 指定返回的用户 ID 类型
    user_id_type: Option<String>,
}

impl ReadingMultipleRangesRequest {
    pub fn builder() -> ReadingMultipleRangesRequestBuilder {
        ReadingMultipleRangesRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ReadingMultipleRangesRequestBuilder {
    request: ReadingMultipleRangesRequest,
}

impl ReadingMultipleRangesRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn ranges(mut self, ranges: Vec<String>) -> Self {
        self.request.ranges = ranges;
        self
    }

    pub fn value_render_option(mut self, value_render_option: impl ToString) -> Self {
        self.request.value_render_option = Some(value_render_option.to_string());
        self
    }

    pub fn date_time_render_option(mut self, date_time_render_option: impl ToString) -> Self {
        self.request.date_time_render_option = Some(date_time_render_option.to_string());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    pub fn build(mut self) -> ReadingMultipleRangesRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 读取多个范围响应体最外层
#[derive(Deserialize, Debug)]
pub struct ReadingMultipleRangesResponseData {
    /// 值与范围列表
    #[serde(rename = "valueRanges")]
    pub value_ranges: Vec<ValueRange>,
}

impl ApiResponseTrait for ReadingMultipleRangesResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

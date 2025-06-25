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
    /// 读取单个范围
    pub async fn reading_single_range(
        &self,
        request: ReadingSingleRangeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ReadingSingleRangeResponseData>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/values/{}",
            request.spreadsheet_token, request.range
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 读取单个范围请求
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ReadingSingleRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 查询范围，包含 sheetId 与单元格范围两部分
    range: String,
    /// 指定单元格数据的格式
    value_render_option: Option<String>,
    /// 指定数据类型为日期、时间、或时间日期的单元格数据的格式
    date_time_render_option: Option<String>,
    /// 指定返回的用户 ID 类型
    user_id_type: Option<String>,
}

impl ReadingSingleRangeRequest {
    pub fn builder() -> ReadingSingleRangeRequestBuilder {
        ReadingSingleRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ReadingSingleRangeRequestBuilder {
    request: ReadingSingleRangeRequest,
}

impl ReadingSingleRangeRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
        self
    }

    pub fn value_render_option(mut self, value_render_option: impl ToString) -> Self {
        let value = value_render_option.to_string();
        self.request.value_render_option = Some(value.clone());
        self.request
            .api_request
            .query_params
            .insert("valueRenderOption".to_string(), value);
        self
    }

    pub fn date_time_render_option(mut self, date_time_render_option: impl ToString) -> Self {
        let value = date_time_render_option.to_string();
        self.request.date_time_render_option = Some(value.clone());
        self.request
            .api_request
            .query_params
            .insert("dateTimeRenderOption".to_string(), value);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        let value = user_id_type.to_string();
        self.request.user_id_type = Some(value.clone());
        self.request
            .api_request
            .query_params
            .insert("user_id_type".to_string(), value);
        self
    }

    pub fn build(self) -> ReadingSingleRangeRequest {
        self.request
    }
}

// Trait implementation
impl_executable_builder_owned!(
    ReadingSingleRangeRequestBuilder,
    DataOperationService,
    ReadingSingleRangeRequest,
    BaseResponse<ReadingSingleRangeResponseData>,
    reading_single_range
);

/// 读取单个范围响应体最外层
#[derive(Deserialize, Debug)]
pub struct ReadingSingleRangeResponseData {
    /// 值与范围
    #[serde(rename = "valueRange")]
    pub value_range: ValueRange,
}

impl ApiResponseTrait for ReadingSingleRangeResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 值与范围
#[derive(Deserialize, Debug)]
pub struct ValueRange {
    /// 查询范围，包含 sheetId 与单元格范围两部分
    pub range: String,
    /// sheet 的版本号
    pub revision: i32,
    /// 范围内的值
    pub values: Vec<Vec<serde_json::Value>>,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::ReadingSingleRangeResponseData;

    #[test]
    fn test_reading_single_range_response() {
        let json = json!({
            "valueRange": {
                "range": "Sheet1!A1:B2",
                "revision": 123,
                "values": [
                    ["姓名", "年龄"],
                    ["张三", 25]
                ]
            }
        });

        let response: ReadingSingleRangeResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.value_range.range, "Sheet1!A1:B2");
        assert_eq!(response.value_range.revision, 123);
        assert_eq!(response.value_range.values.len(), 2);
    }
}

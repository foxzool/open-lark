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
        validation::{self, ValidationResult},
        SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v3::DataOperationService,
};

use super::ValueRange;

impl DataOperationService {
    /// 读取多个范围
    pub async fn reading_multiple_ranges(
        &self,
        request: ReadingMultipleRangesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ReadingMultipleRangesResponseData> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path =
            SHEETS_V3_SPREADSHEET_VALUES_BATCH_GET.replace("{}", &request.spreadsheet_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<ReadingMultipleRangesResponseData> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result()
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

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证必需字段
        if self.spreadsheet_token.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "spreadsheet_token cannot be empty".to_string(),
            ));
        }

        if self.ranges.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "ranges cannot be empty".to_string(),
            ));
        }

        // 验证范围数量限制
        if self.ranges.len() > 500 {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "Too many ranges. Maximum 500 ranges allowed".to_string(),
            ));
        }

        // 验证每个单元格范围格式
        for (i, range) in self.ranges.iter().enumerate() {
            if let ValidationResult::Invalid(msg) = validation::validate_cell_range(range) {
                return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                    "Invalid cell range at index {}: '{}': {}",
                    i, range, msg
                )));
            }
        }

        // 验证值渲染选项
        if let ValidationResult::Invalid(msg) =
            validation::validate_value_render_option(&self.value_render_option)
        {
            return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                "Invalid valueRenderOption: {}",
                msg
            )));
        }

        // 验证日期时间渲染选项
        if let ValidationResult::Invalid(msg) =
            validation::validate_date_time_render_option(&self.date_time_render_option)
        {
            return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                "Invalid dateTimeRenderOption: {}",
                msg
            )));
        }

        Ok(())
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
        let value = value_render_option.to_string();
        self.request.value_render_option = Some(value.clone());
        self.request
            .api_request
            .query_params
            .insert("valueRenderOption", value);
        self
    }

    pub fn date_time_render_option(mut self, date_time_render_option: impl ToString) -> Self {
        let value = date_time_render_option.to_string();
        self.request.date_time_render_option = Some(value.clone());
        self.request
            .api_request
            .query_params
            .insert("dateTimeRenderOption", value);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        let value = user_id_type.to_string();
        self.request.user_id_type = Some(value.clone());
        self.request
            .api_request
            .query_params
            .insert("user_id_type", value);
        self
    }

    pub fn build(self) -> ReadingMultipleRangesRequest {
        let mut request = self.request;
        request.api_request.body = serde_json::to_vec(&request).unwrap();
        request
    }
}

// Trait implementation
impl_executable_builder_owned!(
    ReadingMultipleRangesRequestBuilder,
    DataOperationService,
    ReadingMultipleRangesRequest,
    ReadingMultipleRangesResponseData,
    reading_multiple_ranges
);

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

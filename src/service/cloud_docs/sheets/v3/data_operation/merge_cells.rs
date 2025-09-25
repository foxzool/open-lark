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

impl DataOperationService {
    /// 合并单元格
    pub async fn merge_cells(
        &self,
        request: MergeCellsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<MergeCellsResponseData> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = SHEETS_V3_SPREADSHEET_MERGE_CELLS
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<MergeCellsResponseData> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result()
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

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证必需字段
        if self.spreadsheet_token.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "spreadsheet_token cannot be empty".to_string(),
            ));
        }

        if self.sheet_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "sheet_id cannot be empty".to_string(),
            ));
        }

        if self.range.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "range cannot be empty".to_string(),
            ));
        }

        // 验证合并范围格式
        if let ValidationResult::Invalid(msg) = validation::validate_merge_range(&self.range) {
            return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                "Invalid merge range '{}': {}",
                self.range, msg
            )));
        }

        // 验证合并类型
        let valid_merge_types = ["MERGE_ALL", "MERGE_COLUMNS", "MERGE_ROWS"];
        if !valid_merge_types.contains(&self.merge_type.as_str()) {
            return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                "Invalid merge_type '{}'. Must be one of: MERGE_ALL, MERGE_COLUMNS, MERGE_ROWS",
                self.merge_type
            )));
        }

        Ok(())
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

    pub fn build(self) -> MergeCellsRequest {
        let mut request = self.request;
        request.api_request.body = serde_json::to_vec(&request).unwrap();
        request
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证必需字段
        if self.request.spreadsheet_token.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "spreadsheet_token cannot be empty".to_string(),
            ));
        }

        if self.request.sheet_id.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "sheet_id cannot be empty".to_string(),
            ));
        }

        if self.request.range.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "range cannot be empty".to_string(),
            ));
        }

        // 验证合并范围格式
        if let ValidationResult::Invalid(msg) =
            validation::validate_merge_range(&self.request.range)
        {
            return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                "Invalid merge range '{}': {}",
                self.request.range, msg
            )));
        }

        // 验证合并类型
        let valid_merge_types = ["MERGE_ALL", "MERGE_COLUMNS", "MERGE_ROWS"];
        if !valid_merge_types.contains(&self.request.merge_type.as_str()) {
            return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                "Invalid merge_type '{}'. Must be one of: MERGE_ALL, MERGE_COLUMNS, MERGE_ROWS",
                self.request.merge_type
            )));
        }

        Ok(())
    }
}

// Trait implementation
impl_executable_builder_owned!(
    MergeCellsRequestBuilder,
    DataOperationService,
    MergeCellsRequest,
    MergeCellsResponseData,
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
#[allow(unused_variables, unused_unsafe)]
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

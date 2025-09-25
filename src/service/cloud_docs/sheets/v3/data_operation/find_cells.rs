use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option,
        standard_response::StandardResponse,
        validation::{self, ValidationResult},
        SDKResult,
    },
    impl_executable_builder_owned,
    service::sheets::v3::{
        data_operation::{FindCondition, FindReplaceResult},
        SpreadsheetSheetService,
    },
};

#[derive(Serialize, Debug, Default)]
pub struct FindCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 工作表的id
    #[serde(skip)]
    sheet_id: String,
    /// 查找条件
    find_condition: FindCondition,
    /// 查找的字符串，当search_by_regex字段为 true 时，该字段为正则表达式
    ///
    /// 示例值："如下
    ///
    /// - 普通查找示例: "hello"
    /// - 正则查找示例: "[A-Z]\w+""
    find: String,
}

impl FindCellsRequest {
    pub fn builder() -> FindCellsRequestBuilder {
        FindCellsRequestBuilder::default()
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

        if self.find.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "find cannot be empty".to_string(),
            ));
        }

        // 验证查找条件
        if let ValidationResult::Invalid(msg) = validation::validate_find_options(
            &self.find_condition.match_case,
            &self.find_condition.match_entire_cell,
            &self.find_condition.search_by_regex,
            &self.find_condition.include_formulas,
        ) {
            return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                "Invalid find options: {}",
                msg
            )));
        }

        // 验证范围格式（如果指定了范围）
        if !self.find_condition.range.is_empty() {
            if let ValidationResult::Invalid(msg) =
                validation::validate_cell_range(&self.find_condition.range)
            {
                return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                    "Invalid search range '{}': {}",
                    self.find_condition.range, msg
                )));
            }
        }

        // 验证查找字符串长度
        if self.find.len() > 1000 {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "find string too long. Maximum 1000 characters allowed".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct FindCellsRequestBuilder {
    request: FindCellsRequest,
}

impl FindCellsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.sheet_id = sheet_id.to_string();
        self
    }

    pub fn find(mut self, find: impl ToString) -> Self {
        self.request.find = find.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.find_condition.range = range.to_string();
        self
    }

    pub fn match_case(mut self, match_case: bool) -> Self {
        self.request.find_condition.match_case = Some(match_case);
        self
    }

    pub fn match_entire_cell(mut self, match_entire_cell: bool) -> Self {
        self.request.find_condition.match_entire_cell = Some(match_entire_cell);
        self
    }

    pub fn search_by_regex(mut self, search_by_regex: bool) -> Self {
        self.request.find_condition.search_by_regex = Some(search_by_regex);
        self
    }

    pub fn include_formulas(mut self, include_formulas: bool) -> Self {
        self.request.find_condition.include_formulas = Some(include_formulas);
        self
    }

    pub fn build(self) -> FindCellsRequest {
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

        if self.request.find.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "find cannot be empty".to_string(),
            ));
        }

        // 验证查找条件
        if let ValidationResult::Invalid(msg) = validation::validate_find_options(
            &self.request.find_condition.match_case,
            &self.request.find_condition.match_entire_cell,
            &self.request.find_condition.search_by_regex,
            &self.request.find_condition.include_formulas,
        ) {
            return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                "Invalid find options: {}",
                msg
            )));
        }

        // 验证范围格式（如果指定了范围）
        if !self.request.find_condition.range.is_empty() {
            if let ValidationResult::Invalid(msg) =
                validation::validate_cell_range(&self.request.find_condition.range)
            {
                return Err(crate::core::error::LarkAPIError::illegal_param(format!(
                    "Invalid search range '{}': {}",
                    self.request.find_condition.range, msg
                )));
            }
        }

        // 验证查找字符串长度
        if self.request.find.len() > 1000 {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "find string too long. Maximum 1000 characters allowed".to_string(),
            ));
        }

        Ok(())
    }
}

// Trait implementation
impl_executable_builder_owned!(
    FindCellsRequestBuilder,
    SpreadsheetSheetService,
    FindCellsRequest,
    FindCellsResponse,
    find_cells
);

/// 查找单元格响应
#[derive(Deserialize, Debug)]
pub struct FindCellsResponse {
    /// 符合条件的信息
    pub find_result: FindReplaceResult,
}

impl ApiResponseTrait for FindCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetSheetService {
    /// 查找单元格
    pub async fn find_cells(
        &self,
        request: FindCellsRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<FindCellsResponse> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V3_SPREADSHEET_SHEET_FIND
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.sheet_id);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp: BaseResponse<FindCellsResponse> =
            crate::core::http::Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result()
    }
}

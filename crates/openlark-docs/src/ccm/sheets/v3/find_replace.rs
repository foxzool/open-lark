//! Sheets电子表格查找和替换服务 v3
//!
//! 提供飞书电子表格v3版本的查找和替换管理功能，包括：
//! - 文本查找和替换
//! - 正则表达式支持
//! - 范围限制和匹配选项
//! - 批量查找替换操作

use serde_json::Value;
use openlark_core::error::LarkAPIError;

// 使用统一类型定义
use super::Range;
use serde::{Deserialize, Serialize};

/// 查找匹配类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchType {
    /// 精确匹配
    #[serde(rename = "EXACT")]
    Exact,
    /// 包含匹配
    #[serde(rename = "CONTAINS")]
    Contains,
    /// 开始匹配
    #[serde(rename = "STARTS_WITH")]
    StartsWith,
    /// 结束匹配
    #[serde(rename = "ENDS_WITH")]
    EndsWith,
    /// 正则表达式
    #[serde(rename = "REGEX")]
    Regex,
}

/// 查找范围类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchRangeType {
    /// 全部工作表
    #[serde(rename = "ALL")]
    All,
    /// 指定范围
    #[serde(rename = "RANGE")]
    Range,
    /// 公式
    #[serde(rename = "FORMULAS")]
    Formulas,
    /// 值
    #[serde(rename = "VALUES")]
    Values,
}

/// 查找选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindOptions {
    /// 是否区分大小写
    #[serde(rename = "case_sensitive")]
    pub case_sensitive: Option<bool>,
    /// 是否全字匹配
    #[serde(rename = "match_whole_word")]
    pub match_whole_word: Option<bool>,
    /// 是否区分公式和值
    #[serde(rename = "look_in_formulas")]
    pub look_in_formulas: Option<bool>,
}

impl FindOptions {
    /// 创建默认查找选项
    pub fn new() -> Self {
        Self {
            case_sensitive: None,
            match_whole_word: None,
            look_in_formulas: None,
        }
    }

    /// 设置是否区分大小写
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = Some(case_sensitive);
        self
    }

    /// 设置是否全字匹配
    pub fn match_whole_word(mut self, match_whole_word: bool) -> Self {
        self.match_whole_word = Some(match_whole_word);
        self
    }

    /// 设置是否在公式中查找
    pub fn look_in_formulas(mut self, look_in_formulas: bool) -> Self {
        self.look_in_formulas = Some(look_in_formulas);
        self
    }
}

/// 替换选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceOptions {
    /// 基本查找选项
    #[serde(flatten)]
    pub find_options: FindOptions,
    /// 是否确认替换
    #[serde(rename = "confirm_replacements")]
    pub confirm_replacements: Option<bool>,
}

impl ReplaceOptions {
    /// 创建默认替换选项
    pub fn new() -> Self {
        Self {
            find_options: FindOptions::new(),
            confirm_replacements: None,
        }
    }

    /// 设置是否区分大小写
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.find_options = self.find_options.case_sensitive(case_sensitive);
        self
    }

    /// 设置是否全字匹配
    pub fn match_whole_word(mut self, match_whole_word: bool) -> Self {
        self.find_options = self.find_options.match_whole_word(match_whole_word);
        self
    }

    /// 设置是否在公式中查找
    pub fn look_in_formulas(mut self, look_in_formulas: bool) -> Self {
        self.find_options = self.find_options.look_in_formulas(look_in_formulas);
        self
    }

    /// 设置是否确认替换
    pub fn confirm_replacements(mut self, confirm: bool) -> Self {
        self.confirm_replacements = Some(confirm);
        self
    }
}

/// 查找匹配结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 行索引
    #[serde(rename = "row_index")]
    pub row_index: u32,
    /// 列索引
    #[serde(rename = "column_index")]
    pub column_index: u32,
    /// 匹配的文本
    #[serde(rename = "matched_text")]
    pub matched_text: String,
    /// 单元格值
    #[serde(rename = "cell_value")]
    pub cell_value: String,
}

impl MatchResult {
    /// 创建匹配结果
    pub fn new(
        sheet_id: String,
        row_index: u32,
        column_index: u32,
        matched_text: String,
        cell_value: String,
    ) -> Self {
        Self {
            sheet_id,
            row_index,
            column_index,
            matched_text,
            cell_value,
        }
    }
}

/// 查找请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 查找文本
    #[serde(rename = "find_text")]
    pub find_text: String,
    /// 匹配类型
    #[serde(rename = "match_type")]
    pub match_type: MatchType,
    /// 查找范围类型
    #[serde(rename = "range_type")]
    pub range_type: SearchRangeType,
    /// 查找范围
    #[serde(rename = "range")]
    pub range: Option<Range>,
    /// 查找选项
    #[serde(rename = "find_options")]
    pub find_options: Option<FindOptions>,
}

impl FindRequest {
    /// 创建查找请求
    pub fn new(
        spreadsheet_token: String,
        sheet_id: String,
        find_text: String,
        match_type: MatchType,
    ) -> Self {
        Self {
            spreadsheet_token,
            sheet_id,
            find_text,
            match_type,
            range_type: SearchRangeType::All,
            range: None,
            find_options: None,
        }
    }

    /// 设置查找范围类型
    pub fn range_type(mut self, range_type: SearchRangeType) -> Self {
        self.range_type = range_type;
        self
    }

    /// 设置查找范围
    pub fn range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置查找选项
    pub fn find_options(mut self, options: FindOptions) -> Self {
        self.find_options = Some(options);
        self
    }

    /// 构建器模式
    pub fn builder() -> FindRequestBuilder {
        FindRequestBuilder::default()
    }

    /// 验证请求
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.spreadsheet_token.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "电子表格ID不能为空".to_string(),
            ));
        }

        if self.sheet_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "工作表ID不能为空".to_string(),
            ));
        }

        if self.find_text.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "查找文本不能为空".to_string(),
            ));
        }

        // 如果范围类型是RANGE，必须提供范围
        if matches!(self.range_type, SearchRangeType::Range) && self.range.is_none() {
            return Err(LarkAPIError::IllegalParamError(
                "当范围类型为RANGE时，必须提供查找范围".to_string(),
            ));
        }

        Ok(())
    }
}

/// 查找请求构建器
#[derive(Debug, Default)]
pub struct FindRequestBuilder {
    spreadsheet_token: Option<String>,
    sheet_id: Option<String>,
    find_text: Option<String>,
    match_type: Option<MatchType>,
    range_type: Option<SearchRangeType>,
    range: Option<Range>,
    find_options: Option<FindOptions>,
}

impl FindRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(sheet_id);
        self
    }

    /// 设置查找文本
    pub fn find_text(mut self, find_text: String) -> Self {
        self.find_text = Some(find_text);
        self
    }

    /// 设置匹配类型
    pub fn match_type(mut self, match_type: MatchType) -> Self {
        self.match_type = Some(match_type);
        self
    }

    /// 设置范围类型
    pub fn range_type(mut self, range_type: SearchRangeType) -> Self {
        self.range_type = Some(range_type);
        self
    }

    /// 设置查找范围
    pub fn range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置查找选项
    pub fn find_options(mut self, options: FindOptions) -> Self {
        self.find_options = Some(options);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<FindRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("工作表ID不能为空".to_string()))?;

        let find_text = self
            .find_text
            .ok_or_else(|| LarkAPIError::IllegalParamError("查找文本不能为空".to_string()))?;

        let match_type = self.match_type.unwrap_or(MatchType::Exact);

        let mut request = FindRequest::new(spreadsheet_token, sheet_id, find_text, match_type);

        if let Some(range_type) = self.range_type {
            request = request.range_type(range_type);
        }
        if let Some(range) = self.range {
            request = request.range(range);
        }
        if let Some(find_options) = self.find_options {
            request = request.find_options(find_options);
        }

        request.validate()?;
        Ok(request)
    }
}

/// 查找响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindResponse {
    /// 匹配结果列表
    #[serde(rename = "matches")]
    pub matches: Vec<MatchResult>,
    /// 匹配总数
    #[serde(rename = "total_matches")]
    pub total_matches: u32,
}

impl openlark_core::api::ApiResponseTrait for FindResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// 替换请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 查找文本
    #[serde(rename = "find_text")]
    pub find_text: String,
    /// 替换文本
    #[serde(rename = "replace_text")]
    pub replace_text: String,
    /// 匹配类型
    #[serde(rename = "match_type")]
    pub match_type: MatchType,
    /// 查找范围类型
    #[serde(rename = "range_type")]
    pub range_type: SearchRangeType,
    /// 查找范围
    #[serde(rename = "range")]
    pub range: Option<Range>,
    /// 替换选项
    #[serde(rename = "replace_options")]
    pub replace_options: Option<ReplaceOptions>,
}

impl ReplaceRequest {
    /// 创建替换请求
    pub fn new(
        spreadsheet_token: String,
        sheet_id: String,
        find_text: String,
        replace_text: String,
        match_type: MatchType,
    ) -> Self {
        Self {
            spreadsheet_token,
            sheet_id,
            find_text,
            replace_text,
            match_type,
            range_type: SearchRangeType::All,
            range: None,
            replace_options: None,
        }
    }

    /// 设置范围类型
    pub fn range_type(mut self, range_type: SearchRangeType) -> Self {
        self.range_type = range_type;
        self
    }

    /// 设置查找范围
    pub fn range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置替换选项
    pub fn replace_options(mut self, options: ReplaceOptions) -> Self {
        self.replace_options = Some(options);
        self
    }

    /// 构建器模式
    pub fn builder() -> ReplaceRequestBuilder {
        ReplaceRequestBuilder::default()
    }

    /// 验证请求
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.spreadsheet_token.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "电子表格ID不能为空".to_string(),
            ));
        }

        if self.sheet_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "工作表ID不能为空".to_string(),
            ));
        }

        if self.find_text.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "查找文本不能为空".to_string(),
            ));
        }

        // 如果范围类型是RANGE，必须提供范围
        if matches!(self.range_type, SearchRangeType::Range) && self.range.is_none() {
            return Err(LarkAPIError::IllegalParamError(
                "当范围类型为RANGE时，必须提供查找范围".to_string(),
            ));
        }

        Ok(())
    }
}

/// 替换请求构建器
#[derive(Debug, Default)]
pub struct ReplaceRequestBuilder {
    spreadsheet_token: Option<String>,
    sheet_id: Option<String>,
    find_text: Option<String>,
    replace_text: Option<String>,
    match_type: Option<MatchType>,
    range_type: Option<SearchRangeType>,
    range: Option<Range>,
    replace_options: Option<ReplaceOptions>,
}

impl ReplaceRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(sheet_id);
        self
    }

    /// 设置查找文本
    pub fn find_text(mut self, find_text: String) -> Self {
        self.find_text = Some(find_text);
        self
    }

    /// 设置替换文本
    pub fn replace_text(mut self, replace_text: String) -> Self {
        self.replace_text = Some(replace_text);
        self
    }

    /// 设置匹配类型
    pub fn match_type(mut self, match_type: MatchType) -> Self {
        self.match_type = Some(match_type);
        self
    }

    /// 设置范围类型
    pub fn range_type(mut self, range_type: SearchRangeType) -> Self {
        self.range_type = Some(range_type);
        self
    }

    /// 设置查找范围
    pub fn range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置替换选项
    pub fn replace_options(mut self, options: ReplaceOptions) -> Self {
        self.replace_options = Some(options);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<ReplaceRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("工作表ID不能为空".to_string()))?;

        let find_text = self
            .find_text
            .ok_or_else(|| LarkAPIError::IllegalParamError("查找文本不能为空".to_string()))?;

        let replace_text = self
            .replace_text
            .ok_or_else(|| LarkAPIError::IllegalParamError("替换文本不能为空".to_string()))?;

        let match_type = self.match_type.unwrap_or(MatchType::Exact);

        let mut request = ReplaceRequest::new(
            spreadsheet_token,
            sheet_id,
            find_text,
            replace_text,
            match_type,
        );

        if let Some(range_type) = self.range_type {
            request = request.range_type(range_type);
        }
        if let Some(range) = self.range {
            request = request.range(range);
        }
        if let Some(replace_options) = self.replace_options {
            request = request.replace_options(replace_options);
        }

        request.validate()?;
        Ok(request)
    }
}

/// 替换响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceResponse {
    /// 替换的数量
    #[serde(rename = "replaced_count")]
    pub replaced_count: u32,
    /// 是否成功
    #[serde(rename = "success")]
    pub success: bool,
}

impl openlark_core::api::ApiResponseTrait for ReplaceResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// Sheets电子表格查找和替换服务 v3
#[derive(Clone, Debug)]
pub struct FindReplaceService {
    config: openlark_core::config::Config,
}

impl FindReplaceService {
    /// 创建查找和替换服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 查找文本
    ///
    /// 在指定工作表中查找文本，支持多种匹配模式和范围限制。
    ///
    /// # 参数
    /// - `request`: 查找请求
    ///
    /// # 返回
    /// 返回查找结果，包括匹配的位置和内容
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::find_replace::*;
    /// use open_lark::service::sheets::v3::models::Range;
    ///
    /// // 查找文本
    /// let request = FindRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .sheet_id("sheet_id".to_string())
    ///     .find_text("已完成".to_string())
    ///     .match_type(MatchType::Contains)
    ///     .range_type(SearchRangeType::All)
    ///     .find_options(FindOptions::new()
    ///         .case_sensitive(false)
    ///         .match_whole_word(false))
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.find(&request).await?;
    /// println!("找到 {} 个匹配项", response.total_matches);
    /// ```
    pub async fn find(
        &self,
        request: &FindRequest,
    ) -> openlark_core::error::SDKResult<FindResponse> {
        use openlark_core::{api::ApiRequest, api::Response, error::LarkAPIError, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/find",
            request.spreadsheet_token, request.sheet_id
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response: Response<FindResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 替换文本
    ///
    /// 在指定工作表中查找并替换文本，支持多种匹配模式和范围限制。
    ///
    /// # 参数
    /// - `request`: 替换请求
    ///
    /// # 返回
    /// 返回替换结果，包括替换的数量
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::find_replace::*;
    /// use open_lark::service::sheets::v3::models::Range;
    ///
    /// // 替换文本
    /// let request = ReplaceRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .sheet_id("sheet_id".to_string())
    ///     .find_text("已完成".to_string())
    ///     .replace_text("完成".to_string())
    ///     .match_type(MatchType::Exact)
    ///     .range_type(SearchRangeType::All)
    ///     .replace_options(ReplaceOptions::new()
    ///         .case_sensitive(false)
    ///         .confirm_replacements(false))
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.replace(&request).await?;
    /// println!("替换了 {} 个项目", response.replaced_count);
    /// ```
    pub async fn replace(
        &self,
        request: &ReplaceRequest,
    ) -> openlark_core::error::SDKResult<ReplaceResponse> {
        use openlark_core::{api::ApiRequest, api::Response, error::LarkAPIError, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/replace",
            request.spreadsheet_token, request.sheet_id
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response: Response<ReplaceResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 查找构建器
    pub fn find_builder(&self) -> FindRequestBuilder {
        FindRequestBuilder::default()
    }

    /// 替换构建器
    pub fn replace_builder(&self) -> ReplaceRequestBuilder {
        ReplaceRequestBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_options_creation() {
        let options = FindOptions::new()
            .case_sensitive(true)
            .match_whole_word(true)
            .look_in_formulas(false);

        assert_eq!(options.case_sensitive, Some(true));
        assert_eq!(options.match_whole_word, Some(true));
        assert_eq!(options.look_in_formulas, Some(false));
    }

    #[test]
    fn test_replace_options_creation() {
        let options = ReplaceOptions::new()
            .case_sensitive(true)
            .match_whole_word(false)
            .confirm_replacements(true);

        assert_eq!(options.find_options.case_sensitive, Some(true));
        assert_eq!(options.find_options.match_whole_word, Some(false));
        assert_eq!(options.confirm_replacements, Some(true));
    }

    #[test]
    fn test_match_result_creation() {
        let result = MatchResult::new(
            "sheet123".to_string(),
            5,
            3,
            "已完成".to_string(),
            "任务已完成".to_string(),
        );

        assert_eq!(result.sheet_id, "sheet123");
        assert_eq!(result.row_index, 5);
        assert_eq!(result.column_index, 3);
        assert_eq!(result.matched_text, "已完成");
        assert_eq!(result.cell_value, "任务已完成");
    }

    #[test]
    fn test_find_request_creation() {
        let request = FindRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "test".to_string(),
            MatchType::Contains,
        )
        .range_type(SearchRangeType::All)
        .find_options(FindOptions::new().case_sensitive(false));

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.find_text, "test");
        assert_eq!(request.range_type, SearchRangeType::All);
        assert!(request.find_options.is_some());
    }

    #[test]
    fn test_find_request_validation() {
        use super::super::models::Range;

        // 测试有效请求
        let valid_request = FindRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "test".to_string(),
            MatchType::Exact,
        );
        assert!(valid_request.validate().is_ok());

        // 测试空电子表格ID
        let invalid_request = FindRequest::new(
            "".to_string(),
            "sheet123".to_string(),
            "test".to_string(),
            MatchType::Exact,
        );
        assert!(invalid_request.validate().is_err());

        // 测试空工作表ID
        let invalid_request2 = FindRequest::new(
            "token123".to_string(),
            "".to_string(),
            "test".to_string(),
            MatchType::Exact,
        );
        assert!(invalid_request2.validate().is_err());

        // 测试空查找文本
        let invalid_request3 = FindRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "".to_string(),
            MatchType::Exact,
        );
        assert!(invalid_request3.validate().is_err());

        // 测试Range类型但没有范围
        let invalid_request4 = FindRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "test".to_string(),
            MatchType::Exact,
        )
        .range_type(SearchRangeType::Range);
        assert!(invalid_request4.validate().is_err());

        // 测试Range类型有范围
        let valid_range_request = FindRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "test".to_string(),
            MatchType::Exact,
        )
        .range_type(SearchRangeType::Range)
        .range(Range::from("A1".to_string(), "D10".to_string()));
        assert!(valid_range_request.validate().is_ok());
    }

    #[test]
    fn test_find_request_builder() {
        use super::super::models::Range;

        let request = FindRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .find_text("已完成".to_string())
            .match_type(MatchType::Contains)
            .range_type(SearchRangeType::Range)
            .range(Range::from("A1".to_string(), "D10".to_string()))
            .find_options(
                FindOptions::new()
                    .case_sensitive(false)
                    .match_whole_word(true),
            )
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.find_text, "已完成");
        assert_eq!(request.match_type, MatchType::Contains);
        assert_eq!(request.range_type, SearchRangeType::Range);
        assert!(request.range.is_some());
        assert!(request.find_options.is_some());
    }

    #[test]
    fn test_replace_request_creation() {
        let request = ReplaceRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "old".to_string(),
            "new".to_string(),
            MatchType::Exact,
        )
        .range_type(SearchRangeType::All)
        .replace_options(ReplaceOptions::new().case_sensitive(true));

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.find_text, "old");
        assert_eq!(request.replace_text, "new");
        assert_eq!(request.range_type, SearchRangeType::All);
        assert!(request.replace_options.is_some());
    }

    #[test]
    fn test_replace_request_validation() {
        // 测试有效请求
        let valid_request = ReplaceRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "old".to_string(),
            "new".to_string(),
            MatchType::Exact,
        );
        assert!(valid_request.validate().is_ok());

        // 测试空替换文本
        let invalid_request = ReplaceRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "old".to_string(),
            "".to_string(),
            MatchType::Exact,
        );
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_replace_request_builder() {
        use super::super::models::Range;

        let request = ReplaceRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .find_text("未完成".to_string())
            .replace_text("进行中".to_string())
            .match_type(MatchType::Exact)
            .range_type(SearchRangeType::Values)
            .range(Range::from("A1".to_string(), "A100".to_string()))
            .replace_options(
                ReplaceOptions::new()
                    .case_sensitive(false)
                    .confirm_replacements(false),
            )
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.find_text, "未完成");
        assert_eq!(request.replace_text, "进行中");
        assert_eq!(request.match_type, MatchType::Exact);
        assert_eq!(request.range_type, SearchRangeType::Values);
        assert!(request.range.is_some());
        assert!(request.replace_options.is_some());
    }

    #[test]
    fn test_find_replace_service_creation() {
        let config = openlark_core::config::openlark_core::config::Config::default();
        let service = FindReplaceService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_comprehensive_find_replace_scenarios() {
        use super::super::models::Range;

        // 测试简单查找场景
        let simple_find_request = FindRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .find_text("完成".to_string())
            .match_type(MatchType::Contains)
            .range_type(SearchRangeType::All)
            .find_options(FindOptions::new().case_sensitive(false))
            .build()
            .unwrap();

        assert_eq!(simple_find_request.find_text, "完成");
        assert_eq!(simple_find_request.range_type, SearchRangeType::All);

        // 测试正则表达式查找
        let regex_find_request = FindRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .find_text("\\d{4}-\\d{2}-\\d{2}".to_string())
            .match_type(MatchType::Regex)
            .range_type(SearchRangeType::All)
            .find_options(FindOptions::new().case_sensitive(false))
            .build()
            .unwrap();

        assert_eq!(regex_find_request.match_type, MatchType::Regex);

        // 测试指定范围替换
        let range_replace_request = ReplaceRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .find_text("旧值".to_string())
            .replace_text("新值".to_string())
            .match_type(MatchType::Exact)
            .range_type(SearchRangeType::Range)
            .range(Range::from("A1".to_string(), "D50".to_string()))
            .replace_options(
                ReplaceOptions::new()
                    .case_sensitive(true)
                    .confirm_replacements(false),
            )
            .build()
            .unwrap();

        assert_eq!(range_replace_request.range_type, SearchRangeType::Range);
        assert!(range_replace_request.range.is_some());

        // 测试公式范围查找
        let formula_find_request = FindRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .find_text("SUM".to_string())
            .match_type(MatchType::Contains)
            .range_type(SearchRangeType::Formulas)
            .find_options(FindOptions::new().look_in_formulas(true))
            .build()
            .unwrap();

        assert_eq!(formula_find_request.range_type, SearchRangeType::Formulas);

        // 测试值范围查找
        let values_find_request = FindRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .find_text("总计".to_string())
            .match_type(MatchType::Contains)
            .range_type(SearchRangeType::Values)
            .find_options(FindOptions::new().look_in_formulas(false))
            .build()
            .unwrap();

        assert_eq!(values_find_request.range_type, SearchRangeType::Values);
    }

    #[test]
    fn test_match_types() {
        let match_types = vec![
            MatchType::Exact,
            MatchType::Contains,
            MatchType::StartsWith,
            MatchType::EndsWith,
            MatchType::Regex,
        ];

        for match_type in match_types {
            let request = FindRequest::new(
                "token123".to_string(),
                "sheet123".to_string(),
                "test".to_string(),
                match_type,
            );
            assert_eq!(request.sheet_id, "sheet123");
        }
    }

    #[test]
    fn test_search_range_types() {
        let range_types = vec![
            SearchRangeType::All,
            SearchRangeType::Range,
            SearchRangeType::Formulas,
            SearchRangeType::Values,
        ];

        for range_type in range_types {
            let request = FindRequest::new(
                "token123".to_string(),
                "sheet123".to_string(),
                "test".to_string(),
                MatchType::Exact,
            )
            .range_type(range_type);
            assert_eq!(request.sheet_id, "sheet123");
        }
    }

    #[test]
    fn test_find_replace_serialization() {
        use serde_json;

        let request = FindRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "test".to_string(),
            MatchType::Contains,
        );
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("CONTAINS"));
        assert!(json.contains("token123"));
    }
}

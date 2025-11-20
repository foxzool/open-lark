//! 数据验证服务
//!
//! 提供飞书电子表格v2版本的数据验证功能，包括：
//! - 设置下拉列表验证
//! - 更新数据验证规则
//! - 查询数据验证设置
//! - 删除数据验证规则
//! - 支持多种验证类型和条件

use serde_json::Value;
use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 数据验证类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationType {
    /// 下拉列表
    Dropdown,
    /// 数字范围
    NumberRange,
    /// 文本长度
    TextLength,
    /// 日期范围
    DateRange,
    /// 自定义公式
    CustomFormula,
    /// 列表值（多选）
    MultiSelectDropdown,
}

/// 下拉列表数据源
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropdownSource {
    /// 静态值列表
    Values(Vec<String>),
    /// 范围引用
    Range(String),
}

/// 验证条件操作符
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationOperator {
    /// 等于
    Equal,
    /// 不等于
    NotEqual,
    /// 大于
    GreaterThan,
    /// 大于等于
    GreaterThanOrEqual,
    /// 小于
    LessThan,
    /// 小于等于
    LessThanOrEqual,
    /// 介于
    Between,
    /// 不介于
    NotBetween,
    /// 包含
    Contains,
    /// 不包含
    NotContains,
    /// 开始于
    BeginsWith,
    /// 结束于
    EndsWith,
    /// 在列表中
    InList,
    /// 不在列表中
    NotInList,
}

/// 验证条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCondition {
    /// 操作符
    #[serde(rename = "operator")]
    pub operator: ValidationOperator,
    /// 第一个值
    #[serde(rename = "value1", skip_serializing_if = "Option::is_none")]
    pub value1: Option<String>,
    /// 第二个值（用于Between操作符）
    #[serde(rename = "value2", skip_serializing_if = "Option::is_none")]
    pub value2: Option<String>,
}

impl ValidationCondition {
    /// 创建新的验证条件
    pub fn new(operator: ValidationOperator) -> Self {
        Self {
            operator,
            value1: None,
            value2: None,
        }
    }

    /// 设置第一个值
    pub fn value1(mut self, value: impl Into<String>) -> Self {
        self.value1 = Some(value.into());
        self
    }

    /// 设置第二个值
    pub fn value2(mut self, value: impl Into<String>) -> Self {
        self.value2 = Some(value.into());
        self
    }

    /// 设置值对（用于Between等操作符）
    pub fn values(value1: impl Into<String>, value2: impl Into<String>) -> Self {
        self.value1(Some(value1.into())).value2(value2)
    }
}

/// 数据验证设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidation {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 范围
    #[serde(rename = "range")]
    pub range: String,
    /// 验证类型
    #[serde(rename = "validationType")]
    pub validation_type: ValidationType,
    /// 条件
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<ValidationCondition>,
    /// 下拉列表数据源
    #[serde(rename = "dropdownSource", skip_serializing_if = "Option::is_none")]
    pub dropdown_source: Option<DropdownSource>,
    /// 提示信息
    #[serde(rename = "promptMessage", skip_serializing_if = "Option::is_none")]
    pub prompt_message: Option<String>,
    /// 错误信息
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 是否允许空值
    #[serde(rename = "allowEmpty", skip_serializing_if = "Option::is_none")]
    pub allow_empty: Option<bool>,
    /// 是否严格模式（不允许其他值）
    #[serde(rename = "strictMode", skip_serializing_if = "Option::is_none")]
    pub strict_mode: Option<bool>,
}

impl DataValidation {
    /// 创建新的数据验证设置
    pub fn new(
        sheet_id: impl Into<String>,
        range: impl Into<String>,
        validation_type: ValidationType,
    ) -> Self {
        Self {
            sheet_id: sheet_id.into(),
            range: range.into(),
            validation_type,
            condition: None,
            dropdown_source: None,
            prompt_message: None,
            error_message: None,
            allow_empty: Some(true),
            strict_mode: Some(true),
        }
    }

    /// 设置验证条件
    pub fn condition(mut self, condition: ValidationCondition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// 设置下拉列表数据源
    pub fn dropdown_source(mut self, source: DropdownSource) -> Self {
        self.dropdown_source = Some(source);
        self
    }

    /// 设置提示信息
    pub fn prompt_message(mut self, message: impl Into<String>) -> Self {
        self.prompt_message = Some(message.into());
        self
    }

    /// 设置错误信息
    pub fn error_message(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }

    /// 设置是否允许空值
    pub fn allow_empty(mut self, allow: bool) -> Self {
        self.allow_empty = Some(allow);
        self
    }

    /// 设置是否严格模式
    pub fn strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = Some(strict);
        self
    }

    /// 验证数据验证设置
    pub fn validate(&self) -> Result<(), String> {
        if self.sheet_id.trim().is_empty() {
            return Err("工作表ID不能为空".to_string());
        }

        if self.range.trim().is_empty() {
            return Err("验证范围不能为空".to_string());
        }

        // 验证范围格式
        if !is_valid_range_format(&self.range) {
            return Err("验证范围格式无效，应为类似 A1:B2 的格式".to_string());
        }

        match self.validation_type {
            ValidationType::Dropdown => {
                if self.dropdown_source.is_none() {
                    return Err("下拉列表验证必须设置数据源".to_string());
                }
                // 验证下拉列表数据源
                if let Some(ref source) = self.dropdown_source {
                    match source {
                        DropdownSource::Values(values) => {
                            if values.is_empty() {
                                return Err("下拉列表值不能为空".to_string());
                            }
                            if values.len() > 1000 {
                                return Err("下拉列表值不能超过1000个".to_string());
                            }
                        }
                        DropdownSource::Range(range) => {
                            if range.trim().is_empty() {
                                return Err("下拉列表范围不能为空".to_string());
                            }
                        }
                    }
                }
            }
            ValidationType::NumberRange => {
                if self.condition.is_none() {
                    return Err("数字范围验证必须设置条件".to_string());
                }
                // 验证数字值
                if let Some(ref condition) = self.condition {
                    if let Some(ref value1) = condition.value1 {
                        if !is_valid_number(value1) {
                            return Err("无效的数字值".to_string());
                        }
                    }
                    if let Some(ref value2) = condition.value2 {
                        if !is_valid_number(value2) {
                            return Err("无效的数字值".to_string());
                        }
                    }
                }
            }
            ValidationType::TextLength => {
                if self.condition.is_none() {
                    return Err("文本长度验证必须设置条件".to_string());
                }
                // 验证长度值
                if let Some(ref condition) = self.condition {
                    if let Some(ref value1) = condition.value1 {
                        if !is_valid_number(value1) {
                            return Err("无效的长度值".to_string());
                        }
                    }
                }
            }
            ValidationType::DateRange => {
                if self.condition.is_none() {
                    return Err("日期范围验证必须设置条件".to_string());
                }
                // 验证日期值
                if let Some(ref condition) = self.condition {
                    if let Some(ref value1) = condition.value1 {
                        if !is_valid_date(value1) {
                            return Err("无效的日期格式".to_string());
                        }
                    }
                }
            }
            ValidationType::CustomFormula => {
                if self.condition.is_none() {
                    return Err("自定义公式验证必须设置条件".to_string());
                }
            }
            ValidationType::MultiSelectDropdown => {
                if self.dropdown_source.is_none() {
                    return Err("多选下拉列表验证必须设置数据源".to_string());
                }
            }
        }

        Ok(())
    }
}

/// 验证范围格式
fn is_valid_range_format(range: &str) -> bool {
    if range.is_empty() {
        return false;
    }

    let range = range.trim();

    // 检查是否包含冒号（范围格式）或不包含（单个单元格）
    if range.contains(':') {
        let parts: Vec<&str> = range.split(':').collect();
        if parts.len() != 2 {
            return false;
        }
        is_valid_cell_reference(parts[0]) && is_valid_cell_reference(parts[1])
    } else {
        is_valid_cell_reference(range)
    }
}

/// 验证单元格引用格式
fn is_valid_cell_reference(cell_ref: &str) -> bool {
    if cell_ref.is_empty() {
        return false;
    }

    let cell_ref = cell_ref.trim();
    let mut chars = cell_ref.chars();

    // 第一个字符必须是字母
    if !chars.next().map_or(false, |c| c.is_ascii_alphabetic()) {
        return false;
    }

    // 剩余字符应该是字母或数字，但至少要有一个数字
    let mut has_digit = false;
    for c in chars {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if !c.is_ascii_alphabetic() {
            return false;
        }
    }

    has_digit
}

/// 验证数字格式
fn is_valid_number(value: &str) -> bool {
    value.trim().parse::<f64>().is_ok()
}

/// 验证日期格式 (简单的 YYYY-MM-DD 验证)
fn is_valid_date(value: &str) -> bool {
    let value = value.trim();

    // 检查基本格式 YYYY-MM-DD
    if value.len() != 10 {
        return false;
    }

    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    // 简单的数字验证
    parts
        .iter()
        .all(|part| part.chars().all(|c| c.is_ascii_digit()))
}

/// 设置数据验证请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDataValidationRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 数据验证设置
    #[serde(rename = "dataValidation")]
    pub data_validation: DataValidation,
}

impl SetDataValidationRequest {
    /// 创建新的设置数据验证请求
    pub fn new(spreadsheet_token: impl Into<String>, data_validation: DataValidation) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            data_validation,
        }
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        self.data_validation.validate()
    }
}

/// 设置数据验证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDataValidationResponse {
    /// 响应数据
    pub data: SetDataValidationData,
}

/// 设置数据验证数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDataValidationData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 数据验证ID
    #[serde(rename = "dataValidationId")]
    pub data_validation_id: String,
    /// 验证状态
    #[serde(rename = "status")]
    pub status: String,
}

impl Default for SetDataValidationResponse {
    fn default() -> Self {
        Self {
            data: SetDataValidationData {
                spreadsheet_token: String::new(),
                data_validation_id: String::new(),
                status: "success".to_string(),
            },
        }
    }
}

impl ApiResponseTrait for SetDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新数据验证请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDataValidationRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 数据验证ID
    #[serde(rename = "dataValidationId")]
    pub data_validation_id: String,
    /// 更新的数据验证设置
    #[serde(rename = "dataValidation")]
    pub data_validation: DataValidation,
}

impl UpdateDataValidationRequest {
    /// 创建新的更新数据验证请求
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        data_validation_id: impl Into<String>,
        data_validation: DataValidation,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            data_validation_id: data_validation_id.into(),
            data_validation,
        }
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        if self.sheet_id.trim().is_empty() {
            return Err("工作表ID不能为空".to_string());
        }

        if self.data_validation_id.trim().is_empty() {
            return Err("数据验证ID不能为空".to_string());
        }

        self.data_validation.validate()
    }
}

/// 更新数据验证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDataValidationResponse {
    /// 响应数据
    pub data: UpdateDataValidationData,
}

/// 更新数据验证数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDataValidationData {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 数据验证ID
    #[serde(rename = "dataValidationId")]
    pub data_validation_id: String,
    /// 更新状态
    #[serde(rename = "status")]
    pub status: String,
}

impl Default for UpdateDataValidationResponse {
    fn default() -> Self {
        Self {
            data: UpdateDataValidationData {
                spreadsheet_token: String::new(),
                data_validation_id: String::new(),
                status: "success".to_string(),
            },
        }
    }
}

impl ApiResponseTrait for UpdateDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询数据验证请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataValidationRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 查询的范围，如果不指定则查询整个工作表
    pub range: Option<String>,
}

/// 数据验证项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationItem {
    /// 数据验证ID
    #[serde(rename = "dataValidationId")]
    pub data_validation_id: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 范围
    pub range: String,
    /// 数据验证信息
    pub data_validation: DataValidation,
}

/// 查询数据验证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDataValidationResponse {
    /// 数据验证列表
    #[serde(rename = "dataValidation")]
    pub data_validation: Vec<DataValidationItem>,
}

/// 删除数据验证请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDataValidationRequest {
    /// 电子表格token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 数据验证ID列表，如果不指定则删除整个工作表的所有数据验证
    #[serde(rename = "dataValidationId")]
    pub data_validation_id: Option<String>,
    /// 范围，当指定dataValidationId时可用
    pub range: Option<String>,
}

/// 删除数据验证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDataValidationResponse {
    /// 删除状态
    pub status: String,
    /// 删除的数据验证数量
    pub count: i32,
}

impl Default for GetDataValidationResponse {
    fn default() -> Self {
        Self {
            data_validation: vec![],
        }
    }
}

impl ApiResponseTrait for GetDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl Default for DeleteDataValidationResponse {
    fn default() -> Self {
        Self {
            status: "success".to_string(),
            count: 0,
        }
    }
}

impl ApiResponseTrait for DeleteDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetDataValidationRequest {
    /// 创建新的查询数据验证请求
    pub fn new(spreadsheet_token: impl Into<String>, sheet_id: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            range: None,
        }
    }

    /// 设置查询范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        if self.sheet_id.trim().is_empty() {
            return Err("工作表ID不能为空".to_string());
        }

        Ok(())
    }
}

impl DeleteDataValidationRequest {
    /// 创建新的删除数据验证请求
    pub fn new(spreadsheet_token: impl Into<String>, sheet_id: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            data_validation_id: None,
            range: None,
        }
    }

    /// 设置数据验证ID
    pub fn data_validation_id(mut self, id: impl Into<String>) -> Self {
        self.data_validation_id = Some(id.into());
        self
    }

    /// 设置范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }

        if self.sheet_id.trim().is_empty() {
            return Err("工作表ID不能为空".to_string());
        }

        Ok(())
    }
}

/// 数据验证操作服务
#[derive(Clone, Debug)]
pub struct DataValidationService {
    config: Config,
}

impl DataValidationService {
    /// 创建数据验证服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 设置数据验证
    ///
    /// 为指定范围的单元格设置数据验证规则，支持下拉列表、数字范围、文本长度等多种验证类型。
    ///
    /// # 参数
    /// - `request`: 设置数据验证请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// // 创建下拉列表验证
    /// let data_validation = DataValidation::new(
    ///     "sheet1",
    ///     "A1:A10",
    ///     ValidationType::Dropdown
    /// ).dropdown_source(DropdownSource::Values(vec![
    ///     "选项1".to_string(),
    ///     "选项2".to_string(),
    ///     "选项3".to_string()
    /// ])).prompt_message("请从下拉列表中选择");
    ///
    /// let request = SetDataValidationRequest::new("spreadsheet_token", data_validation);
    /// let response = service.set_data_validation(request, None).await?;
    /// println!("数据验证ID: {}", response.data.data_validation_id);
    /// ```
    pub async fn set_data_validation(
        &self,
        request: SetDataValidationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<SetDataValidationResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::POST,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dataValidation",
                &request.spreadsheet_token
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<SetDataValidationResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 更新数据验证
    ///
    /// 更新已存在的数据验证规则。
    ///
    /// # 参数
    /// - `request`: 更新数据验证请求
    /// - `option`: 可选的请求配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let updated_validation = DataValidation::new(
    ///     "sheet1",
    ///     "A1:A15",
    ///     ValidationType::Dropdown
    /// ).dropdown_source(DropdownSource::Values(vec![
    ///     "新选项1".to_string(),
    ///     "新选项2".to_string()
    /// ]));
    ///
    /// let request = UpdateDataValidationRequest::new(
    ///     "spreadsheet_token",
    ///     "sheet1",
    ///     "validation_123",
    ///     updated_validation
    /// );
    ///
    /// let response = service.update_data_validation(request, None).await?;
    /// println!("更新状态: {}", response.data.status);
    /// ```
    pub async fn update_data_validation(
        &self,
        request: UpdateDataValidationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UpdateDataValidationResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let mut api_req = ApiRequest::with_method_and_path(
            Method::PUT,
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dataValidation/{}/{}",
                &request.spreadsheet_token, &request.sheet_id, &request.data_validation_id
            ),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Some(openlark_core::api::RequestData::Json(&request))?;

        // 发送请求
        let api_resp =
            Transport::<UpdateDataValidationResponse>::request(api_req, &self.config, option)
                .await?;

        Ok(api_resp)
    }

    /// 查询数据验证
    ///
    /// 查询指定工作表或范围内的数据验证设置。
    ///
    /// # 参数
    /// - `request`: 查询数据验证请求
    /// - `option`: 请求选项（可选）
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::data_validation::{GetDataValidationRequest, DataValidationService};
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = DataValidationService::new(config);
    ///
    /// let request = GetDataValidationRequest::new("spreadsheet_token", "sheet1")
    ///     .range("A1:C10");
    ///
    /// let response = service.get_data_validation(request, None).await?;
    /// println!("找到 {} 个数据验证规则", response.data.data_validation.len());
    /// ```
    pub async fn get_data_validation(
        &self,
        request: GetDataValidationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<GetDataValidationResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let endpoint = if let Some(range) = &request.range {
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dataValidation?sheetId={}&range={}",
                &request.spreadsheet_token, &request.sheet_id, range
            )
        } else {
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dataValidation?sheetId={}",
                &request.spreadsheet_token, &request.sheet_id
            )
        };

        let mut api_req = ApiRequest::with_method_and_path(Method::GET, &endpoint);
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 发送请求
        let api_resp =
            Transport::<GetDataValidationResponse>::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }

    /// 删除数据验证
    ///
    /// 删除指定工作表或特定数据验证规则。
    ///
    /// # 参数
    /// - `request`: 删除数据验证请求
    /// - `option`: 请求选项（可选）
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::data_validation::{DeleteDataValidationRequest, DataValidationService};
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = DataValidationService::new(config);
    ///
    /// let request = DeleteDataValidationRequest::new("spreadsheet_token", "sheet1")
    ///     .data_validation_id("validation_123");
    ///
    /// let response = service.delete_data_validation(request, None).await?;
    /// println!("删除状态: {}, 删除数量: {}", response.data.status, response.data.count);
    /// ```
    pub async fn delete_data_validation(
        &self,
        request: DeleteDataValidationRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<DeleteDataValidationResponse>> {
        if let Err(err) = request.validate() {
            return Err(LarkAPIError::InvalidParameter(err));
        }

        // 构建API请求
        let endpoint = if let Some(data_validation_id) = &request.data_validation_id {
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dataValidation?sheetId={}&dataValidationId={}",
                &request.spreadsheet_token, &request.sheet_id, data_validation_id
            )
        } else {
            format!(
                "/open-apis/sheets/v2/spreadsheets/{}/dataValidation?sheetId={}",
                &request.spreadsheet_token, &request.sheet_id
            )
        };

        let mut api_req = ApiRequest::with_method_and_path(Method::DELETE, &endpoint);
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 添加范围参数（如果有）
        if let Some(range) = &request.range {
            // 这里需要在URL中添加range参数，但由于ApiRequest的限制，我们需要使用不同的方式
            let full_endpoint = format!("{}&range={}", endpoint, range);
            let mut new_req = ApiRequest::with_method_and_path(Method::DELETE, &full_endpoint);
            new_req.set_supported_access_token_types(vec![
                AccessTokenType::Tenant,
                AccessTokenType::User,
            ]);

            let api_resp =
                Transport::<DeleteDataValidationResponse>::request(new_req, &self.config, option)
                    .await?;
            Ok(api_resp)
        } else {
            let api_resp =
                Transport::<DeleteDataValidationResponse>::request(api_req, &self.config, option)
                    .await?;
            Ok(api_resp)
        }
    }

    /// 创建下拉列表验证构建器
    pub fn dropdown_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        range: impl Into<String>,
    ) -> DropdownValidationBuilder {
        DropdownValidationBuilder::new(
            self.config.clone()
            spreadsheet_token.into(),
            sheet_id.into(),
            range.into(),
        )
    }

    /// 创建数字范围验证构建器
    pub fn number_range_builder(
        &self,
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        range: impl Into<String>,
    ) -> NumberRangeValidationBuilder {
        NumberRangeValidationBuilder::new(
            self.config.clone()
            spreadsheet_token.into(),
            sheet_id.into(),
            range.into(),
        )
    }
}

/// 下拉列表验证构建器
#[derive(Clone, Debug)]
pub struct DropdownValidationBuilder {
    config: Config,
    spreadsheet_token: String,
    sheet_id: String,
    range: String,
    values: Vec<String>,
    prompt_message: Option<String>,
    error_message: Option<String>,
    allow_empty: bool,
    strict_mode: bool,
}

impl DropdownValidationBuilder {
    /// 创建新的下拉列表验证构建器实例
    pub fn new(config: Config, spreadsheet_token: String, sheet_id: String, range: String) -> Self {
        Self {
            config,
            spreadsheet_token,
            sheet_id,
            range,
            values: vec![],
            prompt_message: None,
            error_message: None,
            allow_empty: true,
            strict_mode: true,
        }
    }

    /// 添加选项值
    pub fn add_value(mut self, value: impl Into<String>) -> Self {
        self.values.push(value.into());
        self
    }

    /// 批量添加选项值
    pub fn add_values(mut self, values: Vec<impl Into<String>>) -> Self {
        for value in values {
            self.values.push(value.into());
        }
        self
    }

    /// 设置提示信息
    pub fn prompt_message(mut self, message: impl Into<String>) -> Self {
        self.prompt_message = Some(message.into());
        self
    }

    /// 设置错误信息
    pub fn error_message(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }

    /// 设置是否允许空值
    pub fn allow_empty(mut self, allow: bool) -> Self {
        self.allow_empty = allow;
        self
    }

    /// 设置是否严格模式
    pub fn strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// 执行设置请求
    pub async fn execute(self) -> SDKResult<Response<SetDataValidationResponse>> {
        let data_validation =
            DataValidation::new(self.sheet_id, self.range, ValidationType::Dropdown)
                .dropdown_source(DropdownSource::Values(self.values))
                .prompt_message(
                    self.prompt_message
                        .unwrap_or_else(|| "请从下拉列表中选择".to_string()),
                )
                .error_message(
                    self.error_message
                        .unwrap_or_else(|| "输入的值不在允许的列表中".to_string()),
                )
                .allow_empty(self.allow_empty)
                .strict_mode(self.strict_mode);

        let request = SetDataValidationRequest::new(self.spreadsheet_token, data_validation);

        let service = DataValidationService {
            config: self.config,
        };
        service.set_data_validation(request, None).await
    }
}

/// 数字范围验证构建器
#[derive(Clone, Debug)]
pub struct NumberRangeValidationBuilder {
    config: Config,
    spreadsheet_token: String,
    sheet_id: String,
    range: String,
    operator: ValidationOperator,
    value1: Option<String>,
    value2: Option<String>,
    prompt_message: Option<String>,
    error_message: Option<String>,
}

impl NumberRangeValidationBuilder {
    /// 创建新的数字范围验证构建器实例
    pub fn new(config: Config, spreadsheet_token: String, sheet_id: String, range: String) -> Self {
        Self {
            config,
            spreadsheet_token,
            sheet_id,
            range,
            operator: ValidationOperator::Between,
            value1: None,
            value2: None,
            prompt_message: None,
            error_message: None,
        }
    }

    /// 设置操作符
    pub fn operator(mut self, operator: ValidationOperator) -> Self {
        self.operator = operator;
        self
    }

    /// 设置第一个值
    pub fn value1(mut self, value: impl Into<String>) -> Self {
        self.value1 = Some(value.into());
        self
    }

    /// 设置第二个值
    pub fn value2(mut self, value: impl Into<String>) -> Self {
        self.value2 = Some(value.into());
        self
    }

    /// 设置值对（用于Between等操作符）
    pub fn values(min_value: impl Into<String>, max_value: impl Into<String>) -> Self {
        self.value1(Some(min_value.into())).value2(max_value)
    }

    /// 设置提示信息
    pub fn prompt_message(mut self, message: impl Into<String>) -> Self {
        self.prompt_message = Some(message.into());
        self
    }

    /// 设置错误信息
    pub fn error_message(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }

    /// 执行设置请求
    pub async fn execute(self) -> SDKResult<Response<SetDataValidationResponse>> {
        let condition = ValidationCondition::new(self.operator)
            .value1(self.value1.unwrap_or_else(|| "0".to_string()))
            .value2(self.value2);

        let data_validation =
            DataValidation::new(self.sheet_id, self.range, ValidationType::NumberRange)
                .condition(condition)
                .prompt_message(
                    self.prompt_message
                        .unwrap_or_else(|| "请输入有效的数字".to_string()),
                )
                .error_message(
                    self.error_message
                        .unwrap_or_else(|| "输入的数字不符合验证条件".to_string()),
                );

        let request = SetDataValidationRequest::new(self.spreadsheet_token, data_validation);

        let service = DataValidationService {
            config: self.config,
        };
        service.set_data_validation(request, None).await
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_type_serialization() {
        let dropdown = ValidationType::Dropdown;
        let serialized = serde_json::to_string(&dropdown).unwrap();
        assert_eq!(serialized, "\"DROPDOWN\"");

        let number_range = ValidationType::NumberRange;
        let serialized = serde_json::to_string(&number_range).unwrap();
        assert_eq!(serialized, "\"NUMBER_RANGE\"");
    }

    #[test]
    fn test_validation_operator_serialization() {
        let equal = ValidationOperator::Equal;
        let serialized = serde_json::to_string(&equal).unwrap();
        assert_eq!(serialized, "\"EQUAL\"");

        let between = ValidationOperator::Between;
        let serialized = serde_json::to_string(&between).unwrap();
        assert_eq!(serialized, "\"BETWEEN\"");
    }

    #[test]
    fn test_validation_condition() {
        let condition = ValidationCondition::new(ValidationOperator::Between).values("10", "100");

        assert_eq!(condition.operator, ValidationOperator::Between);
        assert_eq!(condition.value1, Some("10".to_string()));
        assert_eq!(condition.value2, Some("100".to_string()));
    }

    #[test]
    fn test_dropdown_source() {
        let values_source = DropdownSource::Values(vec![
            "选项1".to_string(),
            "选项2".to_string(),
            "选项3".to_string(),
        ]);

        if let DropdownSource::Values(values) = values_source {
            assert_eq!(values.len(), 3);
            assert_eq!(values[0], "选项1");
        } else {
            panic!("应该是Values类型");
        }

        let range_source = DropdownSource::Range("Sheet1!A1:A5".to_string());
        if let DropdownSource::Range(range) = range_source {
            assert_eq!(range, "Sheet1!A1:A5");
        } else {
            panic!("应该是Range类型");
        }
    }

    #[test]
    fn test_data_validation_creation() {
        let validation = DataValidation::new("sheet1", "A1:A10", ValidationType::Dropdown)
            .dropdown_source(DropdownSource::Values(vec![
                "男".to_string(),
                "女".to_string(),
            ]))
            .prompt_message("请选择性别")
            .allow_empty(true)
            .strict_mode(false);

        assert_eq!(validation.sheet_id, "sheet1");
        assert_eq!(validation.range, "A1:A10");
        assert_eq!(validation.validation_type, ValidationType::Dropdown);
        assert_eq!(validation.allow_empty, Some(true));
        assert_eq!(validation.strict_mode, Some(false));
        assert_eq!(validation.prompt_message, Some("请选择性别".to_string()));
    }

    #[test]
    fn test_data_validation_validation() {
        // 测试正常的下拉列表验证
        let valid_dropdown = DataValidation::new("sheet1", "A1:B2", ValidationType::Dropdown)
            .dropdown_source(DropdownSource::Values(vec!["选项1".to_string()]));
        assert!(valid_dropdown.validate().is_ok());

        // 测试缺少数据源的下拉列表验证
        let invalid_dropdown = DataValidation::new("sheet1", "A1:B2", ValidationType::Dropdown);
        assert!(invalid_dropdown.validate().is_err());

        // 测试空的下拉列表
        let empty_values = DataValidation::new("sheet1", "A1:B2", ValidationType::Dropdown)
            .dropdown_source(DropdownSource::Values(vec![]));
        assert!(empty_values.validate().is_err());

        // 测试正常的数字范围验证
        let valid_number = DataValidation::new("sheet1", "A1:B2", ValidationType::NumberRange)
            .condition(ValidationCondition::new(ValidationOperator::Between).values("10", "100"));
        assert!(valid_number.validate().is_ok());

        // 测试缺少条件的数字范围验证
        let invalid_number = DataValidation::new("sheet1", "A1:B2", ValidationType::NumberRange);
        assert!(invalid_number.validate().is_err());

        // 测试无效的数字值
        let invalid_numeric = DataValidation::new("sheet1", "A1:B2", ValidationType::NumberRange)
            .condition(ValidationCondition::new(ValidationOperator::Equal).value1("abc"));
        assert!(invalid_numeric.validate().is_err());

        // 测试无效的范围格式
        let invalid_range =
            DataValidation::new("sheet1", "invalid_range", ValidationType::Dropdown)
                .dropdown_source(DropdownSource::Values(vec!["选项1".to_string()]));
        assert!(invalid_range.validate().is_err());
    }

    #[test]
    fn test_is_valid_range_format() {
        assert!(is_valid_range_format("A1"));
        assert!(is_valid_range_format("B10"));
        assert!(is_valid_range_format("A1:B2"));
        assert!(is_valid_range_format("Sheet1!A1:B2"));

        assert!(!is_valid_range_format(""));
        assert!(!is_valid_range_format("A1:"));
        assert!(!is_valid_range_format(":B2"));
        assert!(!is_valid_range_format("A1::B2"));
    }

    #[test]
    fn test_is_valid_cell_reference() {
        assert!(is_valid_cell_reference("A1"));
        assert!(is_valid_cell_reference("B10"));
        assert!(is_valid_cell_reference("Z100"));
        assert!(is_valid_cell_reference("AA1"));

        assert!(!is_valid_cell_reference(""));
        assert!(!is_valid_cell_reference("1"));
        assert!(!is_valid_cell_reference("A"));
        assert!(!is_valid_cell_reference("A1B2"));
    }

    #[test]
    fn test_is_valid_number() {
        assert!(is_valid_number("123"));
        assert!(is_valid_number("-456.78"));
        assert!(is_valid_number("0"));
        assert!(is_valid_number("3.14159"));

        assert!(!is_valid_number("abc"));
        assert!(!is_valid_number("12a3"));
        assert!(!is_valid_number(""));
    }

    #[test]
    fn test_is_valid_date() {
        assert!(is_valid_date("2023-12-25"));
        assert!(is_valid_date("2024-01-01"));

        assert!(!is_valid_date("2023/12/25"));
        assert!(!is_valid_date("2023-12-25T00:00:00"));
        assert!(!is_valid_date("2023-13-01"));
        assert!(!is_valid_date("2023-12-32"));
        assert!(!is_valid_date("23-12-25"));
    }

    #[test]
    fn test_set_data_validation_request() {
        let validation = DataValidation::new("sheet1", "A1:A5", ValidationType::Dropdown)
            .dropdown_source(DropdownSource::Values(vec!["选项1".to_string()]));

        let request = SetDataValidationRequest::new("test_token", validation);
        assert_eq!(request.spreadsheet_token, "test_token");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_data_validation_request() {
        let validation = DataValidation::new("sheet1", "A1:A5", ValidationType::Dropdown)
            .dropdown_source(DropdownSource::Values(vec!["选项1".to_string()]));

        let request =
            UpdateDataValidationRequest::new("test_token", "sheet1", "validation_123", validation);
        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.sheet_id, "sheet1");
        assert_eq!(request.data_validation_id, "validation_123");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_data_validation_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = DataValidationService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_dropdown_validation_builder() {
        let config = openlark_core::config::Config::default();
        let builder = DataValidationService::new(config.clone())
            .dropdown_builder(
                "token".to_string(),
                "sheet1".to_string(),
                "A1:A10".to_string(),
            )
            .add_value("选项1")
            .add_value("选项2")
            .add_values(vec!["选项3", "选项4"])
            .prompt_message("请选择")
            .error_message("无效选择")
            .allow_empty(false)
            .strict_mode(true);

        assert_eq!(builder.spreadsheet_token, "token");
        assert_eq!(builder.sheet_id, "sheet1");
        assert_eq!(builder.range, "A1:A10");
        assert_eq!(builder.values.len(), 4);
        assert_eq!(builder.allow_empty, false);
        assert_eq!(builder.strict_mode, true);
        assert!(!format!("{:?}", builder).is_empty());
    }

    #[test]
    fn test_number_range_validation_builder() {
        let config = openlark_core::config::Config::default();
        let builder = DataValidationService::new(config.clone())
            .number_range_builder(
                "token".to_string(),
                "sheet1".to_string(),
                "B1:B10".to_string(),
            )
            .operator(ValidationOperator::Between)
            .values("18", "60")
            .prompt_message("请输入年龄")
            .error_message("年龄必须在18-60之间");

        assert_eq!(builder.spreadsheet_token, "token");
        assert_eq!(builder.sheet_id, "sheet1");
        assert_eq!(builder.range, "B1:B10");
        assert_eq!(builder.operator, ValidationOperator::Between);
        assert_eq!(builder.value1, Some("18".to_string()));
        assert_eq!(builder.value2, Some("60".to_string()));
        assert!(!format!("{:?}", builder).is_empty());
    }

    #[test]
    fn test_response_default() {
        let set_response = SetDataValidationResponse::default();
        assert!(set_response.data.spreadsheet_token.is_empty());
        assert!(set_response.data.data_validation_id.is_empty());
        assert_eq!(set_response.data.status, "success");

        let update_response = UpdateDataValidationResponse::default();
        assert!(update_response.data.spreadsheet_token.is_empty());
        assert!(update_response.data.data_validation_id.is_empty());
        assert_eq!(update_response.data.status, "success");
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            SetDataValidationResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            UpdateDataValidationResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_validation = DataValidation::new("sheet1", "A1:B2", ValidationType::Dropdown)
            .dropdown_source(DropdownSource::Values(vec![
                "男".to_string(),
                "女".to_string(),
            ]))
            .prompt_message("请选择性别")
            .allow_empty(false);

        let serialized = serde_json::to_string(&original_validation).unwrap();
        let deserialized: DataValidation = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_validation.sheet_id, deserialized.sheet_id);
        assert_eq!(original_validation.range, deserialized.range);
        assert_eq!(
            original_validation.validation_type,
            deserialized.validation_type
        );
        assert_eq!(original_validation.allow_empty, deserialized.allow_empty);
    }

    #[test]
    fn test_complex_validation_scenarios() {
        // 测试复杂的文本长度验证
        let text_length_validation =
            DataValidation::new("sheet1", "C1:C100", ValidationType::TextLength)
                .condition(ValidationCondition::new(ValidationOperator::Between).values("5", "50"))
                .prompt_message("文本长度必须在5-50个字符之间")
                .error_message("输入的文本长度不符合要求")
                .allow_empty(false);

        assert!(text_length_validation.validate().is_ok());
        assert_eq!(text_length_validation.range, "C1:C100");

        // 测试日期范围验证
        let date_validation = DataValidation::new("sheet1", "D1:D50", ValidationType::DateRange)
            .condition(
                ValidationCondition::new(ValidationOperator::Between)
                    .values("2024-01-01", "2024-12-31"),
            )
            .prompt_message("请选择2024年内的日期");

        assert!(date_validation.validate().is_ok());

        // 测试多选下拉列表
        let multi_select_validation =
            DataValidation::new("sheet1", "E1:E20", ValidationType::MultiSelectDropdown)
                .dropdown_source(DropdownSource::Values(vec![
                    "选项A".to_string(),
                    "选项B".to_string(),
                    "选项C".to_string(),
                    "选项D".to_string(),
                    "选项E".to_string(),
                ]))
                .prompt_message("可选择多个选项")
                .allow_empty(true)
                .strict_mode(false);

        assert!(multi_select_validation.validate().is_ok());
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界值 - 单个值的下拉列表
        let single_value = DataValidation::new("sheet1", "A1", ValidationType::Dropdown)
            .dropdown_source(DropdownSource::Values(vec!["唯一选项".to_string()]));
        assert!(single_value.validate().is_ok());

        // 测试边界值 - 1000个值的下拉列表
        let mut large_values = vec![];
        for i in 1..=1000 {
            large_values.push(format!("选项{}", i));
        }
        let large_dropdown = DataValidation::new("sheet1", "A1:A1000", ValidationType::Dropdown)
            .dropdown_source(DropdownSource::Values(large_values));
        assert!(large_dropdown.validate().is_ok());

        // 测试过多个值（应该失败）
        let mut too_many_values = vec![];
        for i in 1..=1001 {
            too_many_values.push(format!("选项{}", i));
        }
        let too_many_dropdown = DataValidation::new("sheet1", "A1:A1001", ValidationType::Dropdown)
            .dropdown_source(DropdownSource::Values(too_many_values));
        assert!(too_many_dropdown.validate().is_err());

        // 测试复杂的单元格引用格式
        let complex_range = DataValidation::new("sheet1", "AA100:ZZ999", ValidationType::Dropdown)
            .dropdown_source(DropdownSource::Range("Sheet2!A1:A10".to_string()));
        assert!(complex_range.validate().is_ok());
    }
}

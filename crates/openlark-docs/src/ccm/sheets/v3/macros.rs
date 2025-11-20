//! Sheets电子表格宏服务 v3
//!
//! 提供飞书电子表格v3版本的宏管理功能，包括：
//! - 创建和执行宏
//! - 宏脚本管理和配置
//! - 宏权限控制和安全设置
//! - 宏调试和日志记录
use serde_json::Value;

use openlark_core::error::LarkAPIError;

use serde::{Deserialize, Serialize};

// v3模块核心类型定义
pub type SpreadsheetToken = String;
pub type SheetId = String;
pub type CellValue = serde_json::Value;
pub type SheetPagedResponse<T> = Vec<T>;

/// 宏类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MacroType {
    /// VBA宏
    #[serde(rename = "VBA")]
    VBA,
    /// JavaScript宏
    #[serde(rename = "JAVASCRIPT")]
    JavaScript,
    /// 公式宏
    #[serde(rename = "FORMULA")]
    Formula,
}

/// 宏执行状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MacroStatus {
    /// 等待中
    #[serde(rename = "PENDING")]
    Pending,
    /// 执行中
    #[serde(rename = "RUNNING")]
    Running,
    /// 已完成
    #[serde(rename = "COMPLETED")]
    Completed,
    /// 失败
    #[serde(rename = "FAILED")]
    Failed,
    /// 已取消
    #[serde(rename = "CANCELLED")]
    Cancelled,
}

/// 宏权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroPermissions {
    /// 是否允许读写文件
    #[serde(rename = "allow_file_access")]
    pub allow_file_access: Option<bool>,
    /// 是否允许网络访问
    #[serde(rename = "allow_network_access")]
    pub allow_network_access: Option<bool>,
    /// 是否允许系统调用
    #[serde(rename = "allow_system_calls")]
    pub allow_system_calls: Option<bool>,
    /// 允许访问的工作表ID列表
    #[serde(rename = "allowed_sheets")]
    pub allowed_sheets: Option<Vec<String>>,
    /// 禁止的操作类型
    #[serde(rename = "forbidden_operations")]
    pub forbidden_operations: Option<Vec<String>>,
}

impl MacroPermissions {
    /// 创建默认权限
    pub fn new() -> Self {
        Self {
            allow_file_access: Some(false),
            allow_network_access: Some(false),
            allow_system_calls: Some(false),
            allowed_sheets: None,
            forbidden_operations: None,
        }
    }

    /// 设置文件访问权限
    pub fn allow_file_access(mut self, allow: bool) -> Self {
        self.allow_file_access = Some(allow);
        self
    }

    /// 设置网络访问权限
    pub fn allow_network_access(mut self, allow: bool) -> Self {
        self.allow_network_access = Some(allow);
        self
    }

    /// 设置系统调用权限
    pub fn allow_system_calls(mut self, allow: bool) -> Self {
        self.allow_system_calls = Some(allow);
        self
    }

    /// 设置允许的工作表
    pub fn allowed_sheets(mut self, sheets: Vec<String>) -> Self {
        self.allowed_sheets = Some(sheets);
        self
    }

    /// 设置禁止的操作
    pub fn forbidden_operations(mut self, operations: Vec<String>) -> Self {
        self.forbidden_operations = Some(operations);
        self
    }
}

/// 宏参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroParameter {
    /// 参数名称
    #[serde(rename = "name")]
    pub name: String,
    /// 参数值
    #[serde(rename = "value")]
    pub value: serde_json::Value,
    /// 参数类型
    #[serde(rename = "type")]
    pub param_type: String,
    /// 是否必需
    #[serde(rename = "required")]
    pub required: bool,
}

impl MacroParameter {
    /// 创建字符串参数
    pub fn string(name: String, value: String, required: bool) -> Self {
        Self {
            name,
            value: CellValue::String(value),
            param_type: "string".to_string(),
            required,
        }
    }

    /// 创建数字参数
    pub fn number(name: String, value: f64, required: bool) -> Self {
        Self {
            name,
            value: CellValue::Number(serde_json::Number::from_f64(value).unwrap()),
            param_type: "number".to_string(),
            required,
        }
    }

    /// 创建布尔参数
    pub fn boolean(name: String, value: bool, required: bool) -> Self {
        Self {
            name,
            value: CellValue::Bool(value),
            param_type: "boolean".to_string(),
            required,
        }
    }

    /// 创建数组参数
    pub fn array(name: String, value: Vec<serde_json::Value>, required: bool) -> Self {
        Self {
            name,
            value: CellValue::Array(value),
            param_type: "array".to_string(),
            required,
        }
    }
}

/// 宏脚本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroScript {
    /// 宏名称
    #[serde(rename = "name")]
    pub name: String,
    /// 宏类型
    #[serde(rename = "macro_type")]
    pub macro_type: MacroType,
    /// 脚本内容
    #[serde(rename = "script")]
    pub script: String,
    /// 宏描述
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// 宏作者
    #[serde(rename = "author")]
    pub author: Option<String>,
    /// 创建时间
    #[serde(rename = "created_time")]
    pub created_time: Option<String>,
    /// 修改时间
    #[serde(rename = "modified_time")]
    pub modified_time: Option<String>,
    /// 宏权限
    #[serde(rename = "permissions")]
    pub permissions: Option<MacroPermissions>,
}

impl MacroScript {
    /// 创建宏脚本
    pub fn new(name: String, macro_type: MacroType, script: String) -> Self {
        Self {
            name,
            macro_type,
            script,
            description: None,
            author: None,
            created_time: None,
            modified_time: None,
            permissions: None,
        }
    }

    /// 设置描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置作者
    pub fn author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// 设置权限
    pub fn permissions(mut self, permissions: MacroPermissions) -> Self {
        self.permissions = Some(permissions);
        self
    }

    /// 验证宏脚本
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.name.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "宏名称不能为空".to_string(),
            ));
        }

        if self.script.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "脚本内容不能为空".to_string(),
            ));
        }

        // 验证脚本长度
        if self.script.len() > 1024 * 1024 {
            return Err(LarkAPIError::IllegalParamError(
                "脚本内容过长，最大支持1MB".to_string(),
            ));
        }

        Ok(())
    }
}

/// 执行宏请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteMacroRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 宏名称
    #[serde(rename = "macro_name")]
    pub macro_name: String,
    /// 宏参数
    #[serde(rename = "parameters")]
    pub parameters: Vec<MacroParameter>,
    /// 是否异步执行
    #[serde(rename = "async")]
    pub async_execution: Option<bool>,
}

impl ExecuteMacroRequest {
    /// 创建执行宏请求
    pub fn new(
        spreadsheet_token: String,
        sheet_id: String,
        macro_name: String,
        parameters: Vec<MacroParameter>,
    ) -> Self {
        Self {
            spreadsheet_token,
            sheet_id,
            macro_name,
            parameters,
            async_execution: Some(false),
        }
    }

    /// 设置异步执行
    pub fn async_execution(mut self, async_execution: bool) -> Self {
        self.async_execution = Some(async_execution);
        self
    }

    /// 构建器模式
    pub fn builder() -> ExecuteMacroRequestBuilder {
        ExecuteMacroRequestBuilder::default()
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

        if self.macro_name.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "宏名称不能为空".to_string(),
            ));
        }

        // 验证必需参数
        for param in &self.parameters {
            if param.required && param.value.is_null() {
                return Err(LarkAPIError::IllegalParamError(format!(
                    "必需参数 {} 的值不能为空",
                    param.name
                )));
            }
        }

        Ok(())
    }
}

/// 执行宏请求构建器
#[derive(Debug, Default)]
pub struct ExecuteMacroRequestBuilder {
    spreadsheet_token: Option<String>,
    sheet_id: Option<String>,
    macro_name: Option<String>,
    parameters: Vec<MacroParameter>,
    async_execution: Option<bool>,
}

impl ExecuteMacroRequestBuilder {
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

    /// 设置宏名称
    pub fn macro_name(mut self, macro_name: String) -> Self {
        self.macro_name = Some(macro_name);
        self
    }

    /// 添加参数
    pub fn add_parameter(mut self, parameter: MacroParameter) -> Self {
        self.parameters.push(parameter);
        self
    }

    /// 批量添加参数
    pub fn parameters(mut self, parameters: Vec<MacroParameter>) -> Self {
        self.parameters.extend(parameters);
        self
    }

    /// 设置异步执行
    pub fn async_execution(mut self, async_execution: bool) -> Self {
        self.async_execution = Some(async_execution);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<ExecuteMacroRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("工作表ID不能为空".to_string()))?;

        let macro_name = self
            .macro_name
            .ok_or_else(|| LarkAPIError::IllegalParamError("宏名称不能为空".to_string()))?;

        let mut request =
            ExecuteMacroRequest::new(spreadsheet_token, sheet_id, macro_name, self.parameters);

        if let Some(async_execution) = self.async_execution {
            request = request.async_execution(async_execution);
        }

        request.validate()?;
        Ok(request)
    }
}

/// 执行宏响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteMacroResponse {
    /// 执行ID
    #[serde(rename = "execution_id")]
    pub execution_id: String,
    /// 执行状态
    #[serde(rename = "status")]
    pub status: MacroStatus,
    /// 执行结果
    #[serde(rename = "result")]
    pub result: Option<serde_json::Value>,
    /// 错误信息
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// 开始时间
    #[serde(rename = "start_time")]
    pub start_time: String,
    /// 结束时间
    #[serde(rename = "end_time")]
    pub end_time: Option<String>,
    /// 执行日志
    #[serde(rename = "logs")]
    pub logs: Option<Vec<String>>,
}

impl openlark_core::api::ApiResponseTrait for ExecuteMacroResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// 创建宏脚本请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMacroRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 宏脚本
    #[serde(rename = "macro_script")]
    pub macro_script: MacroScript,
}

impl CreateMacroRequest {
    /// 创建创建宏脚本请求
    pub fn new(spreadsheet_token: String, sheet_id: String, macro_script: MacroScript) -> Self {
        Self {
            spreadsheet_token,
            sheet_id,
            macro_script,
        }
    }

    /// 构建器模式
    pub fn builder() -> CreateMacroRequestBuilder {
        CreateMacroRequestBuilder::default()
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

        self.macro_script.validate()?;
        Ok(())
    }
}

/// 创建宏脚本请求构建器
#[derive(Debug, Default)]
pub struct CreateMacroRequestBuilder {
    spreadsheet_token: Option<String>,
    sheet_id: Option<String>,
    macro_script: Option<MacroScript>,
}

impl CreateMacroRequestBuilder {
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

    /// 设置宏脚本
    pub fn macro_script(mut self, macro_script: MacroScript) -> Self {
        self.macro_script = Some(macro_script);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<CreateMacroRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("工作表ID不能为空".to_string()))?;

        let macro_script = self
            .macro_script
            .ok_or_else(|| LarkAPIError::IllegalParamError("宏脚本不能为空".to_string()))?;

        let request = CreateMacroRequest::new(spreadsheet_token, sheet_id, macro_script);

        request.validate()?;
        Ok(request)
    }
}

/// 创建宏脚本响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMacroResponse {
    /// 宏ID
    #[serde(rename = "macro_id")]
    pub macro_id: String,
    /// 宏脚本
    #[serde(rename = "macro_script")]
    pub macro_script: MacroScript,
}

impl openlark_core::api::ApiResponseTrait for CreateMacroResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// 查询宏状态请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMacroStatusRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 执行ID
    #[serde(rename = "execution_id")]
    pub execution_id: String,
}

impl GetMacroStatusRequest {
    /// 创建查询宏状态请求
    pub fn new(spreadsheet_token: String, execution_id: String) -> Self {
        Self {
            spreadsheet_token,
            execution_id,
        }
    }

    /// 构建器模式
    pub fn builder() -> GetMacroStatusRequestBuilder {
        GetMacroStatusRequestBuilder::default()
    }

    /// 验证请求
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.spreadsheet_token.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "电子表格ID不能为空".to_string(),
            ));
        }

        if self.execution_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "执行ID不能为空".to_string(),
            ));
        }

        Ok(())
    }
}

/// 查询宏状态请求构建器
#[derive(Debug, Default)]
pub struct GetMacroStatusRequestBuilder {
    spreadsheet_token: Option<String>,
    execution_id: Option<String>,
}

impl GetMacroStatusRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置执行ID
    pub fn execution_id(mut self, execution_id: String) -> Self {
        self.execution_id = Some(execution_id);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<GetMacroStatusRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let execution_id = self
            .execution_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("执行ID不能为空".to_string()))?;

        let request = GetMacroStatusRequest::new(spreadsheet_token, execution_id);
        request.validate()?;
        Ok(request)
    }
}

/// 查询宏状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMacroStatusResponse {
    /// 执行信息
    #[serde(rename = "execution")]
    pub execution: ExecuteMacroResponse,
}

impl openlark_core::api::ApiResponseTrait for GetMacroStatusResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// Sheets电子表格宏服务 v3
#[derive(Clone, Debug)]
pub struct MacroService {
    config: openlark_core::config::Config,
}

impl MacroService {
    /// 创建宏服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 执行宏
    ///
    /// 在指定工作表中执行宏，支持同步和异步执行模式。
    ///
    /// # 参数
    /// - `request`: 执行宏请求
    ///
    /// # 返回
    /// 返回执行结果，包括执行ID、状态和结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::macros::*;
    ///
    /// // 准备宏参数
    /// let parameters = vec![
    ///     MacroParameter::string("input_range".to_string(), "A1:D10".to_string(), true),
    ///     MacroParameter::boolean("include_headers".to_string(), true, false),
    /// ];
    ///
    /// // 执行宏
    /// let request = ExecuteMacroRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .sheet_id("sheet_id".to_string())
    ///     .macro_name("FormatReport".to_string())
    ///     .parameters(parameters)
    ///     .async_execution(true)
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.execute_macro(&request).await?;
    /// println!("执行ID: {}", response.execution_id);
    /// ```
    pub async fn execute_macro(
        &self,
        request: &ExecuteMacroRequest,
    ) -> openlark_core::error::SDKResult<ExecuteMacroResponse> {
        use openlark_core::{api::ApiRequest, api::Response, http::Transport};

        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/macros/execute",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(openlark_core::api::HttpMethod::Post, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response: Response<ExecuteMacroResponse> =
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

    /// 创建宏脚本
    ///
    /// 在指定工作表中创建宏脚本，支持多种宏类型。
    ///
    /// # 参数
    /// - `request`: 创建宏脚本请求
    ///
    /// # 返回
    /// 返回创建后的宏脚本信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::macros::*;
    ///
    /// // 创建JavaScript宏脚本
    /// let script = MacroScript::new(
    ///     "DataFormatter".to_string(),
    ///     MacroType::JavaScript,
    ///     "function formatData(range) { return range.map(row => row.map(cell => cell.toFixed(2))); }".to_string(),
    /// ).description("格式化数据到2位小数".to_string())
    ///  .author("张三".to_string())
    ///  .permissions(MacroPermissions::new().allow_file_access(false));
    ///
    /// let request = CreateMacroRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .sheet_id("sheet_id".to_string())
    ///     .macro_script(script)
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.create_macro(&request).await?;
    /// ```
    pub async fn create_macro(
        &self,
        request: &CreateMacroRequest,
    ) -> openlark_core::error::SDKResult<CreateMacroResponse> {
        use openlark_core::{api::ApiRequest, api::Response, http::Transport};

        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/macros",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(openlark_core::api::HttpMethod::Post, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response: Response<CreateMacroResponse> =
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

    /// 查询宏状态
    ///
    /// 查询异步执行宏的状态和结果。
    ///
    /// # 参数
    /// - `request`: 查询宏状态请求
    ///
    /// # 返回
    /// 返回宏执行状态和结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::macros::*;
    ///
    /// let request = GetMacroStatusRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .execution_id("execution_id".to_string())
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.get_macro_status(&request).await?;
    /// println!("宏状态: {:?}", response.execution.status);
    /// ```
    pub async fn get_macro_status(
        &self,
        request: &GetMacroStatusRequest,
    ) -> openlark_core::error::SDKResult<GetMacroStatusResponse> {
        use openlark_core::{api::ApiRequest, api::Response, http::Transport};

        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/macros/status/{}",
            request.spreadsheet_token, request.execution_id
        );

        let api_request = ApiRequest::with_method_and_path(openlark_core::api::HttpMethod::Get, &endpoint);

        let response: Response<GetMacroStatusResponse> =
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

    /// 执行宏构建器
    pub fn execute_macro_builder(&self) -> ExecuteMacroRequestBuilder {
        ExecuteMacroRequestBuilder::default()
    }

    /// 创建宏构建器
    pub fn create_macro_builder(&self) -> CreateMacroRequestBuilder {
        CreateMacroRequestBuilder::default()
    }

    /// 查询宏状态构建器
    pub fn get_macro_status_builder(&self) -> GetMacroStatusRequestBuilder {
        GetMacroStatusRequestBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_permissions_creation() {
        let permissions = MacroPermissions::new()
            .allow_file_access(true)
            .allow_network_access(false)
            .allow_system_calls(false)
            .allowed_sheets(vec!["sheet1".to_string(), "sheet2".to_string()])
            .forbidden_operations(vec!["DELETE".to_string(), "FORMAT".to_string()]);

        assert_eq!(permissions.allow_file_access, Some(true));
        assert_eq!(permissions.allow_network_access, Some(false));
        assert_eq!(permissions.allow_system_calls, Some(false));
        assert_eq!(
            permissions.allowed_sheets,
            Some(vec!["sheet1".to_string(), "sheet2".to_string()])
        );
        assert_eq!(
            permissions.forbidden_operations,
            Some(vec!["DELETE".to_string(), "FORMAT".to_string()])
        );
    }

    #[test]
    fn test_macro_parameter_creation() {
        let string_param = MacroParameter::string("name".to_string(), "test".to_string(), true);
        assert_eq!(string_param.name, "name");
        assert_eq!(string_param.param_type, "string");
        assert_eq!(string_param.required, true);

        let number_param = MacroParameter::number("value".to_string(), 123.45, false);
        assert_eq!(number_param.name, "value");
        assert_eq!(number_param.param_type, "number");
        assert_eq!(number_param.required, false);

        let bool_param = MacroParameter::boolean("enabled".to_string(), true, true);
        assert_eq!(bool_param.name, "enabled");
        assert_eq!(bool_param.param_type, "boolean");
        assert_eq!(bool_param.required, true);

        let array_param = MacroParameter::array(
            "values".to_string(),
            vec![
                CellValue::String("a".to_string()),
                CellValue::String("b".to_string()),
            ],
            false,
        );
        assert_eq!(array_param.name, "values");
        assert_eq!(array_param.param_type, "array");
        assert_eq!(array_param.required, false);
    }

    #[test]
    fn test_macro_script_creation() {
        let permissions = MacroPermissions::new().allow_file_access(false);
        let script = MacroScript::new(
            "TestMacro".to_string(),
            MacroType::JavaScript,
            "function test() { return 'hello'; }".to_string(),
        )
        .description("测试宏".to_string())
        .author("张三".to_string())
        .permissions(permissions);

        assert_eq!(script.name, "TestMacro");
        assert_eq!(script.description, Some("测试宏".to_string()));
        assert_eq!(script.author, Some("张三".to_string()));
        assert!(script.permissions.is_some());
    }

    #[test]
    fn test_macro_script_validation() {
        // 测试有效脚本
        let valid_script = MacroScript::new(
            "TestMacro".to_string(),
            MacroType::JavaScript,
            "function test() { return 'hello'; }".to_string(),
        );
        assert!(valid_script.validate().is_ok());

        // 测试空名称
        let invalid_script1 = MacroScript::new(
            "".to_string(),
            MacroType::JavaScript,
            "function test() { return 'hello'; }".to_string(),
        );
        assert!(invalid_script1.validate().is_err());

        // 测试空脚本
        let invalid_script2 = MacroScript::new(
            "TestMacro".to_string(),
            MacroType::JavaScript,
            "".to_string(),
        );
        assert!(invalid_script2.validate().is_err());
    }

    #[test]
    fn test_execute_macro_request_creation() {
        let parameters = vec![
            MacroParameter::string("input".to_string(), "A1".to_string(), true),
            MacroParameter::boolean("enabled".to_string(), true, false),
        ];

        let request = ExecuteMacroRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "TestMacro".to_string(),
            parameters,
        )
        .async_execution(true);

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.macro_name, "TestMacro");
        assert_eq!(request.parameters.len(), 2);
        assert_eq!(request.async_execution, Some(true));
    }

    #[test]
    fn test_execute_macro_request_validation() {
        // 测试有效请求
        let valid_request = ExecuteMacroRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "TestMacro".to_string(),
            vec![],
        );
        assert!(valid_request.validate().is_ok());

        // 测试空电子表格ID
        let invalid_request1 = ExecuteMacroRequest::new(
            "".to_string(),
            "sheet123".to_string(),
            "TestMacro".to_string(),
            vec![],
        );
        assert!(invalid_request1.validate().is_err());

        // 测试空宏名称
        let invalid_request2 = ExecuteMacroRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "".to_string(),
            vec![],
        );
        assert!(invalid_request2.validate().is_err());

        // 测试必需参数为空
        let invalid_request3 = ExecuteMacroRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            "TestMacro".to_string(),
            vec![MacroParameter::string(
                "required_param".to_string(),
                "".to_string(),
                true,
            )],
        );
        assert!(invalid_request3.validate().is_err());
    }

    #[test]
    fn test_execute_macro_request_builder() {
        let request = ExecuteMacroRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .macro_name("ProcessData".to_string())
            .add_parameter(MacroParameter::string(
                "source".to_string(),
                "A1:D10".to_string(),
                true,
            ))
            .add_parameter(MacroParameter::boolean(
                "include_headers".to_string(),
                true,
                false,
            ))
            .async_execution(true)
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.macro_name, "ProcessData");
        assert_eq!(request.parameters.len(), 2);
        assert_eq!(request.async_execution, Some(true));
    }

    #[test]
    fn test_create_macro_request_creation() {
        let script = MacroScript::new(
            "TestMacro".to_string(),
            MacroType::JavaScript,
            "function test() { return 'hello'; }".to_string(),
        );

        let request =
            CreateMacroRequest::new("token123".to_string(), "sheet123".to_string(), script);

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.macro_script.name, "TestMacro");
    }

    #[test]
    fn test_create_macro_request_builder() {
        let script = MacroScript::new(
            "DataProcessor".to_string(),
            MacroType::VBA,
            "Sub ProcessData()\n    ' Process data here\nEnd Sub".to_string(),
        )
        .description("数据处理宏".to_string());

        let request = CreateMacroRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .macro_script(script)
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.macro_script.name, "DataProcessor");
        assert_eq!(request.macro_script.macro_type, MacroType::VBA);
    }

    #[test]
    fn test_get_macro_status_request() {
        let request =
            GetMacroStatusRequest::new("token123".to_string(), "execution123".to_string());

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.execution_id, "execution123");
    }

    #[test]
    fn test_get_macro_status_request_builder() {
        let request = GetMacroStatusRequest::builder()
            .spreadsheet_token("token123".to_string())
            .execution_id("execution123".to_string())
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.execution_id, "execution123");
    }

    #[test]
    fn test_macro_service_creation() {
        let config = openlark_core::config::openlark_core::config::Config::default();
        let service = MacroService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_macro_types() {
        let macro_types = vec![MacroType::VBA, MacroType::JavaScript, MacroType::Formula];

        for macro_type in macro_types {
            let script = MacroScript::new("Test".to_string(), macro_type, "test".to_string());
            assert_eq!(script.name, "Test");
        }
    }

    #[test]
    fn test_macro_statuses() {
        let statuses = vec![
            MacroStatus::Pending,
            MacroStatus::Running,
            MacroStatus::Completed,
            MacroStatus::Failed,
            MacroStatus::Cancelled,
        ];

        for status in statuses {
            // 验证可以序列化
            let _ = format!("{:?}", status);
        }
    }

    #[test]
    fn test_comprehensive_macro_scenarios() {
        // 测试VBA宏场景
        let vba_script = MacroScript::new(
            "ExcelExporter".to_string(),
            MacroType::VBA,
            "Sub ExportToExcel()\n    ' Export logic here\nEnd Sub".to_string(),
        )
        .description("导出为Excel格式".to_string())
        .author("Excel专家".to_string());

        let vba_request = CreateMacroRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .macro_script(vba_script)
            .build()
            .unwrap();

        assert_eq!(vba_request.macro_script.macro_type, MacroType::VBA);

        // 测试JavaScript宏场景
        let js_script = MacroScript::new(
            "DataValidator".to_string(),
            MacroType::JavaScript,
            "function validate(data) { return data.filter(item => item !== null && item !== undefined); }".to_string(),
        ).permissions(MacroPermissions::new()
            .allow_network_access(true)
            .forbidden_operations(vec!["DELETE".to_string()]));

        let js_request = ExecuteMacroRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .macro_name("DataValidator".to_string())
            .add_parameter(MacroParameter::array("data".to_string(), vec![], false))
            .async_execution(false)
            .build()
            .unwrap();

        assert_eq!(js_request.macro_name, "DataValidator");

        // 测试公式宏场景
        let formula_script = MacroScript::new(
            "Calculator".to_string(),
            MacroType::Formula,
            "SUM(A1:A10)".to_string(),
        )
        .description("计算A1到A10的总和".to_string());

        let formula_request = ExecuteMacroRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .macro_name("Calculator".to_string())
            .parameters(vec![])
            .async_execution(true)
            .build()
            .unwrap();

        assert_eq!(formula_request.macro_name, "Calculator");
        assert_eq!(formula_request.async_execution, Some(true));
    }

    #[test]
    fn test_macro_serialization() {
        use serde_json;

        let script = MacroScript::new(
            "TestMacro".to_string(),
            MacroType::JavaScript,
            "function test() { return 'hello'; }".to_string(),
        );
        let json = serde_json::to_string(&script).unwrap();
        assert!(json.contains("JAVASCRIPT"));
        assert!(json.contains("TestMacro"));
    }
}

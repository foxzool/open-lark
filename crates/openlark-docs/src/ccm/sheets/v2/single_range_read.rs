//! 单个范围读取服务
//!
//! 提供飞书电子表格v2版本的单个范围数据读取功能，包括：
//! - 读取指定范围的单元格数据
//! - 支持多种数据格式和渲染选项
//! - 提供灵活的范围查询和过滤功能
//! - 高效的数据获取和解析
use serde_json::Value;

use serde::{Deserialize, Serialize};

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints_original::Endpoints,
    error::LarkAPIError,
    http::Transport,
    standard_response::StandardResponse,
    trait_system::Service,
    SDKResult,
};

/// 值范围响应
///
/// 表示从电子表格中读取的单个范围的数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueRange {
    /// 主要维度（"ROWS" 或 "COLUMNS"）
    #[serde(rename = "majorDimension")]
    pub major_dimension: String,
    /// 范围标识符
    pub range: String,
    /// 范围内的值
    pub values: serde_json::Value,
    /// 表格版本号
    pub revision: i32,
}

impl Default for ValueRange {
    fn default() -> Self {
        Self {
            major_dimension: "ROWS".to_string(),
            range: String::new(),
            values: Value::Array(vec![]),
            revision: 0,
        }
    }
}

/// 单个范围读取请求
///
/// 用于读取电子表格中指定范围的单元格数据。
/// 根据API描述：根据 spreadsheetToken 和 range 读取表格单个范围的值，返回数据限制为10M。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SingleRangeReadRequest {
    /// 电子表格令牌
    pub spreadsheet_token: String,
    /// 要读取的范围
    /// 支持格式：Sheet1!A1:B2, A1:C10
    pub range: String,
    /// 值渲染选项（可选）
    /// - "ToString": 返回纯文本的值（数值类型除外）
    /// - "FormattedValue": 计算并格式化单元格
    /// - "Formula": 单元格中含有公式时，返回公式本身
    /// - "UnformattedValue": 计算但不对单元格进行格式化
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_render_option: Option<String>,
    /// 日期时间渲染选项（可选）
    /// - "FormattedString": 计算并对时间、日期类型数据进行格式化
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_render_option: Option<String>,
    /// 用户ID类型（可选）
    /// - "open_id": 开放ID（默认）
    /// - "user_id": 用户ID
    /// - "union_id": 联合ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleRangeReadResponse {
    /// 读取的范围数据
    pub value_range: ValueRange,
}

/// 响应体包装结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleRangeReadResponseBody {
    pub data: SingleRangeReadResponse,
}

// 实现API响应特征
impl ApiResponseTrait for SingleRangeReadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for SingleRangeReadResponseBody {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SingleRangeReadRequest {
    /// 创建新的单个范围读取请求
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格令牌
    /// - `range`: 要读取的范围
    ///
    /// # 示例
    ///
    /// ```rust
    /// let request = SingleRangeReadRequest::new(
    ///     "shtcnmBA*****yGehy8",
    ///     "Sheet1!A1:B2"
    /// );
    /// ```
    pub fn new<T: Into<String>, U: Into<String>>(spreadsheet_token: T, range: U) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            range: range.into(),
            value_render_option: None,
            date_time_render_option: None,
            user_id_type: None,
        }
    }

    /// 设置值渲染选项
    ///
    /// # 参数
    /// - `option`: 渲染选项
    ///
    /// # 选项说明
    /// - "ToString": 返回纯文本的值（数值类型除外）
    /// - "FormattedValue": 计算并格式化单元格
    /// - "Formula": 单元格中含有公式时，返回公式本身
    /// - "UnformattedValue": 计算但不对单元格进行格式化
    pub fn value_render_option<T: Into<String>>(mut self, option: T) -> Self {
        self.value_render_option = Some(option.into());
        self
    }

    /// 设置日期时间渲染选项
    ///
    /// # 参数
    /// - `option`: 渲染选项
    ///
    /// # 选项说明
    /// - "FormattedString": 计算并对时间、日期类型数据进行格式化
    pub fn date_time_render_option<T: Into<String>>(mut self, option: T) -> Self {
        self.date_time_render_option = Some(option.into());
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 选项说明
    /// - "open_id": 开放ID（默认）
    /// - "user_id": 用户ID
    /// - "union_id": 联合ID
    pub fn user_id_type<T: Into<String>>(mut self, user_id_type: T) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数是否有效
    pub fn validate(&self) -> SDKResult<()> {
        // 验证电子表格令牌
        if self.spreadsheet_token.trim().is_empty() {
            return Err(LarkAPIError::InvalidParameter(
                "电子表格令牌不能为空".to_string(),
            ));
        }

        // 验证范围参数
        if self.range.trim().is_empty() {
            return Err(LarkAPIError::InvalidParameter("范围不能为空".to_string()));
        }

        // 验证范围格式
        let range = self.range.trim();

        // 基本范围格式验证
        if range.contains('!') {
            // 包含工作表名的格式：Sheet1!A1:B2
            let parts: Vec<&str> = range.split('!').collect();
            if parts.len() != 2 {
                return Err(LarkAPIError::InvalidParameter(format!(
                    "无效的范围格式: {}，工作表名和单元格引用之间应该用!分隔",
                    range
                )));
            }

            // 验证单元格引用格式
            if parts[1].trim().is_empty() {
                return Err(LarkAPIError::InvalidParameter(
                    "单元格引用不能为空".to_string(),
                ));
            }
        } else {
            // 仅单元格引用的格式：A1:B2
            if range.is_empty() {
                return Err(LarkAPIError::InvalidParameter(
                    "单元格引用不能为空".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// 构建查询参数
    pub fn build_query_params(&self) -> String {
        let mut params = vec![];

        if let Some(option) = &self.value_render_option {
            params.push(format!("valueRenderOption={}", urlencoding::encode(option)));
        }

        if let Some(option) = &self.date_time_render_option {
            params.push(format!(
                "dateTimeRenderOption={}",
                urlencoding::encode(option)
            ));
        }

        if let Some(user_id_type) = &self.user_id_type {
            params.push(format!("userIdType={}", urlencoding::encode(user_id_type)));
        }

        params.join("&")
    }
}

/// 单个范围读取服务
#[derive(Clone, Debug)]
pub struct SingleRangeReadService {
    config: Config,
}

impl SingleRangeReadService {
    /// 创建单个范围读取服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 读取单个范围数据
    ///
    /// 根据spreadsheetToken和range读取表格单个范围的值，返回数据限制为10M。
    ///
    /// # 参数
    /// - `request`: 单个范围读取请求
    ///
    /// # 返回
    /// 范围数据响应
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::single_range_read::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = SingleRangeReadService::new(config);
    ///
    /// // 创建读取请求
    /// let request = SingleRangeReadRequest::new("spreadsheet_token", "Sheet1!A1:B2")
    ///     .value_render_option("FormattedValue")
    ///     .date_time_render_option("FormattedString");
    ///
    /// let response = service.read_range(request).await?;
    ///
    /// println!("范围: {}", response.value_range.range);
    /// println!("值: {:?}", response.value_range.values);
    /// ```
    pub async fn read_range(
        &self,
        request: SingleRangeReadRequest,
    ) -> SDKResult<Response<SingleRangeReadResponse>> {
        // 验证请求参数
        request.validate()?;

        // 构建API请求
        let mut api_req = ApiRequest::with_method(openlark_core::api::HttpMethod::Get);
        api_req.set_api_path(
            Endpoints::SHEETS_V2_SPREADSHEET_VALUES_GET
                .replace("{spreadsheet_token}", &request.spreadsheet_token)
                .replace("{range}", &request.range),
        );
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 添加查询参数
        let query_params = request.build_query_params();
        if !query_params.is_empty() {
            for param in query_params.split('&') {
                if let Some((key, value)) = param.split_once('=') {
                    api_req.query_params.insert(key, value);
                }
            }
        }

        // 发送请求
        Transport::<SingleRangeReadResponse>::request(api_req, &self.config, None).await
    }

    /// 创建单个范围读取构建器
    pub fn read_range_builder(
        &self,
        spreadsheet_token: impl Into<String>,
    ) -> SingleRangeReadBuilder {
        SingleRangeReadBuilder::new(self.clone() spreadsheet_token)
    }

    /// 快速读取范围数据（使用默认选项）
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格令牌
    /// - `range`: 要读取的范围
    ///
    /// # 返回
    /// 范围数据响应
    pub async fn read_range_simple(
        &self,
        spreadsheet_token: impl Into<String>,
        range: impl Into<String>,
    ) -> SDKResult<Response<SingleRangeReadResponse>> {
        let request = SingleRangeReadRequest::new(spreadsheet_token, range);
        self.read_range(request).await
    }
}

/// 单个范围读取构建器
pub struct SingleRangeReadBuilder {
    service: SingleRangeReadService,
    spreadsheet_token: String,
    value_render_option: Option<String>,
    date_time_render_option: Option<String>,
    user_id_type: Option<String>,
}

impl SingleRangeReadBuilder {
    /// 创建新的构建器
    pub fn new(service: SingleRangeReadService, spreadsheet_token: impl Into<String>) -> Self {
        Self {
            service,
            spreadsheet_token: spreadsheet_token.into(),
            value_render_option: None,
            date_time_render_option: None,
            user_id_type: None,
        }
    }

    /// 设置值渲染选项
    pub fn value_render_option(mut self, option: impl Into<String>) -> Self {
        self.value_render_option = Some(option.into());
        self
    }

    /// 设置日期时间渲染选项
    pub fn date_time_render_option(mut self, option: impl Into<String>) -> Self {
        self.date_time_render_option = Some(option.into());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行读取操作
    pub async fn execute(
        self,
        range: impl Into<String>,
    ) -> SDKResult<Response<SingleRangeReadResponse>> {
        let mut request = SingleRangeReadRequest::new(self.spreadsheet_token, range);

        if let Some(option) = self.value_render_option {
            request = request.value_render_option(option);
        }

        if let Some(option) = self.date_time_render_option {
            request = request.date_time_render_option(option);
        }

        if let Some(user_id_type) = self.user_id_type {
            request = request.user_id_type(user_id_type);
        }

        self.service.read_range(request).await
    }
}

impl Service for SingleRangeReadService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SingleRangeReadService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_range_read_request_validation() {
        // 测试空令牌
        let request = SingleRangeReadRequest::new("", "Sheet1!A1:B2");
        assert!(request.validate().is_err());

        // 测试空范围
        let request = SingleRangeReadRequest::new("token", "");
        assert!(request.validate().is_err());

        // 测试正常请求
        let request = SingleRangeReadRequest::new("shtcnmBRWQKbsJRHXXXXXXXXXX", "Sheet1!A1:B2");
        assert!(request.validate().is_ok());

        // 测试无效范围格式
        let request = SingleRangeReadRequest::new("token", "Sheet1!!A1");
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_build_query_params() {
        let request = SingleRangeReadRequest::new("token", "Sheet1!A1:B2")
            .value_render_option("FormattedValue")
            .date_time_render_option("FormattedString")
            .user_id_type("open_id");

        let params = request.build_query_params();
        assert!(params.contains("valueRenderOption=FormattedValue"));
        assert!(params.contains("dateTimeRenderOption=FormattedString"));
        assert!(params.contains("userIdType=open_id"));
    }

    #[test]
    fn test_builder_pattern() {
        let config = openlark_core::config::Config::default();
        let service = SingleRangeReadService::new(config);

        let builder = service
            .read_range_builder("test_token")
            .value_render_option("FormattedValue")
            .date_time_render_option("FormattedString");

        // 验证构建器设置
        assert_eq!(builder.spreadsheet_token, "test_token");
        assert_eq!(
            builder.value_render_option,
            Some("FormattedValue".to_string())
        );
        assert_eq!(
            builder.date_time_render_option,
            Some("FormattedString".to_string())
        );
    }

    #[test]
    fn test_single_range_read_service() {
        let config = openlark_core::config::Config::default();
        let service = SingleRangeReadService::new(config);

        assert_eq!(service.service_name(), "SingleRangeReadService");
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_value_range_structure() {
        let value_range = ValueRange {
            major_dimension: "ROWS".to_string(),
            range: "Sheet1!A1:B2".to_string(),
            values: serde_json::json!([["姓名", "年龄"], ["张三", 25]]),
            revision: 1,
        };

        assert_eq!(value_range.major_dimension, "ROWS");
        assert_eq!(value_range.range, "Sheet1!A1:B2");
        assert_eq!(value_range.revision, 1);
    }
}

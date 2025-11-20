//! 多个范围读取服务
//!
//! 提供飞书电子表格v2版本的多范围数据读取功能，包括：
//! - 批量读取指定范围的单元格数据
//! - 支持多种数据格式和渲染选项
//! - 提供灵活的范围查询和过滤功能
//! - 高效的批量数据获取和解析
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

/// 值范围响应（批量读取中的单个范围）
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

/// 多个范围读取请求
///
/// 用于批量读取电子表格中多个范围的单元格数据。
/// 根据API描述：根据 spreadsheetToken 和 ranges 读取表格多个范围的值，返回数据限制为10M。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchRangeReadRequest {
    /// 电子表格令牌
    pub spreadsheet_token: String,
    /// 要读取的范围列表
    /// 支持格式：Sheet1!A1:B2, A1:C10
    /// 使用JSON数组格式传递
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<String>>,
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
pub struct BatchRangeReadResponse {
    /// 读取的范围数据列表
    pub value_ranges: Vec<ValueRange>,
    /// 表格版本号
    pub revision: i32,
    /// 是否已完全读取
    pub read_all: bool,
}

/// 响应体包装结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRangeReadResponseBody {
    pub data: BatchRangeReadResponse,
}

// 实现API响应特征
impl ApiResponseTrait for BatchRangeReadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for BatchRangeReadResponseBody {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchRangeReadRequest {
    /// 创建新的多个范围读取请求
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格令牌
    /// - `ranges`: 要读取的范围列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// let request = BatchRangeReadRequest::new(
    ///     "shtcnmBA*****yGehy8",
    ///     vec!["Sheet1!A1:B2", "Sheet1!D1:E10"]
    /// );
    /// ```
    pub fn new<T: Into<String>, R: Into<String>>(spreadsheet_token: T, ranges: Vec<R>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            ranges: Some(ranges.into_iter().map(|r| r.into()).collect()),
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

    /// 添加范围
    ///
    /// # 参数
    /// - `range`: 要添加的范围
    pub fn add_range<T: Into<String>>(mut self, range: T) -> Self {
        if let Some(ref mut ranges) = self.ranges {
            ranges.push(range.into());
        } else {
            self.ranges = Some(vec![range.into()]);
        }
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
        if let Some(ranges) = &self.ranges {
            if ranges.is_empty() {
                return Err(LarkAPIError::InvalidParameter(
                    "至少需要一个范围".to_string(),
                ));
            }

            if ranges.len() > 10 {
                return Err(LarkAPIError::InvalidParameter(
                    "范围数量不能超过10个".to_string(),
                ));
            }

            // 验证每个范围的格式
            for (index, range) in ranges.iter().enumerate() {
                let range = range.trim();
                if range.is_empty() {
                    return Err(LarkAPIError::InvalidParameter(format!(
                        "范围{}不能为空",
                        index + 1
                    )));
                }

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
                        return Err(LarkAPIError::InvalidParameter(format!(
                            "范围{}的单元格引用不能为空",
                            index + 1
                        )));
                    }
                } else {
                    // 仅单元格引用的格式：A1:B2
                    if range.is_empty() {
                        return Err(LarkAPIError::InvalidParameter(format!(
                            "范围{}的单元格引用不能为空",
                            index + 1
                        )));
                    }
                }
            }
        } else {
            return Err(LarkAPIError::InvalidParameter(
                "范围列表不能为空".to_string(),
            ));
        }

        Ok(())
    }

    /// 构建查询参数
    pub fn build_query_params(&self) -> String {
        let mut params = vec![];

        // 添加范围参数（JSON数组格式）
        if let Some(ranges) = &self.ranges {
            let ranges_json = serde_json::to_string(ranges).unwrap_or_default();
            params.push(format!("ranges={}", urlencoding::encode(&ranges_json)));
        }

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

/// 多个范围读取服务
#[derive(Clone, Debug)]
pub struct BatchRangeReadService {
    config: Config,
}

impl BatchRangeReadService {
    /// 创建多个范围读取服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量读取范围数据
    ///
    /// 根据spreadsheetToken和ranges读取表格多个范围的值，返回数据限制为10M。
    ///
    /// # 参数
    /// - `request`: 多个范围读取请求
    ///
    /// # 返回
    /// 批量范围数据响应
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v2::batch_range_read::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = BatchRangeReadService::new(config);
    ///
    /// // 创建批量读取请求
    /// let request = BatchRangeReadRequest::new(
    ///     "spreadsheet_token",
    ///     vec!["Sheet1!A1:B2", "Sheet1!D1:E10"]
    /// )
    ///     .value_render_option("FormattedValue")
    ///     .date_time_render_option("FormattedString");
    ///
    /// let response = service.read_ranges(request).await?;
    ///
    /// println!("读取到{}个范围", response.value_ranges.len());
    /// for value_range in &response.value_ranges {
    ///     println!("范围: {}", value_range.range);
    ///     println!("值: {:?}", value_range.values);
    /// }
    /// ```
    pub async fn read_ranges(
        &self,
        request: BatchRangeReadRequest,
    ) -> SDKResult<Response<BatchRangeReadResponse>> {
        // 验证请求参数
        request.validate()?;

        // 构建API请求
        let mut api_req = ApiRequest::with_method(openlark_core::api::HttpMethod::Get);
        api_req.set_api_path(
            Endpoints::SHEETS_V2_SPREADSHEET_VALUES_BATCH_GET
                .replace("{spreadsheet_token}", &request.spreadsheet_token),
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
        Transport::<BatchRangeReadResponse>::request(api_req, &self.config, None).await
    }

    /// 创建多个范围读取构建器
    pub fn read_ranges_builder(
        &self,
        spreadsheet_token: impl Into<String>,
    ) -> BatchRangeReadBuilder {
        BatchRangeReadBuilder::new(self.clone() spreadsheet_token)
    }

    /// 快速批量读取范围数据（使用默认选项）
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格令牌
    /// - `ranges`: 要读取的范围列表
    ///
    /// # 返回
    /// 批量范围数据响应
    pub async fn read_ranges_simple<T: Into<String>, R: Into<String>>(
        &self,
        spreadsheet_token: T,
        ranges: Vec<R>,
    ) -> SDKResult<Response<BatchRangeReadResponse>> {
        let request = BatchRangeReadRequest::new(spreadsheet_token, ranges);
        self.read_ranges(request).await
    }
}

/// 多个范围读取构建器
pub struct BatchRangeReadBuilder {
    service: BatchRangeReadService,
    spreadsheet_token: String,
    ranges: Vec<String>,
    value_render_option: Option<String>,
    date_time_render_option: Option<String>,
    user_id_type: Option<String>,
}

impl BatchRangeReadBuilder {
    /// 创建新的构建器
    pub fn new(service: BatchRangeReadService, spreadsheet_token: impl Into<String>) -> Self {
        Self {
            service,
            spreadsheet_token: spreadsheet_token.into(),
            ranges: vec![],
            value_render_option: None,
            date_time_render_option: None,
            user_id_type: None,
        }
    }

    /// 添加范围
    pub fn add_range(mut self, range: impl Into<String>) -> Self {
        self.ranges.push(range.into());
        self
    }

    /// 批量添加范围
    pub fn add_ranges(mut self, ranges: Vec<impl Into<String>>) -> Self {
        for range in ranges {
            self.ranges.push(range.into());
        }
        self
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
    pub async fn execute(self) -> SDKResult<Response<BatchRangeReadResponse>> {
        let mut request = BatchRangeReadRequest::new(self.spreadsheet_token, self.ranges);

        if let Some(option) = self.value_render_option {
            request = request.value_render_option(option);
        }

        if let Some(option) = self.date_time_render_option {
            request = request.date_time_render_option(option);
        }

        if let Some(user_id_type) = self.user_id_type {
            request = request.user_id_type(user_id_type);
        }

        self.service.read_ranges(request).await
    }
}

impl Service for BatchRangeReadService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "BatchRangeReadService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_range_read_request_validation() {
        // 测试空令牌
        let request = BatchRangeReadRequest::new("", vec!["Sheet1!A1:B2"]);
        assert!(request.validate().is_err());

        // 测试空范围列表
        let request = BatchRangeReadRequest::new("token", Vec::<String>::new());
        assert!(request.validate().is_err());

        // 测试正常请求
        let request = BatchRangeReadRequest::new(
            "shtcnmBRWQKbsJRHXXXXXXXXXX",
            vec!["Sheet1!A1:B2", "Sheet2!A1:C10"],
        );
        assert!(request.validate().is_ok());

        // 测试范围数量限制
        let many_ranges: Vec<String> = (0..15)
            .map(|i| format!("Sheet1!A{}:B{}", i * 10 + 1, i * 10 + 10))
            .collect();
        let request = BatchRangeReadRequest::new("token", many_ranges);
        assert!(request.validate().is_err());

        // 测试无效范围格式
        let request = BatchRangeReadRequest::new("token", vec!["Sheet1!!A1"]);
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_build_query_params() {
        let request = BatchRangeReadRequest::new("token", vec!["Sheet1!A1:B2", "Sheet2!A1:C10"])
            .value_render_option("FormattedValue")
            .date_time_render_option("FormattedString")
            .user_id_type("open_id");

        let params = request.build_query_params();
        assert!(params.contains("ranges="));
        assert!(params.contains("valueRenderOption=FormattedValue"));
        assert!(params.contains("dateTimeRenderOption=FormattedString"));
        assert!(params.contains("userIdType=open_id"));
    }

    #[test]
    fn test_builder_pattern() {
        let config = openlark_core::config::Config::default();
        let service = BatchRangeReadService::new(config);

        let builder = service
            .read_ranges_builder("test_token")
            .add_range("Sheet1!A1:B2")
            .add_range("Sheet2!A1:C10")
            .value_render_option("FormattedValue")
            .date_time_render_option("FormattedString");

        // 验证构建器设置
        assert_eq!(builder.spreadsheet_token, "test_token");
        assert_eq!(builder.ranges.len(), 2);
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
    fn test_batch_range_read_service() {
        let config = openlark_core::config::Config::default();
        let service = BatchRangeReadService::new(config);

        assert_eq!(service.service_name(), "BatchRangeReadService");
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

    #[test]
    fn test_add_range_method() {
        let mut request = BatchRangeReadRequest::new("token", vec!["Sheet1!A1:B2"]);
        request = request.add_range("Sheet2!A1:C10");

        assert!(request.ranges.is_some());
        assert_eq!(request.ranges.as_ref().unwrap().len(), 2);
        assert_eq!(request.ranges.as_ref().unwrap()[1], "Sheet2!A1:C10");
    }
}

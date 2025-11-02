//! 单范围数据读取 API
//!
//! 提供读取飞书电子表格中指定单元格范围数据的功能。
//! 支持灵活的数据格式设置和日期时间渲染选项。

use reqwest::Method;
use serde::{Deserialize, Serialize};
use crate::core::SDKResult;
use crate::crate::core::error::LarkAPIError;

use crate::{
    core::{
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::SHEETS_V3_DATA_OPERATION_READING_SINGLE_RANGE,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 数据操作服务
pub struct DataOperationService {
    config: Config,
}

impl DataOperationService {
    /// 创建新的数据操作服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 读取单个范围数据
    ///
    /// 从电子表格的指定范围读取数据，支持多种数据格式和渲染选项。
    ///
    /// # 参数
    ///
    /// * `request` - 读取单范围数据的请求参数
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回指定范围内的单元格数据，包括范围信息和数据值。
    ///
    /// # 示例
    ///
    /// ```rust
    /// let request = ReadingSingleRangeRequest::builder()
    ///     .spreadsheet_token("shtcnmBA*****yGehy8".to_string())
    ///     .range("Sheet1!A1:C10".to_string())
    ///     .value_render_option("FormattedValue".to_string())
    ///     .build();
    ///
    /// let response = client.sheets.v3.data_operation
    ///     .reading_single_range(request, None)
    ///     .await?;
    /// ```
    pub async fn reading_single_range(
        &self,
        request: ReadingSingleRangeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ReadingSingleRangeResponseData>> {
        let mut api_req = request.api_request;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(SHEETS_V3_DATA_OPERATION_READING_SINGLE_RANGE.to_string());
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User
        ]);

        // 添加查询参数
        api_req.query_params.insert("spreadsheet_token", request.spreadsheet_token.clone());
        api_req.query_params.insert("range", request.range.clone());

        if let Some(value_render_option) = &request.value_render_option {
            api_req.query_params.insert("valueRenderOption", value_render_option.clone());
        }

        if let Some(date_time_render_option) = &request.date_time_render_option {
            api_req.query_params.insert("dateTimeRenderOption", date_time_render_option.clone());
        }

        if let Some(user_id_type) = &request.user_id_type {
            api_req.query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

/// 读取单个范围请求
///
/// 用于读取电子表格中指定范围数据的请求参数。
#[derive(Debug, Clone, Serialize)]
pub struct ReadingSingleRangeRequest {
    #[serde(skip)]
    api_request: crate::core::api_req::ApiRequest,
    /// 电子表格token
    ///
    /// 要读取数据的电子表格的唯一标识符
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 查询范围
    ///
    /// 包含sheetId与单元格范围两部分，格式如：Sheet1!A1:C10
    pub range: String,
    /// 值渲染选项
    ///
    /// 指定单元格数据的格式
    ///
    /// # 可选值
    ///
    /// - `UnformattedValue`：返回原始值，不应用格式
    /// - `FormattedValue`：返回格式化后的值（默认）
    /// - `Formula`：返回公式字符串
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_render_option: Option<String>,
    /// 日期时间渲染选项
    ///
    /// 指定数据类型为日期、时间或时间日期的单元格数据的格式
    ///
    /// # 可选值
    ///
    /// - `SerialNumber`：返回序列号
    /// - `FormattedString`：返回格式化字符串（默认）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_render_option: Option<String>,
    /// 用户ID类型
    ///
    /// 指定返回的用户ID类型，影响响应中用户标识的格式
    ///
    /// # 可选值
    ///
    /// - `open_id`：用户在特定应用中的身份标识（默认）
    /// - `union_id`：用户在开发商下的统一身份标识
    /// - `user_id`：用户在租户内的身份标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl ReadingSingleRangeRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    ///
    /// * `spreadsheet_token` - 电子表格的唯一标识符
    /// * `range` - 要读取的单元格范围
    pub fn new(
        spreadsheet_token: impl Into<String>,
        range: impl Into<String>,
    ) -> Self {
        Self {
            api_request: Default::default(),
            spreadsheet_token: spreadsheet_token.into(),
            range: range.into(),
            value_render_option: None,
            date_time_render_option: None,
            user_id_type: None,
        }
    }

    /// 创建构建器
    pub fn builder() -> ReadingSingleRangeRequestBuilder {
        ReadingSingleRangeRequestBuilder::default()
    }

    /// 验证请求参数
    ///
    /// 检查请求参数的有效性，确保符合API要求。
    pub fn validate(&self) -> SDKResult<()> {
        // 验证spreadsheet_token
        if self.spreadsheet_token.is_empty() {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(
                "spreadsheet_token不能为空".to_string()
            ));
        }

        // 验证range
        if self.range.is_empty() {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(
                "range不能为空".to_string()
            ));
        }

        // 验证单元格范围格式（基础验证）
        if !self.range.contains('!') {
            return Err(crate::crate::core::error::LarkAPIError::illegal_param(
                "range格式不正确，应包含工作表名称，如：Sheet1!A1:C10".to_string()
            ));
        }

        // 验证值渲染选项
        if let Some(option) = &self.value_render_option {
            match option.as_str() {
                "UnformattedValue" | "FormattedValue" | "Formula" => {
                    // 有效的值
                }
                _ => {
                    return Err(crate::crate::core::error::LarkAPIError::illegal_param(
                        format!(
                            "valueRenderOption必须是以下值之一：UnformattedValue, FormattedValue, Formula，当前值：{}",
                            option
                        )
                    ));
                }
            }
        }

        // 验证日期时间渲染选项
        if let Some(option) = &self.date_time_render_option {
            match option.as_str() {
                "SerialNumber" | "FormattedString" => {
                    // 有效的值
                }
                _ => {
                    return Err(crate::crate::core::error::LarkAPIError::illegal_param(
                        format!(
                            "dateTimeRenderOption必须是以下值之一：SerialNumber, FormattedString，当前值：{}",
                            option
                        )
                    ));
                }
            }
        }

        // 验证用户ID类型
        if let Some(user_id_type) = &self.user_id_type {
            match user_id_type.as_str() {
                "open_id" | "union_id" | "user_id" => {
                    // 有效的值
                }
                _ => {
                    return Err(crate::crate::core::error::LarkAPIError::illegal_param(
                        format!(
                            "user_id_type必须是以下值之一：open_id, union_id, user_id，当前值：{}",
                            user_id_type
                        )
                    ));
                }
            }
        }

        Ok(())
    }
}

/// 读取单个范围请求构建器
///
/// 提供流式API来构建读取单范围数据的请求。
#[derive(Debug, Clone, Default)]
pub struct ReadingSingleRangeRequestBuilder {
    request: ReadingSingleRangeRequest,
}

impl ReadingSingleRangeRequestBuilder {
    /// 设置电子表格token
    ///
    /// # 参数
    ///
    /// * `spreadsheet_token` - 电子表格的唯一标识符
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置查询范围
    ///
    /// # 参数
    ///
    /// * `range` - 要读取的单元格范围，格式如：Sheet1!A1:C10
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.request.range = range.into();
        self
    }

    /// 设置值渲染选项
    ///
    /// # 参数
    ///
    /// * `value_render_option` - 值渲染选项
    pub fn value_render_option(mut self, value_render_option: impl Into<String>) -> Self {
        self.request.value_render_option = Some(value_render_option.into());
        self
    }

    /// 设置日期时间渲染选项
    ///
    /// # 参数
    ///
    /// * `date_time_render_option` - 日期时间渲染选项
    pub fn date_time_render_option(mut self, date_time_render_option: impl Into<String>) -> Self {
        self.request.date_time_render_option = Some(date_time_render_option.into());
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    ///
    /// * `user_id_type` - 用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 构建请求对象
    ///
    /// 创建最终的请求实例，会自动验证参数。
    ///
    /// # 返回值
    ///
    /// 返回验证通过的请求对象
    ///
    /// # 错误
    ///
    /// 如果参数验证失败，会返回相应的错误
    pub fn build(self) -> SDKResult<ReadingSingleRangeRequest> {
        self.request.validate()?;
        Ok(self.request)
    }
}

/// Trait实现，支持Builder模式
impl_executable_builder_owned!(
    ReadingSingleRangeRequestBuilder,
    DataOperationService,
    ReadingSingleRangeRequest,
    BaseResponse<ReadingSingleRangeResponseData>,
    reading_single_range,
);

/// 读取单个范围响应数据
///
/// 包含读取到的单元格范围数据。
#[derive(Debug, Clone, Deserialize)]
pub struct ReadingSingleRangeResponseData {
    /// 值与范围信息
    #[serde(rename = "valueRange")]
    pub value_range: ValueRange,
}

impl ApiResponseTrait for ReadingSingleRangeResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 值范围信息
///
/// 包含读取到的单元格数据和相关信息。
#[derive(Debug, Clone, Deserialize)]
pub struct ValueRange {
    /// 查询范围
    ///
    /// 实际查询的单元格范围
    pub range: String,
    /// 版本号
    ///
    /// 工作表的版本号，用于数据一致性检查
    pub revision: i32,
    /// 单元格值
    ///
    /// 二维数组，包含范围内的所有单元格值
    pub values: Vec<Vec<serde_json::Value>>,
}

impl ValueRange {
    /// 获取指定位置的单元格值
    ///
    /// # 参数
    ///
    /// * `row` - 行索引（从0开始）
    /// * `col` - 列索引（从0开始）
    ///
    /// # 返回值
    ///
    /// 返回指定位置的单元格值，如果位置超出范围则返回None
    pub fn get_cell_value(&self, row: usize, col: usize) -> Option<&serde_json::Value> {
        self.values.get(row).and_then(|row_data| row_data.get(col))
    }

    /// 获取范围的行数
    pub fn row_count(&self) -> usize {
        self.values.len()
    }

    /// 获取范围的列数
    ///
    /// 返回第一行的列数，如果范围为空则返回0
    pub fn column_count(&self) -> usize {
        self.values.first().map_or(0, |row| row.len())
    }

    /// 检查范围是否为空
    pub fn is_empty(&self) -> bool {
        self.values.is_empty() || self.values.iter().all(|row| row.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_reading_single_range_request_builder() {
        let request = ReadingSingleRangeRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .range("Sheet1!A1:C10")
            .value_render_option("FormattedValue")
            .date_time_render_option("FormattedString")
            .user_id_type("open_id")
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "shtcnmBA*****yGehy8");
        assert_eq!(request.range, "Sheet1!A1:C10");
        assert_eq!(request.value_render_option, Some("FormattedValue".to_string()));
        assert_eq!(request.date_time_render_option, Some("FormattedString".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_reading_single_range_request_validation() {
        // 测试空spreadsheet_token验证
        let request = ReadingSingleRangeRequest {
            api_request: Default::default(),
            spreadsheet_token: "".to_string(),
            range: "Sheet1!A1:C10".to_string(),
            value_render_option: None,
            date_time_render_option: None,
            user_id_type: None,
        };

        assert!(request.validate().is_err());

        // 测试空range验证
        let request = ReadingSingleRangeRequest {
            api_request: Default::default(),
            spreadsheet_token: "valid_token".to_string(),
            range: "".to_string(),
            value_render_option: None,
            date_time_render_option: None,
            user_id_type: None,
        };

        assert!(request.validate().is_err());

        // 测试无效range格式验证
        let request = ReadingSingleRangeRequest {
            api_request: Default::default(),
            spreadsheet_token: "valid_token".to_string(),
            range: "A1:C10".to_string(), // 缺少工作表名称
            value_render_option: None,
            date_time_render_option: None,
            user_id_type: None,
        };

        assert!(request.validate().is_err());

        // 测试无效渲染选项验证
        let invalid_render_options = ["InvalidValue", "WrongFormat"];
        for option in &invalid_render_options {
            let request = ReadingSingleRangeRequest {
                api_request: Default::default(),
                spreadsheet_token: "valid_token".to_string(),
                range: "Sheet1!A1:C10".to_string(),
                value_render_option: Some(option.to_string()),
                date_time_render_option: None,
                user_id_type: None,
            };
            assert!(request.validate().is_err());
        }
    }

    #[test]
    fn test_reading_single_range_response_deserialization() {
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
        let value_range = response.value_range;

        assert_eq!(value_range.range, "Sheet1!A1:B2");
        assert_eq!(value_range.revision, 123);
        assert_eq!(value_range.values.len(), 2);
        assert_eq!(value_range.row_count(), 2);
        assert_eq!(value_range.column_count(), 2);
        assert!(!value_range.is_empty());
    }

    #[test]
    fn test_value_range_methods() {
        let value_range = ValueRange {
            range: "Sheet1!A1:C3".to_string(),
            revision: 1,
            values: vec![
                vec![json!("A1"), json!("B1"), json!("C1")],
                vec![json!("A2"), json!("B2"), json!("C2")],
                vec![json!("A3"), json!("B3"), json!("C3")],
            ],
        };

        // 测试获取单元格值
        assert_eq!(value_range.get_cell_value(0, 0), Some(&json!("A1")));
        assert_eq!(value_range.get_cell_value(1, 2), Some(&json!("C2")));
        assert_eq!(value_range.get_cell_value(2, 1), Some(&json!("B3")));

        // 测试越界访问
        assert_eq!(value_range.get_cell_value(3, 0), None);
        assert_eq!(value_range.get_cell_value(0, 3), None);

        // 测试行列计数
        assert_eq!(value_range.row_count(), 3);
        assert_eq!(value_range.column_count(), 3);
        assert!(!value_range.is_empty());
    }

    #[test]
    fn test_empty_value_range() {
        let empty_value_range = ValueRange {
            range: "Sheet1!A1:A1".to_string(),
            revision: 1,
            values: vec![],
        };

        assert_eq!(empty_value_range.row_count(), 0);
        assert_eq!(empty_value_range.column_count(), 0);
        assert!(empty_value_range.is_empty());

        let value_range_with_empty_row = ValueRange {
            range: "Sheet1!A1:A1".to_string(),
            revision: 1,
            values: vec![vec![]],
        };

        assert_eq!(value_range_with_empty_row.row_count(), 1);
        assert_eq!(value_range_with_empty_row.column_count(), 0);
        assert!(value_range_with_empty_row.is_empty());
    }

    #[test]
    fn test_valid_request_creation() {
        let request = ReadingSingleRangeRequest::new(
            "shtcnmBA*****yGehy8",
            "Sheet1!A1:C10"
        );

        assert_eq!(request.spreadsheet_token, "shtcnmBA*****yGehy8");
        assert_eq!(request.range, "Sheet1!A1:C10");
        assert!(request.validate().is_ok()); // 应该通过验证
    }
}
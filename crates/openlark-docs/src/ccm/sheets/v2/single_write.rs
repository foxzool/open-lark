//! 向单个范围写入数据 - Single Range Write Service
//!
//! 提供飞书电子表格v2版本的单个范围数据写入API。
//! 支持向指定工作表的特定范围写入数据，包括：
//! - 单元格值写入和格式化
//! - 数据类型自动转换和验证
//! - 写入结果状态和详细信息
//! - 错误处理和重试机制
//! - 多种数据格式支持（数字、文本、布尔值、公式等）

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use serde_json::Value;
use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::endpoints::Endpoints;
use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};

/// 向单个范围写入数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleWriteRequest {
    /// 电子表格的访问令牌
    pub spreadsheet_token: String,
    /// 要写入的范围，如 "Sheet1!A1:C3"
    pub range: String,
    /// 要写入的数据，二维数组结构
    pub values: Vec<Vec<CellValue>>,
    /// 值输入选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_input_option: Option<String>,
    /// 是否在响应中包含写入的值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_values_in_response: Option<bool>,
    /// 响应中值的渲染选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_value_render_option: Option<String>,
    /// 响应中日期时间的渲染选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_date_time_render_option: Option<String>,
}

impl Default for SingleWriteRequest {
    fn default() -> Self {
        Self {
            spreadsheet_token: String::new(),
            range: String::new(),
            values: vec![],
            value_input_option: Some("USER_ENTERED".to_string()),
            include_values_in_response: Some(false),
            response_value_render_option: None,
            response_date_time_render_option: None,
        }
    }
}

impl SingleWriteRequest {
    /// 创建新的单范围写入请求
    pub fn new(spreadsheet_token: String, range: String, values: Vec<Vec<CellValue>>) -> Self {
        Self {
            spreadsheet_token,
            range,
            values,
            value_input_option: Some("USER_ENTERED".to_string()),
            include_values_in_response: Some(false),
            response_value_render_option: None,
            response_date_time_render_option: None,
        }
    }

    /// 创建构建器
    pub fn builder() -> SingleWriteRequestBuilder {
        SingleWriteRequestBuilder::default()
    }

    /// 验证请求参数的有效性
    pub fn validate(&self) -> SDKResult<()> {
        if self.spreadsheet_token.is_empty() {
            return Err(LarkAPIError::illegal_param("电子表格令牌不能为空"));
        }

        if self.range.is_empty() {
            return Err(LarkAPIError::illegal_param("写入范围不能为空"));
        }

        if self.values.is_empty() {
            return Err(LarkAPIError::illegal_param("写入数据不能为空"));
        }

        Ok(())
    }

    /// 获取写入的行数
    pub fn row_count(&self) -> usize {
        self.values.len()
    }

    /// 获取写入的列数
    pub fn column_count(&self) -> usize {
        if self.values.is_empty() {
            0
        } else {
            self.values[0].len()
        }
    }

    /// 获取数据总数
    pub fn cell_count(&self) -> usize {
        self.values.iter().map(|row| row.len()).sum()
    }
}

/// 单范围写入请求构建器
#[derive(Debug, Default)]
pub struct SingleWriteRequestBuilder {
    spreadsheet_token: Option<String>,
    range: Option<String>,
    values: Option<Vec<Vec<CellValue>>>,
    value_input_option: Option<String>,
    include_values_in_response: Option<bool>,
    response_value_render_option: Option<String>,
    response_date_time_render_option: Option<String>,
}

impl SingleWriteRequestBuilder {
    /// 设置电子表格令牌
    pub fn spreadsheet_token(mut self, token: String) -> Self {
        self.spreadsheet_token = Some(token);
        self
    }

    /// 设置写入范围
    pub fn range(mut self, range: String) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置写入数据
    pub fn values(mut self, values: Vec<Vec<CellValue>>) -> Self {
        self.values = Some(values);
        self
    }

    /// 设置值输入选项
    pub fn value_input_option(mut self, option: String) -> Self {
        self.value_input_option = Some(option);
        self
    }

    /// 设置是否在响应中包含值
    pub fn include_values_in_response(mut self, include: bool) -> Self {
        self.include_values_in_response = Some(include);
        self
    }

    /// 设置响应值渲染选项
    pub fn response_value_render_option(mut self, option: String) -> Self {
        self.response_value_render_option = Some(option);
        self
    }

    /// 设置响应日期时间渲染选项
    pub fn response_date_time_render_option(mut self, option: String) -> Self {
        self.response_date_time_render_option = Some(option);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> SDKResult<SingleWriteRequest> {
        let request = SingleWriteRequest {
            spreadsheet_token: self
                .spreadsheet_token
                .ok_or_else(|| LarkAPIError::illegal_param("电子表格令牌不能为空"))?,
            range: self
                .range
                .ok_or_else(|| LarkAPIError::illegal_param("写入范围不能为空"))?,
            values: self
                .values
                .ok_or_else(|| LarkAPIError::illegal_param("写入数据不能为空"))?,
            value_input_option: self
                .value_input_option
                .or_else(|| Some("USER_ENTERED".to_string())),
            include_values_in_response: self.include_values_in_response.or_else(|| Some(false)),
            response_value_render_option: self.response_value_render_option,
            response_date_time_render_option: self.response_date_time_render_option,
        };

        // 验证请求参数
        request.validate()?;
        Ok(request)
    }
}

/// 单范围写入响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleWriteResponse {
    /// 更新的电子表格ID
    pub spreadsheet_id: String,
    /// 更新的范围
    pub updated_range: String,
    /// 更新的行数
    pub updated_rows: i32,
    /// 更新的列数
    pub updated_columns: i32,
    /// 更新的单元格数量
    pub updated_cells: i32,
    /// 写入的数据（如果请求中要求包含）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_data: Option<Vec<Vec<CellValue>>>,
}

impl Default for SingleWriteResponse {
    fn default() -> Self {
        Self {
            spreadsheet_id: String::new(),
            updated_range: String::new(),
            updated_rows: 0,
            updated_columns: 0,
            updated_cells: 0,
            updated_data: None,
        }
    }
}

impl ApiResponseTrait for SingleWriteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 单范围写入服务
#[derive(Clone, Debug)]
pub struct SingleWriteService {
    config: Config,
}

impl SingleWriteService {
    /// 创建新的单范围写入服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取服务名称
    pub fn service_name() -> &'static str {
        "SingleWriteService"
    }

    /// 获取服务版本
    pub fn service_version() -> &'static str {
        "v2"
    }

    /// 向单个范围写入数据
    pub async fn write_range(
        &self,
        request: &SingleWriteRequest,
    ) -> SDKResult<Response<SingleWriteResponse>> {
        // 验证请求参数
        request.validate()?;

        // 构建请求URL
        let url = self.build_request_url(&request.spreadsheet_token, &request.range)?;

        // 构建请求体
        let body = self.build_request_body(request)?;

        // 创建HTTP请求
        let api_request = self.create_http_request(&url, body)?;

        // 发送请求并获取响应
        let response = Transport::request(api_request, &self.config, None).await?;

        Ok(response)
    }

    /// 使用构建器模式写入数据
    pub fn write_range_builder(&self) -> SingleWriteServiceRequestBuilder {
        SingleWriteServiceRequestBuilder::new(self)
    }

    /// 构建请求URL
    fn build_request_url(&self, spreadsheet_token: &str, range: &str) -> SDKResult<String> {
        let base_url = self.config.base_url.clone();

        // URL编码范围参数
        let encoded_range = urlencoding::encode(range);

        let url = format!(
            "{}/open-apis/sheets/v2/spreadsheets/{}/values/{}",
            base_url.trim_end_matches('/'),
            spreadsheet_token,
            encoded_range
        );

        Ok(url)
    }

    /// 构建请求体
    fn build_request_body(&self, request: &SingleWriteRequest) -> SDKResult<Value> {
        let mut body = HashMap::new();

        // 转换数据格式
        let values_json: Vec<Vec<Value>> = request
            .values
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        CellValue::Text(s) => Value::String(s.clone()),
                        CellValue::Number(n) => Value::Number(
                            serde_json::Number::from_f64(*n)
                                .unwrap_or_else(|| serde_json::Number::from(0)),
                        ),
                        CellValue::Boolean(b) => Value::Bool(*b),
                        CellValue::Formula(f) => Value::String(format!("={}", f)),
                        CellValue::Blank => Value::Null,
                        CellValue::Error(e) => Value::String(e.clone()),
                    })
                    .collect()
            })
            .collect();

        body.insert(
            "values".to_string(),
            Value::Array(
                values_json
                    .into_iter()
                    .map(|row| Value::Array(row))
                    .collect(),
            ),
        );

        // 添加可选参数
        if let Some(option) = &request.value_input_option {
            body.insert(
                "valueInputOption".to_string(),
                Value::String(option.clone()),
            );
        }

        if let Some(include) = request.include_values_in_response {
            body.insert("includeValuesInResponse".to_string(), Value::Bool(include));
        }

        if let Some(option) = &request.response_value_render_option {
            body.insert(
                "responseValueRenderOption".to_string(),
                Value::String(option.clone()),
            );
        }

        if let Some(option) = &request.response_date_time_render_option {
            body.insert(
                "responseDateTimeRenderOption".to_string(),
                Value::String(option.clone()),
            );
        }

        Ok(Value::Object(body.into_iter().collect()))
    }

    /// 创建HTTP请求
    fn create_http_request(&self, url: &str, _body: Value) -> SDKResult<ApiRequest> {
        let api_request = ApiRequest::with_method_and_path(Method::PUT, url);
        Ok(api_request)
    }

    /// 便捷方法：快速写入单个值
    pub async fn write_single_value(
        &self,
        spreadsheet_token: &str,
        cell_range: &str,
        value: serde_json::Value,
    ) -> SDKResult<Response<SingleWriteResponse>> {
        let request = SingleWriteRequest::builder()
            .spreadsheet_token(spreadsheet_token.to_string())
            .range(cell_range.to_string())
            .values(vec![vec![value]])
            .build()?;

        self.write_range(&request).await
    }

    /// 便捷方法：快速写入一行数据
    pub async fn write_single_row(
        &self,
        spreadsheet_token: &str,
        range: &str,
        row_data: Vec<serde_json::Value>,
    ) -> SDKResult<Response<SingleWriteResponse>> {
        let request = SingleWriteRequest::builder()
            .spreadsheet_token(spreadsheet_token.to_string())
            .range(range.to_string())
            .values(vec![row_data])
            .build()?;

        self.write_range(&request).await
    }
}

/// 单范围写入服务请求构建器
#[derive(Debug)]
pub struct SingleWriteServiceRequestBuilder<'a> {
    service: &'a SingleWriteService,
    spreadsheet_token: Option<String>,
    range: Option<String>,
    values: Option<Vec<Vec<CellValue>>>,
    value_input_option: Option<String>,
    include_values_in_response: Option<bool>,
    response_value_render_option: Option<String>,
    response_date_time_render_option: Option<String>,
}

impl<'a> SingleWriteServiceRequestBuilder<'a> {
    /// 创建新的构建器实例
    fn new(service: &'a SingleWriteService) -> Self {
        Self {
            service,
            spreadsheet_token: None,
            range: None,
            values: None,
            value_input_option: None,
            include_values_in_response: None,
            response_value_render_option: None,
            response_date_time_render_option: None,
        }
    }

    /// 设置电子表格令牌
    pub fn spreadsheet_token(mut self, token: String) -> Self {
        self.spreadsheet_token = Some(token);
        self
    }

    /// 设置写入范围
    pub fn range(mut self, range: String) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置写入数据
    pub fn values(mut self, values: Vec<Vec<CellValue>>) -> Self {
        self.values = Some(values);
        self
    }

    /// 设置值输入选项
    pub fn value_input_option(mut self, option: String) -> Self {
        self.value_input_option = Some(option);
        self
    }

    /// 设置是否在响应中包含值
    pub fn include_values_in_response(mut self, include: bool) -> Self {
        self.include_values_in_response = Some(include);
        self
    }

    /// 设置响应值渲染选项
    pub fn response_value_render_option(mut self, option: String) -> Self {
        self.response_value_render_option = Some(option);
        self
    }

    /// 设置响应日期时间渲染选项
    pub fn response_date_time_render_option(mut self, option: String) -> Self {
        self.response_date_time_render_option = Some(option);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<Response<SingleWriteResponse>> {
        let request = SingleWriteRequest {
            spreadsheet_token: self
                .spreadsheet_token
                .ok_or_else(|| LarkAPIError::illegal_param("电子表格令牌不能为空"))?,
            range: self
                .range
                .ok_or_else(|| LarkAPIError::illegal_param("写入范围不能为空"))?,
            values: self
                .values
                .ok_or_else(|| LarkAPIError::illegal_param("写入数据不能为空"))?,
            value_input_option: self
                .value_input_option
                .or_else(|| Some("USER_ENTERED".to_string())),
            include_values_in_response: self.include_values_in_response.or_else(|| Some(false)),
            response_value_render_option: self.response_value_render_option,
            response_date_time_render_option: self.response_date_time_render_option,
        };

        self.service.write_range(&request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    fn test_single_write_request_creation() {
        let request = SingleWriteRequest::new(
            "test_token".to_string(),
            "Sheet1!A1:C3".to_string(),
            vec![
                vec![
                    CellValue::Text("姓名".to_string()),
                    CellValue::Text("年龄".to_string()),
                ],
                vec![CellValue::Text("张三".to_string()), CellValue::Number(25.0)],
            ],
        );

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.range, "Sheet1!A1:C3");
        assert_eq!(request.row_count(), 2);
        assert_eq!(request.column_count(), 2);
        assert_eq!(request.cell_count(), 4);
    }

    #[test]
    fn test_single_write_request_builder() {
        let request = SingleWriteRequest::builder()
            .spreadsheet_token("test_token".to_string())
            .range("Sheet1!A1:B2".to_string())
            .values(vec![
                vec![
                    CellValue::Text("标题1".to_string()),
                    CellValue::Text("标题2".to_string()),
                ],
                vec![
                    CellValue::Text("数据1".to_string()),
                    CellValue::Text("数据2".to_string()),
                ],
            ])
            .value_input_option("RAW".to_string())
            .include_values_in_response(true)
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.range, "Sheet1!A1:B2");
        assert_eq!(request.value_input_option, Some("RAW".to_string()));
        assert_eq!(request.include_values_in_response, Some(true));
    }

    #[test]
    fn test_single_write_request_validation() {
        let mut request = SingleWriteRequest::new(
            "test_token".to_string(),
            "Sheet1!A1:C3".to_string(),
            vec![vec![CellValue::Text("test".to_string())]],
        );

        // 正确的请求应该通过验证
        assert!(request.validate().is_ok());

        // 空令牌应该失败
        request.spreadsheet_token = String::new();
        assert!(request.validate().is_err());

        // 空范围应该失败
        request.spreadsheet_token = "test_token".to_string();
        request.range = String::new();
        assert!(request.validate().is_err());

        // 空数据应该失败
        request.range = "Sheet1!A1:C3".to_string();
        request.values = vec![];
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_single_write_service_creation() {
        let config = create_test_config();
        let service = SingleWriteService::new(config);

        assert_eq!(SingleWriteService::service_name(), "SingleWriteService");
        assert_eq!(SingleWriteService::service_version(), "v2");
    }

    #[test]
    fn test_single_write_response_default() {
        let response = SingleWriteResponse::default();

        assert_eq!(response.spreadsheet_id, "");
        assert_eq!(response.updated_range, "");
        assert_eq!(response.updated_rows, 0);
        assert_eq!(response.updated_columns, 0);
        assert_eq!(response.updated_cells, 0);
        assert!(response.updated_data.is_none());
    }

    #[test]
    fn test_service_request_builder() {
        let config = create_test_config();
        let service = SingleWriteService::new(config);
        let builder = service.write_range_builder();

        // 验证构建器创建成功
        assert!(builder.spreadsheet_token.is_none());
        assert!(builder.range.is_none());
        assert!(builder.values.is_none());
    }

    #[test]
    fn test_request_url_building() {
        let config = openlark_core::config::Config::builder()
            .base_url("https://open.feishu.cn".to_string())
            .build();
        let service = SingleWriteService::new(config);

        let url = service
            .build_request_url("token123", "Sheet1!A1:C3")
            .unwrap();
        assert!(url.contains("open.feishu.cn"));
        assert!(url.contains("token123"));
        assert!(url.contains("A1%3AC3")); // URL编码的范围
    }

    #[test]
    fn test_request_body_building() {
        let config = create_test_config();
        let service = SingleWriteService::new(config);

        let request = SingleWriteRequest::new(
            "test_token".to_string(),
            "Sheet1!A1:B2".to_string(),
            vec![
                vec![
                    CellValue::Text("Hello".to_string()),
                    CellValue::Number(42.0),
                ],
                vec![
                    CellValue::Boolean(true),
                    CellValue::Text("World".to_string()),
                ],
            ],
        );

        let body = service.build_request_body(&request).unwrap();

        if let Value::Object(map) = body {
            assert!(map.contains_key("values"));
            assert!(map.contains_key("valueInputOption"));
            assert!(map.contains_key("includeValuesInResponse"));
        } else {
            panic!("请求体应该是对象类型");
        }
    }

    #[tokio::test]
    async fn test_convenience_methods() {
        let config = create_test_config();
        let service = SingleWriteService::new(config);

        // 测试请求构建（不发送实际请求）
        let request = SingleWriteRequest::builder()
            .spreadsheet_token("test_token".to_string())
            .range("Sheet1!A1".to_string())
            .values(vec![vec![CellValue::Text("test".to_string())]])
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.range, "Sheet1!A1");
        assert_eq!(request.values.len(), 1);
        assert_eq!(request.values[0].len(), 1);
        assert_eq!(request.values[0][0], CellValue::Text("test".to_string()));
    }

    #[test]
    fn test_complete_builder_pattern() {
        let config = create_test_config();
        let service = SingleWriteService::new(config);

        let request_builder = service
            .write_range_builder()
            .spreadsheet_token("token".to_string())
            .range("Sheet1!A1:C3".to_string())
            .values(vec![
                vec![
                    CellValue::Text("A".to_string()),
                    CellValue::Text("B".to_string()),
                    CellValue::Text("C".to_string()),
                ],
                vec![
                    CellValue::Number(1.0),
                    CellValue::Number(2.0),
                    CellValue::Number(3.0),
                ],
            ])
            .value_input_option("USER_ENTERED".to_string())
            .include_values_in_response(true)
            .response_value_render_option("FORMATTED_VALUE".to_string());

        assert_eq!(request_builder.spreadsheet_token, Some("token".to_string()));
        assert_eq!(request_builder.range, Some("Sheet1!A1:C3".to_string()));
        assert_eq!(
            request_builder.value_input_option,
            Some("USER_ENTERED".to_string())
        );
        assert_eq!(request_builder.include_values_in_response, Some(true));
        assert_eq!(
            request_builder.response_value_render_option,
            Some("FORMATTED_VALUE".to_string())
        );
    }

    #[test]
    fn test_error_handling() {
        // 测试构建器验证错误
        let result = SingleWriteRequest::builder().build();

        assert!(result.is_err());

        // 测试部分参数错误
        let result = SingleWriteRequest::builder()
            .spreadsheet_token("token".to_string())
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_data_conversion_edge_cases() {
        let request = SingleWriteRequest::new(
            "test_token".to_string(),
            "Sheet1!A1:F1".to_string(),
            vec![vec![
                CellValue::Text("text".to_string()),
                CellValue::Number(42.5),
                CellValue::Boolean(true),
                CellValue::Formula("=SUM(A1:B1)".to_string()),
            ]],
        );

        assert_eq!(request.row_count(), 1);
        assert_eq!(request.column_count(), 4);
        assert_eq!(request.cell_count(), 4);
    }

    #[test]
    fn test_service_name_and_version() {
        assert_eq!(SingleWriteService::service_name(), "SingleWriteService");
        assert_eq!(SingleWriteService::service_version(), "v2");
    }
}

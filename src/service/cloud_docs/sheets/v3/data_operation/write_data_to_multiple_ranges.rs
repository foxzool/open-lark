#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 多范围数据写入 API
//!
//! 提供向飞书电子表格中多个单元格范围同时写入数据的功能。
//! 支持批量写入操作，提高数据操作效率，适用于大规模数据处理场景。

use reqwest::Method;
use serde::{Deserialize, Serialize};
use SDKResult;
use openlark_core::error::LarkAPIError;

use crate::{
    core::{
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::SHEETS_V3_DATA_OPERATION_WRITE_DATA_TO_MULTIPLE_RANGES,
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

    /// 向多个范围写入数据
    ///
    /// 同时向电子表格中的多个单元格范围写入数据，支持批量操作。
    /// 适用于需要同时更新多个区域数据的场景，提高操作效率。
    ///
    /// # 参数
    ///
    /// * `request` - 写入多范围数据的请求参数
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回写入操作的结果，包含每个范围的处理状态。
    ///
    /// # 示例
    ///
    /// ```rust
    /// let value_ranges = vec![
    ///     MultiRangeValueData::builder()
    ///         .range("Sheet1!A1:C1".to_string())
    ///         .values(vec![vec!["姓名".to_string(), "年龄".to_string(), "部门".to_string()]])
    ///         .build()?,
    ///     MultiRangeValueData::builder()
    ///         .range("Sheet1!A2:C2".to_string())
    ///         .values(vec![vec!["张三".to_string(), "25".to_string(), "技术部".to_string()]])
    ///         .build()?,
    /// ];
    ///
    /// let request = WriteDataToMultipleRangesRequest::builder()
    ///     .spreadsheet_token("shtcnmBA*****yGehy8".to_string())
    ///     .value_ranges(value_ranges)
    ///     .build();
    ///
    /// let response = client.sheets.v3.data_operation
    ///     .write_data_to_multiple_ranges(request, None)
    ///     .await?;
    /// ```
    pub async fn write_data_to_multiple_ranges(
        &self,
        request: WriteDataToMultipleRangesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<WriteDataToMultipleRangesResponseData>> {
        let mut api_req = request.api_request;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(SHEETS_V3_DATA_OPERATION_WRITE_DATA_TO_MULTIPLE_RANGES.to_string());
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User
        ]);
        api_req.set_body(serde_json::to_vec(&request)?);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

/// 向多个范围写入数据请求
///
/// 用于同时向电子表格中的多个范围写入数据的请求参数。
#[derive(Debug, Clone, Serialize)]
pub struct WriteDataToMultipleRangesRequest {
    #[serde(skip)]
    api_request: crate::core::api_req::ApiRequest,
    /// 电子表格token
    ///
    /// 要写入数据的电子表格的唯一标识符
    pub spreadsheet_token: String,
    /// 值范围数据列表
    ///
    /// 包含要写入的多个范围及其对应的数据
    #[serde(rename = "valueRanges")]
    pub value_ranges: Vec<MultiRangeValueData>,
}

impl WriteDataToMultipleRangesRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    ///
    /// * `spreadsheet_token` - 电子表格的唯一标识符
    /// * `value_ranges` - 要写入的范围数据列表
    pub fn new(
        spreadsheet_token: impl Into<String>,
        value_ranges: Vec<MultiRangeValueData>,
    ) -> Self {
        Self {
            api_request: Default::default(),
            spreadsheet_token: spreadsheet_token.into(),
            value_ranges,
        }
    }

    /// 创建构建器
    pub fn builder() -> WriteDataToMultipleRangesRequestBuilder {
        WriteDataToMultipleRangesRequestBuilder::default()
    }

    /// 验证请求参数
    ///
    /// 检查请求参数的有效性，确保符合API要求。
    pub fn validate(&self) -> SDKResult<()> {
        // 验证spreadsheet_token
        if self.spreadsheet_token.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "spreadsheet_token不能为空".to_string()
            ));
        }

        // 验证value_ranges不为空
        if self.value_ranges.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "value_ranges不能为空".to_string()
            ));
        }

        // 验证数据范围数量限制（API通常有批量操作限制）
        if self.value_ranges.len() > 500 {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                format!(
                    "value_ranges数量超过限制，最大允许500个，当前数量：{}",
                    self.value_ranges.len()
                )
            ));
        }

        // 验证每个范围的数据
        for (index, range_data) in self.value_ranges.iter().enumerate() {
            // 验证范围格式
            if range_data.range.is_empty() {
                return Err(crate::core::error::LarkAPIError::illegal_param(
                    format!("第{}个value_ranges的range不能为空", index + 1)
                ));
            }

            // 基础范围格式验证
            if !range_data.range.contains('!') {
                return Err(crate::core::error::LarkAPIError::illegal_param(
                    format!(
                        "第{}个value_ranges的range格式不正确，应包含工作表名称，如：Sheet1!A1:C10",
                        index + 1
                    )
                ));
            }

            // 验证数据不为空
            if range_data.values.is_empty() {
                return Err(crate::core::error::LarkAPIError::illegal_param(
                    format!("第{}个value_ranges的values不能为空", index + 1)
                ));
            }

            // 验证行数限制（防止过大请求）
            let total_rows: usize = range_data.values.iter().map(|row| row.len()).count();
            if total_rows > 10000 {
                return Err(crate::core::error::LarkAPIError::illegal_param(
                    format!(
                        "第{}个value_ranges的数据量过大，建议分批处理，当前数据量：{}",
                        index + 1, total_rows
                    )
                ));
            }
        }

        // 检查是否有重复的范围（可能导致冲突）
        let mut ranges = std::collections::HashSet::new();
        for range_data in &self.value_ranges {
            if !ranges.insert(&range_data.range) {
                return Err(crate::core::error::LarkAPIError::illegal_param(
                    format!("检测到重复的范围：{}", range_data.range)
                ));
            }
        }

        Ok(())
    }

    /// 获取范围总数
    pub fn range_count(&self) -> usize {
        self.value_ranges.len()
    }

    /// 获取总数据量（所有范围的单元格数量）
    pub fn total_cell_count(&self) -> usize {
        self.value_ranges
            .iter()
            .map(|range| {
                range.values.iter().map(|row| row.len()).sum::<usize>()
            })
            .sum()
    }
}

/// 向多个范围写入数据请求构建器
///
/// 提供流式API来构建写入多范围数据的请求。
#[derive(Debug, Clone, Default)]
pub struct WriteDataToMultipleRangesRequestBuilder {
    request: WriteDataToMultipleRangesRequest,
}

impl WriteDataToMultipleRangesRequestBuilder {
    /// 设置电子表格token
    ///
    /// # 参数
    ///
    /// * `spreadsheet_token` - 电子表格的唯一标识符
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置值范围数据列表
    ///
    /// # 参数
    ///
    /// * `value_ranges` - 要写入的范围数据列表
    pub fn value_ranges(mut self, value_ranges: Vec<MultiRangeValueData>) -> Self {
        self.request.value_ranges = value_ranges;
        self
    }

    /// 添加单个值范围数据
    ///
    /// # 参数
    ///
    /// * `value_range` - 要添加的范围数据
    pub fn add_value_range(mut self, value_range: MultiRangeValueData) -> Self {
        self.request.value_ranges.push(value_range);
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
    pub fn build(self) -> SDKResult<WriteDataToMultipleRangesRequest> {
        self.request.validate()?;
        Ok(self.request)
    }
}

/// Trait实现，支持Builder模式
impl_executable_builder_owned!(
    WriteDataToMultipleRangesRequestBuilder,
    DataOperationService,
    WriteDataToMultipleRangesRequest,
    BaseResponse<WriteDataToMultipleRangesResponseData>,
    write_data_to_multiple_ranges,
);

/// 多范围值数据
///
/// 表示要写入到特定范围的数据。
#[derive(Debug, Clone, Serialize)]
pub struct MultiRangeValueData {
    /// 范围
    ///
    /// 要写入数据的单元格范围，格式如：Sheet1!A1:C10
    pub range: String,
    /// 数据值
    ///
    /// 二维数组，表示要写入的单元格数据
    pub values: Vec<Vec<serde_json::Value>>,
}

impl MultiRangeValueData {
    /// 创建新的范围数据
    ///
    /// # 参数
    ///
    /// * `range` - 单元格范围
    /// * `values` - 要写入的数据值
    pub fn new(
        range: impl Into<String>,
        values: Vec<Vec<serde_json::Value>>,
    ) -> Self {
        Self {
            range: range.into(),
            values,
        }
    }

    /// 创建构建器
    pub fn builder() -> MultiRangeValueDataBuilder {
        MultiRangeValueDataBuilder::default()
    }

    /// 获取行数
    pub fn row_count(&self) -> usize {
        self.values.len()
    }

    /// 获取列数（以第一行为准）
    pub fn column_count(&self) -> usize {
        self.values.first().map_or(0, |row| row.len())
    }

    /// 检查数据是否为空
    pub fn is_empty(&self) -> bool {
        self.values.is_empty() || self.values.iter().all(|row| row.is_empty())
    }
}

/// 多范围值数据构建器
///
/// 提供流式API来构建范围数据。
#[derive(Debug, Clone, Default)]
pub struct MultiRangeValueDataBuilder {
    data: MultiRangeValueData,
}

impl MultiRangeValueDataBuilder {
    /// 设置范围
    ///
    /// # 参数
    ///
    /// * `range` - 单元格范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.data.range = range.into();
        self
    }

    /// 设置数据值
    ///
    /// # 参数
    ///
    /// * `values` - 要写入的数据值
    pub fn values(mut self, values: Vec<Vec<serde_json::Value>>) -> Self {
        self.data.values = values;
        self
    }

    /// 从字符串矩阵构建数据
    ///
    /// # 参数
    ///
    /// * `string_values` - 字符串二维数组
    pub fn from_string_matrix(mut self, string_values: Vec<Vec<String>>) -> Self {
        self.data.values = string_values
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|value| serde_json::Value::String(value))
                    .collect()
            })
            .collect();
        self
    }

    /// 从混合类型数据构建
    ///
    /// # 参数
    ///
    /// * `mixed_values` - 混合类型的二维数组
    pub fn from_mixed_values(mut self, mixed_values: Vec<Vec<impl Into<serde_json::Value>>>) -> Self {
        self.data.values = mixed_values
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|value| value.into())
                    .collect()
            })
            .collect();
        self
    }

    /// 构建范围数据对象
    pub fn build(self) -> MultiRangeValueData {
        self.data
    }
}

/// 向多个范围写入数据响应数据
///
/// 包含写入操作的结果信息。
#[derive(Debug, Clone, Deserialize)]
pub struct WriteDataToMultipleRangesResponseData {
    /// 写入结果列表
    #[serde(rename = "writes")]
    pub writes: Vec<WriteResult>,
}

impl ApiResponseTrait for WriteDataToMultipleRangesResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 写入结果
///
/// 表示单个范围写入操作的结果。
#[derive(Debug, Clone, Deserialize)]
pub struct WriteResult {
    /// 写入范围
    pub range: String,
    /// 写入的行数
    pub rows: i32,
    /// 写入的列数
    pub columns: i32,
    /// 写入的单元格数量
    pub cells: i32,
    /// 更新时间
    #[serde(default)]
    pub updated_time: Option<String>,
}

impl WriteResult {
    /// 获取写入的数据量
    pub fn total_cells(&self) -> i32 {
        self.cells
    }

    /// 检查写入是否成功（非零数据量表示成功）
    pub fn is_successful(&self) -> bool {
        self.cells > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_multi_range_value_data_builder() {
        let range_data = MultiRangeValueData::builder()
            .range("Sheet1!A1:C2")
            .from_string_matrix(vec![
                vec!["姓名".to_string(), "年龄".to_string(), "部门".to_string()],
                vec!["张三".to_string(), "25".to_string(), "技术部".to_string()],
            ])
            .build();

        assert_eq!(range_data.range, "Sheet1!A1:C2");
        assert_eq!(range_data.row_count(), 2);
        assert_eq!(range_data.column_count(), 3);
        assert!(!range_data.is_empty());
    }

    #[test]
    fn test_multi_range_value_data_from_mixed_values() {
        let range_data = MultiRangeValueData::builder()
            .range("Sheet1!A1:B2")
            .from_mixed_values(vec![
                vec!["姓名", 25, true],
                vec!["张三", 30, false],
            ])
            .build();

        assert_eq!(range_data.range, "Sheet1!A1:B2");
        assert_eq!(range_data.values[0][0], json!("姓名"));
        assert_eq!(range_data.values[0][1], json!(25));
        assert_eq!(range_data.values[0][2], json!(true));
    }

    #[test]
    fn test_write_data_to_multiple_ranges_request_builder() {
        let value_ranges = vec![
            MultiRangeValueData::builder()
                .range("Sheet1!A1:C1")
                .from_string_matrix(vec![vec!["标题1", "标题2", "标题3"]])
                .build(),
            MultiRangeValueData::builder()
                .range("Sheet1!A2:C2")
                .from_string_matrix(vec![vec!["数据1", "数据2", "数据3"]])
                .build(),
        ];

        let request = WriteDataToMultipleRangesRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .value_ranges(value_ranges)
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "shtcnmBA*****yGehy8");
        assert_eq!(request.range_count(), 2);
        assert_eq!(request.total_cell_count(), 6);
    }

    #[test]
    fn test_write_data_to_multiple_ranges_request_validation() {
        // 测试空spreadsheet_token验证
        let request = WriteDataToMultipleRangesRequest {
            api_request: Default::default(),
            spreadsheet_token: "".to_string(),
            value_ranges: vec![],
        };

        assert!(request.validate().is_err());

        // 测试空value_ranges验证
        let request = WriteDataToMultipleRangesRequest {
            api_request: Default::default(),
            spreadsheet_token: "valid_token".to_string(),
            value_ranges: vec![],
        };

        assert!(request.validate().is_err());

        // 测试范围数量超限验证
        let too_many_ranges = (0..501).map(|i| {
            MultiRangeValueData::builder()
                .range(format!("Sheet1!A{}:C{}", i + 1, i + 1))
                .from_string_matrix(vec![vec!["test"]])
                .build()
        }).collect();

        let request = WriteDataToMultipleRangesRequest {
            api_request: Default::default(),
            spreadsheet_token: "valid_token".to_string(),
            value_ranges: too_many_ranges,
        };

        assert!(request.validate().is_err());

        // 测试无效范围格式验证
        let invalid_range = MultiRangeValueData::builder()
            .range("A1:C10") // 缺少工作表名称
            .from_string_matrix(vec![vec!["test"]])
            .build();

        let request = WriteDataToMultipleRangesRequest {
            api_request: Default::default(),
            spreadsheet_token: "valid_token".to_string(),
            value_ranges: vec![invalid_range],
        };

        assert!(request.validate().is_err());

        // 测试重复范围验证
        let duplicate_range = MultiRangeValueData::builder()
            .range("Sheet1!A1:C1")
            .from_string_matrix(vec![vec!["test"]])
            .build();

        let request = WriteDataToMultipleRangesRequest {
            api_request: Default::default(),
            spreadsheet_token: "valid_token".to_string(),
            value_ranges: vec![duplicate_range.clone(), duplicate_range],
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_write_data_to_multiple_ranges_response_deserialization() {
        let json = json!({
            "writes": [
                {
                    "range": "Sheet1!A1:C1",
                    "rows": 1,
                    "columns": 3,
                    "cells": 3,
                    "updated_time": "2023-01-01T12:00:00Z"
                },
                {
                    "range": "Sheet1!A2:C2",
                    "rows": 1,
                    "columns": 3,
                    "cells": 3,
                    "updated_time": "2023-01-01T12:00:01Z"
                }
            ]
        });

        let response: WriteDataToMultipleRangesResponseData = serde_json::from_value(json).unwrap();

        assert_eq!(response.writes.len(), 2);

        let write1 = &response.writes[0];
        assert_eq!(write1.range, "Sheet1!A1:C1");
        assert_eq!(write1.rows, 1);
        assert_eq!(write1.columns, 3);
        assert_eq!(write1.cells, 3);
        assert!(write1.is_successful());
        assert_eq!(write1.total_cells(), 3);

        let write2 = &response.writes[1];
        assert_eq!(write2.range, "Sheet1!A2:C2");
        assert_eq!(write2.updated_time, Some("2023-01-01T12:00:01Z".to_string()));
    }

    #[test]
    fn test_empty_multi_range_value_data() {
        let empty_data = MultiRangeValueData {
            range: "Sheet1!A1:A1".to_string(),
            values: vec![],
        };

        assert!(empty_data.is_empty());
        assert_eq!(empty_data.row_count(), 0);
        assert_eq!(empty_data.column_count(), 0);
    }

    #[test]
    fn test_write_data_to_multiple_ranges_request_add_value_range() {
        let mut builder = WriteDataToMultipleRangesRequest::builder()
            .spreadsheet_token("test_token");

        let range1 = MultiRangeValueData::builder()
            .range("Sheet1!A1:B1")
            .from_string_matrix(vec![vec!["标题1", "标题2"]])
            .build();

        let range2 = MultiRangeValueData::builder()
            .range("Sheet1!A2:B2")
            .from_string_matrix(vec![vec!["数据1", "数据2"]])
            .build();

        let request = builder
            .add_value_range(range1)
            .add_value_range(range2)
            .build()
            .unwrap();

        assert_eq!(request.range_count(), 2);
        assert_eq!(request.total_cell_count(), 4);
    }
}
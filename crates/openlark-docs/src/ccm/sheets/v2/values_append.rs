//! 数据追加服务
//!
//! 提供电子表格数据追加功能，包括：
//! - 向指定范围之前插入数据行
//! - 遇到空行时覆盖或追加数据行
//! - 大数据量的高效追加操作
//! - 自动行检测和智能追加

use serde_json::Value;
use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints_original::Endpoints,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use openlark_core::trait_system::Service;

/// 数据追加服务
///
/// 提供电子表格数据的高效追加功能，支持智能的行插入和数据追加。
/// 通过空行检测机制，可以实现灵活的数据写入和日志记录功能。
///
/// # 功能特性
///
/// ## 追加模式
/// - **前插模式**: 在指定范围之前插入新的数据行
/// - **覆盖追加**: 遇到空行时覆盖，否则追加新行
/// - **智能检测**: 自动检测空行位置，优化写入策略
///
/// ## 数据处理
/// - **批量操作**: 支持一次性追加大量数据
/// - **格式保持**: 保持原有格式和样式
/// - **类型安全**: 完整的数据验证和错误处理
///
/// # 常见应用场景
///
/// ```rust
/// # use open_lark::prelude::*;
/// # use open_lark::service::sheets::v2::ValuesAppendService;
///
/// let service = ValuesAppendService::new(client_config);
///
/// // 场景1: 追加销售记录数据
/// let append_request = ValuesAppendRequest::builder()
///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
///     .range("Sheet1!A10:D100".to_string())
///     .add_row(vec![
///         "2025-01-08".to_string(),
///         "产品A".to_string(),
///         "100".to_string(),
///         "客户001".to_string()
///     ])
///     .add_row(vec![
///         "2025-01-08".to_string(),
///         "产品B".to_string(),
///         "200".to_string(),
///         "客户002".to_string()
///     ])
///     .build()?;
///
/// let response = service.append(&append_request).await?;
///
/// // 场景2: 追加日志记录
/// let log_request = ValuesAppendRequest::builder()
///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
///     .range("日志!A2:B1000".to_string())
///     .add_row(vec![
///         chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
///         "系统启动完成".to_string()
///     ])
///     .build()?;
///
/// let log_response = service.append(&log_request).await?;
/// ```
#[derive(Clone, Debug)]
pub struct ValuesAppendService {
    config: openlark_core::config::Config,
}

/// 数据追加请求
///
/// 用于向电子表格指定范围追加数据行。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesAppendRequest {
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: SpreadsheetToken,
    /// 目标范围
    pub range: String,
    /// 追加的数据
    pub values: Vec<Vec<String>>,
}

impl ValuesAppendRequest {
    /// 创建数据追加请求构建器
    pub fn builder() -> ValuesAppendRequestBuilder {
        ValuesAppendRequestBuilder::new()
    }
}

/// 数据追加请求构建器
pub struct ValuesAppendRequestBuilder {
    spreadsheet_token: Option<SpreadsheetToken>,
    range: Option<String>,
    values: Vec<Vec<String>>,
}

impl ValuesAppendRequestBuilder {
    /// 创建新的数据追加请求构建器
    pub fn new() -> Self {
        Self {
            spreadsheet_token: None,
            range: None,
            values: vec![],
        }
    }

    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::new(spreadsheet_token));
        self
    }

    /// 设置目标范围
    pub fn range(mut self, range: String) -> Self {
        self.range = Some(range);
        self
    }

    /// 添加一行数据
    pub fn add_row(mut self, row: Vec<String>) -> Self {
        self.values.push(row);
        self
    }

    /// 添加多行数据
    pub fn add_rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.values.extend(rows);
        self
    }

    /// 设置所有数据
    pub fn values(mut self, values: Vec<Vec<String>>) -> Self {
        self.values = values;
        self
    }

    /// 从HashMap添加数据（行列转换）
    pub fn from_hashmap(mut self, data: HashMap<String, Vec<String>>) -> Self {
        if data.is_empty() {
            return self;
        }

        // 获取所有键（列标题）并排序
        let mut columns: Vec<&String> = data.keys().collect();
        columns.sort();

        if columns.is_empty() {
            return self;
        }

        // 确定行数（以最长的列为准）
        let max_rows = columns
            .iter()
            .map(|col| data.get(col).map_or(&vec![], |v| v.len()))
            .max()
            .unwrap_or(0);

        if max_rows == 0 {
            return self;
        }

        // 构建数据矩阵
        for row_index in 0..max_rows {
            let mut row = vec![];
            for column in &columns {
                let column_data = data.get(column).unwrap_or(&vec![]);
                if row_index < column_data.len() {
                    row.push(column_data[row_index].clone());
                } else {
                    row.push(String::new());
                }
            }
            self.values.push(row);
        }

        self
    }

    /// 从CSV格式字符串添加数据
    pub fn from_csv(mut self, csv_data: &str, has_header: bool) -> Self {
        if csv_data.is_empty() {
            return self;
        }

        let lines: Vec<&str> = csv_data.lines().collect();
        if lines.is_empty() {
            return self;
        }

        let start_index = if has_header { 1 } else { 0 };

        for line in lines.iter().skip(start_index) {
            if line.trim().is_empty() {
                // 跳过空行
                continue;
            }

            let row: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
            self.values.push(row);
        }

        self
    }

    /// 从二维数组添加数据
    pub fn from_array(mut self, array_data: &[&[&str]]) -> Self {
        for row_data in array_data {
            let row: Vec<String> = row_data.iter().map(|s| s.to_string()).collect();
            self.values.push(row);
        }
        self
    }

    /// 构建数据追加请求对象
    pub fn build(self) -> openlark_core::error::SDKResult<ValuesAppendRequest> {
        // 验证必需参数
        match &self.spreadsheet_token {
            Some(_) => {
                // 验证范围
                match &self.range {
                    Some(range) => {
                        if range.is_empty() {
                            return Err(openlark_core::error::LarkAPIError::InvalidParameter(
                                "目标范围不能为空".to_string(),
                            ));
                        }

                        // 验证范围格式
                        if !Self::is_valid_range(range) {
                            return Err(openlark_core::error::LarkAPIError::InvalidParameter(
                                format!("无效的范围格式: {}", range),
                            ));
                        }
                    }
                    _ => {
                        return Err(openlark_core::error::LarkAPIError::InvalidParameter(
                            "目标范围是必需的".to_string(),
                        ));
                    }
                }

                // 验证数据
                if self.values.is_empty() {
                    return Err(openlark_core::error::LarkAPIError::InvalidParameter(
                        "至少需要提供一行数据".to_string(),
                    ));
                }

                // 验证数据行数
                if self.values.len() > 5000 {
                    return Err(openlark_core::error::LarkAPIError::InvalidParameter(
                        "单次追加不能超过5000行数据".to_string(),
                    ));
                }

                // 验证数据列数
                if let Some(first_row) = self.values.first() {
                    if first_row.len() > 100 {
                        return Err(openlark_core::error::LarkAPIError::InvalidParameter(
                            "单行数据不能超过100列".to_string(),
                        ));
                    }

                    // 验证所有行的列数一致性
                    for (row_index, row) in self.values.iter().enumerate() {
                        if row.len() > first_row.len() + 10 {
                            return Err(openlark_core::error::LarkAPIError::InvalidParameter(
                                format!(
                                    "第{}行的列数({})超过首行列数({})过多，最多可超出10列",
                                    row_index + 1,
                                    row.len(),
                                    first_row.len()
                                ),
                            ));
                        }
                    }
                }

                // 验证单元格内容长度
                for (row_index, row) in self.values.iter().enumerate() {
                    for (col_index, cell) in row.iter().enumerate() {
                        if cell.len() > 50000 {
                            return Err(openlark_core::error::LarkAPIError::InvalidParameter(
                                format!(
                                    "第{}行第{}列的单元格内容过长，不能超过50000字符",
                                    row_index + 1,
                                    col_index + 1
                                ),
                            ));
                        }
                    }
                }

                Ok(ValuesAppendRequest {
                    spreadsheet_token: self.spreadsheet_token.unwrap(),
                    range: self.range.unwrap(),
                    values: self.values,
                })
            }
            _ => Err(openlark_core::error::LarkAPIError::InvalidParameter(
                "电子表格Token是必需的".to_string(),
            )),
        }
    }

    /// 验证范围格式
    fn is_valid_range(range: &str) -> bool {
        // 基本的范围格式验证
        // 支持格式：Sheet1!A1、Sheet1!A1:C10、A1:C10等
        let trimmed = range.trim();
        if trimmed.is_empty() {
            return false;
        }

        // 检查是否包含工作表名（可选）
        if trimmed.contains('!') {
            let parts: Vec<&str> = trimmed.split('!').collect();
            if parts.len() != 2 {
                return false;
            }
            // 验证单元格引用格式
            Self::is_valid_cell_reference(parts[1])
        } else {
            // 只验证单元格引用
            Self::is_valid_cell_reference(trimmed)
        }
    }

    /// 验证单元格引用格式
    fn is_valid_cell_reference(cell_ref: &str) -> bool {
        if cell_ref.is_empty() {
            return false;
        }

        // 简单的单元格引用验证
        // 支持格式：A1、B2、A1:C10、A:A等
        // 这里使用简化的验证，实际应用中可能需要更复杂的解析
        cell_ref
            .chars()
            .all(|c| c.is_alphanumeric() || c == ':' || c.is_whitespace())
    }
}

/// 数据追加响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesAppendResponse {
    /// 追加结果数据
    pub data: ValuesAppendResult,
}

/// 数据追加结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesAppendResult {
    /// 更新的范围
    #[serde(rename = "updatedRange")]
    pub updated_range: String,
    /// 电子表格Token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 更新的行数
    #[serde(rename = "updatedRows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updatedColumns")]
    pub updated_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "updatedCells")]
    pub updated_cells: i32,
    /// 表格ID
    #[serde(rename = "tableId")]
    pub table_id: Option<String>,
}

impl ValuesAppendService {
    /// 创建数据追加服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 追加数据到指定范围
    ///
    /// 向电子表格的指定范围追加数据行，支持智能的空行检测。
    ///
    /// # 参数
    /// - `request`: 数据追加请求，包含目标范围和数据
    ///
    /// # 返回
    /// 返回追加操作的详细结果信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::{ValuesAppendService, ValuesAppendRequest};
    ///
    /// let service = ValuesAppendService::new(client_config);
    ///
    /// // 追加单行数据
    /// let request = ValuesAppendRequest::builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .range("Sheet1!A10:D10".to_string())
    ///     .add_row(vec![
    ///         "2025-01-08".to_string(),
    ///         "张三".to_string(),
    ///         "技术部".to_string(),
    ///         "10000".to_string()
    ///     ])
    ///     .build()?;
    ///
    /// let response = service.append(&request).await?;
    ///
    /// println!("成功追加 {} 行", response.data.updated_rows);
    /// println!("更新范围: {}", response.data.updated_range);
    /// println!("更新单元格数: {}", response.data.updated_cells);
    /// ```
    ///
    /// # 追加逻辑
    /// - 在指定范围之前插入新数据行
    /// - 如果范围内存在空行，会覆盖空行
    /// - 如果范围内没有空行，会在范围之后追加新行
    /// - 每行数据以逗号分隔的单元格值形式提供
    ///
    /// # 数据格式
    /// - 所有数据都会以字符串形式处理
    /// - 空字符串对应空单元格
    /// - 支持中文、英文、数字等所有字符
    ///
    /// # 性能限制
    /// - 单次最多追加5000行数据
    /// - 单行最多100列数据
    /// - 单个单元格最多50000字符
    ///
    /// # 错误处理
    /// - 如果电子表格不存在，返回相应错误
    /// - 如果范围格式无效，返回相应错误
    /// - 如果数据格式错误，返回相应错误
    /// - 如果数据量超限，返回相应错误
    pub async fn append(
        &self,
        request: &ValuesAppendRequest,
    ) -> openlark_core::error::SDKResult<Response<ValuesAppendResponse>> {
        let url = format!(
            "{}/open-apis/sheets/v2/spreadsheets/{}/values_append",
            self.config.base_url,
            request.spreadsheet_token.as_str()
        );

        let response = self
            .config
            .transport
            .post(&url)
            .json(request)
            .send()
            .await?;

        let base_resp: Response<ValuesAppendResponse> = response.json().await?;

        if let Some(err) = &base_resp.error {
            return Err(openlark_core::error::LarkAPIError::LarkAPIError(
                err.clone()
            ));
        }

        Ok(base_resp)
    }

    /// 数据追加构建器
    ///
    /// 提供链式调用的构建器模式，便于快速构建数据追加请求。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::ValuesAppendService;
    ///
    /// let service = ValuesAppendService::new(client_config);
    ///
    /// let response = service
    ///     .append_builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .range("Sheet1!A10:D100")
    ///     .add_row(vec!["日期", "产品", "数量", "价格"])
    ///     .add_row(vec!["2025-01-08", "产品A", "100", "299"])
    ///     .add_row(vec!["2025-01-08", "产品B", "200", "399"])
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn append_builder(&self) -> ValuesAppendServiceBuilder<'_> {
        ValuesAppendServiceBuilder {
            service: self,
            spreadsheet_token: None,
            range: None,
            values: vec![],
        }
    }

    /// 从HashMap批量追加构建器
    ///
    /// 适用于结构化数据的快速追加。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::ValuesAppendService;
    /// use std::collections::HashMap;
    ///
    /// let service = ValuesAppendService::new(client_config);
    ///
    /// let mut data = HashMap::new();
    /// data.insert("姓名".to_string(), vec!["张三".to_string(), "李四".to_string()]);
    /// data.insert("年龄".to_string(), vec!["25".to_string(), "30".to_string()]);
    /// data.insert("部门".to_string(), vec!["技术部".to_string(), "市场部".to_string()]);
    ///
    /// let response = service
    ///     .append_from_hashmap_builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .range("数据表!A1:C10")
    ///     .data(data)
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn append_from_hashmap_builder(&self) -> ValuesAppendServiceBuilder<'_> {
        ValuesAppendServiceBuilder {
            service: self,
            spreadsheet_token: None,
            range: None,
            values: vec![],
        }
    }

    /// 从CSV字符串追加构建器
    ///
    /// 适用于CSV格式数据的快速导入。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::ValuesAppendService;
    ///
    /// let service = ValuesAppendService::new(client_config);
    ///
    /// let csv_data = "姓名,年龄,部门\n张三,25,技术部\n李四,30,市场部";
    ///
    /// let response = service
    ///     .append_from_csv_builder()
    ///     .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
    ///     .range("数据表!A1:C100")
    ///     .csv_data(csv_data)
    ///     .has_header(true)
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn append_from_csv_builder(&self) -> ValuesAppendServiceBuilder<'_> {
        ValuesAppendServiceBuilder {
            service: self,
            spreadsheet_token: None,
            range: None,
            values: vec![],
        }
    }
}

/// 数据追加服务构建器
///
/// 提供流畅的API用于构建数据追加请求。
pub struct ValuesAppendServiceBuilder<'a> {
    service: &'a ValuesAppendService,
    spreadsheet_token: Option<SpreadsheetToken>,
    range: Option<String>,
    values: Vec<Vec<String>>,
}

impl<'a> ValuesAppendServiceBuilder<'a> {
    /// 设置电子表格Token
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(SpreadsheetToken::new(spreadsheet_token));
        self
    }

    /// 设置目标范围
    pub fn range(mut self, range: &str) -> Self {
        self.range = Some(range.to_string());
        self
    }

    /// 添加一行数据
    pub fn add_row(mut self, row: Vec<&str>) -> Self {
        let string_row: Vec<String> = row.into_iter().map(|s| s.to_string()).collect();
        self.values.push(string_row);
        self
    }

    /// 添加多行数据
    pub fn add_rows(mut self, rows: Vec<Vec<&str>>) -> Self {
        for row_data in rows {
            let string_row: Vec<String> = row_data.into_iter().map(|s| s.to_string()).collect();
            self.values.push(string_row);
        }
        self
    }

    /// 设置所有数据
    pub fn values(mut self, values: Vec<Vec<String>>) -> Self {
        self.values = values;
        self
    }

    /// 从HashMap添加数据（列优先）
    pub fn from_hashmap(mut self, data: HashMap<String, Vec<String>>) -> Self {
        if data.is_empty() {
            return self;
        }

        // 获取所有键（列标题）并排序
        let mut columns: Vec<&String> = data.keys().collect();
        columns.sort();

        if columns.is_empty() {
            return self;
        }

        // 确定行数（以最长的列为准）
        let max_rows = columns
            .iter()
            .map(|col| data.get(col).map_or(&vec![], |v| v.len()))
            .max()
            .unwrap_or(0);

        if max_rows == 0 {
            return self;
        }

        // 构建数据矩阵
        for row_index in 0..max_rows {
            let mut row = vec![];
            for column in &columns {
                let column_data = data.get(column).unwrap_or(&vec![]);
                if row_index < column_data.len() {
                    row.push(column_data[row_index].clone());
                } else {
                    row.push(String::new());
                }
            }
            self.values.push(row);
        }

        self
    }

    /// 从CSV格式字符串添加数据
    pub fn from_csv(mut self, csv_data: &str, has_header: bool) -> Self {
        if csv_data.is_empty() {
            return self;
        }

        let lines: Vec<&str> = csv_data.lines().collect();
        if lines.is_empty() {
            return self;
        }

        let start_index = if has_header { 1 } else { 0 };

        for line in lines.iter().skip(start_index) {
            if line.trim().is_empty() {
                // 跳过空行
                continue;
            }

            let row: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
            self.values.push(row);
        }

        self
    }

    /// 从二维数组添加数据
    pub fn from_array(mut self, array_data: &[&[&str]]) -> Self {
        for row_data in array_data {
            let row: Vec<String> = row_data.iter().map(|s| s.to_string()).collect();
            self.values.push(row);
        }
        self
    }

    /// 执行追加操作
    pub async fn execute(self) -> openlark_core::error::SDKResult<ValuesAppendResponse> {
        match (self.spreadsheet_token, self.range) {
            (Some(spreadsheet_token), Some(range)) => {
                let request = ValuesAppendRequest {
                    spreadsheet_token,
                    range,
                    values: self.values,
                };
                self.service.append(&request).await
            }
            _ => Err(openlark_core::error::LarkAPIError::InvalidParameter(
                "电子表格Token和目标范围都是必需的".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;
    use std::collections::HashMap;

    #[test]
    fn test_values_append_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = ValuesAppendService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_values_append_request_builder() {
        let request = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("Sheet1!A10:D100".to_string())
            .add_row(vec![
                "2025-01-08".to_string(),
                "张三".to_string(),
                "技术部".to_string(),
                "10000".to_string(),
            ])
            .add_row(vec![
                "2025-01-08".to_string(),
                "李四".to_string(),
                "市场部".to_string(),
                "15000".to_string(),
            ])
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.spreadsheet_token.as_str(), "shtcnmBRWQKbsJRHXXXXXXXXXX");
        assert_eq!(req.range, "Sheet1!A10:D100");
        assert_eq!(req.values.len(), 2);
        assert_eq!(req.values[0].len(), 4);
        assert_eq!(req.values[1].len(), 4);
    }

    #[test]
    fn test_values_append_request_validation() {
        // 测试缺少电子表格Token
        let no_token_request = ValuesAppendRequest::builder()
            .range("Sheet1!A10:D10".to_string())
            .add_row(vec!["test".to_string()])
            .build();
        assert!(no_token_request.is_err());

        // 测试缺少范围
        let no_range_request = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .add_row(vec!["test".to_string()])
            .build();
        assert!(no_range_request.is_err());

        // 测试空范围
        let empty_range = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("".to_string())
            .add_row(vec!["test".to_string()])
            .build();
        assert!(empty_range.is_err());

        // 测试无效范围格式
        let invalid_range = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("invalid range".to_string())
            .add_row(vec!["test".to_string()])
            .build();
        assert!(invalid_range.is_err());

        // 测试没有数据
        let no_data = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("Sheet1!A10:D10".to_string())
            .build();
        assert!(no_data.is_err());

        // 测试行数过多
        let too_many_rows = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("Sheet1!A10:D10".to_string());
        let mut builder = too_many_rows;
        for _ in 0..5001 {
            builder = builder.add_row(vec!["test".to_string()]);
        }
        assert!(builder.build().is_err());

        // 测试列数过多
        let too_many_cols = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("Sheet1!A10:D10".to_string())
            .add_row((0..101).map(|i| i.to_string()).collect::<Vec<_>>()) // 101列
            .build();
        assert!(too_many_cols.is_err());

        // 测试单元格内容过长
        let long_content = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("Sheet1!A1:A1".to_string())
            .add_row(vec![("x".repeat(50001))]); // 50001个字符
        assert!(long_content.is_err());
    }

    #[test]
    fn test_range_validation() {
        // 测试有效的单元格引用
        assert!(ValuesAppendRequestBuilder::is_valid_cell_reference("A1"));
        assert!(ValuesAppendRequestBuilder::is_valid_cell_reference("B10"));
        assert!(ValuesAppendRequestBuilder::is_valid_cell_reference(
            "A1:C10"
        ));
        assert!(ValuesAppendRequestBuilder::is_valid_cell_reference(
            "Sheet1!A1"
        ));

        // 测试无效的单元格引用
        assert!(!ValuesAppendRequestBuilder::is_valid_cell_reference(""));
        assert!(!ValuesAppendRequestBuilder::is_valid_cell_reference("!@#$"));
        assert!(!ValuesAppendRequestBuilder::is_valid_cell_reference(
            "A1:C10:D20:E30"
        ));

        // 测试有效的范围
        assert!(ValuesAppendRequestBuilder::is_valid_range("Sheet1!A1"));
        assert!(ValuesAppendRequestBuilder::is_valid_range("A1:C10"));
        assert!(ValuesAppendRequestBuilder::is_valid_range("数据表!A1:D100"));

        // 测试无效的范围
        assert!(!ValuesAppendRequestBuilder::is_valid_range(""));
        assert!(!ValuesAppendRequestBuilder::is_valid_range("invalid!@#$"));
    }

    #[test]
    fn test_values_append_service_builder() {
        let config = openlark_core::config::Config::default();
        let service = ValuesAppendService::new(config);

        // 测试基本构建器
        let builder = service
            .append_builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("Sheet1!A10:D100")
            .add_row(vec!["标题1", "标题2", "标题3", "标题4"])
            .add_row(vec!["数据1", "数据2", "数据3", "数据4"]);

        assert_eq!(
            builder.spreadsheet_token.unwrap().as_str(),
            "shtcnmBRWQKbsJRHXXXXXXXXXX"
        );
        assert_eq!(builder.range.unwrap(), "Sheet1!A10:D100");
        assert_eq!(builder.values.len(), 2);

        // 测试HashMap构建器
        let mut data = HashMap::new();
        data.insert(
            "姓名".to_string(),
            vec!["张三".to_string(), "李四".to_string()],
        );
        data.insert("年龄".to_string(), vec!["25".to_string(), "30".to_string()]);

        let hashmap_builder = service
            .append_from_hashmap_builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("数据表!A1:C10")
            .data(data);

        assert_eq!(hashmap_builder.values.len(), 2); // 2行数据

        // 测试CSV构建器
        let csv_data = "姓名,年龄,部门\n张三,25,技术部\n李四,30,市场部";
        let csv_builder = service
            .append_from_csv_builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("人员表!A1:C10")
            .csv_data(csv_data)
            .has_header(true);

        assert_eq!(csv_builder.values.len(), 2); // 2行数据（跳过标题行）
    }

    #[test]
    fn test_data_conversion_methods() {
        // 测试HashMap转换
        let mut data = HashMap::new();
        data.insert(
            "产品".to_string(),
            vec!["产品A".to_string(), "产品B".to_string()],
        );
        data.insert(
            "销量".to_string(),
            vec!["100".to_string(), "200".to_string()],
        );
        data.insert("价格".to_string(), vec!["299".to_string()]);

        let request = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("数据表!A1:C3")
            .from_hashmap(data)
            .build();

        assert!(request.is_ok());
        let req = request.unwrap();
        assert_eq!(req.values.len(), 3); // 3行数据
        assert_eq!(req.values[0][0], "产品"); // 第一列：产品
        assert_eq!(req.values[0][1], "销量"); // 第二列：销量

        // 测试CSV转换
        let csv_data = "A,B,C\n1,2,3\n4,5,6";
        let csv_request = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("数据表!A1:C3")
            .from_csv(csv_data, false)
            .build();

        assert!(csv_request.is_ok());
        let csv_req = csv_request.unwrap();
        assert_eq!(csv_req.values.len(), 3);
        assert_eq!(csv_req.values[0][0], "A");
        assert_eq!(csv_req.values[1][1], "2");

        // 测试数组转换
        let array_data = &[&["X", "Y", "Z"], &[&["1", "2", "3"], &["4", "5", "6"]]];
        let array_request = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("数据表!A1:C3")
            .from_array(array_data)
            .build();

        assert!(array_request.is_ok());
        let array_req = array_request.unwrap();
        assert_eq!(array_req.values.len(), 3);
        assert_eq!(array_req.values[0][0], "X");
        assert_eq!(array_req.values[2][2], "6");
    }

    #[test]
    fn test_complete_append_workflow() {
        let config = openlark_core::config::Config::default();
        let service = ValuesAppendService::new(config);

        // 场景1: 销售数据记录
        let sales_data = vec![
            vec!["日期", "产品", "数量", "单价", "总金额"],
            vec!["2025-01-08", "产品A", "100", "299", "29900"],
            vec!["2025-01-08", "产品B", "200", "399", "79800"],
            vec!["2025-01-08", "产品C", "150", "199", "29850"],
        ];

        let sales_request = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("销售数据!A1:E100")
            .values(sales_data)
            .build()
            .unwrap();

        assert_eq!(sales_request.values.len(), 4);
        assert_eq!(sales_request.range, "销售数据!A1:E100");

        // 场景2: 日志记录（从HashMap转换）
        let mut log_data = HashMap::new();
        log_data.insert("时间".to_string(), vec!["2025-01-08 10:00:00".to_string()]);
        log_data.insert("事件".to_string(), vec!["系统启动".to_string()]);
        log_data.insert("状态".to_string(), vec!["成功".to_string()]);
        log_data.insert("详情".to_string(), vec!["所有服务正常运行".to_string()]);

        let log_request = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("系统日志!A2:D2")
            .from_hashmap(log_data)
            .build()
            .unwrap();

        assert_eq!(log_request.values.len(), 1);
        assert_eq!(log_request.values[0][0], "时间");

        // 场景3: CSV数据导入
        let csv_content = "员工编号,姓名,部门,入职日期\nEMP001,张三,技术部,2024-01-15\nEMP002,李四,市场部,2024-02-20";
        let csv_request = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("员工信息!A2:D100")
            .from_csv(csv_content, true)
            .build()
            .unwrap();

        assert_eq!(csv_request.values.len(), 2); // 2行数据（跳过标题行）

        // 场景4: 性能测试 - 大量数据
        let mut large_data = vec![];
        for i in 0..1000 {
            large_data.push(vec![
                format!("行{}", i + 1),
                format!("数据{}", i + 1),
                format!("值{}", i + 1),
                format!("备注{}", i + 1),
            ]);
        }

        let large_request = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("大数据测试!A1:D1000")
            .values(large_data)
            .build()
            .unwrap();

        assert_eq!(large_request.values.len(), 1000);
        assert_eq!(large_request.values[999][3], "备注1000");

        // 验证所有请求都能正确构建
        assert!(sales_request.values.len() > 0);
        assert!(log_request.values.len() > 0);
        assert!(csv_request.values.len() > 0);
        assert!(large_request.values.len() == 1000);
    }

    #[test]
    fn test_edge_cases_and_error_handling() {
        // 测试空字符串处理
        let empty_string = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("Sheet1!A1:A1")
            .from_csv("", false)
            .build();

        assert!(empty_string.is_ok());
        assert_eq!(empty_string.unwrap().values.len(), 0);

        // 测试只有标题行的CSV
        let header_only_csv = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKBSJRHXXXXXXXXXX".to_string())
            .range("Sheet1!A1:D1")
            .from_csv("A,B,C,D", true)
            .build();

        assert!(header_only_csv.is_ok());
        assert_eq!(header_only_csv.unwrap().values.len(), 0);

        // 测试包含空行的CSV
        let csv_with_empty = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKBSJRHXXXXXXXXXX".to_string())
            .range("Sheet1!A1:C5")
            .from_csv("A,B,C\n\nD,E,F\nG,H,I", false)
            .build();

        assert!(csv_with_empty.is_ok());
        assert_eq!(csv_with_empty.unwrap().values.len(), 2); // 2有效行

        // 测试列数不一致的HashMap
        let mut uneven_data = HashMap::new();
        uneven_data.insert(
            "A".to_string(),
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
        );
        uneven_data.insert("B".to_string(), vec!["4".to_string()]); // 较短的列

        let uneven_request = ValuesAppendRequest::builder()
            .spreadsheet_token("shtcnmBRWQKbsJRHXXXXXXXXXX".to_string())
            .range("Sheet1!A1:B2")
            .from_hashmap(uneven_data)
            .build();

        assert!(uneven_request.is_ok());
        let req = uneven_request.unwrap();
        assert_eq!(req.values.len(), 3); // 以最长的列为准
        assert_eq!(req.values[1].len(), 3); // 第二行用空字符串补齐
        assert_eq!(req.values[2].len(), 3); // 第三行用空字符串补齐
    }
}

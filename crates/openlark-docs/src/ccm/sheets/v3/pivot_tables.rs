//! Sheets电子表格数据透视表服务 v3
//!
//! 提供飞书电子表格v3版本的数据透视表管理功能，包括：
//! - 创建和删除数据透视表
//! - 行列字段配置
//! - 值字段汇总方式
//! - 筛选器和布局设置

use serde_json::Value;
use openlark_core::error::LarkAPIError;

// 使用统一类型定义
use super::Range;

use serde::{Deserialize, Serialize};

/// 汇总函数类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SummaryFunction {
    /// 求和
    #[serde(rename = "SUM")]
    Sum,
    /// 计数
    #[serde(rename = "COUNT")]
    Count,
    /// 计数非空
    #[serde(rename = "COUNTA")]
    CountA,
    /// 平均值
    #[serde(rename = "AVERAGE")]
    Average,
    /// 最大值
    #[serde(rename = "MAX")]
    Max,
    /// 最小值
    #[serde(rename = "MIN")]
    Min,
    /// 乘积
    #[serde(rename = "PRODUCT")]
    Product,
    /// 标准差
    #[serde(rename = "STDEV")]
    StDev,
    /// 总体标准差
    #[serde(rename = "STDEVP")]
    StDevP,
    /// 方差
    #[serde(rename = "VAR")]
    Var,
    /// 总体方差
    #[serde(rename = "VARP")]
    VarP,
}

/// 排序方式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    /// 升序
    #[serde(rename = "ASC")]
    Asc,
    /// 降序
    #[serde(rename = "DESC")]
    Desc,
}

/// 透视表字段配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotField {
    /// 字段名称
    #[serde(rename = "name")]
    pub name: String,
    /// 字段来源列索引（从0开始）
    #[serde(rename = "source_column")]
    pub source_column: u32,
    /// 是否显示小计
    #[serde(rename = "show_subtotals")]
    pub show_subtotals: Option<bool>,
    /// 排序方式
    #[serde(rename = "sort_order")]
    pub sort_order: Option<SortOrder>,
    /// 自定义名称
    #[serde(rename = "custom_name")]
    pub custom_name: Option<String>,
}

impl PivotField {
    /// 创建透视表字段
    pub fn new(name: String, source_column: u32) -> Self {
        Self {
            name,
            source_column,
            show_subtotals: None,
            sort_order: None,
            custom_name: None,
        }
    }

    /// 设置是否显示小计
    pub fn show_subtotals(mut self, show: bool) -> Self {
        self.show_subtotals = Some(show);
        self
    }

    /// 设置排序方式
    pub fn sort_order(mut self, order: SortOrder) -> Self {
        self.sort_order = Some(order);
        self
    }

    /// 设置自定义名称
    pub fn custom_name(mut self, name: String) -> Self {
        self.custom_name = Some(name);
        self
    }
}

/// 值字段配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueField {
    /// 字段名称
    #[serde(rename = "name")]
    pub name: String,
    /// 字段来源列索引（从0开始）
    #[serde(rename = "source_column")]
    pub source_column: u32,
    /// 汇总函数
    #[serde(rename = "summary_function")]
    pub summary_function: SummaryFunction,
    /// 数字格式
    #[serde(rename = "number_format")]
    pub number_format: Option<String>,
    /// 自定义名称
    #[serde(rename = "custom_name")]
    pub custom_name: Option<String>,
}

impl ValueField {
    /// 创建值字段
    pub fn new(name: String, source_column: u32, summary_function: SummaryFunction) -> Self {
        Self {
            name,
            source_column,
            summary_function,
            number_format: None,
            custom_name: None,
        }
    }

    /// 设置数字格式
    pub fn number_format(mut self, format: String) -> Self {
        self.number_format = Some(format);
        self
    }

    /// 设置自定义名称
    pub fn custom_name(mut self, name: String) -> Self {
        self.custom_name = Some(name);
        self
    }
}

/// 筛选器字段配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterField {
    /// 字段名称
    #[serde(rename = "name")]
    pub name: String,
    /// 字段来源列索引（从0开始）
    #[serde(rename = "source_column")]
    pub source_column: u32,
    /// 筛选值列表
    #[serde(rename = "values")]
    pub values: Option<Vec<String>>,
    /// 是否多选
    #[serde(rename = "multiple_selections")]
    pub multiple_selections: Option<bool>,
}

impl FilterField {
    /// 创建筛选器字段
    pub fn new(name: String, source_column: u32) -> Self {
        Self {
            name,
            source_column,
            values: None,
            multiple_selections: None,
        }
    }

    /// 设置筛选值
    pub fn values(mut self, values: Vec<String>) -> Self {
        self.values = Some(values);
        self
    }

    /// 设置是否多选
    pub fn multiple_selections(mut self, multiple: bool) -> Self {
        self.multiple_selections = Some(multiple);
        self
    }
}

/// 数据透视表布局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotTableLayout {
    /// 是否显示行总计
    #[serde(rename = "show_row_grand_totals")]
    pub show_row_grand_totals: Option<bool>,
    /// 是否显示列总计
    #[serde(rename = "show_column_grand_totals")]
    pub show_column_grand_totals: Option<bool>,
    /// 是否显示行标题
    #[serde(rename = "show_row_headers")]
    pub show_row_headers: Option<bool>,
    /// 是否显示列标题
    #[serde(rename = "show_column_headers")]
    pub show_column_headers: Option<bool>,
    /// 合并单元格标签
    #[serde(rename = "merge_labels")]
    pub merge_labels: Option<bool>,
}

impl PivotTableLayout {
    /// 创建默认布局
    pub fn new() -> Self {
        Self {
            show_row_grand_totals: None,
            show_column_grand_totals: None,
            show_row_headers: None,
            show_column_headers: None,
            merge_labels: None,
        }
    }

    /// 设置是否显示行总计
    pub fn show_row_grand_totals(mut self, show: bool) -> Self {
        self.show_row_grand_totals = Some(show);
        self
    }

    /// 设置是否显示列总计
    pub fn show_column_grand_totals(mut self, show: bool) -> Self {
        self.show_column_grand_totals = Some(show);
        self
    }

    /// 设置是否显示行标题
    pub fn show_row_headers(mut self, show: bool) -> Self {
        self.show_row_headers = Some(show);
        self
    }

    /// 设置是否显示列标题
    pub fn show_column_headers(mut self, show: bool) -> Self {
        self.show_column_headers = Some(show);
        self
    }

    /// 设置是否合并标签
    pub fn merge_labels(mut self, merge: bool) -> Self {
        self.merge_labels = Some(merge);
        self
    }
}

/// 数据透视表位置配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotTablePosition {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 起始行
    #[serde(rename = "start_row")]
    pub start_row: u32,
    /// 起始列
    #[serde(rename = "start_column")]
    pub start_column: u32,
}

impl PivotTablePosition {
    /// 创建透视表位置
    pub fn new(sheet_id: String, start_row: u32, start_column: u32) -> Self {
        Self {
            sheet_id,
            start_row,
            start_column,
        }
    }
}

/// 数据透视表配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotTableConfig {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 数据源范围
    #[serde(rename = "source_range")]
    pub source_range: Range,
    /// 透视表位置
    #[serde(rename = "position")]
    pub position: PivotTablePosition,
    /// 行字段
    #[serde(rename = "row_fields")]
    pub row_fields: Vec<PivotField>,
    /// 列字段
    #[serde(rename = "column_fields")]
    pub column_fields: Vec<PivotField>,
    /// 值字段
    #[serde(rename = "value_fields")]
    pub value_fields: Vec<ValueField>,
    /// 筛选器字段
    #[serde(rename = "filter_fields")]
    pub filter_fields: Vec<FilterField>,
    /// 布局配置
    #[serde(rename = "layout")]
    pub layout: Option<PivotTableLayout>,
}

impl PivotTableConfig {
    /// 创建数据透视表配置
    pub fn new(sheet_id: String, source_range: Range, position: PivotTablePosition) -> Self {
        Self {
            sheet_id,
            source_range,
            position,
            row_fields: vec![],
            column_fields: vec![],
            value_fields: vec![],
            filter_fields: vec![],
            layout: None,
        }
    }

    /// 添加行字段
    pub fn add_row_field(mut self, field: PivotField) -> Self {
        self.row_fields.push(field);
        self
    }

    /// 添加列字段
    pub fn add_column_field(mut self, field: PivotField) -> Self {
        self.column_fields.push(field);
        self
    }

    /// 添加值字段
    pub fn add_value_field(mut self, field: ValueField) -> Self {
        self.value_fields.push(field);
        self
    }

    /// 添加筛选器字段
    pub fn add_filter_field(mut self, field: FilterField) -> Self {
        self.filter_fields.push(field);
        self
    }

    /// 设置布局
    pub fn layout(mut self, layout: PivotTableLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    /// 验证透视表配置
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.sheet_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "工作表ID不能为空".to_string(),
            ));
        }

        if self.value_fields.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "至少需要一个值字段".to_string(),
            ));
        }

        if self.row_fields.is_empty() && self.column_fields.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "至少需要一个行字段或列字段".to_string(),
            ));
        }

        // 验证字段配置
        for field in &self.row_fields {
            if field.name.is_empty() {
                return Err(LarkAPIError::IllegalParamError(
                    "行字段名称不能为空".to_string(),
                ));
            }
        }

        for field in &self.column_fields {
            if field.name.is_empty() {
                return Err(LarkAPIError::IllegalParamError(
                    "列字段名称不能为空".to_string(),
                ));
            }
        }

        for field in &self.value_fields {
            if field.name.is_empty() {
                return Err(LarkAPIError::IllegalParamError(
                    "值字段名称不能为空".to_string(),
                ));
            }
        }

        for field in &self.filter_fields {
            if field.name.is_empty() {
                return Err(LarkAPIError::IllegalParamError(
                    "筛选器字段名称不能为空".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// 创建数据透视表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePivotTableRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 透视表配置
    #[serde(rename = "pivot_table_config")]
    pub pivot_table_config: PivotTableConfig,
}

impl CreatePivotTableRequest {
    /// 创建创建数据透视表请求
    pub fn new(spreadsheet_token: String, pivot_table_config: PivotTableConfig) -> Self {
        Self {
            spreadsheet_token,
            pivot_table_config,
        }
    }

    /// 构建器模式
    pub fn builder() -> CreatePivotTableRequestBuilder {
        CreatePivotTableRequestBuilder::default()
    }
}

/// 创建数据透视表请求构建器
#[derive(Debug, Default)]
pub struct CreatePivotTableRequestBuilder {
    spreadsheet_token: Option<String>,
    sheet_id: Option<String>,
    source_range: Option<Range>,
    position_sheet_id: Option<String>,
    position_start_row: Option<u32>,
    position_start_column: Option<u32>,
    row_fields: Vec<PivotField>,
    column_fields: Vec<PivotField>,
    value_fields: Vec<ValueField>,
    filter_fields: Vec<FilterField>,
    layout: Option<PivotTableLayout>,
}

impl CreatePivotTableRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置源工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(sheet_id);
        self
    }

    /// 设置数据源范围
    pub fn source_range(mut self, source_range: Range) -> Self {
        self.source_range = Some(source_range);
        self
    }

    /// 设置透视表位置
    pub fn position(mut self, sheet_id: String, start_row: u32, start_column: u32) -> Self {
        self.position_sheet_id = Some(sheet_id);
        self.position_start_row = Some(start_row);
        self.position_start_column = Some(start_column);
        self
    }

    /// 添加行字段
    pub fn add_row_field(mut self, field: PivotField) -> Self {
        self.row_fields.push(field);
        self
    }

    /// 添加列字段
    pub fn add_column_field(mut self, field: PivotField) -> Self {
        self.column_fields.push(field);
        self
    }

    /// 添加值字段
    pub fn add_value_field(mut self, field: ValueField) -> Self {
        self.value_fields.push(field);
        self
    }

    /// 添加筛选器字段
    pub fn add_filter_field(mut self, field: FilterField) -> Self {
        self.filter_fields.push(field);
        self
    }

    /// 设置布局
    pub fn layout(mut self, layout: PivotTableLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<CreatePivotTableRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("源工作表ID不能为空".to_string()))?;

        let source_range = self
            .source_range
            .ok_or_else(|| LarkAPIError::IllegalParamError("数据源范围不能为空".to_string()))?;

        let position_sheet_id = self.position_sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("透视表位置工作表ID不能为空".to_string())
        })?;

        let position_start_row = self
            .position_start_row
            .ok_or_else(|| LarkAPIError::IllegalParamError("透视表起始行不能为空".to_string()))?;

        let position_start_column = self
            .position_start_column
            .ok_or_else(|| LarkAPIError::IllegalParamError("透视表起始列不能为空".to_string()))?;

        let position =
            PivotTablePosition::new(position_sheet_id, position_start_row, position_start_column);

        let mut pivot_table_config = PivotTableConfig::new(sheet_id, source_range, position);

        for field in self.row_fields {
            pivot_table_config = pivot_table_config.add_row_field(field);
        }
        for field in self.column_fields {
            pivot_table_config = pivot_table_config.add_column_field(field);
        }
        for field in self.value_fields {
            pivot_table_config = pivot_table_config.add_value_field(field);
        }
        for field in self.filter_fields {
            pivot_table_config = pivot_table_config.add_filter_field(field);
        }
        if let Some(layout) = self.layout {
            pivot_table_config = pivot_table_config.layout(layout);
        }

        pivot_table_config.validate()?;

        Ok(CreatePivotTableRequest {
            spreadsheet_token,
            pivot_table_config,
        })
    }
}

/// 创建数据透视表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePivotTableResponse {
    /// 透视表ID
    #[serde(rename = "pivot_table_id")]
    pub pivot_table_id: String,
    /// 透视表配置
    #[serde(rename = "pivot_table_config")]
    pub pivot_table_config: PivotTableConfig,
}

/// 删除数据透视表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePivotTableRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 透视表ID
    #[serde(rename = "pivot_table_id")]
    pub pivot_table_id: String,
}

impl DeletePivotTableRequest {
    /// 创建删除数据透视表请求
    pub fn new(spreadsheet_token: String, pivot_table_id: String) -> Self {
        Self {
            spreadsheet_token,
            pivot_table_id,
        }
    }

    /// 构建器模式
    pub fn builder() -> DeletePivotTableRequestBuilder {
        DeletePivotTableRequestBuilder::default()
    }
}

/// 删除数据透视表请求构建器
#[derive(Debug, Default)]
pub struct DeletePivotTableRequestBuilder {
    spreadsheet_token: Option<String>,
    pivot_table_id: Option<String>,
}

impl DeletePivotTableRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置透视表ID
    pub fn pivot_table_id(mut self, pivot_table_id: String) -> Self {
        self.pivot_table_id = Some(pivot_table_id);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<DeletePivotTableRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let pivot_table_id = self
            .pivot_table_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("透视表ID不能为空".to_string()))?;

        Ok(DeletePivotTableRequest {
            spreadsheet_token,
            pivot_table_id,
        })
    }
}

/// 删除数据透视表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePivotTableResponse {
    /// 是否成功删除
    pub success: bool,
}

/// Sheets电子表格数据透视表服务 v3
#[derive(Clone, Debug)]
pub struct PivotTableService {
    config: openlark_core::config::Config,
}

impl PivotTableService {
    /// 创建数据透视表服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 创建数据透视表
    ///
    /// 在指定工作表中创建数据透视表，支持行列字段、值字段和筛选器配置。
    ///
    /// # 参数
    /// - `request`: 创建数据透视表请求
    ///
    /// # 返回
    /// 返回创建后的数据透视表信息，包括透视表ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::pivot_tables::*;
    /// use open_lark::service::sheets::v3::models::Range;
    ///
    /// // 创建数据源范围
    /// let source_range = Range::from("A1".to_string(), "D100".to_string());
    ///
    /// // 创建行字段
    /// let row_field = PivotField::new("部门".to_string(), 0)
    ///     .show_subtotals(true)
    ///     .sort_order(SortOrder::Asc);
    ///
    /// // 创建列字段
    /// let column_field = PivotField::new("季度".to_string(), 1);
    ///
    /// // 创建值字段
    /// let value_field = ValueField::new(
    ///     "销售额".to_string(),
    ///     2,
    ///     SummaryFunction::Sum
    /// ).custom_name("总销售额".to_string());
    ///
    /// // 创建透视表
    /// let request = CreatePivotTableRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .sheet_id("source_sheet".to_string())
    ///     .source_range(source_range)
    ///     .position("target_sheet".to_string(), 1, 1)
    ///     .add_row_field(row_field)
    ///     .add_column_field(column_field)
    ///     .add_value_field(value_field)
    ///     .layout(PivotTableLayout::new()
    ///         .show_row_grand_totals(true)
    ///         .show_column_grand_totals(true))
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.create_pivot_table(&request).await?;
    /// ```
    pub async fn create_pivot_table(
        &self,
        request: &CreatePivotTableRequest,
    ) -> openlark_core::error::SDKResult<CreatePivotTableResponse> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/pivot_tables",
            self.config.base_url, request.spreadsheet_token
        );
        // 发送HTTP请求
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| LarkAPIError::RequestError(e.to_string()))?;

        // 解析响应
        let create_response: openlark_core::api::Response<CreatePivotTableResponse> =
            serde_json::from_str(
                &response
                    .text()
                    .await
                    .map_err(|e| LarkAPIError::RequestError(e.to_string()))?,
            )?;

        if create_response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: create_response.code(),
                msg: create_response.msg().to_string(),
                error: None,
            });
        }

        create_response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 删除数据透视表
    ///
    /// 删除指定的数据透视表。
    ///
    /// # 参数
    /// - `request`: 删除数据透视表请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::pivot_tables::*;
    ///
    /// // 删除透视表
    /// let request = DeletePivotTableRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .pivot_table_id("pivot_table_id".to_string())
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.delete_pivot_table(&request).await?;
    /// assert!(response.success);
    /// ```
    pub async fn delete_pivot_table(
        &self,
        request: &DeletePivotTableRequest,
    ) -> openlark_core::error::SDKResult<DeletePivotTableResponse> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/pivot_tables/{}",
            self.config.base_url, request.spreadsheet_token, request.pivot_table_id
        );
        // 发送HTTP请求
        let client = reqwest::Client::new();
        let response = client
            .delete(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| LarkAPIError::RequestError(e.to_string()))?;

        // 解析响应
        let delete_response: openlark_core::api::Response<DeletePivotTableResponse> =
            serde_json::from_str(
                &response
                    .text()
                    .await
                    .map_err(|e| LarkAPIError::RequestError(e.to_string()))?,
            )?;

        if delete_response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: delete_response.code(),
                msg: delete_response.msg().to_string(),
                error: None,
            });
        }

        delete_response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 创建数据透视表构建器
    pub fn create_pivot_table_builder(&self) -> CreatePivotTableRequestBuilder {
        CreatePivotTableRequestBuilder::default()
    }

    /// 删除数据透视表构建器
    pub fn delete_pivot_table_builder(&self) -> DeletePivotTableRequestBuilder {
        DeletePivotTableRequestBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_field_creation() {
        let field = PivotField::new("部门".to_string(), 0)
            .show_subtotals(true)
            .sort_order(SortOrder::Asc)
            .custom_name("部门分类".to_string());

        assert_eq!(field.name, "部门");
        assert_eq!(field.source_column, 0);
        assert_eq!(field.show_subtotals, Some(true));
        assert_eq!(field.sort_order, Some(SortOrder::Asc));
        assert_eq!(field.custom_name, Some("部门分类".to_string()));
    }

    #[test]
    fn test_value_field_creation() {
        let field = ValueField::new("销售额".to_string(), 2, SummaryFunction::Sum)
            .number_format("#,##0.00".to_string())
            .custom_name("总销售额".to_string());

        assert_eq!(field.name, "销售额");
        assert_eq!(field.source_column, 2);
        assert!(matches!(field.summary_function, SummaryFunction::Sum));
        assert_eq!(field.number_format, Some("#,##0.00".to_string()));
        assert_eq!(field.custom_name, Some("总销售额".to_string()));
    }

    #[test]
    fn test_filter_field_creation() {
        let field = FilterField::new("地区".to_string(), 3)
            .values(vec!["北京".to_string(), "上海".to_string()])
            .multiple_selections(true);

        assert_eq!(field.name, "地区");
        assert_eq!(field.source_column, 3);
        assert_eq!(
            field.values,
            Some(vec!["北京".to_string(), "上海".to_string()])
        );
        assert_eq!(field.multiple_selections, Some(true));
    }

    #[test]
    fn test_pivot_table_layout_creation() {
        let layout = PivotTableLayout::new()
            .show_row_grand_totals(true)
            .show_column_grand_totals(true)
            .show_row_headers(true)
            .show_column_headers(true)
            .merge_labels(false);

        assert_eq!(layout.show_row_grand_totals, Some(true));
        assert_eq!(layout.show_column_grand_totals, Some(true));
        assert_eq!(layout.show_row_headers, Some(true));
        assert_eq!(layout.show_column_headers, Some(true));
        assert_eq!(layout.merge_labels, Some(false));
    }

    #[test]
    fn test_pivot_table_position_creation() {
        let position = PivotTablePosition::new("sheet123".to_string(), 1, 1);

        assert_eq!(position.sheet_id, "sheet123");
        assert_eq!(position.start_row, 1);
        assert_eq!(position.start_column, 1);
    }

    #[test]
    fn test_pivot_table_config_creation() {
        use super::super::models::Range;

        let source_range = Range::from("A1".to_string(), "D100".to_string());
        let position = PivotTablePosition::new("sheet123".to_string(), 1, 1);
        let row_field = PivotField::new("部门".to_string(), 0);
        let column_field = PivotField::new("季度".to_string(), 1);
        let value_field = ValueField::new("销售额".to_string(), 2, SummaryFunction::Sum);
        let layout = PivotTableLayout::new();

        let config =
            openlark_core::config::Config::new("sheet123".to_string(), source_range, position)
                .add_row_field(row_field)
                .add_column_field(column_field)
                .add_value_field(value_field)
                .layout(layout);

        assert_eq!(config.sheet_id, "sheet123");
        assert_eq!(config.row_fields.len(), 1);
        assert_eq!(config.column_fields.len(), 1);
        assert_eq!(config.value_fields.len(), 1);
        assert!(config.layout.is_some());
    }

    #[test]
    fn test_pivot_table_config_validation() {
        use super::super::models::Range;

        let source_range = Range::from("A1".to_string(), "D100".to_string());
        let position = PivotTablePosition::new("sheet123".to_string(), 1, 1);
        let value_field = ValueField::new("销售额".to_string(), 2, SummaryFunction::Sum);

        // 测试有效配置
        let valid_config =
            openlark_core::config::Config::new("sheet123".to_string(), source_range, position)
                .add_row_field(PivotField::new("部门".to_string(), 0))
                .add_value_field(value_field);
        assert!(valid_config.validate().is_ok());

        // 测试空工作表ID
        let invalid_config = openlark_core::config::Config::new(
            "".to_string(),
            Range::from("A1".to_string(), "D100".to_string()),
            PivotTablePosition::new("sheet123".to_string(), 1, 1),
        )
        .add_row_field(PivotField::new("部门".to_string(), 0))
        .add_value_field(ValueField::new(
            "销售额".to_string(),
            2,
            SummaryFunction::Sum,
        ));
        assert!(invalid_config.validate().is_err());

        // 测试无值字段
        let invalid_config2 = openlark_core::config::Config::new(
            "sheet123".to_string(),
            Range::from("A1".to_string(), "D100".to_string()),
            PivotTablePosition::new("sheet123".to_string(), 1, 1),
        )
        .add_row_field(PivotField::new("部门".to_string(), 0));
        assert!(invalid_config2.validate().is_err());

        // 测试无行列字段
        let invalid_config3 = openlark_core::config::Config::new(
            "sheet123".to_string(),
            Range::from("A1".to_string(), "D100".to_string()),
            PivotTablePosition::new("sheet123".to_string(), 1, 1),
        )
        .add_value_field(ValueField::new(
            "销售额".to_string(),
            2,
            SummaryFunction::Sum,
        ));
        assert!(invalid_config3.validate().is_err());
    }

    #[test]
    fn test_create_pivot_table_request_builder() {
        use super::super::models::Range;

        let request = CreatePivotTableRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("source_sheet".to_string())
            .source_range(Range::from("A1".to_string(), "D100".to_string()))
            .position("target_sheet".to_string(), 1, 1)
            .add_row_field(PivotField::new("部门".to_string(), 0))
            .add_column_field(PivotField::new("季度".to_string(), 1))
            .add_value_field(ValueField::new(
                "销售额".to_string(),
                2,
                SummaryFunction::Sum,
            ))
            .layout(PivotTableLayout::new())
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.pivot_table_config.sheet_id, "source_sheet");
        assert_eq!(request.pivot_table_config.row_fields.len(), 1);
        assert_eq!(request.pivot_table_config.column_fields.len(), 1);
        assert_eq!(request.pivot_table_config.value_fields.len(), 1);
        assert!(request.pivot_table_config.layout.is_some());
    }

    #[test]
    fn test_delete_pivot_table_request() {
        let request = DeletePivotTableRequest::new("token123".to_string(), "pivot123".to_string());

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.pivot_table_id, "pivot123");
    }

    #[test]
    fn test_delete_pivot_table_request_builder() {
        let request = DeletePivotTableRequest::builder()
            .spreadsheet_token("token123".to_string())
            .pivot_table_id("pivot123".to_string())
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.pivot_table_id, "pivot123");
    }

    #[test]
    fn test_pivot_table_service_creation() {
        let config = openlark_core::config::openlark_core::config::Config::default();
        let service = PivotTableService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_comprehensive_pivot_table_scenarios() {
        use super::super::models::Range;

        // 测试简单透视表场景
        let simple_request = CreatePivotTableRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("source_sheet".to_string())
            .source_range(Range::from("A1".to_string(), "C50".to_string()))
            .position("target_sheet".to_string(), 1, 1)
            .add_row_field(PivotField::new("部门".to_string(), 0))
            .add_value_field(ValueField::new(
                "销售额".to_string(),
                2,
                SummaryFunction::Sum,
            ))
            .build()
            .unwrap();

        assert_eq!(simple_request.pivot_table_config.row_fields.len(), 1);
        assert_eq!(simple_request.pivot_table_config.value_fields.len(), 1);

        // 测试复杂透视表场景
        let complex_request = CreatePivotTableRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("source_sheet".to_string())
            .source_range(Range::from("A1".to_string(), "F200".to_string()))
            .position("target_sheet".to_string(), 1, 1)
            .add_row_field(PivotField::new("部门".to_string(), 0).show_subtotals(true))
            .add_row_field(PivotField::new("产品".to_string(), 1))
            .add_column_field(PivotField::new("季度".to_string(), 2))
            .add_value_field(
                ValueField::new("销售额".to_string(), 3, SummaryFunction::Sum)
                    .custom_name("总销售额".to_string()),
            )
            .add_value_field(
                ValueField::new("利润".to_string(), 4, SummaryFunction::Average)
                    .number_format("#,##0.00".to_string()),
            )
            .add_filter_field(
                FilterField::new("地区".to_string(), 5)
                    .values(vec!["北京".to_string(), "上海".to_string()]),
            )
            .layout(
                PivotTableLayout::new()
                    .show_row_grand_totals(true)
                    .show_column_grand_totals(true)
                    .merge_labels(true),
            )
            .build()
            .unwrap();

        assert_eq!(complex_request.pivot_table_config.row_fields.len(), 2);
        assert_eq!(complex_request.pivot_table_config.column_fields.len(), 1);
        assert_eq!(complex_request.pivot_table_config.value_fields.len(), 2);
        assert_eq!(complex_request.pivot_table_config.filter_fields.len(), 1);
        assert!(complex_request.pivot_table_config.layout.is_some());
    }

    #[test]
    fn test_summary_functions() {
        let functions = vec![
            SummaryFunction::Sum,
            SummaryFunction::Count,
            SummaryFunction::CountA,
            SummaryFunction::Average,
            SummaryFunction::Max,
            SummaryFunction::Min,
            SummaryFunction::Product,
            SummaryFunction::StDev,
            SummaryFunction::StDevP,
            SummaryFunction::Var,
            SummaryFunction::VarP,
        ];

        for function in functions {
            let field = ValueField::new("test".to_string(), 0, function);
            assert_eq!(field.source_column, 0);
        }
    }

    #[test]
    fn test_pivot_table_serialization() {
        use serde_json;

        let config = openlark_core::config::Config::new(
            "sheet123".to_string(),
            super::super::models::Range::from("A1".to_string(), "D100".to_string()),
            PivotTablePosition::new("sheet123".to_string(), 1, 1),
        );
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("sheet123"));
    }

    #[test]
    fn test_sort_order() {
        let field = PivotField::new("测试".to_string(), 0).sort_order(SortOrder::Desc);

        assert_eq!(field.sort_order, Some(SortOrder::Desc));
    }
}

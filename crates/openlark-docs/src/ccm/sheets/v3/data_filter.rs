//! Sheets电子表格数据过滤器服务 v3
//!
//! 提供飞书电子表格v3版本的数据过滤器管理功能，包括：
//! - 元数据过滤器设置和删除
//! - 数据范围过滤和条件匹配
//! - 多条件组合过滤
//! - 过滤器状态管理

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    error::LarkAPIError,
    http::Transport,
};

// 使用统一类型定义
use super::Range;

use reqwest::Method;
use serde::{Deserialize, Serialize};

// v3模块核心类型定义
pub type SpreadsheetToken = String;
pub type SheetId = String;
pub type CellValue = serde_json::Value;
pub type SheetPagedResponse<T> = Vec<T>;

/// 过滤条件操作符
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    /// 等于
    #[serde(rename = "EQUAL")]
    Equal,
    /// 不等于
    #[serde(rename = "NOT_EQUAL")]
    NotEqual,
    /// 包含
    #[serde(rename = "CONTAINS")]
    Contains,
    /// 不包含
    #[serde(rename = "NOT_CONTAINS")]
    NotContains,
    /// 开始于
    #[serde(rename = "STARTS_WITH")]
    StartsWith,
    /// 结束于
    #[serde(rename = "ENDS_WITH")]
    EndsWith,
    /// 大于
    #[serde(rename = "GREATER")]
    Greater,
    /// 大于等于
    #[serde(rename = "GREATER_EQUAL")]
    GreaterEqual,
    /// 小于
    #[serde(rename = "LESS")]
    Less,
    /// 小于等于
    #[serde(rename = "LESS_EQUAL")]
    LessEqual,
    /// 介于之间
    #[serde(rename = "BETWEEN")]
    Between,
    /// 不介于之间
    #[serde(rename = "NOT_BETWEEN")]
    NotBetween,
    /// 为空
    #[serde(rename = "IS_EMPTY")]
    IsEmpty,
    /// 不为空
    #[serde(rename = "IS_NOT_EMPTY")]
    IsNotEmpty,
}

/// 过滤条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// 条件标题
    pub title: String,
    /// 条件值列表（支持多值）
    pub values: Vec<String>,
    /// 过滤操作符
    #[serde(rename = "operator")]
    pub filter_operator: FilterOperator,
}

impl FilterCondition {
    /// 创建新的过滤条件
    pub fn new(title: String, values: Vec<String>, filter_operator: FilterOperator) -> Self {
        Self {
            title,
            values,
            filter_operator,
        }
    }

    /// 创建文本相等条件
    pub fn text_equals(title: String, value: String) -> Self {
        Self::new(title, vec![value], FilterOperator::Equal)
    }

    /// 创建文本包含条件
    pub fn text_contains(title: String, value: String) -> Self {
        Self::new(title, vec![value], FilterOperator::Contains)
    }

    /// 创建数值范围条件
    pub fn number_between(title: String, min: String, max: String) -> Self {
        Self::new(title, vec![min, max], FilterOperator::Between)
    }

    /// 创建空值条件
    pub fn is_empty(title: String) -> Self {
        Self::new(title, vec![], FilterOperator::IsEmpty)
    }

    /// 创建非空值条件
    pub fn is_not_empty(title: String) -> Self {
        Self::new(title, vec![], FilterOperator::IsNotEmpty)
    }
}

/// 数据过滤器设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFilter {
    /// 过滤器范围
    #[serde(rename = "filter_range")]
    pub filter_range: Range,
    /// 排序范围
    #[serde(rename = "sort_range")]
    pub sort_range: Option<Range>,
    /// 过滤条件列表
    #[serde(rename = "conditions")]
    pub filter_conditions: Vec<FilterCondition>,
    /// 关联的工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
}

impl DataFilter {
    /// 创建新的数据过滤器
    pub fn new(
        sheet_id: String,
        filter_range: Range,
        filter_conditions: Vec<FilterCondition>,
    ) -> Self {
        Self {
            filter_range,
            sort_range: None,
            filter_conditions,
            sheet_id,
        }
    }

    /// 添加过滤条件
    pub fn add_condition(mut self, condition: FilterCondition) -> Self {
        self.filter_conditions.push(condition);
        self
    }

    /// 设置排序范围
    pub fn with_sort_range(mut self, sort_range: Range) -> Self {
        self.sort_range = Some(sort_range);
        self
    }

    /// 验证过滤器配置
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.sheet_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "工作表ID不能为空".to_string(),
            ));
        }

        if self.filter_conditions.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "至少需要一个过滤条件".to_string(),
            ));
        }

        // 验证每个条件
        for condition in &self.filter_conditions {
            if condition.title.is_empty() {
                return Err(LarkAPIError::IllegalParamError(
                    "条件标题不能为空".to_string(),
                ));
            }

            // 检查操作符和值的匹配
            match condition.filter_operator {
                FilterOperator::IsEmpty | FilterOperator::IsNotEmpty => {
                    if !condition.values.is_empty() {
                        return Err(LarkAPIError::IllegalParamError(format!(
                            "操作符 {:?} 不应该有条件值",
                            condition.filter_operator
                        )));
                    }
                }
                FilterOperator::Between | FilterOperator::NotBetween => {
                    if condition.values.len() != 2 {
                        return Err(LarkAPIError::IllegalParamError(
                            "Between操作符需要两个条件值".to_string(),
                        ));
                    }
                }
                _ => {
                    if condition.values.is_empty() {
                        return Err(LarkAPIError::IllegalParamError(format!(
                            "操作符 {:?} 需要至少一个条件值",
                            condition.filter_operator
                        )));
                    }
                }
            }
        }

        Ok(())
    }
}

/// 设置数据过滤器请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDataFilterRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 数据过滤器设置
    #[serde(rename = "data_filter")]
    pub data_filter: DataFilter,
}

impl SetDataFilterRequest {
    /// 创建设置数据过滤器请求
    pub fn new(spreadsheet_token: String, data_filter: DataFilter) -> Self {
        Self {
            spreadsheet_token,
            data_filter,
        }
    }

    /// 构建器模式
    pub fn builder() -> SetDataFilterRequestBuilder {
        SetDataFilterRequestBuilder::default()
    }
}

/// 设置数据过滤器请求构建器
#[derive(Debug, Default)]
pub struct SetDataFilterRequestBuilder {
    spreadsheet_token: Option<String>,
    sheet_id: Option<String>,
    filter_range: Option<Range>,
    sort_range: Option<Range>,
    conditions: Vec<FilterCondition>,
}

impl SetDataFilterRequestBuilder {
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

    /// 设置过滤范围
    pub fn filter_range(mut self, filter_range: Range) -> Self {
        self.filter_range = Some(filter_range);
        self
    }

    /// 设置排序范围
    pub fn sort_range(mut self, sort_range: Range) -> Self {
        self.sort_range = Some(sort_range);
        self
    }

    /// 添加过滤条件
    pub fn add_condition(mut self, condition: FilterCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// 批量添加过滤条件
    pub fn conditions(mut self, conditions: Vec<FilterCondition>) -> Self {
        self.conditions.extend(conditions);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<SetDataFilterRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("工作表ID不能为空".to_string()))?;

        let filter_range = self
            .filter_range
            .ok_or_else(|| LarkAPIError::IllegalParamError("过滤范围不能为空".to_string()))?;

        let data_filter = DataFilter {
            filter_range,
            sort_range: self.sort_range,
            filter_conditions: self.conditions,
            sheet_id,
        };

        data_filter.validate()?;

        Ok(SetDataFilterRequest {
            spreadsheet_token,
            data_filter,
        })
    }
}

/// 设置数据过滤器响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDataFilterResponse {
    /// 过滤器ID
    #[serde(rename = "filter_id")]
    pub filter_id: String,
    /// 数据过滤器设置
    #[serde(rename = "data_filter")]
    pub data_filter: DataFilter,
}

/// 删除数据过滤器请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDataFilterRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
}

impl DeleteDataFilterRequest {
    /// 创建删除数据过滤器请求
    pub fn new(spreadsheet_token: String, sheet_id: String) -> Self {
        Self {
            spreadsheet_token,
            sheet_id,
        }
    }

    /// 构建器模式
    pub fn builder() -> DeleteDataFilterRequestBuilder {
        DeleteDataFilterRequestBuilder::default()
    }
}

/// 删除数据过滤器请求构建器
#[derive(Debug, Default)]
pub struct DeleteDataFilterRequestBuilder {
    spreadsheet_token: Option<String>,
    sheet_id: Option<String>,
}

impl DeleteDataFilterRequestBuilder {
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

    /// 构建请求对象
    pub fn build(self) -> Result<DeleteDataFilterRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("工作表ID不能为空".to_string()))?;

        Ok(DeleteDataFilterRequest {
            spreadsheet_token,
            sheet_id,
        })
    }
}

/// 删除数据过滤器响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDataFilterResponse {
    /// 是否成功删除
    pub success: bool,
}

/// Sheets电子表格数据过滤器服务 v3
#[derive(Clone, Debug)]
pub struct DataFilterService {
    config: openlark_core::config::Config,
}

impl DataFilterService {
    /// 创建数据过滤器服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 设置数据过滤器
    ///
    /// 在指定工作表中设置数据过滤器，支持多条件组合过滤。
    ///
    /// # 参数
    /// - `request`: 设置数据过滤器请求
    ///
    /// # 返回
    /// 返回设置后的数据过滤器信息，包括过滤器ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::data_filter::*;
    /// use open_lark::service::sheets::v3::models::Range;
    ///
    /// // 创建范围对象
    /// let filter_range = Range::from("A1".to_string(), "D10".to_string());
    ///
    /// // 创建过滤条件
    /// let condition1 = FilterCondition::text_equals("状态".to_string(), "已完成".to_string());
    /// let condition2 = FilterCondition::number_between("分数".to_string(), "80".to_string(), "100".to_string());
    ///
    /// // 设置过滤器
    /// let request = SetDataFilterRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .sheet_id("sheet_id".to_string())
    ///     .filter_range(filter_range)
    ///     .add_condition(condition1)
    ///     .add_condition(condition2)
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.set_data_filter(&request).await?;
    /// ```
    pub async fn set_data_filter(
        &self,
        request: &SetDataFilterRequest,
    ) -> openlark_core::error::SDKResult<SetDataFilterResponse> {
        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/data_filter",
            request.spreadsheet_token
        );

        // 创建HTTP请求并序列化请求体
        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        // 发送请求并获取响应
        let response =
            Transport::<SetDataFilterResponse>::request(api_request, &self.config, None).await?;

        // 检查响应是否成功
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

    /// 删除数据过滤器
    ///
    /// 删除指定工作表中的数据过滤器。
    ///
    /// # 参数
    /// - `request`: 删除数据过滤器请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::data_filter::*;
    ///
    /// // 删除过滤器
    /// let request = DeleteDataFilterRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .sheet_id("sheet_id".to_string())
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.delete_data_filter(&request).await?;
    /// assert!(response.success);
    /// ```
    pub async fn delete_data_filter(
        &self,
        request: &DeleteDataFilterRequest,
    ) -> openlark_core::error::SDKResult<DeleteDataFilterResponse> {
        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/data_filter",
            self.config.base_url, request.spreadsheet_token
        );

        // 构建带查询参数的URL
        let url_with_params = format!("{}?sheet_id={}", url, &request.sheet_id);

        // 创建HTTP请求
        let api_request = ApiRequest::with_method_and_path(Method::DELETE, &url_with_params);

        // 发送请求并获取响应
        let response =
            Transport::<DeleteDataFilterResponse>::request(api_request, &self.config, None).await?;

        // 检查响应是否成功
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

    /// 设置数据过滤器构建器
    pub fn set_data_filter_builder(&self) -> SetDataFilterRequestBuilder {
        SetDataFilterRequestBuilder::default()
    }

    /// 删除数据过滤器构建器
    pub fn delete_data_filter_builder(&self) -> DeleteDataFilterRequestBuilder {
        DeleteDataFilterRequestBuilder::default()
    }
}

// 为响应类型实现 ApiResponseTrait
impl ApiResponseTrait for SetDataFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteDataFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_condition_creation() {
        let condition = FilterCondition::text_equals("状态".to_string(), "已完成".to_string());

        assert_eq!(condition.title, "状态");
        assert_eq!(condition.values, vec!["已完成"]);
        assert!(matches!(condition.filter_operator, FilterOperator::Equal));
    }

    #[test]
    fn test_filter_condition_operators() {
        // 测试文本操作符
        let contains = FilterCondition::text_contains("名称".to_string(), "测试".to_string());
        assert!(matches!(contains.filter_operator, FilterOperator::Contains));

        // 测试数值操作符
        let between = FilterCondition::number_between(
            "分数".to_string(),
            "80".to_string(),
            "100".to_string(),
        );
        assert!(matches!(between.filter_operator, FilterOperator::Between));
        assert_eq!(between.values.len(), 2);

        // 测试空值操作符
        let empty = FilterCondition::is_empty("备注".to_string());
        assert!(matches!(empty.filter_operator, FilterOperator::IsEmpty));
        assert!(empty.values.is_empty());

        let not_empty = FilterCondition::is_not_empty("备注".to_string());
        assert!(matches!(
            not_empty.filter_operator,
            FilterOperator::IsNotEmpty
        ));
        assert!(not_empty.values.is_empty());
    }

    #[test]
    fn test_data_filter_creation() {
        use super::super::models::Range;

        let filter_range = Range::from("A1".to_string(), "D10".to_string());
        let condition = FilterCondition::text_equals("状态".to_string(), "已完成".to_string());

        let data_filter = DataFilter::new("sheet123".to_string(), filter_range, vec![condition]);

        assert_eq!(data_filter.sheet_id, "sheet123");
        assert_eq!(data_filter.filter_conditions.len(), 1);
        assert!(data_filter.sort_range.is_none());
    }

    #[test]
    fn test_data_filter_with_conditions() {
        use super::super::models::Range;

        let filter_range = Range::from("A1".to_string(), "D10".to_string());
        let condition1 = FilterCondition::text_equals("状态".to_string(), "已完成".to_string());
        let condition2 = FilterCondition::number_between(
            "分数".to_string(),
            "80".to_string(),
            "100".to_string(),
        );

        let data_filter = DataFilter::new("sheet123".to_string(), filter_range, vec![condition1])
            .add_condition(condition2);

        assert_eq!(data_filter.filter_conditions.len(), 2);
    }

    #[test]
    fn test_data_filter_with_sort_range() {
        use super::super::models::Range;

        let filter_range = Range::from("A1".to_string(), "D10".to_string());
        let sort_range = Range::from("A1".to_string(), "D1".to_string());
        let condition = FilterCondition::text_equals("状态".to_string(), "已完成".to_string());

        let data_filter = DataFilter::new("sheet123".to_string(), filter_range, vec![condition])
            .with_sort_range(sort_range);

        assert!(data_filter.sort_range.is_some());
    }

    #[test]
    fn test_data_filter_validation() {
        use super::super::models::Range;

        // 测试有效过滤器
        let filter_range = Range::from("A1".to_string(), "D10".to_string());
        let condition = FilterCondition::text_equals("状态".to_string(), "已完成".to_string());
        let data_filter = DataFilter::new("sheet123".to_string(), filter_range, vec![condition]);

        assert!(data_filter.validate().is_ok());

        // 测试空工作表ID
        let mut invalid_filter = data_filter.clone();
        invalid_filter.sheet_id = "".to_string();
        assert!(invalid_filter.validate().is_err());

        // 测试空条件列表
        let mut invalid_filter = data_filter;
        invalid_filter.filter_conditions = vec![];
        assert!(invalid_filter.validate().is_err());
    }

    #[test]
    fn test_filter_condition_validation() {
        use super::super::models::Range;

        let filter_range = Range::from("A1".to_string(), "D10".to_string());

        // 测试空标题
        let condition = FilterCondition::new(
            "".to_string(),
            vec!["值".to_string()],
            FilterOperator::Equal,
        );
        let data_filter = DataFilter::new("sheet123".to_string(), filter_range, vec![condition]);
        assert!(data_filter.validate().is_err());

        // 测试空条件值的Equal操作符
        let condition = FilterCondition::new("标题".to_string(), vec![], FilterOperator::Equal);
        let data_filter = DataFilter::new("sheet123".to_string(), filter_range, vec![condition]);
        assert!(data_filter.validate().is_err());

        // 测试Between操作符值数量错误
        let condition = FilterCondition::new(
            "标题".to_string(),
            vec!["80".to_string()],
            FilterOperator::Between,
        );
        let data_filter = DataFilter::new("sheet123".to_string(), filter_range, vec![condition]);
        assert!(data_filter.validate().is_err());

        // 测试IsEmpty操作符不应该有值
        let condition = FilterCondition::new(
            "标题".to_string(),
            vec!["值".to_string()],
            FilterOperator::IsEmpty,
        );
        let data_filter = DataFilter::new("sheet123".to_string(), filter_range, vec![condition]);
        assert!(data_filter.validate().is_err());
    }

    #[test]
    fn test_set_data_filter_request_builder() {
        use super::super::models::Range;

        let filter_range = Range::from("A1".to_string(), "D10".to_string());
        let condition = FilterCondition::text_equals("状态".to_string(), "已完成".to_string());

        let request = SetDataFilterRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .filter_range(filter_range)
            .add_condition(condition)
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.data_filter.sheet_id, "sheet123");
        assert_eq!(request.data_filter.filter_conditions.len(), 1);
    }

    #[test]
    fn test_set_data_filter_request_builder_error() {
        // 测试缺少必需参数
        let result = SetDataFilterRequest::builder().build();
        assert!(result.is_err());

        let result = SetDataFilterRequest::builder()
            .spreadsheet_token("token123".to_string())
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_data_filter_request() {
        let request = DeleteDataFilterRequest::new("token123".to_string(), "sheet123".to_string());

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
    }

    #[test]
    fn test_delete_data_filter_request_builder() {
        let request = DeleteDataFilterRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
    }

    #[test]
    fn test_delete_data_filter_request_builder_error() {
        let result = DeleteDataFilterRequest::builder().build();
        assert!(result.is_err());

        let result = DeleteDataFilterRequest::builder()
            .spreadsheet_token("token123".to_string())
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_data_filter_service_creation() {
        let config = openlark_core::config::openlark_core::config::Config::default();
        let service = DataFilterService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_comprehensive_filter_scenarios() {
        use super::super::models::Range;

        let filter_range = Range::from("A1".to_string(), "D10".to_string());

        // 测试多条件组合
        let condition1 = FilterCondition::text_equals("状态".to_string(), "已完成".to_string());
        let condition2 = FilterCondition::number_between(
            "分数".to_string(),
            "80".to_string(),
            "100".to_string(),
        );
        let condition3 = FilterCondition::is_not_empty("备注".to_string());

        let request = SetDataFilterRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .filter_range(filter_range)
            .conditions(vec![condition1, condition2, condition3])
            .build()
            .unwrap();

        assert_eq!(request.data_filter.filter_conditions.len(), 3);

        // 验证各条件类型
        let conditions = request.data_filter.filter_conditions;
        assert!(matches!(
            conditions[0].filter_operator,
            FilterOperator::Equal
        ));
        assert!(matches!(
            conditions[1].filter_operator,
            FilterOperator::Between
        ));
        assert!(matches!(
            conditions[2].filter_operator,
            FilterOperator::IsNotEmpty
        ));
    }

    #[test]
    fn test_data_filter_serialization() {
        use serde_json;

        let filter_range = super::super::models::Range::from("A1".to_string(), "D10".to_string());
        let condition = FilterCondition::text_equals("状态".to_string(), "已完成".to_string());
        let data_filter = DataFilter::new("sheet123".to_string(), filter_range, vec![condition]);

        let json = serde_json::to_string(&data_filter).unwrap();
        assert!(json.contains("sheet_id"));
        assert!(json.contains("filter_range"));
        assert!(json.contains("conditions"));
    }

    #[test]
    fn test_filter_operator_serialization() {
        use serde_json;

        let condition = FilterCondition::text_equals("状态".to_string(), "已完成".to_string());
        let json = serde_json::to_string(&condition).unwrap();
        assert!(json.contains("EQUAL"));

        let condition2 = FilterCondition::text_contains("名称".to_string(), "测试".to_string());
        let json2 = serde_json::to_string(&condition2).unwrap();
        assert!(json2.contains("CONTAINS"));
    }
}

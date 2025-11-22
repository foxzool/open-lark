//! Bitable V2 增强查询 API
//!
//! 提供智能查询和数据分析功能，包括：
//! - 聚合查询和统计分析
//! - 关联数据查询
//! - 智能推荐和预测
//! - 数据导出功能
use std::collections::HashMap;

use openlark_core::{
use serde_json::Value;
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 数据模型
pub mod models {
    use super::*;

    /// 智能查询请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct SmartQueryRequest {
        /// 应用token
        pub app_token: String,
        /// 数据表ID
        pub table_id: String,
        /// 查询类型
        pub query_type: SmartQueryType,
        /// 查询配置
        pub config: SmartQueryConfig,
        /// 过滤条件（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filters: Option<Vec<QueryFilter>>,
    }

    /// 智能查询类型
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum SmartQueryType {
        /// 聚合查询
        #[serde(rename = "aggregation")]
        Aggregation,
        /// 关联查询
        #[serde(rename = "correlation")]
        Correlation,
        /// 趋势分析
        #[serde(rename = "trend_analysis")]
        TrendAnalysis,
        /// 统计分析
        #[serde(rename = "statistics")]
        Statistics,
        /// 数据分组
        #[serde(rename = "grouping")]
        Grouping,
    }

    /// 智能查询配置
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct SmartQueryConfig {
        /// 聚合字段
        pub aggregate_fields: Vec<AggregateField>,
        /// 分组字段（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub group_by: Option<Vec<String>>,
        /// 排序规则（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sort: Option<Vec<SortRule>>,
        /// 分页信息（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<PageInfo>,
        /// 时间范围（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_range: Option<TimeRange>,
    }

    /// 聚合字段配置
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct AggregateField {
        /// 字段ID或字段名
        pub field_id_or_name: String,
        /// 聚合函数
        pub function: AggregateFunction,
        /// 输出别名（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub alias: Option<String>,
    }

    /// 聚合函数
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum AggregateFunction {
        /// 计数
        #[serde(rename = "count")]
        Count,
        /// 求和
        #[serde(rename = "sum")]
        Sum,
        /// 平均值
        #[serde(rename = "avg")]
        Average,
        /// 最大值
        #[serde(rename = "max")]
        Max,
        /// 最小值
        #[serde(rename = "min")]
        Min,
        /// 去重计数
        #[serde(rename = "count_distinct")]
        CountDistinct,
    }

    /// 查询过滤器
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct QueryFilter {
        /// 字段ID或字段名
        pub field_id_or_name: String,
        /// 操作符
        pub operator: FilterOperator,
        /// 值
        pub value: Value,
    }

    /// 过滤器操作符
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum FilterOperator {
        /// 等于
        #[serde(rename = "eq")]
        Equal,
        /// 不等于
        #[serde(rename = "neq")]
        NotEqual,
        /// 大于
        #[serde(rename = "gt")]
        GreaterThan,
        /// 大于等于
        #[serde(rename = "gte")]
        GreaterThanOrEqual,
        /// 小于
        #[serde(rename = "lt")]
        LessThan,
        /// 小于等于
        #[serde(rename = "lte")]
        LessThanOrEqual,
        /// 在范围内
        #[serde(rename = "in")]
        In,
        /// 包含
        #[serde(rename = "contains")]
        Contains,
    }

    /// 排序规则
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct SortRule {
        /// 字段ID或字段名
        pub field_id_or_name: String,
        /// 排序方向
        pub direction: SortDirection,
    }

    /// 排序方向
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum SortDirection {
        /// 升序
        #[serde(rename = "asc")]
        Ascending,
        /// 降序
        #[serde(rename = "desc")]
        Descending,
    }

    /// 分页信息
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct PageInfo {
        /// 页面大小
        pub page_size: i32,
        /// 页面token（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page_token: Option<String>,
    }

    /// 时间范围
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct TimeRange {
        /// 开始时间
        pub start_time: String,
        /// 结束时间
        pub end_time: String,
    }

    /// 智能查询响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct SmartQueryResponse {
        /// 查询结果
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<SmartQueryData>,
    }

    impl ApiResponseTrait for SmartQueryResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 查询结果数据
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct SmartQueryData {
        /// 结果数据
        #[serde(skip_serializing_if = "Option::is_none")]
        pub results: Option<Vec<QueryResult>>,
        /// 分页信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<PageInfo>,
        /// 统计信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub statistics: Option<QueryStatistics>,
        /// 查询元数据
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metadata: Option<QueryMetadata>,
    }

    /// 查询结果
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct QueryResult {
        /// 结果数据
        #[serde(skip_serializing_if = "Option::is_none")]
        pub values: Option<Value>,
        /// 分组值（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub group_values: Option<Vec<Value>>,
    }

    /// 查询统计信息
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct QueryStatistics {
        /// 总记录数
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_count: Option<i32>,
        /// 查询耗时（毫秒）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub execution_time: Option<i32>,
        /// 缓存命中
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cache_hit: Option<bool>,
    }

    /// 查询元数据
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct QueryMetadata {
        /// 查询ID
        #[serde(skip_serializing_if = "Option::is_none")]
        pub query_id: Option<String>,
        /// 查询计划
        #[serde(skip_serializing_if = "Option::is_none")]
        pub query_plan: Option<Value>,
        /// 性能指标
        #[serde(skip_serializing_if = "Option::is_none")]
        pub performance_metrics: Option<Value>,
    }
}

/// 服务实现
pub mod services {
    use super::*;

    /// 增强查询服务
    #[derive(Debug, Clone)]
    pub struct EnhancedQueryService {
        config: Config,
    }

    impl EnhancedQueryService {
        /// 创建增强查询服务实例
        pub fn new(config: Config) -> Self {
            Self { config }
        }

        /// 执行智能查询
        ///
        /// 执行复杂的数据分析和统计查询
        ///
        /// # 参数
        /// * `request` - 智能查询请求
        ///
        /// # 返回
        /// 返回查询结果和统计信息
        pub async fn execute_smart_query(
            &self,
            request: &models::SmartQueryRequest,
        ) -> SDKResult<models::SmartQueryResponse> {
            // 验证请求参数
            request
                .validate()
                .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

            log::info!(
                "执行智能查询: app_token={}, table_id={}, query_type={:?}",
                request.app_token,
                request.table_id,
                request.query_type
            );

            // 构建请求体
            let body = serde_json::to_value(request)?;

            // 构建API请求
            let api_req = ApiRequest::
                url: format!(
                url: format!(
                    "/open-apis/bitable/v2/apps/{}/tables/{}/smart_query",
                    request.app_token, request.table_id
                ),
                body: Some(serde_json::to_string(body: serde_json::to_vec(&body)?,body)?.into()),
                .query(HashMap::new())
                .query(HashMap::new())
            };

            // 发送请求
            let resp =
                Transport::<models::SmartQueryResponse>::request(api_req, &self.config, None)
                    .await?;
            let response = resp.data.unwrap_or_default();

            if let Some(ref data) = response.data {
                log::info!(
                    "智能查询完成: app_token={}, table_id={}, results={}",
                    request.app_token,
                    request.table_id,
                    data.results.as_ref().map(|r| r.len()).unwrap_or(0)
                );
            }

            Ok(response)
        }
    }
}

// 为请求结构体添加验证方法
impl models::SmartQueryRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.config.aggregate_fields.is_empty() {
            return Err("聚合字段不能为空".to_string());
        }
        if self.config.aggregate_fields.len() > 50 {
            return Err("聚合字段数量不能超过50个".to_string());
        }

        // 验证时间范围
        if let Some(ref time_range) = self.config.time_range {
            if time_range.start_time.trim().is_empty() || time_range.end_time.trim().is_empty() {
                return Err("时间范围开始时间和结束时间不能为空".to_string());
            }
        }

        // 验证分页信息
        if let Some(ref page) = self.config.page {
            if page.page_size <= 0 || page.page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }

        Ok(())
    }
}

// 重新导出
pub use models::*;
pub use services::EnhancedQueryService;

// 构建器模式实现
pub struct SmartQueryRequestBuilder {
    request: models::SmartQueryRequest,
}

impl SmartQueryRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        query_type: models::SmartQueryType,
    ) -> Self {
        Self {
            request: models::SmartQueryRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                query_type,
                config: models::SmartQueryConfig {
                    aggregate_fields: Vec::new(),
                    group_by: None,
                    sort: None,
                    page: None,
                    time_range: None,
                },
                filters: None,
            },
        }
    }

    pub fn aggregate_field(
        mut self,
        field_id_or_name: impl Into<String>,
        function: models::AggregateFunction,
    ) -> Self {
        self.request
            .config
            .aggregate_fields
            .push(models::AggregateField {
                field_id_or_name: field_id_or_name.into(),
                function,
                alias: None,
            });
        self
    }

    pub fn group_by(mut self, fields: Vec<String>) -> Self {
        self.request.config.group_by = Some(fields);
        self
    }

    pub fn sort(mut self, sort: Vec<models::SortRule>) -> Self {
        self.request.config.sort = Some(sort);
        self
    }

    pub fn page(mut self, page: models::PageInfo) -> Self {
        self.request.config.page = Some(page);
        self
    }

    pub fn time_range(mut self, time_range: models::TimeRange) -> Self {
        self.request.config.time_range = Some(time_range);
        self
    }

    pub fn filters(mut self, filters: Vec<models::QueryFilter>) -> Self {
        self.request.filters = Some(filters);
        self
    }

    pub async fn execute(
        self,
        service: &EnhancedQueryService,
    ) -> SDKResult<models::SmartQueryResponse> {
        service.execute_smart_query(&self.request).await
    }
}

//! Bitable V2 高级搜索 API
//!
//! 提供强大的数据搜索和过滤功能，包括：
//! - 复杂查询条件支持
//! - 多字段联合搜索
//! - 高性能索引搜索
//! - 智能排序和分页
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

    /// 高级搜索请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct AdvancedSearchRequest {
        /// 应用token
        pub app_token: String,
        /// 数据表ID
        pub table_id: String,
        /// 搜索条件列表
        pub conditions: Vec<SearchCondition>,
        /// 排序规则
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sort: Option<Vec<SortRule>>,
        /// 分页信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<PageInfo>,
        /// 是否返回字段名称
        #[serde(skip_serializing_if = "Option::is_none")]
        pub need_field_name: Option<bool>,
    }

    /// 搜索条件
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct SearchCondition {
        /// 字段ID或字段名
        pub field_id_or_name: String,
        /// 操作符
        pub operator: SearchOperator,
        /// 值
        pub value: Value,
        /// 条件组标识（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub group_id: Option<String>,
    }

    /// 搜索操作符
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum SearchOperator {
        /// 等于
        #[serde(rename = "eq")]
        Equal,
        /// 不等于
        #[serde(rename = "neq")]
        NotEqual,
        /// 包含
        #[serde(rename = "contains")]
        Contains,
        /// 不包含
        #[serde(rename = "not_contains")]
        NotContains,
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
        /// 不在范围内
        #[serde(rename = "not_in")]
        NotIn,
        /// 为空
        #[serde(rename = "is_empty")]
        IsEmpty,
        /// 不为空
        #[serde(rename = "is_not_empty")]
        IsNotEmpty,
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

    /// 高级搜索响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct AdvancedSearchResponse {
        /// 搜索结果
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<AdvancedSearchData>,
    }

    impl ApiResponseTrait for AdvancedSearchResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 搜索结果数据
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct AdvancedSearchData {
        /// 记录列表
        #[serde(skip_serializing_if = "Option::is_none")]
        pub records: Option<Vec<SearchRecord>>,
        /// 分页信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub page: Option<PageInfo>,
        /// 总数量
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<i32>,
    }

    /// 搜索记录
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct SearchRecord {
        /// 记录ID
        #[serde(skip_serializing_if = "Option::is_none")]
        pub record_id: Option<String>,
        /// 字段值
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fields: Option<Value>,
        /// 记录创建时间
        #[serde(skip_serializing_if = "Option::is_none")]
        pub create_time: Option<String>,
        /// 记录更新时间
        #[serde(skip_serializing_if = "Option::is_none")]
        pub update_time: Option<String>,
    }
}

/// 服务实现
pub mod services {
    use super::*;

    /// 高级搜索服务
    #[derive(Debug, Clone)]
    pub struct AdvancedSearchService {
        config: Config,
    }

    impl AdvancedSearchService {
        /// 创建高级搜索服务实例
        pub fn new(config: Config) -> Self {
            Self { config }
        }

        /// 执行高级搜索
        ///
        /// 使用复杂的查询条件搜索记录
        ///
        /// # 参数
        /// * `request` - 高级搜索请求
        ///
        /// # 返回
        /// 返回搜索结果
        pub async fn search(
            &self,
            request: &models::AdvancedSearchRequest,
        ) -> SDKResult<models::AdvancedSearchResponse> {
            // 验证请求参数
            request
                .validate()
                .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

            log::info!(
                "执行高级搜索: app_token={}, table_id={}, conditions={}",
                request.app_token,
                request.table_id,
                request.conditions.len()
            );

            // 构建请求体
            let body = serde_json::to_value(request)?;

            // 构建API请求
            let api_req = ApiRequest::post(format!(
                    "/open-apis/bitable/v2/apps/{}/tables/{}/records/search",
                    request.app_token, request.table_id
                ))
                .body(serde_json::to_string(&body)?)
                .query(HashMap::new());

            // 发送请求
            let resp =
                Transport::<models::AdvancedSearchResponse>::request(api_req, &self.config, None)
                    .await?;
            let response = resp.data.unwrap_or_default();

            log::info!(
                "高级搜索完成: app_token={}, table_id={}, found={}",
                request.app_token,
                request.table_id,
                response
                    .data
                    .as_ref()
                    .and_then(|d| d.records.as_ref())
                    .map(|r| r.len())
                    .unwrap_or(0)
            );

            Ok(response)
        }
    }
}

// 为请求结构体添加验证方法
impl models::AdvancedSearchRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.conditions.is_empty() {
            return Err("搜索条件不能为空".to_string());
        }
        if self.conditions.len() > 100 {
            return Err("搜索条件数量不能超过100个".to_string());
        }

        for (i, condition) in self.conditions.iter().enumerate() {
            if condition.field_id_or_name.trim().is_empty() {
                return Err(format!("第{}个条件的字段ID或名称不能为空", i + 1));
            }
        }

        if let Some(ref page) = self.page {
            if page.page_size <= 0 || page.page_size > 1000 {
                return Err("页面大小必须在1-1000之间".to_string());
            }
        }

        Ok(())
    }
}

// 重新导出
pub use models::*;
pub use services::AdvancedSearchService;

// 构建器模式实现
pub struct AdvancedSearchRequestBuilder {
    request: models::AdvancedSearchRequest,
}

impl AdvancedSearchRequestBuilder {
    pub fn new(app_token: impl Into<String>, table_id: impl Into<String>) -> Self {
        Self {
            request: models::AdvancedSearchRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                conditions: Vec::new(),
                sort: None,
                page: None,
                need_field_name: None,
            },
        }
    }

    pub fn condition(
        mut self,
        field_id_or_name: impl Into<String>,
        operator: models::SearchOperator,
        value: Value,
    ) -> Self {
        self.request.conditions.push(models::SearchCondition {
            field_id_or_name: field_id_or_name.into(),
            operator,
            value,
            group_id: None,
        });
        self
    }

    pub fn sort(mut self, sort: Vec<models::SortRule>) -> Self {
        self.request.sort = Some(sort);
        self
    }

    pub fn page(mut self, page: models::PageInfo) -> Self {
        self.request.page = Some(page);
        self
    }

    pub fn need_field_name(mut self, need_field_name: bool) -> Self {
        self.request.need_field_name = Some(need_field_name);
        self
    }

    pub async fn execute(
        self,
        service: &AdvancedSearchService,
    ) -> SDKResult<models::AdvancedSearchResponse> {
        service.search(&self.request).await
    }
}

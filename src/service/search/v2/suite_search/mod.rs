//! Suite Search v2 服务模块
//!
//! 提供飞书云文档搜索功能，支持按关键字、文档类型、所有者等多维度搜索

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use chrono;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use super::models::{SearchAppRequest, SearchMessageRequest, SearchResponse};

/// 套件搜索服务
#[derive(Debug, Clone)]
pub struct SuiteSearchService {
    pub config: Config,
}

impl SuiteSearchService {
    /// 创建SuiteSearchService实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 搜索云文档对象请求
#[derive(Debug, Clone, Serialize)]
pub struct SuiteSearchObjectRequest {
    /// 搜索关键字
    pub query: String,
    /// 文档类型过滤，支持 doc, docx, sheet, slide, bitable, mindnote 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    /// 文档所有者过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// 文档所属部门过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 排序方式，支持 create_time, update_time, title 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// 排序方向，asc 或 desc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    /// 分页大小，默认20，最大200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl SuiteSearchObjectRequest {
    /// 创建新的搜索请求
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            doc_type: None,
            owner_id: None,
            department_id: None,
            sort_by: None,
            sort_direction: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置文档类型过滤
    pub fn doc_type(mut self, doc_type: impl Into<String>) -> Self {
        self.doc_type = Some(doc_type.into());
        self
    }

    /// 设置文档所有者过滤
    pub fn owner_id(mut self, owner_id: impl Into<String>) -> Self {
        self.owner_id = Some(owner_id.into());
        self
    }

    /// 设置部门过滤
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
        self
    }

    /// 设置排序方式
    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = Some(sort_by.into());
        self
    }

    /// 设置排序方向
    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.sort_direction = Some(sort_direction.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页token
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.query.trim().is_empty() {
            return Err("搜索关键字不能为空".to_string());
        }

        if let Some(ref page_size) = self.page_size {
            if *page_size < 1 || *page_size > 200 {
                return Err("分页大小必须在1-200之间".to_string());
            }
        }

        if let Some(ref sort_direction) = self.sort_direction {
            if sort_direction != "asc" && sort_direction != "desc" {
                return Err("排序方向只能是asc或desc".to_string());
            }
        }

        if let Some(ref doc_type) = self.doc_type {
            let valid_types = ["doc", "docx", "sheet", "slide", "bitable", "mindnote"];
            if !valid_types.contains(&doc_type.as_str()) {
                return Err(format!("不支持的文档类型: {}", doc_type));
            }
        }

        Ok(())
    }
}

/// 云文档搜索结果项
#[derive(Debug, Clone, Deserialize)]
pub struct SuiteSearchResultItem {
    /// 文档ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_id: Option<String>,
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文档类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    /// 文档所有者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// 文档所有者名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    /// 文档所属部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 文档所属部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 文档创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 文档更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 文档URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 文档内容摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 文档权限级别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<String>,
}

impl SuiteSearchResultItem {
    /// 获取文档类型的中文名称
    pub fn doc_type_chinese(&self) -> &str {
        match self.doc_type.as_deref() {
            Some("doc") => "文档",
            Some("docx") => "Word文档",
            Some("sheet") => "电子表格",
            Some("slide") => "幻灯片",
            Some("bitable") => "多维表格",
            Some("mindnote") => "思维导图",
            _ => "未知类型",
        }
    }

    /// 格式化创建时间
    pub fn formatted_create_time(&self) -> Option<String> {
        self.create_time.map(|timestamp| {
            let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
                .unwrap_or_else(|| chrono::Utc::now());
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// 格式化更新时间
    pub fn formatted_update_time(&self) -> Option<String> {
        self.update_time.map(|timestamp| {
            let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
                .unwrap_or_else(|| chrono::Utc::now());
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }
}

/// 搜索云文档对象响应
#[derive(Debug, Clone, Deserialize)]
pub struct SuiteSearchObjectResponse {
    /// 搜索结果列表
    pub items: Vec<SuiteSearchResultItem>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl SuiteSearchObjectResponse {
    /// 获取结果数量
    pub fn result_count(&self) -> usize {
        self.items.len()
    }

    /// 是否有更多结果
    pub fn has_more_results(&self) -> bool {
        self.has_more.unwrap_or(false)
    }

    /// 获取下一页分页token
    pub fn next_page_token(&self) -> Option<&String> {
        self.page_token.as_ref()
    }

    /// 按文档类型分组结果
    pub fn group_by_doc_type(&self) -> HashMap<String, Vec<&SuiteSearchResultItem>> {
        let mut groups = HashMap::new();
        for item in &self.items {
            let doc_type = item.doc_type.as_deref().unwrap_or("unknown").to_string();
            groups.entry(doc_type).or_insert_with(Vec::new).push(item);
        }
        groups
    }

    /// 按所有者分组结果
    pub fn group_by_owner(&self) -> HashMap<String, Vec<&SuiteSearchResultItem>> {
        let mut groups = HashMap::new();
        for item in &self.items {
            let owner = item.owner_name.as_deref().unwrap_or("未知所有者").to_string();
            groups.entry(owner).or_insert_with(Vec::new).push(item);
        }
        groups
    }
}

impl ApiResponseTrait for SuiteSearchObjectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SuiteSearchService {
    /// 搜索云文档对象
    ///
    /// 在飞书平台内搜索云文档，支持多种过滤条件和排序方式
    ///
    /// # 参数
    /// * `req` - 搜索请求参数
    ///
    /// # 返回值
    /// 返回搜索结果列表，包含文档基本信息和访问链接
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::search::v2::suite_search::SuiteSearchObjectRequest;
    ///
    /// let request = SuiteSearchObjectRequest::new("项目文档")
    ///     .doc_type("doc")
    ///     .page_size(20)
    ///     .sort_by("update_time")
    ///     .sort_direction("desc");
    ///
    /// let response = service.search_suite_object(&request).await?;
    /// println!("找到 {} 个文档", response.result_count());
    /// ```
    pub async fn search_suite_object(&self, req: &SuiteSearchObjectRequest) -> SDKResult<SuiteSearchObjectResponse> {
        req.validate()?;
        log::debug!("开始搜索云文档: query={}", req.query);

        // 构建查询参数
        let mut query_params: HashMap<&str, String> = HashMap::new();

        if let Some(ref page_size) = req.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = req.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建请求体
        let mut body = json!({
            "query": req.query
        });

        if let Some(ref doc_type) = req.doc_type {
            body["doc_type"] = json!(doc_type);
        }
        if let Some(ref owner_id) = req.owner_id {
            body["owner_id"] = json!(owner_id);
        }
        if let Some(ref department_id) = req.department_id {
            body["department_id"] = json!(department_id);
        }
        if let Some(ref sort_by) = req.sort_by {
            body["sort_by"] = json!(sort_by);
        }
        if let Some(ref sort_direction) = req.sort_direction {
            body["sort_direction"] = json!(sort_direction);
        }

        // 构建API请求
        let api_path = crate::core::endpoints_original::Endpoints::SUITE_DOCS_SEARCH_OBJECT;
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&body).unwrap_or_default(),
            ..Default::default()
        };

        let resp = Transport::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!("云文档搜索完成: 找到 {} 个结果", response.result_count());
        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 搜索云文档对象请求构建器
#[derive(Debug, Clone, Default)]
pub struct SuiteSearchObjectRequestBuilder {
    request: SuiteSearchObjectRequest,
}

impl SuiteSearchObjectRequestBuilder {
    /// 创建新的构建器
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            request: SuiteSearchObjectRequest::new(query),
        }
    }

    /// 设置文档类型过滤
    pub fn doc_type(mut self, doc_type: impl Into<String>) -> Self {
        self.request = self.request.doc_type(doc_type);
        self
    }

    /// 设置为Word文档
    pub fn doc(self) -> Self {
        self.doc_type("doc")
    }

    /// 设置为Excel文档
    pub fn sheet(self) -> Self {
        self.doc_type("sheet")
    }

    /// 设置为PPT文档
    pub fn slide(self) -> Self {
        self.doc_type("slide")
    }

    /// 设置为多维表格
    pub fn bitable(self) -> Self {
        self.doc_type("bitable")
    }

    /// 设置文档所有者过滤
    pub fn owner_id(mut self, owner_id: impl Into<String>) -> Self {
        self.request = self.request.owner_id(owner_id);
        self
    }

    /// 设置部门过滤
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.request = self.request.department_id(department_id);
        self
    }

    /// 按创建时间排序
    pub fn sort_by_create_time(mut self) -> Self {
        self.request = self.request.sort_by("create_time");
        self
    }

    /// 按更新时间排序
    pub fn sort_by_update_time(mut self) -> Self {
        self.request = self.request.sort_by("update_time");
        self
    }

    /// 按标题排序
    pub fn sort_by_title(mut self) -> Self {
        self.request = self.request.sort_by("title");
        self
    }

    /// 设置升序排序
    pub fn ascending(mut self) -> Self {
        self.request = self.request.sort_direction("asc");
        self
    }

    /// 设置降序排序
    pub fn descending(mut self) -> Self {
        self.request = self.request.sort_direction("desc");
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置分页token
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 执行搜索操作
    pub async fn execute(self, service: &SuiteSearchService) -> SDKResult<SuiteSearchObjectResponse> {
        service.search_suite_object(&self.request).await
    }

    /// 构建请求对象
    pub fn build(self) -> SuiteSearchObjectRequest {
        self.request
    }
}

impl SuiteSearchService {
    /// 创建搜索云文档对象请求构建器
    pub fn search_suite_object_builder(&self, query: impl Into<String>) -> SuiteSearchObjectRequestBuilder {
        SuiteSearchObjectRequestBuilder::new(query)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suite_search_object_request_creation() {
        let request = SuiteSearchObjectRequest::new("测试查询");
        assert_eq!(request.query, "测试查询");
        assert_eq!(request.doc_type, None);
        assert_eq!(request.page_size, None);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_suite_search_object_request_with_fields() {
        let request = SuiteSearchObjectRequest::new("项目文档")
            .doc_type("doc")
            .owner_id("user_123")
            .page_size(20)
            .sort_by("update_time")
            .sort_direction("desc");

        assert_eq!(request.query, "项目文档");
        assert_eq!(request.doc_type, Some("doc".to_string()));
        assert_eq!(request.owner_id, Some("user_123".to_string()));
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.sort_by, Some("update_time".to_string()));
        assert_eq!(request.sort_direction, Some("desc".to_string()));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_suite_search_object_request_validation() {
        // 测试空查询
        let empty_query = SuiteSearchObjectRequest::new("");
        assert!(empty_query.validate().is_err());

        // 测试无效分页大小
        let invalid_page_size = SuiteSearchObjectRequest::new("测试")
            .page_size(0);
        assert!(invalid_page_size.validate().is_err());

        let invalid_page_size2 = SuiteSearchObjectRequest::new("测试")
            .page_size(201);
        assert!(invalid_page_size2.validate().is_err());

        // 测试无效排序方向
        let invalid_direction = SuiteSearchObjectRequest::new("测试")
            .sort_direction("invalid");
        assert!(invalid_direction.validate().is_err());

        // 测试无效文档类型
        let invalid_doc_type = SuiteSearchObjectRequest::new("测试")
            .doc_type("invalid_type");
        assert!(invalid_doc_type.validate().is_err());
    }

    #[test]
    fn test_suite_search_object_request_builder() {
        let request = SuiteSearchObjectRequestBuilder::new("项目文档")
            .doc()
            .owner_id("user_123")
            .sort_by_update_time()
            .descending()
            .page_size(20)
            .build();

        assert_eq!(request.query, "项目文档");
        assert_eq!(request.doc_type, Some("doc".to_string()));
        assert_eq!(request.owner_id, Some("user_123".to_string()));
        assert_eq!(request.sort_by, Some("update_time".to_string()));
        assert_eq!(request.sort_direction, Some("desc".to_string()));
        assert_eq!(request.page_size, Some(20));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_suite_search_result_item_chinese_names() {
        let item = SuiteSearchResultItem {
            doc_type: Some("doc".to_string()),
            ..Default::default()
        };

        assert_eq!(item.doc_type_chinese(), "文档");

        let sheet_item = SuiteSearchResultItem {
            doc_type: Some("sheet".to_string()),
            ..Default::default()
        };

        assert_eq!(sheet_item.doc_type_chinese(), "电子表格");
    }

    #[test]
    fn test_suite_search_object_response_methods() {
        let items = vec![
            SuiteSearchResultItem {
                doc_id: Some("doc1".to_string()),
                doc_type: Some("doc".to_string()),
                ..Default::default()
            },
            SuiteSearchResultItem {
                doc_id: Some("doc2".to_string()),
                doc_type: Some("sheet".to_string()),
                ..Default::default()
            },
        ];

        let response = SuiteSearchObjectResponse {
            items,
            has_more: Some(true),
            page_token: Some("next_token".to_string()),
        };

        assert_eq!(response.result_count(), 2);
        assert!(response.has_more_results());
        assert_eq!(response.next_page_token(), Some(&"next_token".to_string()));

        let groups = response.group_by_doc_type();
        assert_eq!(groups.len(), 2);
        assert!(groups.contains_key("doc"));
        assert!(groups.contains_key("sheet"));
    }

    #[test]
    fn test_builder_fluent_interface() {
        let config = Config::default();
        let service = SuiteSearchService::new(config);

        let builder = service.search_suite_object_builder("重要文档")
            .doc()
            .sort_by_update_time()
            .descending()
            .page_size(10);

        let request = builder.build();
        assert_eq!(request.query, "重要文档");
        assert_eq!(request.doc_type, Some("doc".to_string()));
        assert_eq!(request.sort_by, Some("update_time".to_string()));
        assert_eq!(request.sort_direction, Some("desc".to_string()));
        assert_eq!(request.page_size, Some(10));
    }

    #[test]
    fn test_all_doc_types() {
        let doc_request = SuiteSearchObjectRequestBuilder::new("测试")
            .doc()
            .build();

        let sheet_request = SuiteSearchObjectRequestBuilder::new("测试")
            .sheet()
            .build();

        let slide_request = SuiteSearchObjectRequestBuilder::new("测试")
            .slide()
            .build();

        let bitable_request = SuiteSearchObjectRequestBuilder::new("测试")
            .bitable()
            .build();

        assert_eq!(doc_request.doc_type, Some("doc".to_string()));
        assert_eq!(sheet_request.doc_type, Some("sheet".to_string()));
        assert_eq!(slide_request.doc_type, Some("slide".to_string()));
        assert_eq!(bitable_request.doc_type, Some("bitable".to_string()));
    }

    #[test]
    fn test_comprehensive_search_request() {
        let request = SuiteSearchObjectRequestBuilder::new("项目规划文档")
            .docx()
            .owner_id("manager_123")
            .department_id("dept_456")
            .sort_by_create_time()
            .ascending()
            .page_size(50)
            .page_token("page_token_abc")
            .build();

        assert_eq!(request.query, "项目规划文档");
        assert_eq!(request.doc_type, Some("docx".to_string()));
        assert_eq!(request.owner_id, Some("manager_123".to_string()));
        assert_eq!(request.department_id, Some("dept_456".to_string()));
        assert_eq!(request.sort_by, Some("create_time".to_string()));
        assert_eq!(request.sort_direction, Some("asc".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("page_token_abc".to_string()));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_response_grouping_functionality() {
        let items = vec![
            SuiteSearchResultItem {
                doc_id: Some("doc1".to_string()),
                doc_type: Some("doc".to_string()),
                owner_name: Some("张三".to_string()),
                department_name: Some("技术部".to_string()),
                ..Default::default()
            },
            SuiteSearchResultItem {
                doc_id: Some("doc2".to_string()),
                doc_type: Some("doc".to_string()),
                owner_name: Some("张三".to_string()),
                department_name: Some("技术部".to_string()),
                ..Default::default()
            },
            SuiteSearchResultItem {
                doc_id: Some("doc3".to_string()),
                doc_type: Some("sheet".to_string()),
                owner_name: Some("李四".to_string()),
                department_name: Some("产品部".to_string()),
                ..Default::default()
            },
        ];

        let response = SuiteSearchObjectResponse {
            items,
            has_more: Some(false),
            page_token: None,
        };

        // 按文档类型分组
        let type_groups = response.group_by_doc_type();
        assert_eq!(type_groups.get("doc").unwrap().len(), 2);
        assert_eq!(type_groups.get("sheet").unwrap().len(), 1);

        // 按所有者分组
        let owner_groups = response.group_by_owner();
        assert_eq!(owner_groups.get("张三").unwrap().len(), 2);
        assert_eq!(owner_groups.get("李四").unwrap().len(), 1);
    }
}
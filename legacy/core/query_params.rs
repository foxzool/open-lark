//! 查询参数常量定义模块
//!
//! 本模块定义了飞书API中常用的查询参数名称常量，旨在：
//! 1. 减少字符串分配开销 - 使用&'static str替代重复的.to_string()调用
//! 2. 提高代码一致性 - 统一参数名称，避免拼写错误  
//! 3. 便于维护管理 - 集中管理所有查询参数常量
//!
//! # 性能影响
//!
//! 使用本模块常量可以显著减少内存分配：
//! ```rust
//! use open_lark::core::query_params::QueryParams;
//! use std::collections::HashMap;
//!
//! # fn main() {
//! let mut params = HashMap::new();
//! let value = "20".to_string();
//! // 优化前: 每次调用都分配新字符串
//! params.insert("page_size", value.clone());
//!
//! // 优化后: 使用静态字符串常量
//! params.insert(QueryParams::PAGE_SIZE, value);
//! # }
//! ```
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::core::query_params::QueryParams;
//! use std::collections::HashMap;
//!
//! # fn main() {
//! let mut params = HashMap::new();
//! params.insert(QueryParams::PAGE_SIZE, "20".to_string());
//! let token = "next-page-token".to_string();
//! params.insert(QueryParams::PAGE_TOKEN, token);
//! # }
//! ```

use std::collections::HashMap;

/// 飞书API查询参数常量定义
///
/// 提供所有常用查询参数的静态字符串常量，避免运行时字符串分配开销。
/// 按功能分组组织，便于查找和维护。
pub struct QueryParams;

impl QueryParams {
    // ==================== 分页参数 ====================
    /// 分页大小参数 - 指定每页返回的项目数量
    pub const PAGE_SIZE: &'static str = "page_size";

    /// 分页令牌参数 - 用于获取下一页数据的令牌
    pub const PAGE_TOKEN: &'static str = "page_token";

    /// 页码参数 - 指定要获取的页面编号（某些API使用）
    pub const PAGE: &'static str = "page";

    /// 偏移量参数 - 指定从第几条记录开始返回
    pub const OFFSET: &'static str = "offset";

    /// 限制参数 - 限制返回的最大记录数
    pub const LIMIT: &'static str = "limit";

    // ==================== 时间范围参数 ====================
    /// 开始时间参数 - 查询范围的起始时间
    pub const START_TIME: &'static str = "start_time";

    /// 结束时间参数 - 查询范围的结束时间  
    pub const END_TIME: &'static str = "end_time";

    /// 创建时间开始
    pub const CREATE_TIME_START: &'static str = "create_time_start";

    /// 创建时间结束
    pub const CREATE_TIME_END: &'static str = "create_time_end";

    /// 更新时间开始
    pub const UPDATE_TIME_START: &'static str = "update_time_start";

    /// 更新时间结束
    pub const UPDATE_TIME_END: &'static str = "update_time_end";

    // ==================== 用户和身份参数 ====================
    /// 用户ID参数 - 指定目标用户的唯一标识
    pub const USER_ID: &'static str = "user_id";

    /// 用户ID类型参数 - 指定用户ID的类型格式
    pub const USER_ID_TYPE: &'static str = "user_id_type";

    /// 部门ID参数 - 指定目标部门的唯一标识
    pub const DEPARTMENT_ID: &'static str = "department_id";

    /// 部门ID类型参数 - 指定部门ID的类型格式
    pub const DEPARTMENT_ID_TYPE: &'static str = "department_id_type";

    /// 组织ID参数 - 指定组织的唯一标识
    pub const ORG_ID: &'static str = "org_id";

    /// 员工ID参数 - 指定员工的唯一标识
    pub const EMPLOYEE_ID: &'static str = "employee_id";

    // ==================== 状态和类型参数 ====================
    /// 状态参数 - 指定资源的状态筛选条件
    pub const STATUS: &'static str = "status";

    /// 类型参数 - 指定资源的类型筛选条件
    pub const TYPE: &'static str = "type";

    /// 规则类型参数 - 指定规则的类型
    pub const RULE_TYPE: &'static str = "rule_type";

    /// 访问类型参数 - 指定访问的类型
    pub const ACCESS_TYPE: &'static str = "access_type";

    /// 排序参数 - 指定结果的排序方式
    pub const SORT: &'static str = "sort";

    /// 排序顺序参数 - 指定排序的方向（asc/desc）
    pub const ORDER: &'static str = "order";

    // ==================== 内容和搜索参数 ====================
    /// 查询关键词参数 - 用于搜索的关键词
    pub const QUERY: &'static str = "query";

    /// 搜索关键词参数 - 另一种搜索关键词字段名
    pub const KEYWORD: &'static str = "keyword";

    /// 名称参数 - 按名称筛选
    pub const NAME: &'static str = "name";

    /// 标题参数 - 按标题筛选
    pub const TITLE: &'static str = "title";

    /// 语言参数 - 指定语言偏好
    pub const LANGUAGE: &'static str = "language";

    /// 地区参数 - 指定地区设置
    pub const LOCALE: &'static str = "locale";

    // ==================== 会议和VC参数 ====================
    /// 会议ID参数 - 会议的唯一标识
    pub const MEETING_ID: &'static str = "meeting_id";

    /// 房间ID参数 - 会议室的唯一标识
    pub const ROOM_ID: &'static str = "room_id";

    /// 录制ID参数 - 录制文件的唯一标识
    pub const RECORDING_ID: &'static str = "recording_id";

    // ==================== 工作台和应用参数 ====================
    /// 自定义工作台ID参数
    pub const CUSTOM_WORKPLACE_ID: &'static str = "custom_workplace_id";

    /// 小部件ID参数  
    pub const WIDGET_ID: &'static str = "widget_id";

    /// 应用ID参数
    pub const APP_ID: &'static str = "app_id";

    // ==================== 招聘相关参数 ====================
    /// 收入类型参数
    pub const INCOME_TYPE: &'static str = "income_type";
    /// 开始日期参数
    pub const START_DATE: &'static str = "start_date";
    /// 结束日期参数
    pub const END_DATE: &'static str = "end_date";

    // ==================== 文件和文档参数 ====================
    /// 文件ID参数 - 文件的唯一标识
    pub const FILE_ID: &'static str = "file_id";

    /// 文档ID参数 - 文档的唯一标识
    pub const DOC_ID: &'static str = "doc_id";

    /// 文件夹ID参数 - 文件夹的唯一标识
    pub const FOLDER_ID: &'static str = "folder_id";

    /// 文件类型参数 - 文件类型筛选
    pub const FILE_TYPE: &'static str = "file_type";

    // ==================== 消息和IM参数 ====================
    /// 消息ID参数 - 消息的唯一标识
    pub const MESSAGE_ID: &'static str = "message_id";

    /// 聊天ID参数 - 聊天会话的唯一标识
    pub const CHAT_ID: &'static str = "chat_id";

    /// 频道ID参数 - 频道的唯一标识
    pub const CHANNEL_ID: &'static str = "channel_id";

    // ==================== 设备和访问参数 ====================
    /// 设备ID参数 - 设备的唯一标识
    pub const DEVICE_ID: &'static str = "device_id";

    /// 访问方法参数 - 访问方式筛选
    pub const ACCESS_METHOD: &'static str = "access_method";

    /// 结果参数 - 访问结果筛选
    pub const RESULT: &'static str = "result";

    // ==================== 勋章相关参数 ====================
    /// 勋章ID参数 - 勋章的唯一标识
    pub const BADGE_ID: &'static str = "badge_id";
}

/// 查询参数构建器
///
/// 提供类型安全的查询参数构建接口，支持链式调用，
/// 内部使用静态字符串常量避免不必要的字符串分配。
///
/// # 示例
///
/// ```rust
/// use open_lark::core::query_params::{QueryParams, QueryParamsBuilder};
///
/// let params = QueryParamsBuilder::new()
///     .page_size(20)
///     .page_token("token_123")
///     .start_time("2024-01-01T00:00:00Z")
///     .build();
///
/// assert_eq!(params.get(QueryParams::PAGE_SIZE), Some(&"20".to_string()));
/// ```
#[derive(Debug, Default)]
pub struct QueryParamsBuilder {
    params: HashMap<&'static str, String>,
}

impl QueryParamsBuilder {
    /// 创建新的查询参数构建器
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// 预分配HashMap容量，适用于已知参数数量的场景
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            params: HashMap::with_capacity(capacity),
        }
    }

    // ==================== 分页参数方法 ====================

    /// 设置分页大小
    pub fn page_size(mut self, size: i32) -> Self {
        self.params.insert(QueryParams::PAGE_SIZE, size.to_string());
        self
    }

    /// 设置分页令牌
    pub fn page_token<S: ToString>(mut self, token: S) -> Self {
        self.params
            .insert(QueryParams::PAGE_TOKEN, token.to_string());
        self
    }

    /// 设置页码
    pub fn page(mut self, page: i32) -> Self {
        self.params.insert(QueryParams::PAGE, page.to_string());
        self
    }

    /// 设置偏移量
    pub fn offset(mut self, offset: i32) -> Self {
        self.params.insert(QueryParams::OFFSET, offset.to_string());
        self
    }

    /// 设置限制数量
    pub fn limit(mut self, limit: i32) -> Self {
        self.params.insert(QueryParams::LIMIT, limit.to_string());
        self
    }

    // ==================== 时间参数方法 ====================

    /// 设置开始时间
    pub fn start_time<S: ToString>(mut self, time: S) -> Self {
        self.params
            .insert(QueryParams::START_TIME, time.to_string());
        self
    }

    /// 设置结束时间
    pub fn end_time<S: ToString>(mut self, time: S) -> Self {
        self.params.insert(QueryParams::END_TIME, time.to_string());
        self
    }

    // ==================== 用户参数方法 ====================

    /// 设置用户ID
    pub fn user_id<S: ToString>(mut self, id: S) -> Self {
        self.params.insert(QueryParams::USER_ID, id.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type<S: ToString>(mut self, id_type: S) -> Self {
        self.params
            .insert(QueryParams::USER_ID_TYPE, id_type.to_string());
        self
    }

    /// 设置部门ID
    pub fn department_id<S: ToString>(mut self, id: S) -> Self {
        self.params
            .insert(QueryParams::DEPARTMENT_ID, id.to_string());
        self
    }

    /// 设置组织ID
    pub fn org_id<S: ToString>(mut self, id: S) -> Self {
        self.params.insert(QueryParams::ORG_ID, id.to_string());
        self
    }

    // ==================== 状态参数方法 ====================

    /// 设置状态
    pub fn status<S: ToString>(mut self, status: S) -> Self {
        self.params.insert(QueryParams::STATUS, status.to_string());
        self
    }

    /// 设置类型
    pub fn r#type<S: ToString>(mut self, type_value: S) -> Self {
        self.params
            .insert(QueryParams::TYPE, type_value.to_string());
        self
    }

    /// 设置规则类型
    pub fn rule_type<S: ToString>(mut self, rule_type: S) -> Self {
        self.params
            .insert(QueryParams::RULE_TYPE, rule_type.to_string());
        self
    }

    /// 设置访问类型
    pub fn access_type<S: ToString>(mut self, access_type: S) -> Self {
        self.params
            .insert(QueryParams::ACCESS_TYPE, access_type.to_string());
        self
    }

    // ==================== 自定义参数方法 ====================

    /// 添加自定义参数，使用静态字符串作为key
    pub fn custom_static(mut self, key: &'static str, value: String) -> Self {
        self.params.insert(key, value);
        self
    }

    /// 添加可选参数，仅当值存在时添加
    pub fn optional<S: ToString>(mut self, key: &'static str, value: Option<S>) -> Self {
        if let Some(v) = value {
            self.params.insert(key, v.to_string());
        }
        self
    }

    /// 构建最终的查询参数HashMap
    pub fn build(self) -> HashMap<String, String> {
        // 转换为HashMap<String, String>以兼容现有API
        self.params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect()
    }

    /// 构建为引用形式，避免额外的String分配
    /// 注意：返回的引用仅在构建器实例存在期间有效
    pub fn build_ref(&self) -> &HashMap<&'static str, String> {
        &self.params
    }

    /// 获取参数数量
    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Constants Testing ====================

    #[test]
    fn test_pagination_constants() {
        assert_eq!(QueryParams::PAGE_SIZE, "page_size");
        assert_eq!(QueryParams::PAGE_TOKEN, "page_token");
        assert_eq!(QueryParams::PAGE, "page");
        assert_eq!(QueryParams::OFFSET, "offset");
        assert_eq!(QueryParams::LIMIT, "limit");
    }

    #[test]
    fn test_time_range_constants() {
        assert_eq!(QueryParams::START_TIME, "start_time");
        assert_eq!(QueryParams::END_TIME, "end_time");
        assert_eq!(QueryParams::CREATE_TIME_START, "create_time_start");
        assert_eq!(QueryParams::CREATE_TIME_END, "create_time_end");
        assert_eq!(QueryParams::UPDATE_TIME_START, "update_time_start");
        assert_eq!(QueryParams::UPDATE_TIME_END, "update_time_end");
    }

    #[test]
    fn test_user_identity_constants() {
        assert_eq!(QueryParams::USER_ID, "user_id");
        assert_eq!(QueryParams::USER_ID_TYPE, "user_id_type");
        assert_eq!(QueryParams::DEPARTMENT_ID, "department_id");
        assert_eq!(QueryParams::DEPARTMENT_ID_TYPE, "department_id_type");
        assert_eq!(QueryParams::ORG_ID, "org_id");
        assert_eq!(QueryParams::EMPLOYEE_ID, "employee_id");
    }

    #[test]
    fn test_status_type_constants() {
        assert_eq!(QueryParams::STATUS, "status");
        assert_eq!(QueryParams::TYPE, "type");
        assert_eq!(QueryParams::RULE_TYPE, "rule_type");
        assert_eq!(QueryParams::ACCESS_TYPE, "access_type");
        assert_eq!(QueryParams::SORT, "sort");
        assert_eq!(QueryParams::ORDER, "order");
    }

    #[test]
    fn test_content_search_constants() {
        assert_eq!(QueryParams::QUERY, "query");
        assert_eq!(QueryParams::KEYWORD, "keyword");
        assert_eq!(QueryParams::NAME, "name");
        assert_eq!(QueryParams::TITLE, "title");
        assert_eq!(QueryParams::LANGUAGE, "language");
        assert_eq!(QueryParams::LOCALE, "locale");
    }

    #[test]
    fn test_meeting_vc_constants() {
        assert_eq!(QueryParams::MEETING_ID, "meeting_id");
        assert_eq!(QueryParams::ROOM_ID, "room_id");
        assert_eq!(QueryParams::RECORDING_ID, "recording_id");
    }

    #[test]
    fn test_workplace_app_constants() {
        assert_eq!(QueryParams::CUSTOM_WORKPLACE_ID, "custom_workplace_id");
        assert_eq!(QueryParams::WIDGET_ID, "widget_id");
        assert_eq!(QueryParams::APP_ID, "app_id");
    }

    #[test]
    fn test_hire_related_constants() {
        assert_eq!(QueryParams::INCOME_TYPE, "income_type");
        assert_eq!(QueryParams::START_DATE, "start_date");
        assert_eq!(QueryParams::END_DATE, "end_date");
    }

    #[test]
    fn test_file_document_constants() {
        assert_eq!(QueryParams::FILE_ID, "file_id");
        assert_eq!(QueryParams::DOC_ID, "doc_id");
        assert_eq!(QueryParams::FOLDER_ID, "folder_id");
        assert_eq!(QueryParams::FILE_TYPE, "file_type");
    }

    #[test]
    fn test_message_im_constants() {
        assert_eq!(QueryParams::MESSAGE_ID, "message_id");
        assert_eq!(QueryParams::CHAT_ID, "chat_id");
        assert_eq!(QueryParams::CHANNEL_ID, "channel_id");
    }

    #[test]
    fn test_device_access_constants() {
        assert_eq!(QueryParams::DEVICE_ID, "device_id");
        assert_eq!(QueryParams::ACCESS_METHOD, "access_method");
        assert_eq!(QueryParams::RESULT, "result");
    }

    #[test]
    fn test_badge_constants() {
        assert_eq!(QueryParams::BADGE_ID, "badge_id");
    }

    // ==================== Builder Testing ====================

    #[test]
    fn test_query_params_builder_creation() {
        let builder = QueryParamsBuilder::new();
        assert_eq!(builder.len(), 0);
        assert!(builder.is_empty());
    }

    #[test]
    fn test_query_params_builder_default() {
        let builder: QueryParamsBuilder = Default::default();
        assert_eq!(builder.len(), 0);
        assert!(builder.is_empty());
    }

    #[test]
    fn test_builder_with_capacity() {
        let builder = QueryParamsBuilder::with_capacity(5);
        assert_eq!(builder.len(), 0);
        assert!(builder.is_empty());
        // Capacity doesn't affect length/empty state, just memory allocation
    }

    #[test]
    fn test_builder_debug_trait() {
        let builder = QueryParamsBuilder::new().page_size(10).user_id("test123");

        let debug_str = format!("{:?}", builder);
        assert!(debug_str.contains("QueryParamsBuilder"));
        assert!(debug_str.contains("params"));
    }

    // ==================== Pagination Methods Testing ====================

    #[test]
    fn test_pagination_methods() {
        let params = QueryParamsBuilder::new()
            .page_size(20)
            .page_token("token_123")
            .page(1)
            .offset(100)
            .limit(50)
            .build();

        assert_eq!(params.len(), 5);
        assert_eq!(params.get("page_size"), Some(&"20".to_string()));
        assert_eq!(params.get("page_token"), Some(&"token_123".to_string()));
        assert_eq!(params.get("page"), Some(&"1".to_string()));
        assert_eq!(params.get("offset"), Some(&"100".to_string()));
        assert_eq!(params.get("limit"), Some(&"50".to_string()));
    }

    #[test]
    fn test_page_size_edge_cases() {
        let params = QueryParamsBuilder::new().page_size(0).build();
        assert_eq!(params.get("page_size"), Some(&"0".to_string()));

        let params = QueryParamsBuilder::new().page_size(-1).build();
        assert_eq!(params.get("page_size"), Some(&"-1".to_string()));

        let params = QueryParamsBuilder::new().page_size(i32::MAX).build();
        assert_eq!(params.get("page_size"), Some(&i32::MAX.to_string()));
    }

    #[test]
    fn test_page_token_different_types() {
        let params = QueryParamsBuilder::new().page_token("string_token").build();
        assert_eq!(params.get("page_token"), Some(&"string_token".to_string()));

        let params = QueryParamsBuilder::new()
            .page_token(String::from("owned_string"))
            .build();
        assert_eq!(params.get("page_token"), Some(&"owned_string".to_string()));

        let params = QueryParamsBuilder::new().page_token(123).build();
        assert_eq!(params.get("page_token"), Some(&"123".to_string()));
    }

    #[test]
    fn test_offset_limit_combinations() {
        let params = QueryParamsBuilder::new().offset(0).limit(10).build();
        assert_eq!(params.get("offset"), Some(&"0".to_string()));
        assert_eq!(params.get("limit"), Some(&"10".to_string()));

        let params = QueryParamsBuilder::new().offset(100).limit(0).build();
        assert_eq!(params.get("offset"), Some(&"100".to_string()));
        assert_eq!(params.get("limit"), Some(&"0".to_string()));
    }

    // ==================== Time Methods Testing ====================

    #[test]
    fn test_time_methods() {
        let params = QueryParamsBuilder::new()
            .start_time("2024-01-01T00:00:00Z")
            .end_time("2024-12-31T23:59:59Z")
            .build();

        assert_eq!(params.len(), 2);
        assert_eq!(
            params.get("start_time"),
            Some(&"2024-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            params.get("end_time"),
            Some(&"2024-12-31T23:59:59Z".to_string())
        );
    }

    #[test]
    fn test_time_methods_different_formats() {
        let params = QueryParamsBuilder::new()
            .start_time("1640995200") // Unix timestamp
            .end_time("2024-01-01") // Date only
            .build();

        assert_eq!(params.get("start_time"), Some(&"1640995200".to_string()));
        assert_eq!(params.get("end_time"), Some(&"2024-01-01".to_string()));
    }

    #[test]
    fn test_time_methods_empty_strings() {
        let params = QueryParamsBuilder::new()
            .start_time("")
            .end_time("")
            .build();

        assert_eq!(params.get("start_time"), Some(&"".to_string()));
        assert_eq!(params.get("end_time"), Some(&"".to_string()));
    }

    // ==================== User Identity Methods Testing ====================

    #[test]
    fn test_user_identity_methods() {
        let params = QueryParamsBuilder::new()
            .user_id("user_123")
            .user_id_type("open_id")
            .department_id("dept_456")
            .org_id("org_789")
            .build();

        assert_eq!(params.len(), 4);
        assert_eq!(params.get("user_id"), Some(&"user_123".to_string()));
        assert_eq!(params.get("user_id_type"), Some(&"open_id".to_string()));
        assert_eq!(params.get("department_id"), Some(&"dept_456".to_string()));
        assert_eq!(params.get("org_id"), Some(&"org_789".to_string()));
    }

    #[test]
    fn test_user_id_type_variants() {
        let params = QueryParamsBuilder::new().user_id_type("open_id").build();
        assert_eq!(params.get("user_id_type"), Some(&"open_id".to_string()));

        let params = QueryParamsBuilder::new().user_id_type("union_id").build();
        assert_eq!(params.get("user_id_type"), Some(&"union_id".to_string()));

        let params = QueryParamsBuilder::new().user_id_type("user_id").build();
        assert_eq!(params.get("user_id_type"), Some(&"user_id".to_string()));
    }

    #[test]
    fn test_user_methods_with_unicode() {
        let params = QueryParamsBuilder::new()
            .user_id("用户_123")
            .department_id("部门_456")
            .build();

        assert_eq!(params.get("user_id"), Some(&"用户_123".to_string()));
        assert_eq!(params.get("department_id"), Some(&"部门_456".to_string()));
    }

    // ==================== Status and Type Methods Testing ====================

    #[test]
    fn test_status_type_methods() {
        let params = QueryParamsBuilder::new()
            .status("active")
            .r#type("document")
            .rule_type("approval")
            .access_type("read")
            .build();

        assert_eq!(params.len(), 4);
        assert_eq!(params.get("status"), Some(&"active".to_string()));
        assert_eq!(params.get("type"), Some(&"document".to_string()));
        assert_eq!(params.get("rule_type"), Some(&"approval".to_string()));
        assert_eq!(params.get("access_type"), Some(&"read".to_string()));
    }

    #[test]
    fn test_type_method_with_raw_identifier() {
        // Test the r#type method specifically since it uses a Rust keyword
        let params = QueryParamsBuilder::new().r#type("special_type").build();

        assert_eq!(params.get("type"), Some(&"special_type".to_string()));
    }

    #[test]
    fn test_status_variations() {
        let status_values = ["active", "inactive", "pending", "approved", "rejected"];

        for status in &status_values {
            let params = QueryParamsBuilder::new().status(*status).build();
            assert_eq!(params.get("status"), Some(&status.to_string()));
        }
    }

    // ==================== Custom and Optional Methods Testing ====================

    #[test]
    fn test_custom_static_method() {
        let params = QueryParamsBuilder::new()
            .custom_static("custom_param", "custom_value".to_string())
            .custom_static("another_param", "another_value".to_string())
            .build();

        assert_eq!(params.len(), 2);
        assert_eq!(
            params.get("custom_param"),
            Some(&"custom_value".to_string())
        );
        assert_eq!(
            params.get("another_param"),
            Some(&"another_value".to_string())
        );
    }

    #[test]
    fn test_optional_method_with_some() {
        let params = QueryParamsBuilder::new()
            .optional(QueryParams::PAGE_SIZE, Some(10))
            .optional(QueryParams::USER_ID, Some("user123"))
            .optional(QueryParams::STATUS, Some(String::from("active")))
            .build();

        assert_eq!(params.len(), 3);
        assert_eq!(params.get("page_size"), Some(&"10".to_string()));
        assert_eq!(params.get("user_id"), Some(&"user123".to_string()));
        assert_eq!(params.get("status"), Some(&"active".to_string()));
    }

    #[test]
    fn test_optional_method_with_none() {
        let params = QueryParamsBuilder::new()
            .optional(QueryParams::PAGE_SIZE, None::<i32>)
            .optional(QueryParams::PAGE_TOKEN, None::<String>)
            .optional(QueryParams::USER_ID, None::<&str>)
            .build();

        assert_eq!(params.len(), 0);
        assert!(!params.contains_key("page_size"));
        assert!(!params.contains_key("page_token"));
        assert!(!params.contains_key("user_id"));
    }

    #[test]
    fn test_optional_method_mixed() {
        let params = QueryParamsBuilder::new()
            .optional(QueryParams::PAGE_SIZE, Some(10))
            .optional(QueryParams::PAGE_TOKEN, None::<String>)
            .optional(QueryParams::USER_ID, Some("user123"))
            .optional(QueryParams::STATUS, None::<&str>)
            .build();

        assert_eq!(params.len(), 2);
        assert_eq!(params.get("page_size"), Some(&"10".to_string()));
        assert_eq!(params.get("user_id"), Some(&"user123".to_string()));
        assert!(!params.contains_key("page_token"));
        assert!(!params.contains_key("status"));
    }

    // ==================== Build Methods Testing ====================

    #[test]
    fn test_build_method() {
        let builder = QueryParamsBuilder::new().page_size(20).user_id("test_user");

        let params = builder.build();

        assert_eq!(params.len(), 2);
        assert_eq!(params.get("page_size"), Some(&"20".to_string()));
        assert_eq!(params.get("user_id"), Some(&"test_user".to_string()));

        // Verify the HashMap is the expected type
        let _: HashMap<String, String> = params;
    }

    #[test]
    fn test_build_ref_method() {
        let builder = QueryParamsBuilder::new().page_size(20).user_id("test_user");

        let params_ref = builder.build_ref();

        assert_eq!(params_ref.len(), 2);
        assert_eq!(
            params_ref.get(QueryParams::PAGE_SIZE),
            Some(&"20".to_string())
        );
        assert_eq!(
            params_ref.get(QueryParams::USER_ID),
            Some(&"test_user".to_string())
        );

        // Verify the HashMap is the expected type with static str keys
        let _: &HashMap<&'static str, String> = params_ref;
    }

    #[test]
    fn test_build_empty() {
        let params = QueryParamsBuilder::new().build();
        assert_eq!(params.len(), 0);
        assert!(params.is_empty());
    }

    #[test]
    fn test_build_ref_empty() {
        let builder = QueryParamsBuilder::new();
        let params_ref = builder.build_ref();
        assert_eq!(params_ref.len(), 0);
        assert!(params_ref.is_empty());
    }

    // ==================== Chain Building Testing ====================

    #[test]
    fn test_comprehensive_chaining() {
        let params = QueryParamsBuilder::new()
            .page_size(50)
            .page_token("next_page_123")
            .start_time("2024-01-01T00:00:00Z")
            .end_time("2024-12-31T23:59:59Z")
            .user_id("user_456")
            .user_id_type("open_id")
            .status("active")
            .r#type("document")
            .custom_static("custom_field", "custom_value".to_string())
            .optional(QueryParams::LIMIT, Some(100))
            .optional(QueryParams::OFFSET, None::<i32>)
            .build();

        assert_eq!(params.len(), 10); // All non-None values: page_size, page_token, start_time, end_time, user_id, user_id_type, status, type, custom_field, limit
        assert_eq!(params.get("page_size"), Some(&"50".to_string()));
        assert_eq!(params.get("page_token"), Some(&"next_page_123".to_string()));
        assert_eq!(
            params.get("start_time"),
            Some(&"2024-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            params.get("end_time"),
            Some(&"2024-12-31T23:59:59Z".to_string())
        );
        assert_eq!(params.get("user_id"), Some(&"user_456".to_string()));
        assert_eq!(params.get("user_id_type"), Some(&"open_id".to_string()));
        assert_eq!(params.get("status"), Some(&"active".to_string()));
        assert_eq!(params.get("type"), Some(&"document".to_string()));
        assert_eq!(
            params.get("custom_field"),
            Some(&"custom_value".to_string())
        );
        assert_eq!(params.get("limit"), Some(&"100".to_string()));
        assert!(!params.contains_key("offset"));
    }

    #[test]
    fn test_method_overwriting() {
        let params = QueryParamsBuilder::new()
            .page_size(10)
            .page_size(20) // Should overwrite the previous value
            .user_id("user1")
            .user_id("user2") // Should overwrite the previous value
            .build();

        assert_eq!(params.len(), 2);
        assert_eq!(params.get("page_size"), Some(&"20".to_string()));
        assert_eq!(params.get("user_id"), Some(&"user2".to_string()));
    }

    #[test]
    fn test_large_chain_building() {
        let mut builder = QueryParamsBuilder::new();

        // Chain 20 different parameters
        builder = builder
            .page_size(10)
            .page_token("token")
            .page(1)
            .offset(0)
            .limit(50)
            .start_time("2024-01-01")
            .end_time("2024-12-31")
            .user_id("user123")
            .user_id_type("open_id")
            .department_id("dept456")
            .org_id("org789")
            .status("active")
            .r#type("document")
            .rule_type("approval")
            .access_type("read")
            .custom_static("param1", "value1".to_string())
            .custom_static("param2", "value2".to_string())
            .optional(QueryParams::LANGUAGE, Some("en"))
            .optional(QueryParams::LOCALE, Some("en_US"))
            .optional("extra_param", Some("extra_value"));

        let params = builder.build();
        assert_eq!(params.len(), 20);
    }

    // ==================== Memory and Performance Testing ====================

    #[test]
    fn test_with_capacity_performance() {
        let large_capacity = 1000;
        let builder = QueryParamsBuilder::with_capacity(large_capacity);

        // Builder should start empty regardless of capacity
        assert_eq!(builder.len(), 0);
        assert!(builder.is_empty());
    }

    #[test]
    fn test_memory_efficiency_static_strings() {
        let params1 = QueryParamsBuilder::new().page_size(10).build();

        let params2 = QueryParamsBuilder::new().page_size(20).build();

        // Both should use the same static string key "page_size"
        let key1 = params1.keys().next().unwrap();
        let key2 = params2.keys().next().unwrap();

        assert_eq!(key1, key2);
        assert_eq!(key1, "page_size");
    }

    #[test]
    fn test_string_conversion_consistency() {
        let builder = QueryParamsBuilder::new()
            .page_size(42)
            .user_id("test_user")
            .status("active");

        let params_owned = builder.build();
        let params_ref = {
            let temp_builder = QueryParamsBuilder::new()
                .page_size(42)
                .user_id("test_user")
                .status("active");
            // Create a temporary reference to test the ref method
            temp_builder.build_ref().clone()
        };

        // Values should be the same between build() and build_ref()
        assert_eq!(params_owned.get("page_size"), Some(&"42".to_string()));
        assert_eq!(
            params_ref.get(QueryParams::PAGE_SIZE),
            Some(&"42".to_string())
        );
    }

    // ==================== Edge Cases and Error Conditions ====================

    #[test]
    fn test_empty_string_values() {
        let params = QueryParamsBuilder::new()
            .page_token("")
            .user_id("")
            .status("")
            .build();

        assert_eq!(params.len(), 3);
        assert_eq!(params.get("page_token"), Some(&"".to_string()));
        assert_eq!(params.get("user_id"), Some(&"".to_string()));
        assert_eq!(params.get("status"), Some(&"".to_string()));
    }

    #[test]
    fn test_unicode_and_special_characters() {
        let params = QueryParamsBuilder::new()
            .user_id("用户_123_éñ")
            .page_token("🚀🎯📊")
            .status("状态_active")
            .custom_static("unicode_key", "测试值_with_emoji_🧪".to_string())
            .build();

        assert_eq!(params.get("user_id"), Some(&"用户_123_éñ".to_string()));
        assert_eq!(params.get("page_token"), Some(&"🚀🎯📊".to_string()));
        assert_eq!(params.get("status"), Some(&"状态_active".to_string()));
        assert_eq!(
            params.get("unicode_key"),
            Some(&"测试值_with_emoji_🧪".to_string())
        );
    }

    #[test]
    fn test_extreme_numeric_values() {
        let params = QueryParamsBuilder::new()
            .page_size(i32::MAX)
            .offset(i32::MIN)
            .limit(0)
            .page(-1)
            .build();

        assert_eq!(params.get("page_size"), Some(&i32::MAX.to_string()));
        assert_eq!(params.get("offset"), Some(&i32::MIN.to_string()));
        assert_eq!(params.get("limit"), Some(&"0".to_string()));
        assert_eq!(params.get("page"), Some(&"-1".to_string()));
    }

    #[test]
    fn test_very_long_strings() {
        let long_string = "a".repeat(10000);
        let params = QueryParamsBuilder::new()
            .page_token(&long_string)
            .user_id(&long_string)
            .build();

        assert_eq!(params.get("page_token"), Some(&long_string));
        assert_eq!(params.get("user_id"), Some(&long_string));
    }

    #[test]
    fn test_builder_is_consumed_by_build() {
        let builder = QueryParamsBuilder::new().page_size(10).user_id("test");

        let _params = builder.build();
        // builder is now consumed and cannot be used again
        // This test just verifies the consumption pattern compiles correctly
    }

    #[test]
    fn test_optional_with_different_types() {
        let params = QueryParamsBuilder::new()
            .optional("int_param", Some(42))
            .optional("string_param", Some("test"))
            .optional("owned_string_param", Some(String::from("owned")))
            .optional("none_int", None::<i32>)
            .optional("none_string", None::<String>)
            .build();

        assert_eq!(params.len(), 3);
        assert_eq!(params.get("int_param"), Some(&"42".to_string()));
        assert_eq!(params.get("string_param"), Some(&"test".to_string()));
        assert_eq!(params.get("owned_string_param"), Some(&"owned".to_string()));
        assert!(!params.contains_key("none_int"));
        assert!(!params.contains_key("none_string"));
    }

    // ==================== Documentation Example Verification ====================

    #[test]
    fn test_documentation_example() {
        // Verify the example from the module documentation works correctly
        let params = QueryParamsBuilder::new()
            .page_size(20)
            .page_token("token_123")
            .start_time("2024-01-01T00:00:00Z")
            .build();

        assert_eq!(params.get(QueryParams::PAGE_SIZE), Some(&"20".to_string()));
        assert_eq!(
            params.get(QueryParams::PAGE_TOKEN),
            Some(&"token_123".to_string())
        );
        assert_eq!(
            params.get(QueryParams::START_TIME),
            Some(&"2024-01-01T00:00:00Z".to_string())
        );
    }

    #[test]
    fn test_static_string_memory_optimization() {
        // Test that demonstrates the memory optimization of using static strings
        let mut params = HashMap::new();
        let value = "20".to_string();

        // This is the optimized approach using constants
        params.insert(QueryParams::PAGE_SIZE, value.clone());
        assert_eq!(params.get(QueryParams::PAGE_SIZE), Some(&value));

        // Verify the key is indeed a static string
        assert_eq!(QueryParams::PAGE_SIZE, "page_size");

        // The key should be from static memory (pointer comparison would be same)
        let key1 = QueryParams::PAGE_SIZE;
        let key2 = QueryParams::PAGE_SIZE;
        assert_eq!(key1.as_ptr(), key2.as_ptr()); // Same memory location
    }
}

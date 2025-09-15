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
//! // 优化前: 每次调用都分配新字符串
//! params.insert("page_size", value); // ~9字节堆分配
//!
//! // 优化后: 使用静态字符串常量
//! params.insert(QueryParams::PAGE_SIZE, value);  // ~8字节栈引用
//! ```
//!
//! # 使用示例
//!
//! ```rust
//! use crate::core::query_params::QueryParams;
//! use std::collections::HashMap;
//!
//! let mut params = HashMap::new();
//! params.insert(QueryParams::PAGE_SIZE, "20".to_string());
//! params.insert(QueryParams::PAGE_TOKEN, token);
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
/// let params = QueryParamsBuilder::new()
///     .page_size(20)
///     .page_token("token_123")
///     .start_time("2024-01-01T00:00:00Z")
///     .build();
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

    #[test]
    fn test_query_params_constants() {
        assert_eq!(QueryParams::PAGE_SIZE, "page_size");
        assert_eq!(QueryParams::PAGE_TOKEN, "page_token");
        assert_eq!(QueryParams::USER_ID, "user_id");
    }

    #[test]
    fn test_query_params_builder() {
        let params = QueryParamsBuilder::new()
            .page_size(20)
            .page_token("token_123")
            .user_id("user_456")
            .build();

        assert_eq!(params.len(), 3);
        assert_eq!(params.get("page_size"), Some(&"20".to_string()));
        assert_eq!(params.get("page_token"), Some(&"token_123".to_string()));
        assert_eq!(params.get("user_id"), Some(&"user_456".to_string()));
    }

    #[test]
    fn test_builder_with_capacity() {
        let builder = QueryParamsBuilder::with_capacity(5);
        assert_eq!(builder.len(), 0);
        assert!(builder.is_empty());
    }

    #[test]
    fn test_optional_params() {
        let params = QueryParamsBuilder::new()
            .optional(QueryParams::PAGE_SIZE, Some(10))
            .optional(QueryParams::PAGE_TOKEN, None::<String>)
            .build();

        assert_eq!(params.len(), 1);
        assert_eq!(params.get("page_size"), Some(&"10".to_string()));
        assert!(!params.contains_key("page_token"));
    }
}

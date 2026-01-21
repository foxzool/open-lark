//! 数据分析常量

/// API 端点常量
pub mod endpoints {
    /// 搜索 API 基础路径
    pub const SEARCH_BASE: &str = "/open-apis/search";
}

/// 搜索类型常量
pub mod search_type {
    /// 全文搜索
    pub const FULL_TEXT: &str = "full_text";
    /// 智能搜索
    pub const SMART: &str = "smart";
    /// 模糊搜索
    pub const FUZZY: &str = "fuzzy";
}

//! Search 服务端点常量定义（v1/v2）

// ==================== v1 ====================
pub const SEARCH_V1_USER: &str = "/open-apis/search/v1/user";

// ==================== v2 数据项 ====================
pub const SEARCH_V2_DATA_ITEM_CREATE: &str =
    "/open-apis/search/v2/data_sources/{data_source_id}/items";
pub const SEARCH_V2_DATA_ITEM_BATCH_CREATE: &str =
    "/open-apis/search/v2/data_sources/{data_source_id}/items/batch_create";
pub const SEARCH_V2_DATA_ITEM_OPERATION: &str =
    "/open-apis/search/v2/data_sources/{data_source_id}/items/{data_item_id}";

// ==================== v2 套件搜索 ====================
pub const SEARCH_V2_MESSAGE: &str = "/open-apis/search/v2/message";
pub const SEARCH_V2_APP: &str = "/open-apis/search/v2/app";

// ==================== v2 数据源 ====================
pub const SEARCH_V2_DATA_SOURCES: &str = "/open-apis/search/v2/data_sources";
pub const SEARCH_V2_DATA_SOURCE_OPERATION: &str =
    "/open-apis/search/v2/data_sources/{data_source_id}";

// ==================== v2 Schema ====================
pub const SEARCH_V2_SCHEMA_CREATE: &str =
    "/open-apis/search/v2/data_sources/{data_source_id}/schemas";
pub const SEARCH_V2_SCHEMA_OPERATION: &str =
    "/open-apis/search/v2/data_sources/{data_source_id}/schemas/{schema_id}";

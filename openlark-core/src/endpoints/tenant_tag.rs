//! tenant_tag 服务端点常量定义
//!
//! 企业标签管理相关 API 端点常量

/// 标签管理
pub const TENANT_TAG_V1_TAGS: &str = "/open-apis/tenant-tag/v1/tags";

/// 标签操作（增删改查）
pub const TENANT_TAG_V1_TAG_OPERATION: &str = "/open-apis/tenant-tag/v1/tags/{tag_id}";

/// 标签绑定管理
pub const TENANT_TAG_V1_TAG_BINDINGS: &str = "/open-apis/tenant-tag/v1/tag_bindings";

/// 标签绑定操作
pub const TENANT_TAG_V1_TAG_BINDING_OPERATION: &str =
    "/open-apis/tenant-tag/v1/tag_bindings/{binding_id}";

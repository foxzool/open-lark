//! Moments (动态分享) 端点定义
//!
//! 朋友圈动态 - 内容分享、社交互动
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_communication::endpoints::moments::*;
//!
//! let post_get_endpoint = MOMENTS_V1_POST_GET;
//! ```

// ==================== Moments (动态分享) v1 ====================
// 朋友圈动态 - 内容分享、社交互动

/// Moments 帖子查询 v1
/// 查询指定帖子信息
pub const MOMENTS_V1_POST_GET: &str = "/open-apis/moments/v1/posts/{post_id}";

/// Moments 帖子列表 v1
/// 获取帖子列表
pub const MOMENTS_V1_POSTS: &str = "/open-apis/moments/v1/posts";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moments_v1_endpoints() {
        assert!(MOMENTS_V1_POST_GET.starts_with("/open-apis/moments/v1/"));
        assert!(MOMENTS_V1_POST_GET.contains("{post_id}"));
        assert!(MOMENTS_V1_POSTS.starts_with("/open-apis/moments/v1/"));
}
}

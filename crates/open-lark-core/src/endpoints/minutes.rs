//! Minutes 妙记服务端点常量定义

// ===== 妙记管理 =====

/// 获取妙记信息 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
pub const MINUTES_V1_MINUTE_GET: &str = "/open-apis/minutes/v1/{minute_token}";

/// 获取妙记统计数据 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
pub const MINUTES_V1_STATISTICS_GET: &str = "/open-apis/minutes/v1/{minute_token}/statistics";

/// 导出妙记文字记录 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
pub const MINUTES_V1_TRANSCRIPT_GET: &str = "/open-apis/minutes/v1/{minute_token}/transcript";

/// 下载妙记音视频文件 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
pub const MINUTES_V1_MEDIA_GET: &str = "/open-apis/minutes/v1/{minute_token}/media";

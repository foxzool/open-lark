//! ACS (Access Control System) 服务端点常量定义

// ===== 设备管理 =====

/// 设备列表
pub const ACS_V1_DEVICES: &str = "/open-apis/acs/v1/devices";

// ===== 访客管理 =====

/// 访客列表
pub const ACS_V1_VISITORS: &str = "/open-apis/acs/v1/visitors";

/// 获取访客详情
pub const ACS_V1_VISITOR_GET: &str = "/open-apis/acs/v1/visitors/{visitor_id}";

// ===== 门禁记录 =====

/// 门禁记录
pub const ACS_V1_ACCESS_RECORDS: &str = "/open-apis/acs/v1/access_records";

/// 门禁记录人脸照片
pub const ACS_V1_ACCESS_RECORD_FACE_IMAGE: &str =
    "/open-apis/acs/v1/access_records/{access_record_id}/face_image";

// ===== 门禁规则 =====

/// 外部门禁规则
pub const ACS_V1_RULE_EXTERNAL: &str = "/open-apis/acs/v1/rule_external";

/// 外部门禁规则操作
pub const ACS_V1_RULE_EXTERNAL_OPERATION: &str = "/open-apis/acs/v1/rule_external/{rule_id}";

/// 外部门禁规则设备绑定
pub const ACS_V1_RULE_EXTERNAL_DEVICE_BIND: &str =
    "/open-apis/acs/v1/rule_external/{rule_id}/device_bind";

// ===== 用户管理 =====

/// 用户列表
pub const ACS_V1_USERS: &str = "/open-apis/acs/v1/users";

/// 用户操作
pub const ACS_V1_USER_OPERATION: &str = "/open-apis/acs/v1/users/{user_id}";

/// 用户人脸照片
pub const ACS_V1_USER_FACE_IMAGE: &str = "/open-apis/acs/v1/users/{user_id}/face_image";

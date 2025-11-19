//! Drive（云文档/文件）验证模块
//!
//! 提供文件相关功能的验证服务，包括文件上传、权限管理、搜索等功能。

use crate::validation::{ValidateBuilder, ValidationResult};

/// 验证文件类型
///
/// # 参数
/// - `file_type`: 文件类型
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_drive_file_type(file_type: &str) -> ValidationResult {
    const VALID_FILE_TYPES: &[&str] = &[
        "doc", "docx", "sheet", "bitable", "wiki", "file", "mindnote", "minutes", "slides", "pdf",
        "txt",
    ];

    if file_type.trim().is_empty() {
        return ValidationResult::Invalid("文件类型不能为空".to_string());
    }

    if !VALID_FILE_TYPES.contains(&file_type) {
        return ValidationResult::Invalid(format!(
            "不支持的文件类型: {}。支持的类型: {}",
            file_type,
            VALID_FILE_TYPES.join(", ")
        ));
    }

    ValidationResult::Valid
}

/// 验证文件夹名称
///
/// # 参数
/// - `name`: 文件夹名称
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_folder_name(name: &str) -> ValidationResult {
    if name.trim().is_empty() {
        return ValidationResult::Invalid("文件夹名称不能为空".to_string());
    }

    const MAX_FOLDER_NAME_LENGTH: usize = 255;
    if name.len() > MAX_FOLDER_NAME_LENGTH {
        return ValidationResult::Invalid(format!(
            "文件夹名称过长 ({} > {})",
            name.len(),
            MAX_FOLDER_NAME_LENGTH
        ));
    }

    // 检查非法字符
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    for c in name.chars() {
        if invalid_chars.contains(&c) {
            return ValidationResult::Invalid(format!("文件夹名称包含非法字符: '{}'", c));
        }
    }

    // 检查保留名称
    let reserved_names = ["CON", "PRN", "AUX", "NUL"];
    if reserved_names.contains(&name.to_uppercase().as_str()) {
        return ValidationResult::Invalid(format!("'{}' 是系统保留名称，不能使用", name));
    }

    ValidationResult::Valid
}

/// 验证文件上传参数
///
/// # 参数
/// - `file_name`: 文件名
/// - `file_size`: 文件大小（字节）
/// - `parent_token`: 父文件夹令牌
/// - `file_type`: 文件类型
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_file_upload_params(
    file_name: &str,
    file_size: u64,
    parent_token: &str,
    file_type: &str,
) -> ValidationResult {
    // 验证文件名
    if file_name.trim().is_empty() {
        return ValidationResult::Invalid("文件名不能为空".to_string());
    }

    const MAX_FILENAME_LENGTH: usize = 255;
    if file_name.len() > MAX_FILENAME_LENGTH {
        return ValidationResult::Invalid(format!(
            "文件名过长 ({} > {})",
            file_name.len(),
            MAX_FILENAME_LENGTH
        ));
    }

    // 检查文件名非法字符
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if file_name.chars().any(|c| invalid_chars.contains(&c)) {
        return ValidationResult::Invalid("文件名包含非法字符".to_string());
    }

    // 验证文件大小
    if file_size == 0 {
        return ValidationResult::Invalid("文件大小必须大于0".to_string());
    }

    // 根据文件类型设置不同的大小限制
    let max_size = match file_type {
        "image" => 20 * 1024 * 1024,  // 20MB
        "video" => 100 * 1024 * 1024, // 100MB
        "audio" => 50 * 1024 * 1024,  // 50MB
        _ => 100 * 1024 * 1024,       // 默认100MB
    };

    if file_size > max_size {
        return ValidationResult::Invalid(format!(
            "文件大小超过限制 ({} > {})",
            file_size, max_size
        ));
    }

    // 验证父文件夹令牌
    if let ValidationResult::Invalid(e) = validate_token_format(parent_token) {
        return ValidationResult::Invalid(e);
    }

    ValidationResult::Valid
}

/// 验证分片上传参数
///
/// # 参数
/// - `file_size`: 文件大小（字节）
/// - `block_size`: 分片大小（字节）
/// - `block_num`: 分片数量
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_multipart_upload_params(
    file_size: u64,
    block_size: Option<u32>,
    block_num: Option<u32>,
) -> ValidationResult {
    if file_size == 0 {
        return ValidationResult::Invalid("文件大小必须大于0".to_string());
    }

    // 验证分片大小
    if let Some(size) = block_size {
        if size < 1024 {
            // 最小1KB
            return ValidationResult::Invalid("分片大小不能小于1KB".to_string());
        }

        if size > 10 * 1024 * 1024 {
            // 最大10MB
            return ValidationResult::Invalid("分片大小不能超过10MB".to_string());
        }

        if file_size < size as u64 {
            return ValidationResult::Invalid("分片大小不能大于文件大小".to_string());
        }
    }

    // 验证分片数量
    if let Some(num) = block_num {
        if num == 0 {
            return ValidationResult::Invalid("分片数量必须大于0".to_string());
        }

        if num > 10000 {
            return ValidationResult::Invalid("分片数量不能超过10000".to_string());
        }
    }

    ValidationResult::Valid
}

/// 验证权限级别
///
/// # 参数
/// - `permission`: 权限级别
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_permission_level(permission: &str) -> ValidationResult {
    const VALID_PERMISSIONS: &[&str] = &[
        "FullAccess",
        "Edit",
        "View",
        "Comment",
        "CanDownload",
        "CanManageMembers",
    ];

    if permission.trim().is_empty() {
        return ValidationResult::Invalid("权限级别不能为空".to_string());
    }

    if !VALID_PERMISSIONS.contains(&permission) {
        return ValidationResult::Invalid(format!(
            "无效的权限级别: {}。支持的权限: {}",
            permission,
            VALID_PERMISSIONS.join(", ")
        ));
    }

    ValidationResult::Valid
}

/// 验证成员类型
///
/// # 参数
/// - `member_type`: 成员类型
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_member_type(member_type: &str) -> ValidationResult {
    const VALID_MEMBER_TYPES: &[&str] = &["user", "chat", "department"];

    if member_type.trim().is_empty() {
        return ValidationResult::Invalid("成员类型不能为空".to_string());
    }

    if !VALID_MEMBER_TYPES.contains(&member_type) {
        return ValidationResult::Invalid(format!(
            "无效的成员类型: {}。支持的类型: {}",
            member_type,
            VALID_MEMBER_TYPES.join(", ")
        ));
    }

    ValidationResult::Valid
}

/// 验证成员ID格式
///
/// # 参数
/// - `member_type`: 成员类型
/// - `member_id`: 成员ID
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_member_id(member_type: &str, member_id: &str) -> ValidationResult {
    if let ValidationResult::Invalid(e) = validate_member_type(member_type) {
        return ValidationResult::Invalid(e);
    }

    if member_id.trim().is_empty() {
        return ValidationResult::Invalid("成员ID不能为空".to_string());
    }

    // 根据成员类型验证ID格式
    match member_type {
        "user" => {
            // 用户ID通常以 ou_ 开头或为纯数字
            if !(member_id.starts_with("ou_") || member_id.chars().all(|c| c.is_ascii_digit())) {
                return ValidationResult::Invalid("用户ID格式无效".to_string());
            }

            if member_id.len() > 64 {
                return ValidationResult::Invalid("用户ID过长".to_string());
            }
        }
        "chat" => {
            // 群组ID通常以 oc_ 开头
            if !member_id.starts_with("oc_") {
                return ValidationResult::Invalid("群组ID应以'oc_'开头".to_string());
            }

            if member_id.len() != 28 {
                return ValidationResult::Invalid("群组ID长度应为28字符".to_string());
            }
        }
        "department" => {
            // 部门ID通常以 od_ 开头
            if !member_id.starts_with("od_") {
                return ValidationResult::Invalid("部门ID应以'od_'开头".to_string());
            }

            if member_id.is_empty() || member_id.len() > 64 {
                return ValidationResult::Invalid("部门ID长度无效".to_string());
            }
        }
        _ => {}
    }

    ValidationResult::Valid
}

/// 验证权限设置
///
/// # 参数
/// - `external_access`: 外部访问设置
/// - `security_entity`: 安全实体设置
/// - `comment_entity`: 评论实体设置
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_permission_settings(
    external_access: Option<&str>,
    security_entity: Option<&str>,
    comment_entity: Option<&str>,
) -> ValidationResult {
    // 验证外部访问设置
    if let Some(access) = external_access {
        const VALID_EXTERNAL_ACCESS: &[&str] = &[
            "open",
            "closed",
            "allow_share_partner_tenant",
            "limited_open",
        ];
        if !VALID_EXTERNAL_ACCESS.contains(&access) {
            return ValidationResult::Invalid(format!(
                "无效的外部访问设置: {}。支持的选项: {}",
                access,
                VALID_EXTERNAL_ACCESS.join(", ")
            ));
        }
    }

    // 验证安全实体设置
    if let Some(security) = security_entity {
        const VALID_SECURITY_ENTITIES: &[&str] = &[
            "anyone_can_view",
            "anyone_can_edit",
            "only_full_access",
            "specified_members",
        ];
        if !VALID_SECURITY_ENTITIES.contains(&security) {
            return ValidationResult::Invalid(format!(
                "无效的安全实体设置: {}。支持的选项: {}",
                security,
                VALID_SECURITY_ENTITIES.join(", ")
            ));
        }
    }

    // 验证评论实体设置
    if let Some(comment) = comment_entity {
        const VALID_COMMENT_ENTITIES: &[&str] =
            &["anyone_can_view", "anyone_can_edit", "no_comment"];
        if !VALID_COMMENT_ENTITIES.contains(&comment) {
            return ValidationResult::Invalid(format!(
                "无效的评论实体设置: {}。支持的选项: {}",
                comment,
                VALID_COMMENT_ENTITIES.join(", ")
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证分页参数
///
/// # 参数
/// - `page_size`: 页面大小
/// - `page_token`: 分页标记
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_pagination_params(
    page_size: Option<i32>,
    page_token: Option<&str>,
) -> ValidationResult {
    if let Some(size) = page_size {
        if size <= 0 {
            return ValidationResult::Invalid("页面大小必须大于0".to_string());
        }

        if size > 500 {
            return ValidationResult::Invalid("页面大小不能超过500".to_string());
        }
    }

    if let Some(token) = page_token {
        if token.trim().is_empty() {
            return ValidationResult::Invalid("页面令牌不能为空".to_string());
        }

        // 验证页面令牌格式（通常是Base64编码的字符串）
        let min_len = 1;
        let max_len = 1000;

        if token.len() < min_len || token.len() > max_len {
            return ValidationResult::Invalid(format!(
                "页面令牌长度无效 ({}-{} 字符)",
                min_len, max_len
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证搜索参数
///
/// # 参数
/// - `search_key`: 搜索关键词
/// - `count`: 返回数量
/// - `offset`: 偏移量
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_search_params(
    search_key: &str,
    count: Option<i32>,
    offset: Option<i32>,
) -> ValidationResult {
    if search_key.trim().is_empty() {
        return ValidationResult::Invalid("搜索关键词不能为空".to_string());
    }

    if search_key.len() > 200 {
        return ValidationResult::Invalid("搜索关键词过长 (最多200字符)".to_string());
    }

    if let Some(cnt) = count {
        if cnt <= 0 {
            return ValidationResult::Invalid("搜索数量必须大于0".to_string());
        }

        if cnt > 200 {
            return ValidationResult::Invalid("搜索数量不能超过200".to_string());
        }
    }

    if let Some(off) = offset {
        if off < 0 {
            return ValidationResult::Invalid("偏移量不能为负数".to_string());
        }

        if off > 10000 {
            return ValidationResult::Invalid("偏移量过大 (最大10000)".to_string());
        }
    }

    ValidationResult::Valid
}

/// 验证文件/文件夹令牌格式
///
/// # 参数
/// - `token`: 令牌
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_token_format(token: &str) -> ValidationResult {
    if token.trim().is_empty() {
        return ValidationResult::Invalid("令牌不能为空".to_string());
    }

    // 飞书文档令牌通常是Base64格式或特定前缀
    let min_len = 10;
    let max_len = 100;

    if token.len() < min_len || token.len() > max_len {
        return ValidationResult::Invalid(format!("令牌长度无效 ({}-{} 字符)", min_len, max_len));
    }

    // 检查令牌字符（允许字母、数字、下划线、连字符、等号）
    if !token
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '+' || c == '=' || c == '/')
    {
        return ValidationResult::Invalid("令牌包含非法字符".to_string());
    }

    ValidationResult::Valid
}

/// 验证排序参数
///
/// # 参数
/// - `order_by`: 排序字段
/// - `direction`: 排序方向
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_sort_params(order_by: Option<&str>, direction: Option<&str>) -> ValidationResult {
    if let Some(order) = order_by {
        const VALID_ORDER_FIELDS: &[&str] = &[
            "created_time",
            "edited_time",
            "file_type",
            "size",
            "name",
            "viewed_time",
        ];

        if !VALID_ORDER_FIELDS.contains(&order) {
            return ValidationResult::Invalid(format!(
                "无效的排序字段: {}。支持的字段: {}",
                order,
                VALID_ORDER_FIELDS.join(", ")
            ));
        }
    }

    if let Some(dir) = direction {
        const VALID_DIRECTIONS: &[&str] = &["ASC", "DESC"];

        if !VALID_DIRECTIONS.contains(&dir) {
            return ValidationResult::Invalid(format!(
                "无效的排序方向: {}。支持的方向: {}",
                dir,
                VALID_DIRECTIONS.join(", ")
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证文件复制/移动参数
///
/// # 参数
/// - `source_token`: 源文件/文件夹令牌
/// - `destination_folder_token`: 目标文件夹令牌
/// - `new_name`: 新名称（可选）
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_file_copy_move_params(
    source_token: &str,
    destination_folder_token: &str,
    new_name: Option<&str>,
) -> ValidationResult {
    if let ValidationResult::Invalid(e) = validate_token_format(source_token) {
        return ValidationResult::Invalid(e);
    }
    if let ValidationResult::Invalid(e) = validate_token_format(destination_folder_token) {
        return ValidationResult::Invalid(e);
    }

    // 如果有新名称，验证名称
    if let Some(name) = new_name {
        if name.trim().is_empty() {
            return ValidationResult::Invalid("新名称不能为空".to_string());
        }

        const MAX_NAME_LENGTH: usize = 255;
        if name.len() > MAX_NAME_LENGTH {
            return ValidationResult::Invalid(format!(
                "新名称过长 ({} > {})",
                name.len(),
                MAX_NAME_LENGTH
            ));
        }

        // 检查非法字符
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        if name.chars().any(|c| invalid_chars.contains(&c)) {
            return ValidationResult::Invalid("新名称包含非法字符".to_string());
        }
    }

    // 不能复制到自身
    if source_token == destination_folder_token {
        return ValidationResult::Invalid("不能将文件复制/移动到自身".to_string());
    }

    ValidationResult::Valid
}

/// 验证批量操作参数
///
/// # 参数
/// - `tokens`: 文件/文件夹令牌列表
/// - `max_batch_size`: 最大批量大小
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_batch_operation(tokens: &[String], max_batch_size: usize) -> ValidationResult {
    if tokens.is_empty() {
        return ValidationResult::Invalid("批量操作列表不能为空".to_string());
    }

    if tokens.len() > max_batch_size {
        return ValidationResult::Invalid(format!(
            "批量操作数量超过限制 ({} > {})",
            tokens.len(),
            max_batch_size
        ));
    }

    // 验证每个令牌
    for (i, token) in tokens.iter().enumerate() {
        if let ValidationResult::Invalid(msg) = validate_token_format(token) {
            return ValidationResult::Invalid(format!("第{}个令牌无效: {}", i + 1, msg));
        }
    }

    // 检查重复令牌
    let unique_tokens: std::collections::HashSet<_> = tokens.iter().collect();
    if unique_tokens.len() != tokens.len() {
        return ValidationResult::Invalid("批量操作列表包含重复的令牌".to_string());
    }

    ValidationResult::Valid
}

/// 验证文件下载参数
///
/// # 参数
/// - `file_token`: 文件令牌
/// - `file_type`: 文件类型
/// - `expire_time`: 过期时间（秒）
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_file_download_params(
    file_token: &str,
    file_type: &str,
    expire_time: Option<u32>,
) -> ValidationResult {
    if let ValidationResult::Invalid(e) = validate_token_format(file_token) {
        return ValidationResult::Invalid(e);
    }
    if let ValidationResult::Invalid(e) = validate_drive_file_type(file_type) {
        return ValidationResult::Invalid(e);
    }

    if let Some(expire) = expire_time {
        if expire == 0 {
            return ValidationResult::Invalid("过期时间必须大于0".to_string());
        }

        const MAX_EXPIRE_TIME: u32 = 7 * 24 * 3600; // 7天
        if expire > MAX_EXPIRE_TIME {
            return ValidationResult::Invalid(format!("过期时间不能超过{}秒", MAX_EXPIRE_TIME));
        }
    }

    ValidationResult::Valid
}

/// Builder trait for Drive validation
pub trait ValidateDriveBuilder {
    /// 验证文件类型
    fn validate_drive_file_type(&self, file_type: &str) -> ValidationResult {
        validate_drive_file_type(file_type)
    }

    /// 验证文件夹名称
    fn validate_folder_name(&self, name: &str) -> ValidationResult {
        validate_folder_name(name)
    }

    /// 验证文件上传参数
    fn validate_file_upload_params(
        &self,
        file_name: &str,
        file_size: u64,
        parent_token: &str,
        file_type: &str,
    ) -> ValidationResult {
        validate_file_upload_params(file_name, file_size, parent_token, file_type)
    }

    /// 验证权限级别
    fn validate_permission_level(&self, permission: &str) -> ValidationResult {
        validate_permission_level(permission)
    }

    /// 验证成员ID
    fn validate_member_id(&self, member_type: &str, member_id: &str) -> ValidationResult {
        validate_member_id(member_type, member_id)
    }

    /// 验证分页参数
    fn validate_pagination_params(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
    ) -> ValidationResult {
        validate_pagination_params(page_size, page_token)
    }

    /// 验证搜索参数
    fn validate_search_params(
        &self,
        search_key: &str,
        count: Option<i32>,
        offset: Option<i32>,
    ) -> ValidationResult {
        validate_search_params(search_key, count, offset)
    }

    /// 验证批量操作
    fn validate_batch_operation(
        &self,
        tokens: &[String],
        max_batch_size: usize,
    ) -> ValidationResult {
        validate_batch_operation(tokens, max_batch_size)
    }
}

// 为所有实现了 ValidateBuilder 的类型自动实现 ValidateDriveBuilder
impl<T: ValidateBuilder> ValidateDriveBuilder for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_drive_file_type() {
        assert!(matches!(
            validate_drive_file_type("doc"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_drive_file_type("sheet"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_drive_file_type("invalid"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_folder_name() {
        assert!(matches!(
            validate_folder_name("Valid Folder"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_folder_name(""),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_folder_name("Invalid/Name"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_folder_name("CON"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_file_upload_params() {
        assert!(matches!(
            validate_file_upload_params("test.txt", 1024, "folder_token", "file"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_file_upload_params("", 1024, "folder_token", "file"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_file_upload_params("test.txt", 0, "folder_token", "file"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_permission_level() {
        assert!(matches!(
            validate_permission_level("FullAccess"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_permission_level("View"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_permission_level("Invalid"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_member_id() {
        assert!(matches!(
            validate_member_id("user", "ou_1234567890"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_member_id("chat", "oc_1234567890123456789012345"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_member_id("department", "od_1234567890"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_member_id("user", ""),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_batch_operation() {
        let tokens = vec![
            "doccnABC123DEF456GHI789JKL012MN".to_string(),
            "doccnXYZ789ABC456DEF789GHI012JK".to_string(),
        ];
        assert!(matches!(
            validate_batch_operation(&tokens, 10),
            ValidationResult::Valid
        ));

        let empty_tokens: Vec<String> = vec![];
        assert!(matches!(
            validate_batch_operation(&empty_tokens, 10),
            ValidationResult::Invalid(_)
        ));

        let duplicate_tokens = vec!["token1".to_string(), "token1".to_string()];
        assert!(matches!(
            validate_batch_operation(&duplicate_tokens, 10),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_search_params() {
        assert!(matches!(
            validate_search_params("test", Some(10), Some(0)),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_search_params("", Some(10), Some(0)),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_search_params("test", Some(0), Some(0)),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_search_params("test", Some(300), Some(0)),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_multipart_upload_params() {
        // Valid case
        assert!(matches!(
            validate_multipart_upload_params(1024 * 1024, Some(64 * 1024), Some(16)),
            ValidationResult::Valid
        ));

        // Invalid file size
        assert!(matches!(
            validate_multipart_upload_params(0, Some(64 * 1024), Some(16)),
            ValidationResult::Invalid(_)
        ));

        // Block size too small
        assert!(matches!(
            validate_multipart_upload_params(1024, Some(512), Some(16)),
            ValidationResult::Invalid(_)
        ));

        // Block size too large
        assert!(matches!(
            validate_multipart_upload_params(1024, Some(20 * 1024 * 1024), Some(16)),
            ValidationResult::Invalid(_)
        ));

        // Block size larger than file
        assert!(matches!(
            validate_multipart_upload_params(1024, Some(2048), Some(16)),
            ValidationResult::Invalid(_)
        ));

        // Invalid block number
        assert!(matches!(
            validate_multipart_upload_params(1024, Some(64 * 1024), Some(0)),
            ValidationResult::Invalid(_)
        ));

        // Block number too large
        assert!(matches!(
            validate_multipart_upload_params(1024, Some(64 * 1024), Some(20000)),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_member_type() {
        assert!(matches!(
            validate_member_type("user"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_member_type("chat"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_member_type("department"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_member_type("invalid"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_member_type(""),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_permission_settings() {
        // All valid settings
        assert!(matches!(
            validate_permission_settings(
                Some("open"),
                Some("anyone_can_view"),
                Some("anyone_can_view")
            ),
            ValidationResult::Valid
        ));

        // Invalid external access
        assert!(matches!(
            validate_permission_settings(
                Some("invalid_access"),
                Some("anyone_can_view"),
                Some("anyone_can_view")
            ),
            ValidationResult::Invalid(_)
        ));

        // Invalid security entity
        assert!(matches!(
            validate_permission_settings(
                Some("open"),
                Some("invalid_security"),
                Some("anyone_can_view")
            ),
            ValidationResult::Invalid(_)
        ));

        // Invalid comment entity
        assert!(matches!(
            validate_permission_settings(
                Some("open"),
                Some("anyone_can_view"),
                Some("invalid_comment")
            ),
            ValidationResult::Invalid(_)
        ));

        // All None values (should be valid)
        assert!(matches!(
            validate_permission_settings(None, None, None),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_pagination_params() {
        // Valid case with all parameters
        assert!(matches!(
            validate_pagination_params(Some(50), Some("page_token_123")),
            ValidationResult::Valid
        ));

        // Valid case with only page size
        assert!(matches!(
            validate_pagination_params(Some(100), None),
            ValidationResult::Valid
        ));

        // Valid case with only page token
        assert!(matches!(
            validate_pagination_params(None, Some("valid_token")),
            ValidationResult::Valid
        ));

        // Invalid page size - too small
        assert!(matches!(
            validate_pagination_params(Some(0), Some("token")),
            ValidationResult::Invalid(_)
        ));

        // Invalid page size - too large
        assert!(matches!(
            validate_pagination_params(Some(1000), Some("token")),
            ValidationResult::Invalid(_)
        ));

        // Invalid page token - empty
        assert!(matches!(
            validate_pagination_params(Some(10), Some("")),
            ValidationResult::Invalid(_)
        ));

        // Invalid page token - too short (minimum length is 1, so this is valid)
        // This test was incorrect - removing it

        // Invalid page token - too long
        let long_token = "a".repeat(1001);
        assert!(matches!(
            validate_pagination_params(Some(10), Some(&long_token)),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_token_format() {
        // Valid tokens
        assert!(matches!(
            validate_token_format("doccnABC123DEF456GHI789JKL012MN"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_token_format("oc_1234567890123456789012345"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_token_format("valid_token_123"),
            ValidationResult::Valid
        ));

        // Empty token
        assert!(matches!(
            validate_token_format(""),
            ValidationResult::Invalid(_)
        ));

        // Token too short
        assert!(matches!(
            validate_token_format("short"),
            ValidationResult::Invalid(_)
        ));

        // Token too long
        let long_token = "a".repeat(101);
        assert!(matches!(
            validate_token_format(&long_token),
            ValidationResult::Invalid(_)
        ));

        // Token with invalid characters
        assert!(matches!(
            validate_token_format("token@invalid"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_token_format("token#invalid"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_token_format("token space"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_sort_params() {
        // Valid parameters
        assert!(matches!(
            validate_sort_params(Some("created_time"), Some("ASC")),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_sort_params(Some("name"), Some("DESC")),
            ValidationResult::Valid
        ));

        // Only order by
        assert!(matches!(
            validate_sort_params(Some("size"), None),
            ValidationResult::Valid
        ));

        // Only direction
        assert!(matches!(
            validate_sort_params(None, Some("ASC")),
            ValidationResult::Valid
        ));

        // None values
        assert!(matches!(
            validate_sort_params(None, None),
            ValidationResult::Valid
        ));

        // Invalid order field
        assert!(matches!(
            validate_sort_params(Some("invalid_field"), Some("ASC")),
            ValidationResult::Invalid(_)
        ));

        // Invalid direction
        assert!(matches!(
            validate_sort_params(Some("created_time"), Some("INVALID")),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_file_copy_move_params() {
        // Valid case
        assert!(matches!(
            validate_file_copy_move_params("src_token_123", "dest_token_456", Some("new_name")),
            ValidationResult::Valid
        ));

        // Valid case without new name
        assert!(matches!(
            validate_file_copy_move_params("src_token_12345", "dest_token_67890", None),
            ValidationResult::Valid
        ));

        // Invalid source token
        assert!(matches!(
            validate_file_copy_move_params("", "dest_token", Some("name")),
            ValidationResult::Invalid(_)
        ));

        // Invalid destination token
        assert!(matches!(
            validate_file_copy_move_params("src_token", "", Some("name")),
            ValidationResult::Invalid(_)
        ));

        // Empty new name
        assert!(matches!(
            validate_file_copy_move_params("src_token", "dest_token", Some("")),
            ValidationResult::Invalid(_)
        ));

        // Invalid new name (too long)
        let long_name = "a".repeat(256);
        assert!(matches!(
            validate_file_copy_move_params("src_token", "dest_token", Some(&long_name)),
            ValidationResult::Invalid(_)
        ));

        // Invalid new name (contains invalid chars)
        assert!(matches!(
            validate_file_copy_move_params("src_token", "dest_token", Some("invalid/name")),
            ValidationResult::Invalid(_)
        ));

        // Same source and destination
        assert!(matches!(
            validate_file_copy_move_params("same_token", "same_token", None),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_file_download_params() {
        // Valid case with all parameters
        assert!(matches!(
            validate_file_download_params("file_token_123", "doc", Some(3600)),
            ValidationResult::Valid
        ));

        // Valid case with default expire time
        assert!(matches!(
            validate_file_download_params("file_token", "pdf", None),
            ValidationResult::Valid
        ));

        // Invalid expire time - zero
        assert!(matches!(
            validate_file_download_params("file_token", "doc", Some(0)),
            ValidationResult::Invalid(_)
        ));

        // Invalid expire time - too long (7 days + 1 second)
        assert!(matches!(
            validate_file_download_params("file_token", "doc", Some(7 * 24 * 3600 + 1)),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_drive_file_type_edge_cases() {
        // Test all valid file types
        let valid_types = [
            "doc", "docx", "sheet", "bitable", "wiki", "file", "mindnote", "minutes", "slides",
            "pdf", "txt",
        ];

        for file_type in &valid_types {
            assert!(matches!(
                validate_drive_file_type(file_type),
                ValidationResult::Valid
            ));
        }

        // Test case sensitivity (should be case-sensitive)
        assert!(matches!(
            validate_drive_file_type("DOC"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_drive_file_type("Doc"),
            ValidationResult::Invalid(_)
        ));

        // Test whitespace-only input
        assert!(matches!(
            validate_drive_file_type("   "),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_folder_name_edge_cases() {
        // Test maximum length boundary
        let max_len_name = "a".repeat(255);
        assert!(matches!(
            validate_folder_name(&max_len_name),
            ValidationResult::Valid
        ));

        let too_long_name = "a".repeat(256);
        assert!(matches!(
            validate_folder_name(&too_long_name),
            ValidationResult::Invalid(_)
        ));

        // Test all invalid characters
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        for char in invalid_chars.iter() {
            let name = format!("test{}name", char);
            assert!(matches!(
                validate_folder_name(&name),
                ValidationResult::Invalid(_)
            ));
        }

        // Test reserved names (case-insensitive)
        let reserved_names = ["CON", "PRN", "AUX", "NUL"];
        for name in reserved_names.iter() {
            assert!(matches!(
                validate_folder_name(name),
                ValidationResult::Invalid(_)
            ));
            assert!(matches!(
                validate_folder_name(&name.to_lowercase()),
                ValidationResult::Invalid(_)
            ));
            assert!(matches!(
                validate_folder_name(&name.to_uppercase()),
                ValidationResult::Invalid(_)
            ));
        }

        // Test Unicode characters (should be valid)
        let unicode_name = "测试文件夹";
        assert!(matches!(
            validate_folder_name(unicode_name),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_file_upload_params_edge_cases() {
        // Test maximum file size for different types
        assert!(matches!(
            validate_file_upload_params("image.jpg", 20 * 1024 * 1024, "parent_token", "image"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_file_upload_params("image.jpg", 20 * 1024 * 1024 + 1, "parent_token", "image"),
            ValidationResult::Invalid(_)
        ));

        assert!(matches!(
            validate_file_upload_params("video.mp4", 100 * 1024 * 1024, "parent_token", "video"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_file_upload_params("audio.mp3", 50 * 1024 * 1024, "parent_token", "audio"),
            ValidationResult::Valid
        ));

        // Test default file size limit
        assert!(matches!(
            validate_file_upload_params("document.pdf", 100 * 1024 * 1024, "parent_token", "other"),
            ValidationResult::Valid
        ));

        // Test filename with extension
        assert!(matches!(
            validate_file_upload_params("test.document.txt", 1024, "parent_token", "file"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_file_upload_params("test.pdf", 1024, "parent_token", "pdf"),
            ValidationResult::Valid
        ));

        // Test very long filename
        let long_filename = "a".repeat(251) + ".txt"; // 251 + 4 = 255 total
        assert!(matches!(
            validate_file_upload_params(&long_filename, 1024, "parent_token_12345", "file"),
            ValidationResult::Valid
        ));
        let too_long_filename = "a".repeat(252) + ".txt"; // 252 + 4 = 256 total
        assert!(matches!(
            validate_file_upload_params(&too_long_filename, 1024, "parent_token_12345", "file"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_member_id_edge_cases() {
        // Test user ID with digits only
        assert!(matches!(
            validate_member_id("user", "1234567890"),
            ValidationResult::Valid
        ));

        // Test user ID too long
        let long_user_id = "ou_".to_string() + &"a".repeat(61); // Total 64 chars
        assert!(matches!(
            validate_member_id("user", &long_user_id),
            ValidationResult::Valid
        ));
        let too_long_user_id = "ou_".to_string() + &"a".repeat(62); // Total 65 chars
        assert!(matches!(
            validate_member_id("user", &too_long_user_id),
            ValidationResult::Invalid(_)
        ));

        // Test chat ID with correct length
        assert!(matches!(
            validate_member_id("chat", "oc_1234567890123456789012345"),
            ValidationResult::Valid
        ));

        // Test chat ID with wrong length
        assert!(matches!(
            validate_member_id("chat", "oc_123456789012345678901234"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_member_id("chat", "oc_12345678901234567890123456"),
            ValidationResult::Invalid(_)
        ));

        // Test department ID with various lengths
        assert!(matches!(
            validate_member_id("department", "od_123"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_member_id("department", &("od_".to_string() + &"a".repeat(60))),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_member_id("department", &("od_".to_string() + &"a".repeat(63))),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_batch_operation_edge_cases() {
        // Test single item batch
        let single_token = vec!["valid_token_1".to_string()];
        assert!(matches!(
            validate_batch_operation(&single_token, 10),
            ValidationResult::Valid
        ));

        // Test maximum batch size
        let max_tokens: Vec<String> = (0..10).map(|i| format!("token_{}_12345", i)).collect();
        assert!(matches!(
            validate_batch_operation(&max_tokens, 10),
            ValidationResult::Valid
        ));

        // Test over maximum batch size
        let too_many_tokens: Vec<String> = (0..11).map(|i| format!("token_{}", i)).collect();
        assert!(matches!(
            validate_batch_operation(&too_many_tokens, 10),
            ValidationResult::Invalid(_)
        ));

        // Test with invalid token in the middle
        let mixed_tokens = vec![
            "valid_token_12345".to_string(),
            "invalid@token".to_string(),
            "valid_token_67890".to_string(),
        ];
        assert!(matches!(
            validate_batch_operation(&mixed_tokens, 10),
            ValidationResult::Invalid(_)
        ));

        // Test with whitespace tokens
        let whitespace_tokens = vec![
            "valid_token_12345".to_string(),
            "   ".to_string(),
            "another_token_67890".to_string(),
        ];
        assert!(matches!(
            validate_batch_operation(&whitespace_tokens, 10),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_builder_trait_implementation() {
        // Test that the auto-implementation works
        struct TestValidator;

        impl ValidateBuilder for TestValidator {
            type Output = Result<String, Vec<String>>;

            fn required(self, _value: Option<String>, _field_name: &str) -> Self {
                self
            }

            fn length(
                self,
                _value: String,
                _min_len: usize,
                _max_len: usize,
                _field_name: &str,
            ) -> Self {
                self
            }

            fn custom<F>(self, _value: String, _validator: F, _error_msg: &str) -> Self
            where
                F: FnOnce(&str) -> bool,
            {
                self
            }

            fn validate(&self) -> ValidationResult {
                ValidationResult::Valid
            }

            fn build(self) -> Self::Output {
                Ok("test".to_string())
            }
        }

        let validator = TestValidator;

        // Test that we can call the trait methods
        assert!(matches!(
            validator.validate_drive_file_type("doc"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validator.validate_folder_name("test_folder"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validator.validate_file_upload_params("test.txt", 1024, "token_12345", "file"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validator.validate_permission_level("View"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validator.validate_member_id("user", "ou_123"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validator.validate_pagination_params(Some(50), Some("token_12345")),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validator.validate_search_params("test", Some(10), Some(0)),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validator.validate_batch_operation(&["token1_12345".to_string()], 5),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validation_error_messages() {
        // Test that error messages are descriptive
        let result = validate_drive_file_type("invalid_type");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("不支持的文件类型"));
            assert!(msg.contains("invalid_type"));
            assert!(msg.contains("支持的类型"));
        }

        let result = validate_folder_name("Invalid/Name");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("非法字符"));
            assert!(msg.contains("/"));
        }

        let result = validate_file_upload_params("test.txt", 0, "token", "file");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("文件大小必须大于0"));
        }

        let result = validate_member_id("user", "invalid_id");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("用户ID格式无效"));
        }
    }

    #[test]
    fn test_comprehensive_validation_scenarios() {
        // Test complex real-world scenarios

        // Scenario: Upload a large image file
        let file_name = "project_screenshot.png";
        let file_size = 15 * 1024 * 1024; // 15MB
        let parent_token = "folder_abc123456";
        let file_type = "image";

        assert!(matches!(
            validate_file_upload_params(file_name, file_size, parent_token, file_type),
            ValidationResult::Valid
        ));

        // Scenario: Search with pagination
        let search_key = "project documentation";
        let count = 50;
        let offset = 100;

        assert!(matches!(
            validate_search_params(search_key, Some(count), Some(offset)),
            ValidationResult::Valid
        ));

        // Scenario: Batch move files
        let tokens = vec![
            "file1_token".to_string(),
            "file2_token".to_string(),
            "file3_token".to_string(),
        ];
        let max_batch_size = 5;

        assert!(matches!(
            validate_batch_operation(&tokens, max_batch_size),
            ValidationResult::Valid
        ));

        // Scenario: Complex permission settings
        let external_access = "open";
        let security_entity = "specified_members";
        let comment_entity = "anyone_can_view";

        assert!(matches!(
            validate_permission_settings(
                Some(external_access),
                Some(security_entity),
                Some(comment_entity)
            ),
            ValidationResult::Valid
        ));
    }
}

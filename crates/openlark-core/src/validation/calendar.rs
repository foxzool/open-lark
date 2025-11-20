/// 日历服务验证模块
///
/// 提供日历相关操作的验证功能，包括：
/// - 日历创建和管理参数验证
/// - 日程创建和修改参数验证
/// - 参与人管理验证
/// - 时间和日期格式验证
/// - 提醒设置验证
/// - 重复规则验证
/// - 会议室资源验证
///
/// 使用示例：
/// ```rust
/// use open_lark::core::validation::{calendar::*, ValidationResult};
///
/// // 验证日程创建参数
/// let result = validate_calendar_event_creation_params(
///     "团队会议",
///     "2024-01-01T10:00:00+08:00",
///     "2024-01-01T11:00:00+08:00",
///     false,
///     5,
/// );
/// assert!(matches!(result, ValidationResult::Valid));
/// ```
use super::ValidationResult;
use chrono::{DateTime, NaiveDate};
use regex::Regex;

/// 日程标题最大长度
const CALENDAR_EVENT_TITLE_MAX_LENGTH: usize = 500;
/// 日程描述最大长度
const CALENDAR_EVENT_DESCRIPTION_MAX_LENGTH: usize = 2000;
/// 日历摘要最大长度
const CALENDAR_SUMMARY_MAX_LENGTH: usize = 100;
/// 日历描述最大长度
const CALENDAR_DESCRIPTION_MAX_LENGTH: usize = 2000;
/// 位置名称最大长度
const LOCATION_NAME_MAX_LENGTH: usize = 100;
/// 位置地址最大长度
const LOCATION_ADDRESS_MAX_LENGTH: usize = 300;
/// 参与人显示名称最大长度
const ATTENDEE_DISPLAY_NAME_MAX_LENGTH: usize = 100;
/// 会议室显示名称最大长度
const MEETING_ROOM_DISPLAY_NAME_MAX_LENGTH: usize = 100;
/// 日程重复规则最大长度
const RECURRENCE_RULE_MAX_LENGTH: usize = 500;
/// 日历ID格式正则
const CALENDAR_ID_REGEX: &str = r"^[a-zA-Z0-9_-]+$";
/// 用户ID格式正则
const USER_ID_REGEX: &str = r"^[a-zA-Z0-9_-]+$";

/// 验证日历ID格式
pub fn validate_calendar_id(calendar_id: &str) -> ValidationResult {
    if calendar_id.is_empty() {
        return ValidationResult::Invalid("日历ID不能为空".to_string());
    }

    let re = Regex::new(CALENDAR_ID_REGEX).unwrap();
    if !re.is_match(calendar_id) {
        return ValidationResult::Invalid(
            "日历ID格式无效，只能包含字母、数字、下划线和连字符".to_string(),
        );
    }

    ValidationResult::Valid
}

/// 验证日历摘要（标题）
pub fn validate_calendar_summary(summary: &str) -> ValidationResult {
    if summary.is_empty() {
        return ValidationResult::Invalid("日历标题不能为空".to_string());
    }

    if summary.len() > CALENDAR_SUMMARY_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "日历标题不能超过 {} 个字符",
            CALENDAR_SUMMARY_MAX_LENGTH
        ));
    }

    // 检查是否包含敏感词
    let sensitive_words = ["管理员", "系统", "官方"];
    for word in sensitive_words {
        if summary.contains(word) {
            return ValidationResult::Invalid(format!("日历标题不能包含敏感词: {}", word));
        }
    }

    ValidationResult::Valid
}

/// 验证日历描述
pub fn validate_calendar_description(description: &str) -> ValidationResult {
    if description.len() > CALENDAR_DESCRIPTION_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "日历描述不能超过 {} 个字符",
            CALENDAR_DESCRIPTION_MAX_LENGTH
        ));
    }

    ValidationResult::Valid
}

/// 验证日程ID格式
pub fn validate_event_id(event_id: &str) -> ValidationResult {
    if event_id.is_empty() {
        return ValidationResult::Invalid("日程ID不能为空".to_string());
    }

    let re = Regex::new(CALENDAR_ID_REGEX).unwrap();
    if !re.is_match(event_id) {
        return ValidationResult::Invalid(
            "日程ID格式无效，只能包含字母、数字、下划线和连字符".to_string(),
        );
    }

    ValidationResult::Valid
}

/// 验证日程标题
pub fn validate_event_title(title: &str) -> ValidationResult {
    if title.is_empty() {
        return ValidationResult::Invalid("日程标题不能为空".to_string());
    }

    if title.len() > CALENDAR_EVENT_TITLE_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "日程标题不能超过 {} 个字符",
            CALENDAR_EVENT_TITLE_MAX_LENGTH
        ));
    }

    // 检查标题格式
    if title.trim() != title {
        return ValidationResult::Invalid("日程标题前后不能有空格".to_string());
    }

    ValidationResult::Valid
}

/// 验证日程描述
pub fn validate_event_description(description: &str) -> ValidationResult {
    if description.len() > CALENDAR_EVENT_DESCRIPTION_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "日程描述不能超过 {} 个字符",
            CALENDAR_EVENT_DESCRIPTION_MAX_LENGTH
        ));
    }

    ValidationResult::Valid
}

/// 验证时间格式和有效性
pub fn validate_time_format(time_str: &str) -> ValidationResult {
    if time_str.is_empty() {
        return ValidationResult::Invalid("时间字符串不能为空".to_string());
    }

    // 尝试解析RFC3339格式
    if DateTime::parse_from_rfc3339(time_str).is_ok() {
        return ValidationResult::Valid;
    }

    // 尝试解析日期格式（全天日程）
    if NaiveDate::parse_from_str(time_str, "%Y-%m-%d").is_ok() {
        return ValidationResult::Valid;
    }

    ValidationResult::Invalid(
        "时间格式无效，请使用RFC3339格式（如2024-01-01T10:00:00+08:00）或日期格式（如2024-01-01）"
            .to_string(),
    )
}

/// 验证时间范围
pub fn validate_time_range(start_time: &str, end_time: &str, is_all_day: bool) -> ValidationResult {
    let start_result = validate_time_format(start_time);
    if start_result != ValidationResult::Valid {
        return start_result;
    }

    let end_result = validate_time_format(end_time);
    if end_result != ValidationResult::Valid {
        return end_result;
    }

    if is_all_day {
        // 全天日程验证
        let start_date = match NaiveDate::parse_from_str(start_time, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return ValidationResult::Invalid("开始时间格式无效".to_string()),
        };

        let end_date = match NaiveDate::parse_from_str(end_time, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return ValidationResult::Invalid("结束时间格式无效".to_string()),
        };

        if end_date <= start_date {
            return ValidationResult::Invalid("全天日程的结束日期必须晚于开始日期".to_string());
        }

        // 检查日期范围是否过大
        let days_diff = (end_date - start_date).num_days();
        if days_diff > 365 {
            return ValidationResult::Invalid("全天日程的持续时间不能超过365天".to_string());
        }
    } else {
        // 定时日程验证
        let start_dt = match DateTime::parse_from_rfc3339(start_time) {
            Ok(dt) => dt,
            Err(_) => return ValidationResult::Invalid("开始时间格式无效".to_string()),
        };

        let end_dt = match DateTime::parse_from_rfc3339(end_time) {
            Ok(dt) => dt,
            Err(_) => return ValidationResult::Invalid("结束时间格式无效".to_string()),
        };

        if end_dt <= start_dt {
            return ValidationResult::Invalid("结束时间必须晚于开始时间".to_string());
        }

        // 检查时长是否合理
        let duration = end_dt - start_dt;
        if duration.num_minutes() < 5 {
            return ValidationResult::Invalid("日程时长不能少于5分钟".to_string());
        }

        if duration.num_days() > 30 {
            return ValidationResult::Invalid("单次日程时长不能超过30天".to_string());
        }
    }

    ValidationResult::Valid
}

/// 验证提醒设置
pub fn validate_reminder_minutes(minutes: i32) -> ValidationResult {
    if minutes < 0 {
        return ValidationResult::Invalid("提醒时间不能为负数".to_string());
    }

    if minutes > 10080 {
        // 7天 = 10080分钟
        return ValidationResult::Invalid("提醒时间不能超过7天（10080分钟）".to_string());
    }

    // 常用提醒时间检查
    let valid_reminders = [0, 5, 10, 15, 30, 60, 120, 1440, 2880, 10080];
    if !valid_reminders.contains(&minutes) && minutes != 0 {
        // 0表示不提醒
        return ValidationResult::Sanitized(
            "建议使用常用提醒时间：0（不提醒）、5、10、15、30、60、120、1440（1天）、2880（2天）、10080（7天）分钟".to_string()
        );
    }

    ValidationResult::Valid
}

/// 验证重复规则
pub fn validate_recurrence_rule(rule: &str) -> ValidationResult {
    if rule.is_empty() {
        return ValidationResult::Valid; // 空表示不重复
    }

    if rule.len() > RECURRENCE_RULE_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "重复规则不能超过 {} 个字符",
            RECURRENCE_RULE_MAX_LENGTH
        ));
    }

    // 基本的RRULE格式验证
    if !rule.starts_with("RRULE:") {
        return ValidationResult::Invalid("重复规则必须以RRULE:开头".to_string());
    }

    // 检查必需的FREQ参数
    if !rule.contains("FREQ=") {
        return ValidationResult::Invalid("重复规则必须包含FREQ参数".to_string());
    }

    // 验证FREQ值
    let valid_freqs = ["DAILY", "WEEKLY", "MONTHLY", "YEARLY"];
    let freq_match = rule.find("FREQ=").and_then(|pos| {
        let rest = &rule[pos + 5..];
        rest.find(';')
            .or_else(|| rest.find(','))
            .map(|end| &rest[..end])
            .or(Some(rest))
    });

    if let Some(freq) = freq_match {
        if !valid_freqs.contains(&freq) {
            return ValidationResult::Invalid(format!(
                "无效的重复频率: {}，必须是DAILY、WEEKLY、MONTHLY或YEARLY之一",
                freq
            ));
        }
    }

    // 检查UNTIL参数（如果有）
    if rule.contains("UNTIL=") {
        let until_part = rule.split("UNTIL=").nth(1).unwrap_or("");
        let until_value = until_part
            .split(';')
            .next()
            .unwrap_or(until_part)
            .split(',')
            .next()
            .unwrap_or(until_part);

        if NaiveDate::parse_from_str(until_value, "%Y%m%d").is_err()
            && DateTime::parse_from_rfc3339(&format!("{}T000000Z", until_value)).is_err()
        {
            return ValidationResult::Invalid("UNTIL参数格式无效，请使用YYYYMMDD格式".to_string());
        }
    }

    // 检查COUNT参数（如果有）
    if let Some(count_str) = rule.split("COUNT=").nth(1) {
        let count_value = count_str
            .split(';')
            .next()
            .unwrap_or(count_str)
            .split(',')
            .next()
            .unwrap_or(count_str);

        if let Ok(count) = count_value.parse::<i32>() {
            if !(1..=999).contains(&count) {
                return ValidationResult::Invalid("重复次数必须在1-999之间".to_string());
            }
        }
    }

    ValidationResult::Valid
}

/// 验证参与人类型和ID
pub fn validate_attendee_type_and_id(attendee_type: &str, attendee_id: &str) -> ValidationResult {
    if attendee_id.is_empty() {
        return ValidationResult::Invalid("参与人ID不能为空".to_string());
    }

    match attendee_type {
        "user" => {
            if attendee_id.len() > 64 {
                return ValidationResult::Invalid("用户ID不能超过64个字符".to_string());
            }
            let re = Regex::new(USER_ID_REGEX).unwrap();
            if !re.is_match(attendee_id) {
                return ValidationResult::Invalid("用户ID格式无效".to_string());
            }
        }
        "chat" => {
            if !attendee_id.starts_with("oc_") {
                return ValidationResult::Invalid("群组ID必须以oc_开头".to_string());
            }
            if attendee_id.len() > 64 {
                return ValidationResult::Invalid("群组ID不能超过64个字符".to_string());
            }
        }
        "resource" => {
            if attendee_id.len() > 64 {
                return ValidationResult::Invalid("资源ID不能超过64个字符".to_string());
            }
        }
        "third_party" => {
            if !attendee_id.contains('@') {
                return ValidationResult::Invalid("第三方邮箱必须包含@符号".to_string());
            }
            if attendee_id.len() > 100 {
                return ValidationResult::Invalid("第三方邮箱不能超过100个字符".to_string());
            }
        }
        _ => {
            return ValidationResult::Invalid(format!("无效的参与人类型: {}", attendee_type));
        }
    }

    ValidationResult::Valid
}

/// 验证参与人显示名称
pub fn validate_attendee_display_name(display_name: &str) -> ValidationResult {
    if display_name.len() > ATTENDEE_DISPLAY_NAME_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "参与人显示名称不能超过 {} 个字符",
            ATTENDEE_DISPLAY_NAME_MAX_LENGTH
        ));
    }

    // 检查特殊字符
    if display_name.contains('<') || display_name.contains('>') {
        return ValidationResult::Invalid("参与人显示名称不能包含<或>符号".to_string());
    }

    ValidationResult::Valid
}

/// 验证位置信息
pub fn validate_location(name: &str, address: Option<&str>) -> ValidationResult {
    if name.is_empty() {
        return ValidationResult::Valid; // 位置是可选的
    }

    if name.len() > LOCATION_NAME_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "位置名称不能超过 {} 个字符",
            LOCATION_NAME_MAX_LENGTH
        ));
    }

    if let Some(addr) = address {
        if addr.len() > LOCATION_ADDRESS_MAX_LENGTH {
            return ValidationResult::Invalid(format!(
                "位置地址不能超过 {} 个字符",
                LOCATION_ADDRESS_MAX_LENGTH
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证会议室ID
pub fn validate_meeting_room_id(room_id: &str) -> ValidationResult {
    if room_id.is_empty() {
        return ValidationResult::Invalid("会议室ID不能为空".to_string());
    }

    let re = Regex::new(CALENDAR_ID_REGEX).unwrap();
    if !re.is_match(room_id) {
        return ValidationResult::Invalid("会议室ID格式无效".to_string());
    }

    ValidationResult::Valid
}

/// 验证会议室显示名称
pub fn validate_meeting_room_display_name(display_name: &str) -> ValidationResult {
    if display_name.len() > MEETING_ROOM_DISPLAY_NAME_MAX_LENGTH {
        return ValidationResult::Invalid(format!(
            "会议室显示名称不能超过 {} 个字符",
            MEETING_ROOM_DISPLAY_NAME_MAX_LENGTH
        ));
    }

    ValidationResult::Valid
}

/// 验证日程状态
pub fn validate_event_status(status: &str) -> ValidationResult {
    let valid_statuses = ["confirmed", "tentative", "cancelled"];
    if !valid_statuses.contains(&status.to_lowercase().as_str()) {
        return ValidationResult::Invalid(format!(
            "无效的日程状态: {}，必须是confirmed、tentative或cancelled",
            status
        ));
    }

    ValidationResult::Valid
}

/// 验证RSVP状态
pub fn validate_rsvp_status(status: &str) -> ValidationResult {
    let valid_statuses = ["needs_action", "accepted", "declined", "tentative"];
    if !valid_statuses.contains(&status.to_lowercase().as_str()) {
        return ValidationResult::Invalid(format!(
            "无效的RSVP状态: {}，必须是needs_action、accepted、declined或tentative",
            status
        ));
    }

    ValidationResult::Valid
}

/// 验证日历权限级别
pub fn validate_calendar_permission_level(permission: &str) -> ValidationResult {
    let valid_permissions = ["none", "free_busy_reader", "reader", "writer", "owner"];
    if !valid_permissions.contains(&permission.to_lowercase().as_str()) {
        return ValidationResult::Invalid(format!(
            "无效的日历权限: {}，必须是none、free_busy_reader、reader、writer或owner",
            permission
        ));
    }

    ValidationResult::Valid
}

/// 验证用户ID类型
pub fn validate_user_id_type(user_id_type: &str) -> ValidationResult {
    let valid_types = ["user_id", "open_id", "union_id"];
    if !valid_types.contains(&user_id_type.to_lowercase().as_str()) {
        return ValidationResult::Invalid(format!(
            "无效的用户ID类型: {}，必须是user_id、open_id或union_id",
            user_id_type
        ));
    }

    ValidationResult::Valid
}

/// 验证分页参数
pub fn validate_calendar_pagination_params(
    page_size: Option<i32>,
    page_token: Option<&str>,
) -> ValidationResult {
    if let Some(size) = page_size {
        if !(1..=500).contains(&size) {
            return ValidationResult::Invalid("每页数量必须在1-500之间".to_string());
        }
    }

    if let Some(token) = page_token {
        if token.len() > 100 {
            return ValidationResult::Invalid("分页令牌不能超过100个字符".to_string());
        }
    }

    ValidationResult::Valid
}

/// 验证日程创建完整参数
pub fn validate_calendar_event_creation_params(
    title: &str,
    start_time: &str,
    end_time: &str,
    is_all_day: bool,
    attendees_count: usize,
) -> ValidationResult {
    let title_result = validate_event_title(title);
    if title_result != ValidationResult::Valid {
        return title_result;
    }

    let time_result = validate_time_range(start_time, end_time, is_all_day);
    if time_result != ValidationResult::Valid {
        return time_result;
    }

    // 检查参与人数量
    if attendees_count > 1000 {
        return ValidationResult::Invalid("参与人数量不能超过1000人".to_string());
    }

    // 特殊验证：全天日程的参与人限制
    if is_all_day && attendees_count > 100 {
        return ValidationResult::Invalid("全天日程的参与人数量不能超过100人".to_string());
    }

    ValidationResult::Valid
}

/// 验证日程查询时间范围
pub fn validate_calendar_query_time_range(
    start_time: Option<&str>,
    end_time: Option<&str>,
) -> ValidationResult {
    if let Some(start) = start_time {
        let result = validate_time_format(start);
        if result != ValidationResult::Valid {
            return ValidationResult::Invalid("开始时间格式无效".to_string());
        }
    }

    if let Some(end) = end_time {
        let result = validate_time_format(end);
        if result != ValidationResult::Valid {
            return ValidationResult::Invalid("结束时间格式无效".to_string());
        }
    }

    if let (Some(start), Some(end)) = (start_time, end_time) {
        let start_dt = match DateTime::parse_from_rfc3339(start) {
            Ok(dt) => dt,
            Err(_) => return ValidationResult::Invalid("开始时间格式无效".to_string()),
        };

        let end_dt = match DateTime::parse_from_rfc3339(end) {
            Ok(dt) => dt,
            Err(_) => return ValidationResult::Invalid("结束时间格式无效".to_string()),
        };

        // 查询时间范围不能超过1年
        let days_diff = (end_dt - start_dt).num_days();
        if days_diff.abs() > 365 {
            return ValidationResult::Invalid("查询时间范围不能超过1年".to_string());
        }
    }

    ValidationResult::Valid
}

/// 验证搜索参数
pub fn validate_calendar_search_params(query: &str, max_results: Option<i32>) -> ValidationResult {
    if query.is_empty() {
        return ValidationResult::Invalid("搜索关键词不能为空".to_string());
    }

    if query.len() > 100 {
        return ValidationResult::Invalid("搜索关键词不能超过100个字符".to_string());
    }

    if let Some(max) = max_results {
        if !(1..=100).contains(&max) {
            return ValidationResult::Invalid("最大结果数必须在1-100之间".to_string());
        }
    }

    ValidationResult::Valid
}

/// 日历验证Builder特征
pub trait ValidateCalendarBuilder {
    /// 验证日历创建参数
    fn validate_calendar_creation(&self) -> ValidationResult;

    /// 验证日程创建参数
    fn validate_event_creation(&self) -> ValidationResult;

    /// 验证参与人添加参数
    fn validate_attendee_addition(&self) -> ValidationResult;

    /// 验证会议室预订参数
    fn validate_meeting_room_booking(&self) -> ValidationResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== validate_calendar_id 测试 ==========

    #[test]
    fn test_validate_calendar_id_valid_cases() {
        // 简单有效的日历ID
        assert!(matches!(
            validate_calendar_id("calendar123"),
            ValidationResult::Valid
        ));

        // 包含下划线
        assert!(matches!(
            validate_calendar_id("team_calendar_2024"),
            ValidationResult::Valid
        ));

        // 包含连字符
        assert!(matches!(
            validate_calendar_id("meeting-room-1"),
            ValidationResult::Valid
        ));

        // 混合字符
        assert!(matches!(
            validate_calendar_id("Calendar_ID-123"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最小长度
        assert!(matches!(validate_calendar_id("a"), ValidationResult::Valid));
    }

    #[test]
    fn test_validate_calendar_id_invalid_cases() {
        // 空字符串
        assert!(matches!(
            validate_calendar_id(""),
            ValidationResult::Invalid(msg) if msg.contains("不能为空")
        ));

        // 包含无效字符
        assert!(matches!(
            validate_calendar_id("calendar@123"),
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));

        assert!(matches!(
            validate_calendar_id("calendar#123"),
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));

        assert!(matches!(
            validate_calendar_id("calendar 123"), // 包含空格
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));

        // 包含中文
        assert!(matches!(
            validate_calendar_id("日历123"),
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));
    }

    // ========== validate_calendar_summary 测试 ==========

    #[test]
    fn test_validate_calendar_summary_valid_cases() {
        // 正常长度的摘要
        assert!(matches!(
            validate_calendar_summary("团队日历"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大长度
        let max_summary = "A".repeat(100);
        assert!(matches!(
            validate_calendar_summary(&max_summary),
            ValidationResult::Valid
        ));

        // 包含数字和特殊字符（但不是敏感词）
        assert!(matches!(
            validate_calendar_summary("2024年工作计划-项目组"),
            ValidationResult::Valid
        ));

        // 英文和数字混合
        assert!(matches!(
            validate_calendar_summary("Team Schedule 2024"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_calendar_summary_invalid_cases() {
        // 空字符串
        assert!(matches!(
            validate_calendar_summary(""),
            ValidationResult::Invalid(msg) if msg.contains("不能为空")
        ));

        // 超过最大长度
        let too_long_summary = "A".repeat(101);
        assert!(matches!(
            validate_calendar_summary(&too_long_summary),
            ValidationResult::Invalid(msg) if msg.contains("不能超过")
        ));

        // 包含敏感词
        assert!(matches!(
            validate_calendar_summary("管理员日历"),
            ValidationResult::Invalid(msg) if msg.contains("敏感词")
        ));

        assert!(matches!(
            validate_calendar_summary("系统通知"),
            ValidationResult::Invalid(msg) if msg.contains("敏感词")
        ));

        assert!(matches!(
            validate_calendar_summary("官方发布"),
            ValidationResult::Invalid(msg) if msg.contains("敏感词")
        ));
    }

    // ========== validate_calendar_description 测试 ==========

    #[test]
    fn test_validate_calendar_description_valid_cases() {
        // 空描述（应该允许）
        assert!(matches!(
            validate_calendar_description(""),
            ValidationResult::Valid
        ));

        // 正常长度描述
        assert!(matches!(
            validate_calendar_description("这是一个团队共享日历"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大长度
        let max_description = "A".repeat(2000);
        assert!(matches!(
            validate_calendar_description(&max_description),
            ValidationResult::Valid
        ));

        // Unicode内容
        assert!(matches!(
            validate_calendar_description("🎯 这是一个包含emoji的日历描述"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_calendar_description_invalid_cases() {
        // 超过最大长度
        let too_long_description = "A".repeat(2001);
        assert!(matches!(
            validate_calendar_description(&too_long_description),
            ValidationResult::Invalid(msg) if msg.contains("不能超过")
        ));
    }

    // ========== validate_event_id 测试 ==========

    #[test]
    fn test_validate_event_id_valid_cases() {
        // 简单有效的日程ID
        assert!(matches!(
            validate_event_id("event_123"),
            ValidationResult::Valid
        ));

        // 复杂格式
        assert!(matches!(
            validate_event_id("meeting-room-booking-2024"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_event_id_invalid_cases() {
        // 空字符串
        assert!(matches!(
            validate_event_id(""),
            ValidationResult::Invalid(msg) if msg.contains("不能为空")
        ));

        // 无效字符
        assert!(matches!(
            validate_event_id("event@123"),
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));
    }

    // ========== validate_event_title 测试 ==========

    #[test]
    fn test_validate_event_title_valid_cases() {
        // 正常标题
        assert!(matches!(
            validate_event_title("团队会议"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大长度
        let max_title = "A".repeat(500);
        assert!(matches!(
            validate_event_title(&max_title),
            ValidationResult::Valid
        ));

        // 包含数字和特殊字符
        assert!(matches!(
            validate_event_title("2024-Q1 季度总结会议"),
            ValidationResult::Valid
        ));

        // 英文标题
        assert!(matches!(
            validate_event_title("Team Weekly Meeting"),
            ValidationResult::Valid
        ));

        // 最小长度
        assert!(matches!(
            validate_event_title("会"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_event_title_invalid_cases() {
        // 空字符串
        assert!(matches!(
            validate_event_title(""),
            ValidationResult::Invalid(msg) if msg.contains("不能为空")
        ));

        // 前后有空格
        assert!(matches!(
            validate_event_title(" 带空格 "),
            ValidationResult::Invalid(msg) if msg.contains("前后不能有空格")
        ));

        // 前置空格
        assert!(matches!(
            validate_event_title(" 前置空格"),
            ValidationResult::Invalid(msg) if msg.contains("前后不能有空格")
        ));

        // 后置空格
        assert!(matches!(
            validate_event_title("后置空格 "),
            ValidationResult::Invalid(msg) if msg.contains("前后不能有空格")
        ));

        // 超过最大长度
        let too_long_title = "A".repeat(501);
        assert!(matches!(
            validate_event_title(&too_long_title),
            ValidationResult::Invalid(msg) if msg.contains("不能超过")
        ));
    }

    // ========== validate_event_description 测试 ==========

    #[test]
    fn test_validate_event_description_valid_cases() {
        // 空描述
        assert!(matches!(
            validate_event_description(""),
            ValidationResult::Valid
        ));

        // 正常长度描述
        assert!(matches!(
            validate_event_description("这是一个重要的团队会议"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大长度
        let max_description = "A".repeat(2000);
        assert!(matches!(
            validate_event_description(&max_description),
            ValidationResult::Valid
        ));

        // 多行内容
        let multi_line_description = "第一行\n第二行\n第三行";
        assert!(matches!(
            validate_event_description(multi_line_description),
            ValidationResult::Valid
        ));

        // 包含特殊字符
        assert!(matches!(
            validate_event_description("讨论要点：\n1. 项目进度\n2. 下一步计划\n3. 风险评估"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_event_description_invalid_cases() {
        // 超过最大长度
        let too_long_description = "A".repeat(2001);
        assert!(matches!(
            validate_event_description(&too_long_description),
            ValidationResult::Invalid(msg) if msg.contains("不能超过")
        ));
    }

    // ========== validate_time_format 测试 ==========

    #[test]
    fn test_validate_time_format_valid_cases() {
        // RFC3339格式 - 基本格式
        assert!(matches!(
            validate_time_format("2024-01-01T10:00:00+08:00"),
            ValidationResult::Valid
        ));

        // RFC3339格式 - UTC时间
        assert!(matches!(
            validate_time_format("2024-01-01T10:00:00Z"),
            ValidationResult::Valid
        ));

        // RFC3339格式 - 带毫秒
        assert!(matches!(
            validate_time_format("2024-01-01T10:00:00.123+08:00"),
            ValidationResult::Valid
        ));

        // 日期格式 - 全天日程
        assert!(matches!(
            validate_time_format("2024-01-01"),
            ValidationResult::Valid
        ));

        // 边界情况 - 年初年末
        assert!(matches!(
            validate_time_format("2024-01-01"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_time_format("2024-12-31"),
            ValidationResult::Valid
        ));

        // 不同时区
        assert!(matches!(
            validate_time_format("2024-01-01T10:00:00-05:00"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_time_format_invalid_cases() {
        // 空字符串
        assert!(matches!(
            validate_time_format(""),
            ValidationResult::Invalid(msg) if msg.contains("不能为空")
        ));

        // 无效日期格式
        assert!(matches!(
            validate_time_format("2024-13-01"), // 无效月份
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));

        assert!(matches!(
            validate_time_format("2024-02-30"), // 无效日期
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));

        // 错误的时间格式
        assert!(matches!(
            validate_time_format("2024/01/01 10:00:00"), // 错误的分隔符
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));

        assert!(matches!(
            validate_time_format("2024-01-01 10:00"), // 缺少秒和时区
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));

        // 完全错误的格式
        assert!(matches!(
            validate_time_format("invalid-time"),
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));

        assert!(matches!(
            validate_time_format("2024年1月1日"),
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));
    }

    // ========== validate_time_range 测试 ==========

    #[test]
    fn test_validate_time_range_valid_cases() {
        // 定时日程 - 正常范围
        assert!(matches!(
            validate_time_range(
                "2024-01-01T10:00:00+08:00",
                "2024-01-01T11:00:00+08:00",
                false
            ),
            ValidationResult::Valid
        ));

        // 定时日程 - 最小有效时长（5分钟）
        assert!(matches!(
            validate_time_range(
                "2024-01-01T10:00:00+08:00",
                "2024-01-01T10:05:00+08:00",
                false
            ),
            ValidationResult::Valid
        ));

        // 全天日程 - 正常范围
        assert!(matches!(
            validate_time_range("2024-01-01", "2024-01-02", true),
            ValidationResult::Valid
        ));

        // 全天日程 - 多天事件
        assert!(matches!(
            validate_time_range("2024-01-01", "2024-01-07", true),
            ValidationResult::Valid
        ));

        // 全天日程 - 边界情况（364天，刚好在范围内）
        assert!(matches!(
            validate_time_range("2024-01-01", "2024-12-31", true),
            ValidationResult::Valid
        ));

        // 定时日程 - 长时间事件（30天边界）
        assert!(matches!(
            validate_time_range(
                "2024-01-01T00:00:00+08:00",
                "2024-01-31T00:00:00+08:00",
                false
            ),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_time_range_invalid_cases() {
        // 结束时间早于开始时间
        assert!(matches!(
            validate_time_range(
                "2024-01-01T11:00:00+08:00",
                "2024-01-01T10:00:00+08:00",
                false
            ),
            ValidationResult::Invalid(msg) if msg.contains("晚于开始时间")
        ));

        // 结束时间等于开始时间
        assert!(matches!(
            validate_time_range(
                "2024-01-01T10:00:00+08:00",
                "2024-01-01T10:00:00+08:00",
                false
            ),
            ValidationResult::Invalid(msg) if msg.contains("晚于开始时间")
        ));

        // 定时日程 - 时长太短（少于5分钟）
        assert!(matches!(
            validate_time_range(
                "2024-01-01T10:00:00+08:00",
                "2024-01-01T10:04:00+08:00",
                false
            ),
            ValidationResult::Invalid(msg) if msg.contains("不能少于5分钟")
        ));

        // 定时日程 - 时长太长（超过30天）
        assert!(matches!(
            validate_time_range(
                "2024-01-01T00:00:00+08:00",
                "2024-02-01T00:00:00+08:00",
                false
            ),
            ValidationResult::Invalid(msg) if msg.contains("不能超过30天")
        ));

        // 全天日程 - 结束日期早于开始日期
        assert!(matches!(
            validate_time_range("2024-01-02", "2024-01-01", true),
            ValidationResult::Invalid(msg) if msg.contains("晚于开始日期")
        ));

        // 全天日程 - 持续时间过长（超过365天）
        assert!(matches!(
            validate_time_range("2024-01-01", "2025-01-02", true),
            ValidationResult::Invalid(msg) if msg.contains("不能超过365天")
        ));

        // 无效的时间格式
        assert!(matches!(
            validate_time_range("invalid-time", "2024-01-01T11:00:00+08:00", false),
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));
    }

    // ========== validate_reminder_minutes 测试 ==========

    #[test]
    fn test_validate_reminder_minutes_valid_cases() {
        // 标准提醒时间
        let valid_times = [0, 5, 10, 15, 30, 60, 120, 1440, 2880, 10080];
        for minutes in valid_times {
            assert!(
                matches!(validate_reminder_minutes(minutes), ValidationResult::Valid),
                "Should be valid: {} minutes",
                minutes
            );
        }

        // 边界情况 - 最大值（7天）
        assert!(matches!(
            validate_reminder_minutes(10080),
            ValidationResult::Valid
        ));

        // 中间值（不在推荐列表中，但应该被清理为建议值）
        assert!(matches!(
            validate_reminder_minutes(20),
            ValidationResult::Sanitized(msg) if msg.contains("建议使用常用提醒时间")
        ));
    }

    #[test]
    fn test_validate_reminder_minutes_invalid_cases() {
        // 负数
        assert!(matches!(
            validate_reminder_minutes(-1),
            ValidationResult::Invalid(msg) if msg.contains("不能为负数")
        ));

        assert!(matches!(
            validate_reminder_minutes(-100),
            ValidationResult::Invalid(msg) if msg.contains("不能为负数")
        ));

        // 超过最大值
        assert!(matches!(
            validate_reminder_minutes(10081),
            ValidationResult::Invalid(msg) if msg.contains("不能超过7天")
        ));

        assert!(matches!(
            validate_reminder_minutes(20000),
            ValidationResult::Invalid(msg) if msg.contains("不能超过7天")
        ));
    }

    #[test]
    fn test_validate_reminder_minutes_warning_cases() {
        // 非标准但有效的提醒时间
        let non_standard_times = [1, 2, 3, 4, 6, 7, 8, 9, 20, 45, 90, 200, 1000];
        for minutes in non_standard_times {
            assert!(
                matches!(
                    validate_reminder_minutes(minutes),
                    ValidationResult::Sanitized(msg) if msg.contains("建议使用常用提醒时间")
                ),
                "Should generate warning for: {} minutes",
                minutes
            );
        }
    }

    // ========== validate_recurrence_rule 测试 ==========

    #[test]
    fn test_validate_recurrence_rule_valid_cases() {
        // 空规则（不重复）
        assert!(matches!(
            validate_recurrence_rule(""),
            ValidationResult::Valid
        ));

        // 基本重复规则
        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=DAILY"),
            ValidationResult::Valid
        ));

        // 所有支持的频率
        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=WEEKLY"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=MONTHLY"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=YEARLY"),
            ValidationResult::Valid
        ));

        // 带COUNT的规则
        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=DAILY;COUNT=10"),
            ValidationResult::Valid
        ));

        // 带UNTIL的规则（日期格式）
        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=WEEKLY;UNTIL=20241231"),
            ValidationResult::Valid
        ));

        // 复杂规则
        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=DAILY;COUNT=5;INTERVAL=1"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大长度
        let max_rule = "RRULE:FREQ=DAILY;"
            .chars()
            .chain("A".repeat(470).chars())
            .collect::<String>();
        assert!(matches!(
            validate_recurrence_rule(&max_rule),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_recurrence_rule_invalid_cases() {
        // 缺少RRULE:前缀
        assert!(matches!(
            validate_recurrence_rule("FREQ=DAILY"),
            ValidationResult::Invalid(msg) if msg.contains("必须以RRULE:开头")
        ));

        // 缺少FREQ参数
        assert!(matches!(
            validate_recurrence_rule("RRULE:COUNT=5"),
            ValidationResult::Invalid(msg) if msg.contains("必须包含FREQ参数")
        ));

        // 无效的频率
        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=HOURLY"),
            ValidationResult::Invalid(msg) if msg.contains("无效的重复频率")
        ));

        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=INVALID"),
            ValidationResult::Invalid(msg) if msg.contains("无效的重复频率")
        ));

        // COUNT超出范围
        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=DAILY;COUNT=0"),
            ValidationResult::Invalid(msg) if msg.contains("重复次数必须在1-999之间")
        ));

        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=DAILY;COUNT=1000"),
            ValidationResult::Invalid(msg) if msg.contains("重复次数必须在1-999之间")
        ));

        // 无效的UNTIL格式
        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=DAILY;UNTIL=invalid"),
            ValidationResult::Invalid(msg) if msg.contains("UNTIL参数格式无效")
        ));

        // 超过最大长度
        let too_long_rule = "RRULE:FREQ=DAILY;"
            .chars()
            .chain("A".repeat(500).chars())
            .collect::<String>();
        assert!(matches!(
            validate_recurrence_rule(&too_long_rule),
            ValidationResult::Invalid(msg) if msg.contains("不能超过")
        ));
    }

    // ========== validate_attendee_type_and_id 测试 ==========

    #[test]
    fn test_validate_attendee_type_and_id_valid_cases() {
        // 用户类型
        assert!(matches!(
            validate_attendee_type_and_id("user", "user123"),
            ValidationResult::Valid
        ));

        // 用户类型 - 最大长度边界
        let max_user_id = "A".repeat(64);
        assert!(matches!(
            validate_attendee_type_and_id("user", &max_user_id),
            ValidationResult::Valid
        ));

        // 群组类型
        assert!(matches!(
            validate_attendee_type_and_id("chat", "oc_chat123"),
            ValidationResult::Valid
        ));

        // 群组类型 - 最大长度边界
        let max_chat_id = format!("oc_{}", "A".repeat(60));
        assert!(matches!(
            validate_attendee_type_and_id("chat", &max_chat_id),
            ValidationResult::Valid
        ));

        // 资源类型
        assert!(matches!(
            validate_attendee_type_and_id("resource", "room123"),
            ValidationResult::Valid
        ));

        // 第三方邮箱
        assert!(matches!(
            validate_attendee_type_and_id("third_party", "user@example.com"),
            ValidationResult::Valid
        ));

        // 复杂邮箱格式
        assert!(matches!(
            validate_attendee_type_and_id("third_party", "user.name+tag@company.co.uk"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_attendee_type_and_id_invalid_cases() {
        // 无效的参与人类型
        assert!(matches!(
            validate_attendee_type_and_id("invalid", "id123"),
            ValidationResult::Invalid(msg) if msg.contains("无效的参与人类型")
        ));

        // 空ID
        assert!(matches!(
            validate_attendee_type_and_id("user", ""),
            ValidationResult::Invalid(msg) if msg.contains("不能为空")
        ));

        // 用户ID过长
        let too_long_user_id = "A".repeat(65);
        assert!(matches!(
            validate_attendee_type_and_id("user", &too_long_user_id),
            ValidationResult::Invalid(msg) if msg.contains("不能超过64个字符")
        ));

        // 用户ID包含无效字符
        assert!(matches!(
            validate_attendee_type_and_id("user", "user@123"),
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));

        // 群组ID不以oc_开头
        assert!(matches!(
            validate_attendee_type_and_id("chat", "chat123"),
            ValidationResult::Invalid(msg) if msg.contains("必须以oc_开头")
        ));

        // 群组ID过长
        let too_long_chat_id = format!("oc_{}", "A".repeat(65)); // oc_ + 65 = 68 characters
        assert!(matches!(
            validate_attendee_type_and_id("chat", &too_long_chat_id),
            ValidationResult::Invalid(msg) if msg.contains("不能超过64个字符")
        ));

        // 资源ID过长
        let too_long_resource_id = "A".repeat(65);
        assert!(matches!(
            validate_attendee_type_and_id("resource", &too_long_resource_id),
            ValidationResult::Invalid(msg) if msg.contains("不能超过64个字符")
        ));

        // 第三方邮箱不包含@
        assert!(matches!(
            validate_attendee_type_and_id("third_party", "userexample.com"),
            ValidationResult::Invalid(msg) if msg.contains("必须包含@符号")
        ));

        // 第三方邮箱过长
        let too_long_email = format!("{}@example.com", "A".repeat(90));
        assert!(matches!(
            validate_attendee_type_and_id("third_party", &too_long_email),
            ValidationResult::Invalid(msg) if msg.contains("不能超过100个字符")
        ));
    }

    // ========== validate_attendee_display_name 测试 ==========

    #[test]
    fn test_validate_attendee_display_name_valid_cases() {
        // 正常显示名称
        assert!(matches!(
            validate_attendee_display_name("张三"),
            ValidationResult::Valid
        ));

        // 英文名称
        assert!(matches!(
            validate_attendee_display_name("John Doe"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大长度
        let max_name = "A".repeat(100);
        assert!(matches!(
            validate_attendee_display_name(&max_name),
            ValidationResult::Valid
        ));

        // 包含合法特殊字符
        assert!(matches!(
            validate_attendee_display_name("张三-经理"),
            ValidationResult::Valid
        ));

        // 包含数字
        assert!(matches!(
            validate_attendee_display_name("开发工程师001"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_attendee_display_name_invalid_cases() {
        // 超过最大长度
        let too_long_name = "A".repeat(101);
        assert!(matches!(
            validate_attendee_display_name(&too_long_name),
            ValidationResult::Invalid(msg) if msg.contains("不能超过")
        ));

        // 包含非法字符
        assert!(matches!(
            validate_attendee_display_name("张三<tag>"),
            ValidationResult::Invalid(msg) if msg.contains("不能包含<或>")
        ));

        assert!(matches!(
            validate_attendee_display_name("John</script>"),
            ValidationResult::Invalid(msg) if msg.contains("不能包含<或>")
        ));

        assert!(matches!(
            validate_attendee_display_name("<div>名称</div>"),
            ValidationResult::Invalid(msg) if msg.contains("不能包含<或>")
        ));
    }

    // ========== validate_location 测试 ==========

    #[test]
    fn test_validate_location_valid_cases() {
        // 空位置（可选）
        assert!(matches!(
            validate_location("", None),
            ValidationResult::Valid
        ));

        // 只有位置名称
        assert!(matches!(
            validate_location("会议室A", None),
            ValidationResult::Valid
        ));

        // 位置名称和地址
        assert!(matches!(
            validate_location("会议室A", Some("北京市朝阳区")),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大长度
        let max_name = "A".repeat(100);
        let max_address = "A".repeat(300);
        assert!(matches!(
            validate_location(&max_name, Some(&max_address)),
            ValidationResult::Valid
        ));

        // 包含特殊字符的地址
        assert!(matches!(
            validate_location(
                "公司总部",
                Some("北京市朝阳区建国路88号SOHO现代城A座10层1001室")
            ),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_location_invalid_cases() {
        // 位置名称过长
        let too_long_name = "A".repeat(101);
        assert!(matches!(
            validate_location(&too_long_name, None),
            ValidationResult::Invalid(msg) if msg.contains("位置名称不能超过")
        ));

        // 地址过长
        let too_long_address = "A".repeat(301);
        assert!(matches!(
            validate_location("会议室A", Some(&too_long_address)),
            ValidationResult::Invalid(msg) if msg.contains("位置地址不能超过")
        ));
    }

    // ========== validate_meeting_room_id 测试 ==========

    #[test]
    fn test_validate_meeting_room_id_valid_cases() {
        // 简单有效的会议室ID
        assert!(matches!(
            validate_meeting_room_id("room123"),
            ValidationResult::Valid
        ));

        // 包含下划线和连字符
        assert!(matches!(
            validate_meeting_room_id("meeting_room-1"),
            ValidationResult::Valid
        ));

        // 复杂格式
        assert!(matches!(
            validate_meeting_room_id("floor3-room-A"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_meeting_room_id_invalid_cases() {
        // 空字符串
        assert!(matches!(
            validate_meeting_room_id(""),
            ValidationResult::Invalid(msg) if msg.contains("不能为空")
        ));

        // 包含无效字符
        assert!(matches!(
            validate_meeting_room_id("room@123"),
            ValidationResult::Invalid(msg) if msg.contains("格式无效")
        ));
    }

    // ========== validate_event_status 测试 ==========

    #[test]
    fn test_validate_event_status_valid_cases() {
        // 所有有效状态
        let valid_statuses = ["confirmed", "tentative", "cancelled"];
        for status in valid_statuses {
            assert!(
                matches!(validate_event_status(status), ValidationResult::Valid),
                "Should be valid: {}",
                status
            );
        }

        // 大小写混合
        assert!(matches!(
            validate_event_status("CONFIRMED"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_event_status("TENTATIVE"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_event_status("CANCELLED"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_event_status_invalid_cases() {
        // 无效状态
        let invalid_statuses = ["pending", "completed", "unknown", ""];
        for status in invalid_statuses {
            assert!(
                matches!(
                    validate_event_status(status),
                    ValidationResult::Invalid(msg) if msg.contains("无效的日程状态")
                ),
                "Should be invalid: {}",
                status
            );
        }
    }

    // ========== validate_calendar_event_creation_params 测试 ==========

    #[test]
    fn test_validate_calendar_event_creation_params_valid_cases() {
        // 普通定时日程
        assert!(matches!(
            validate_calendar_event_creation_params(
                "团队会议",
                "2024-01-01T10:00:00+08:00",
                "2024-01-01T11:00:00+08:00",
                false,
                5
            ),
            ValidationResult::Valid
        ));

        // 全天日程
        assert!(matches!(
            validate_calendar_event_creation_params(
                "团队建设活动",
                "2024-01-01",
                "2024-01-02",
                true,
                50
            ),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大参与人数（定时日程）
        assert!(matches!(
            validate_calendar_event_creation_params(
                "大型会议",
                "2024-01-01T10:00:00+08:00",
                "2024-01-01T11:00:00+08:00",
                false,
                1000
            ),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大参与人数（全天日程）
        assert!(matches!(
            validate_calendar_event_creation_params(
                "全天活动",
                "2024-01-01",
                "2024-01-02",
                true,
                100
            ),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_calendar_event_creation_params_invalid_cases() {
        // 空标题
        assert!(matches!(
            validate_calendar_event_creation_params(
                "",
                "2024-01-01T10:00:00+08:00",
                "2024-01-01T11:00:00+08:00",
                false,
                5
            ),
            ValidationResult::Invalid(msg) if msg.contains("日程标题不能为空")
        ));

        // 定时日程参与人过多
        assert!(matches!(
            validate_calendar_event_creation_params(
                "超大型会议",
                "2024-01-01T10:00:00+08:00",
                "2024-01-01T11:00:00+08:00",
                false,
                1001
            ),
            ValidationResult::Invalid(msg) if msg.contains("参与人数量不能超过1000人")
        ));

        // 全天日程参与人过多
        assert!(matches!(
            validate_calendar_event_creation_params(
                "超大型全天活动",
                "2024-01-01",
                "2024-01-02",
                true,
                101
            ),
            ValidationResult::Invalid(msg) if msg.contains("参与人数量不能超过100人")
        ));
    }

    // ========== validate_calendar_search_params 测试 ==========

    #[test]
    fn test_validate_calendar_search_params_valid_cases() {
        // 正常搜索参数
        assert!(matches!(
            validate_calendar_search_params("团队", Some(10)),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大结果数
        assert!(matches!(
            validate_calendar_search_params("会议", Some(100)),
            ValidationResult::Valid
        ));

        // 最小结果数
        assert!(matches!(
            validate_calendar_search_params("活动", Some(1)),
            ValidationResult::Valid
        ));

        // 不指定最大结果数
        assert!(matches!(
            validate_calendar_search_params("日程", None),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大搜索词长度
        let max_query = "A".repeat(100);
        assert!(matches!(
            validate_calendar_search_params(&max_query, Some(10)),
            ValidationResult::Valid
        ));

        // 包含特殊字符的搜索词
        assert!(matches!(
            validate_calendar_search_params("2024年Q1季度", Some(10)),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_calendar_search_params_invalid_cases() {
        // 空搜索词
        assert!(matches!(
            validate_calendar_search_params("", Some(10)),
            ValidationResult::Invalid(msg) if msg.contains("搜索关键词不能为空")
        ));

        // 搜索词过长
        let too_long_query = "A".repeat(101);
        assert!(matches!(
            validate_calendar_search_params(&too_long_query, Some(10)),
            ValidationResult::Invalid(msg) if msg.contains("搜索关键词不能超过100个字符")
        ));

        // 结果数超出范围
        assert!(matches!(
            validate_calendar_search_params("会议", Some(0)),
            ValidationResult::Invalid(msg) if msg.contains("最大结果数必须在1-100之间")
        ));

        assert!(matches!(
            validate_calendar_search_params("会议", Some(101)),
            ValidationResult::Invalid(msg) if msg.contains("最大结果数必须在1-100之间")
        ));
    }

    // ========== 综合场景测试 ==========

    #[test]
    fn test_complete_calendar_event_workflow() {
        // 测试完整的日程创建验证流程

        // 1. 验证日程基本信息
        let title_result = validate_event_title("季度总结会议");
        assert!(matches!(title_result, ValidationResult::Valid));

        let description_result = validate_event_description("讨论本季度的工作成果和下季度计划");
        assert!(matches!(description_result, ValidationResult::Valid));

        // 2. 验证时间设置
        let time_result = validate_time_range(
            "2024-03-15T14:00:00+08:00",
            "2024-03-15T16:00:00+08:00",
            false,
        );
        assert!(matches!(time_result, ValidationResult::Valid));

        // 3. 验证参与人
        let user_result = validate_attendee_type_and_id("user", "user123");
        assert!(matches!(user_result, ValidationResult::Valid));

        let user_name_result = validate_attendee_display_name("张三");
        assert!(matches!(user_name_result, ValidationResult::Valid));

        // 4. 验证位置
        let location_result = validate_location("会议室A", Some("北京市朝阳区"));
        assert!(matches!(location_result, ValidationResult::Valid));

        // 5. 验证提醒设置
        let reminder_result = validate_reminder_minutes(15);
        assert!(matches!(reminder_result, ValidationResult::Valid));

        // 6. 验证重复规则
        let recurrence_result = validate_recurrence_rule("RRULE:FREQ=WEEKLY;COUNT=4");
        assert!(matches!(recurrence_result, ValidationResult::Valid));

        // 7. 综合验证
        let creation_result = validate_calendar_event_creation_params(
            "季度总结会议",
            "2024-03-15T14:00:00+08:00",
            "2024-03-15T16:00:00+08:00",
            false,
            8,
        );
        assert!(matches!(creation_result, ValidationResult::Valid));
    }

    #[test]
    fn test_error_message_content() {
        // 测试错误消息的内容是否包含有用信息

        let result = validate_event_title("");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("空"));
            assert!(msg.contains("标题"));
        }

        let result = validate_time_range("invalid", "2024-01-01T10:00:00+08:00", false);
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("格式") || msg.contains("无效"));
        }

        let result = validate_attendee_type_and_id("invalid_type", "id123");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("类型") || msg.contains("无效"));
        }
    }

    #[test]
    fn test_unicode_and_special_characters() {
        // 测试Unicode字符支持

        // 中文标题和描述
        assert!(matches!(
            validate_event_title("2024年第一季度工作总结会议"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_event_description(
                "讨论内容包括：\n📊 业绩回顾\n🎯 目标达成情况\n📈 增长趋势分析"
            ),
            ValidationResult::Valid
        ));

        // 参与人显示名称包含Unicode
        assert!(matches!(
            validate_attendee_display_name("张三 (产品经理)"),
            ValidationResult::Valid
        ));

        // 位置地址包含详细Unicode信息
        assert!(matches!(
            validate_location(
                "北京总部",
                Some("北京市朝阳区建国路88号SOHO现代城A座10层1001室")
            ),
            ValidationResult::Valid
        ));

        // 搜索关键词包含Unicode
        assert!(matches!(
            validate_calendar_search_params("2024年工作计划", Some(10)),
            ValidationResult::Valid
        ));
    }
}

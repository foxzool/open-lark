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
        return ValidationResult::Warning(
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

    #[test]
    fn test_validate_calendar_summary() {
        assert!(matches!(
            validate_calendar_summary("团队日历"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_calendar_summary("a".repeat(101).as_str()),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_calendar_summary("管理员日历"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_calendar_summary(""),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_event_title() {
        assert!(matches!(
            validate_event_title("团队会议"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_event_title(""),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_event_title(" 带空格 "),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_event_title("a".repeat(501).as_str()),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_time_format() {
        assert!(matches!(
            validate_time_format("2024-01-01T10:00:00+08:00"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_time_format("2024-01-01"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_time_format(""),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_time_format("invalid-time"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_time_range() {
        assert!(matches!(
            validate_time_range(
                "2024-01-01T10:00:00+08:00",
                "2024-01-01T11:00:00+08:00",
                false
            ),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_time_range("2024-01-01", "2024-01-02", true),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_time_range(
                "2024-01-01T11:00:00+08:00",
                "2024-01-01T10:00:00+08:00",
                false
            ),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_reminder_minutes() {
        assert!(matches!(
            validate_reminder_minutes(15),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_reminder_minutes(-1),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_reminder_minutes(10081),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_reminder_minutes(7),
            ValidationResult::Warning(_)
        )); // 非标准时间
    }

    #[test]
    fn test_validate_recurrence_rule() {
        assert!(matches!(
            validate_recurrence_rule(""),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_recurrence_rule("RRULE:FREQ=DAILY;COUNT=5"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_recurrence_rule("FREQ=DAILY"),
            ValidationResult::Invalid(_)
        )); // 缺少RRULE:
        assert!(matches!(
            validate_recurrence_rule("RRULE:INVALID"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_attendee_type_and_id() {
        assert!(matches!(
            validate_attendee_type_and_id("user", "user123"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_attendee_type_and_id("chat", "oc_chat123"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_attendee_type_and_id("third_party", "user@example.com"),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_attendee_type_and_id("invalid", "id123"),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_attendee_type_and_id("user", ""),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_calendar_event_creation_params() {
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

        assert!(matches!(
            validate_calendar_event_creation_params(
                "",
                "2024-01-01T10:00:00+08:00",
                "2024-01-01T11:00:00+08:00",
                false,
                5
            ),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_calendar_search_params() {
        assert!(matches!(
            validate_calendar_search_params("团队", Some(10)),
            ValidationResult::Valid
        ));
        assert!(matches!(
            validate_calendar_search_params("", Some(10)),
            ValidationResult::Invalid(_)
        ));
        assert!(matches!(
            validate_calendar_search_params("a".repeat(101).as_str(), Some(10)),
            ValidationResult::Invalid(_)
        ));
    }
}

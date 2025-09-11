//! 招聘服务验证模块
//!
//! 提供招聘相关功能的验证服务，包括职位管理、人才信息、面试安排、Offer管理等。

use crate::core::validation::{ValidateBuilder, ValidationResult};
use chrono::{Datelike, NaiveDate};

/// 验证职位信息
///
/// # 参数
/// - `title`: 职位标题
/// - `description`: 职位描述
/// - `department_id`: 部门ID
/// - `recruiter_id`: 招聘负责人ID
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_job_position(
    title: &str,
    description: &str,
    department_id: &str,
    recruiter_id: &str,
) -> ValidationResult {
    // 验证职位标题
    if title.is_empty() {
        return ValidationResult::Invalid("Job title cannot be empty".to_string());
    }

    if title.len() > 100 {
        return ValidationResult::Invalid(
            "Job title too long. Maximum 100 characters allowed".to_string(),
        );
    }

    // 验证职位描述
    if description.is_empty() {
        return ValidationResult::Invalid("Job description cannot be empty".to_string());
    }

    if description.len() > 10_000 {
        return ValidationResult::Invalid(
            "Job description too long. Maximum 10,000 characters allowed".to_string(),
        );
    }

    // 验证部门ID格式
    if department_id.is_empty() {
        return ValidationResult::Invalid("Department ID cannot be empty".to_string());
    }

    if !department_id.starts_with("od_") {
        return ValidationResult::Invalid("Department ID must start with 'od_'".to_string());
    }

    // 验证招聘负责人ID
    if recruiter_id.is_empty() {
        return ValidationResult::Invalid("Recruiter ID cannot be empty".to_string());
    }

    if !recruiter_id.starts_with("ou_") {
        return ValidationResult::Invalid("Recruiter ID must start with 'ou_'".to_string());
    }

    ValidationResult::Valid
}

/// 验证工作经验要求
///
/// # 参数
/// - `min_years`: 最小工作年限
/// - `max_years`: 最大工作年限
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_work_experience(min_years: u32, max_years: Option<u32>) -> ValidationResult {
    if min_years > 50 {
        return ValidationResult::Invalid("Minimum work years cannot exceed 50".to_string());
    }

    if let Some(max) = max_years {
        if max > 50 {
            return ValidationResult::Invalid("Maximum work years cannot exceed 50".to_string());
        }

        if min_years > max {
            return ValidationResult::Invalid(
                "Minimum work years cannot be greater than maximum work years".to_string(),
            );
        }
    }

    ValidationResult::Valid
}

/// 验证薪资范围
///
/// # 参数
/// - `min_salary`: 最低薪资
/// - `max_salary`: 最高薪资
/// - `currency`: 货币类型
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_salary_range(min_salary: u32, max_salary: u32, currency: &str) -> ValidationResult {
    if min_salary == 0 {
        return ValidationResult::Invalid("Minimum salary cannot be zero".to_string());
    }

    if max_salary == 0 {
        return ValidationResult::Invalid("Maximum salary cannot be zero".to_string());
    }

    if min_salary > max_salary {
        return ValidationResult::Invalid(
            "Minimum salary cannot be greater than maximum salary".to_string(),
        );
    }

    // 验证薪资上限（1000万）
    if max_salary > 10_000_000 {
        return ValidationResult::Invalid("Maximum salary cannot exceed 10,000,000".to_string());
    }

    // 验证货币类型
    let valid_currencies = ["CNY", "USD", "EUR", "GBP", "JPY", "HKD"];
    if !valid_currencies.contains(&currency) {
        return ValidationResult::Invalid(format!(
            "Invalid currency '{}'. Valid currencies are: {}",
            currency,
            valid_currencies.join(", ")
        ));
    }

    ValidationResult::Valid
}

/// 验证人才基本信息
///
/// # 参数
/// - `name`: 姓名
/// - `email`: 邮箱
/// - `phone`: 电话号码
/// - `resume_url`: 简历URL
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_candidate_basic_info(
    name: &str,
    email: &str,
    phone: &str,
    resume_url: Option<&str>,
) -> ValidationResult {
    // 验证姓名
    if name.is_empty() {
        return ValidationResult::Invalid("Candidate name cannot be empty".to_string());
    }

    if name.len() > 50 {
        return ValidationResult::Invalid(
            "Candidate name too long. Maximum 50 characters allowed".to_string(),
        );
    }

    // 验证邮箱
    if let ValidationResult::Invalid(msg) = crate::core::validation::validate_email(email, "email")
    {
        return ValidationResult::Invalid(format!("Invalid email address: {}", msg));
    }

    // 验证电话号码
    if phone.is_empty() {
        return ValidationResult::Invalid("Phone number cannot be empty".to_string());
    }

    if !phone
        .chars()
        .all(|c| c.is_ascii_digit() || c == '+' || c == '-')
    {
        return ValidationResult::Invalid("Phone number contains invalid characters".to_string());
    }

    if phone.len() < 7 || phone.len() > 20 {
        return ValidationResult::Invalid(
            "Phone number length must be between 7 and 20 characters".to_string(),
        );
    }

    // 验证简历URL
    if let Some(url) = resume_url {
        if !url.is_empty() {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return ValidationResult::Invalid(
                    "Resume URL must start with http:// or https://".to_string(),
                );
            }

            if url.len() > 1000 {
                return ValidationResult::Invalid(
                    "Resume URL too long. Maximum 1000 characters allowed".to_string(),
                );
            }
        }
    }

    ValidationResult::Valid
}

/// 验证生日格式
///
/// # 参数
/// - `birthday`: 生日字符串（格式：YYYY-MM-DD）
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_birthday(birthday: &str) -> ValidationResult {
    if birthday.is_empty() {
        return ValidationResult::Invalid("Birthday cannot be empty".to_string());
    }

    // 解析日期
    let date = match NaiveDate::parse_from_str(birthday, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            return ValidationResult::Invalid(
                "Invalid birthday format. Expected YYYY-MM-DD".to_string(),
            );
        }
    };

    // 验证日期合理性（不能是未来日期，不能太早）
    let today = chrono::Utc::now().date_naive();
    if date > today {
        return ValidationResult::Invalid("Birthday cannot be in the future".to_string());
    }

    let min_date = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
    if date < min_date {
        return ValidationResult::Invalid("Birthday cannot be earlier than 1900-01-01".to_string());
    }

    // 验证年龄（通常在18-70岁之间）
    let age = today.year() - date.year();
    if !(18..=70).contains(&age) {
        return ValidationResult::Invalid("Age must be between 18 and 70 years".to_string());
    }

    ValidationResult::Valid
}

/// 验证教育背景
///
/// # 参数
/// - `school`: 学校名称
/// - `degree`: 学位
/// - `major`: 专业
/// - `graduation_year`: 毕业年份
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_education_background(
    school: &str,
    degree: &str,
    major: &str,
    graduation_year: u32,
) -> ValidationResult {
    // 验证学校名称
    if school.is_empty() {
        return ValidationResult::Invalid("School name cannot be empty".to_string());
    }

    if school.len() > 100 {
        return ValidationResult::Invalid(
            "School name too long. Maximum 100 characters allowed".to_string(),
        );
    }

    // 验证学位
    let valid_degrees = ["高中", "大专", "本科", "硕士", "博士", "博士后"];
    if !valid_degrees.contains(&degree) {
        return ValidationResult::Invalid(format!(
            "Invalid degree '{}'. Valid degrees are: {}",
            degree,
            valid_degrees.join(", ")
        ));
    }

    // 验证专业
    if major.is_empty() {
        return ValidationResult::Invalid("Major cannot be empty".to_string());
    }

    if major.len() > 50 {
        return ValidationResult::Invalid(
            "Major too long. Maximum 50 characters allowed".to_string(),
        );
    }

    // 验证毕业年份
    let current_year = chrono::Utc::now().year() as u32;
    if graduation_year < 1950 || graduation_year > current_year + 5 {
        return ValidationResult::Invalid("Invalid graduation year".to_string());
    }

    ValidationResult::Valid
}

/// 验证面试安排
///
/// # 参数
/// - `interviewer_ids`: 面试官ID列表
/// - `interview_time`: 面试时间（时间戳）
/// - `duration`: 面试时长（分钟）
/// - `interview_type`: 面试类型
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_interview_arrangement(
    interviewer_ids: &[String],
    interview_time: i64,
    duration: u32,
    interview_type: &str,
) -> ValidationResult {
    // 验证面试官
    if interviewer_ids.is_empty() {
        return ValidationResult::Invalid("At least one interviewer is required".to_string());
    }

    if interviewer_ids.len() > 10 {
        return ValidationResult::Invalid("Too many interviewers. Maximum 10 allowed".to_string());
    }

    for (i, interviewer_id) in interviewer_ids.iter().enumerate() {
        if !interviewer_id.starts_with("ou_") {
            return ValidationResult::Invalid(format!(
                "Invalid interviewer ID at index {}: must start with 'ou_'",
                i
            ));
        }
    }

    // 验证面试时间
    let current_time = chrono::Utc::now().timestamp();
    if interview_time <= current_time {
        return ValidationResult::Invalid("Interview time must be in the future".to_string());
    }

    // 验证面试时间不能太远（比如1年后）
    if interview_time > current_time + 365 * 24 * 60 * 60 {
        return ValidationResult::Invalid(
            "Interview time cannot be more than 1 year in the future".to_string(),
        );
    }

    // 验证面试时长
    if !(15..=480).contains(&duration) {
        return ValidationResult::Invalid(
            "Interview duration must be between 15 and 480 minutes".to_string(),
        );
    }

    // 验证面试类型
    let valid_types = ["电话面试", "视频面试", "现场面试", "群面"];
    if !valid_types.contains(&interview_type) {
        return ValidationResult::Invalid(format!(
            "Invalid interview type '{}'. Valid types are: {}",
            interview_type,
            valid_types.join(", ")
        ));
    }

    ValidationResult::Valid
}

/// 验证Offer信息
///
/// # 参数
/// - `candidate_id`: 候选人ID
/// - `position_id`: 职位ID
/// - `salary`: 薪资
/// - `start_date`: 入职日期
/// - `expiry_date`: Offer有效期
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_offer_info(
    candidate_id: &str,
    position_id: &str,
    salary: u32,
    start_date: &str,
    expiry_date: &str,
) -> ValidationResult {
    // 验证候选人ID
    if candidate_id.is_empty() {
        return ValidationResult::Invalid("Candidate ID cannot be empty".to_string());
    }

    if !candidate_id.starts_with("oc_") {
        return ValidationResult::Invalid("Candidate ID must start with 'oc_'".to_string());
    }

    // 验证职位ID
    if position_id.is_empty() {
        return ValidationResult::Invalid("Position ID cannot be empty".to_string());
    }

    // 验证薪资
    if salary == 0 {
        return ValidationResult::Invalid("Salary cannot be zero".to_string());
    }

    if salary > 1_000_000 {
        return ValidationResult::Invalid("Salary cannot exceed 1,000,000 per month".to_string());
    }

    // 验证入职日期
    let start = match NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            return ValidationResult::Invalid(
                "Invalid start date format. Expected YYYY-MM-DD".to_string(),
            );
        }
    };

    // 验证Offer有效期
    let expiry = match NaiveDate::parse_from_str(expiry_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            return ValidationResult::Invalid(
                "Invalid expiry date format. Expected YYYY-MM-DD".to_string(),
            );
        }
    };

    // 验证日期逻辑
    let today = chrono::Utc::now().date_naive();
    if start <= today {
        return ValidationResult::Invalid("Start date must be in the future".to_string());
    }

    if expiry <= today {
        return ValidationResult::Invalid("Expiry date must be in the future".to_string());
    }

    if expiry < start {
        return ValidationResult::Invalid("Expiry date must be after start date".to_string());
    }

    // 验证Offer有效期（通常不超过30天）
    let days_diff = (expiry - start).num_days();
    if days_diff > 30 {
        return ValidationResult::Invalid(
            "Offer expiry date cannot be more than 30 days after start date".to_string(),
        );
    }

    ValidationResult::Valid
}

/// 验证招聘流程状态
///
/// # 参数
/// - `current_status`: 当前状态
/// - `new_status`: 新状态
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_hiring_status_transition(
    current_status: &str,
    new_status: &str,
) -> ValidationResult {
    // 定义状态流转规则
    let valid_transitions = [
        ("简历筛选", "初试"),
        ("初试", "复试"),
        ("复试", "终试"),
        ("终试", "Offer审批"),
        ("Offer审批", "Offer已发"),
        ("Offer已发", "已接受"),
        ("Offer已发", "已拒绝"),
        ("简历筛选", "不合适"),
        ("初试", "不合适"),
        ("复试", "不合适"),
        ("终试", "不合适"),
    ];

    // 检查状态流转是否合法
    if !valid_transitions.contains(&(current_status, new_status)) {
        return ValidationResult::Invalid(format!(
            "Invalid status transition from '{}' to '{}'",
            current_status, new_status
        ));
    }

    ValidationResult::Valid
}

/// 验证招聘需求数量
///
/// # 参数
/// - `headcount`: 招聘人数
/// - `priority`: 优先级（1-5）
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_hiring_requirement(headcount: u32, priority: u8) -> ValidationResult {
    if headcount == 0 {
        return ValidationResult::Invalid("Headcount cannot be zero".to_string());
    }

    if headcount > 100 {
        return ValidationResult::Invalid("Headcount cannot exceed 100".to_string());
    }

    if !(1..=5).contains(&priority) {
        return ValidationResult::Invalid("Priority must be between 1 and 5".to_string());
    }

    ValidationResult::Valid
}

/// 验证人才库标签
///
/// # 参数
/// - `tags`: 标签列表
/// - `max_tags`: 最大标签数量
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_candidate_tags(tags: &[String], max_tags: usize) -> ValidationResult {
    if tags.len() > max_tags {
        return ValidationResult::Invalid(format!(
            "Too many tags. Maximum {} allowed, got {}",
            max_tags,
            tags.len()
        ));
    }

    for (i, tag) in tags.iter().enumerate() {
        if tag.is_empty() {
            return ValidationResult::Invalid(format!("Tag at index {} cannot be empty", i));
        }

        if tag.len() > 20 {
            return ValidationResult::Invalid(format!(
                "Tag at index {} too long. Maximum 20 characters allowed",
                i
            ));
        }

        // 验证标签字符
        if !tag
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == ' ')
        {
            return ValidationResult::Invalid(format!(
                "Tag at index {} contains invalid characters",
                i
            ));
        }
    }

    ValidationResult::Valid
}

/// 验证面试反馈
///
/// # 参数
/// - `feedback`: 反馈内容
/// - `rating`: 评分（1-5）
/// - `interview_phase`: 面试阶段
///
/// # 返回
/// - ValidationResult: 验证结果
pub fn validate_interview_feedback(
    feedback: &str,
    rating: u8,
    interview_phase: &str,
) -> ValidationResult {
    // 验证反馈内容
    if feedback.is_empty() {
        return ValidationResult::Invalid("Feedback cannot be empty".to_string());
    }

    if feedback.len() > 2000 {
        return ValidationResult::Invalid(
            "Feedback too long. Maximum 2000 characters allowed".to_string(),
        );
    }

    // 验证评分
    if !(1..=5).contains(&rating) {
        return ValidationResult::Invalid("Rating must be between 1 and 5".to_string());
    }

    // 验证面试阶段
    let valid_phases = ["初试", "复试", "终试"];
    if !valid_phases.contains(&interview_phase) {
        return ValidationResult::Invalid(format!(
            "Invalid interview phase '{}'. Valid phases are: {}",
            interview_phase,
            valid_phases.join(", ")
        ));
    }

    ValidationResult::Valid
}

/// Builder trait for hire validation
pub trait ValidateHireBuilder {
    /// 验证职位信息
    fn validate_job_position(
        &self,
        title: &str,
        description: &str,
        department_id: &str,
        recruiter_id: &str,
    ) -> ValidationResult {
        validate_job_position(title, description, department_id, recruiter_id)
    }

    /// 验证人才基本信息
    fn validate_candidate_basic_info(
        &self,
        name: &str,
        email: &str,
        phone: &str,
        resume_url: Option<&str>,
    ) -> ValidationResult {
        validate_candidate_basic_info(name, email, phone, resume_url)
    }

    /// 验证面试安排
    fn validate_interview_arrangement(
        &self,
        interviewer_ids: &[String],
        interview_time: i64,
        duration: u32,
        interview_type: &str,
    ) -> ValidationResult {
        validate_interview_arrangement(interviewer_ids, interview_time, duration, interview_type)
    }

    /// 验证Offer信息
    fn validate_offer_info(
        &self,
        candidate_id: &str,
        position_id: &str,
        salary: u32,
        start_date: &str,
        expiry_date: &str,
    ) -> ValidationResult {
        validate_offer_info(candidate_id, position_id, salary, start_date, expiry_date)
    }
}

// 为所有实现了 ValidateBuilder 的类型自动实现 ValidateHireBuilder
impl<T: ValidateBuilder> ValidateHireBuilder for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_job_position() {
        // 有效职位信息
        assert!(matches!(
            validate_job_position(
                "Rust开发工程师",
                "负责Rust后端开发",
                "od_12345678",
                "ou_12345678"
            ),
            ValidationResult::Valid
        ));

        // 空标题
        assert!(matches!(
            validate_job_position("", "描述", "od_123", "ou_123"),
            ValidationResult::Invalid(_)
        ));

        // 无效部门ID
        assert!(matches!(
            validate_job_position("标题", "描述", "dept123", "ou_123"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_work_experience() {
        // 有效工作经验
        assert!(matches!(
            validate_work_experience(3, Some(5)),
            ValidationResult::Valid
        ));

        // 最小年限超过最大
        assert!(matches!(
            validate_work_experience(5, Some(3)),
            ValidationResult::Invalid(_)
        ));

        // 年限过大
        assert!(matches!(
            validate_work_experience(60, None),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_salary_range() {
        // 有效薪资范围
        assert!(matches!(
            validate_salary_range(10000, 15000, "CNY"),
            ValidationResult::Valid
        ));

        // 最低薪资为零
        assert!(matches!(
            validate_salary_range(0, 15000, "CNY"),
            ValidationResult::Invalid(_)
        ));

        // 无效货币
        assert!(matches!(
            validate_salary_range(10000, 15000, "BTC"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_birthday() {
        // 有效生日
        assert!(matches!(
            validate_birthday("1990-01-01"),
            ValidationResult::Valid
        ));

        // 无效格式
        assert!(matches!(
            validate_birthday("1990/01/01"),
            ValidationResult::Invalid(_)
        ));

        // 未来日期
        let future_date = (chrono::Utc::now().year() + 1).to_string() + "-01-01";
        assert!(matches!(
            validate_birthday(&future_date),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_interview_arrangement() {
        // 有效面试安排
        let interviewers = vec!["ou_12345678".to_string(), "ou_87654321".to_string()];
        let future_time = chrono::Utc::now().timestamp() + 24 * 60 * 60;
        assert!(matches!(
            validate_interview_arrangement(&interviewers, future_time, 60, "视频面试"),
            ValidationResult::Valid
        ));

        // 无面试官
        assert!(matches!(
            validate_interview_arrangement(&[], future_time, 60, "视频面试"),
            ValidationResult::Invalid(_)
        ));

        // 面试时间已过
        let past_time = chrono::Utc::now().timestamp() - 24 * 60 * 60;
        assert!(matches!(
            validate_interview_arrangement(&interviewers, past_time, 60, "视频面试"),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validate_offer_info() {
        // 有效Offer信息
        assert!(matches!(
            validate_offer_info(
                "oc_12345678",
                "position_123",
                20000,
                "2025-10-01",
                "2025-10-15"
            ),
            ValidationResult::Valid
        ));

        // 无效候选人ID
        assert!(matches!(
            validate_offer_info(
                "candidate123",
                "position_123",
                20000,
                "2024-02-01",
                "2024-01-15"
            ),
            ValidationResult::Invalid(_)
        ));

        // 薪资为零
        assert!(matches!(
            validate_offer_info("oc_12345678", "position_123", 0, "2024-02-01", "2024-01-15"),
            ValidationResult::Invalid(_)
        ));
    }
}

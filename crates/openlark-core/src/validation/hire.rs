//! 招聘服务验证模块
//!
//! 提供招聘相关功能的验证服务，包括职位管理、人才信息、面试安排、Offer管理等。

use crate::validation::{ValidateBuilder, ValidationResult};
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
    if let ValidationResult::Invalid(msg) = crate::validation::validate_email(email, "email") {
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

    // ========== validate_job_position 测试 ==========

    #[test]
    fn test_validate_job_position_valid_cases() {
        // 有效职位信息 - 中文标题
        assert!(matches!(
            validate_job_position(
                "高级Rust开发工程师",
                "负责公司核心系统的架构设计与开发，需要精通Rust语言及相关生态系统",
                "od_1234567890abcdef",
                "ou_1234567890abcdef"
            ),
            ValidationResult::Valid
        ));

        // 有效职位信息 - 英文标题
        assert!(matches!(
            validate_job_position(
                "Senior Rust Developer",
                "Develop and maintain high-performance backend systems using Rust programming language",
                "od_abcdef1234567890",
                "ou_abcdef1234567890"
            ),
            ValidationResult::Valid
        ));

        // 边界情况 - 最小长度
        assert!(matches!(
            validate_job_position("A", "B", "od_1", "ou_1"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大长度标题
        let max_title = "A".repeat(100);
        assert!(matches!(
            validate_job_position(&max_title, "Valid description", "od_123", "ou_123"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_job_position_invalid_cases() {
        // 空标题
        assert!(matches!(
            validate_job_position("", "Valid description", "od_123", "ou_123"),
            ValidationResult::Invalid(msg) if msg.contains("Job title cannot be empty")
        ));

        // 标题过长
        let long_title = "A".repeat(101);
        assert!(matches!(
            validate_job_position(&long_title, "Valid description", "od_123", "ou_123"),
            ValidationResult::Invalid(msg) if msg.contains("Job title too long")
        ));

        // 空描述
        assert!(matches!(
            validate_job_position("Valid title", "", "od_123", "ou_123"),
            ValidationResult::Invalid(msg) if msg.contains("Job description cannot be empty")
        ));

        // 描述过长
        let long_description = "A".repeat(10_001);
        assert!(matches!(
            validate_job_position("Valid title", &long_description, "od_123", "ou_123"),
            ValidationResult::Invalid(msg) if msg.contains("Job description too long")
        ));

        // 空部门ID
        assert!(matches!(
            validate_job_position("Valid title", "Valid description", "", "ou_123"),
            ValidationResult::Invalid(msg) if msg.contains("Department ID cannot be empty")
        ));

        // 部门ID格式错误
        assert!(matches!(
            validate_job_position("Valid title", "Valid description", "dept_123", "ou_123"),
            ValidationResult::Invalid(msg) if msg.contains("must start with 'od_'")
        ));

        // 空招聘负责人ID
        assert!(matches!(
            validate_job_position("Valid title", "Valid description", "od_123", ""),
            ValidationResult::Invalid(msg) if msg.contains("Recruiter ID cannot be empty")
        ));

        // 招聘负责人ID格式错误
        assert!(matches!(
            validate_job_position("Valid title", "Valid description", "od_123", "user_123"),
            ValidationResult::Invalid(msg) if msg.contains("must start with 'ou_'")
        ));
    }

    // ========== validate_work_experience 测试 ==========

    #[test]
    fn test_validate_work_experience_valid_cases() {
        // 有效工作经验范围
        assert!(matches!(
            validate_work_experience(0, Some(0)),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_work_experience(1, Some(5)),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_work_experience(10, Some(20)),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大值
        assert!(matches!(
            validate_work_experience(50, Some(50)),
            ValidationResult::Valid
        ));

        // 只有最小值
        assert!(matches!(
            validate_work_experience(25, None),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_work_experience_invalid_cases() {
        // 最小年限超过最大年限
        assert!(matches!(
            validate_work_experience(5, Some(3)),
            ValidationResult::Invalid(msg) if msg.contains("Minimum work years cannot be greater than maximum")
        ));

        // 最小年限过大
        assert!(matches!(
            validate_work_experience(51, Some(60)),
            ValidationResult::Invalid(msg) if msg.contains("Minimum work years cannot exceed 50")
        ));

        // 最大年限过大
        assert!(matches!(
            validate_work_experience(10, Some(51)),
            ValidationResult::Invalid(msg) if msg.contains("Maximum work years cannot exceed 50")
        ));

        // 相等都过大
        assert!(matches!(
            validate_work_experience(60, Some(60)),
            ValidationResult::Invalid(msg) if msg.contains("Minimum work years cannot exceed 50")
        ));
    }

    // ========== validate_salary_range 测试 ==========

    #[test]
    fn test_validate_salary_range_valid_cases() {
        // 有效薪资范围 - 各种货币
        let valid_currencies = ["CNY", "USD", "EUR", "GBP", "JPY", "HKD"];
        for currency in valid_currencies {
            assert!(matches!(
                validate_salary_range(10000, 15000, currency),
                ValidationResult::Valid
            ));
        }

        // 边界情况 - 最小薪资
        assert!(matches!(
            validate_salary_range(1, 2, "CNY"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大薪资
        assert!(matches!(
            validate_salary_range(5_000_000, 10_000_000, "CNY"),
            ValidationResult::Valid
        ));

        // 相等薪资
        assert!(matches!(
            validate_salary_range(50000, 50000, "USD"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_salary_range_invalid_cases() {
        // 最低薪资为零
        assert!(matches!(
            validate_salary_range(0, 15000, "CNY"),
            ValidationResult::Invalid(msg) if msg.contains("Minimum salary cannot be zero")
        ));

        // 最高薪资为零
        assert!(matches!(
            validate_salary_range(10000, 0, "CNY"),
            ValidationResult::Invalid(msg) if msg.contains("Maximum salary cannot be zero")
        ));

        // 最低薪资大于最高薪资
        assert!(matches!(
            validate_salary_range(20000, 15000, "CNY"),
            ValidationResult::Invalid(msg) if msg.contains("Minimum salary cannot be greater than maximum")
        ));

        // 薪资超过上限
        assert!(matches!(
            validate_salary_range(5000000, 15000000, "CNY"),
            ValidationResult::Invalid(msg) if msg.contains("Maximum salary cannot exceed 10,000,000")
        ));

        // 无效货币类型
        let invalid_currencies = ["BTC", "ETH", "RUB", "KRW", "SGD", "AUD"];
        for currency in invalid_currencies {
            assert!(matches!(
                validate_salary_range(10000, 15000, currency),
                ValidationResult::Invalid(msg) if msg.contains("Invalid currency")
            ));
        }
    }

    // ========== validate_candidate_basic_info 测试 ==========

    #[test]
    fn test_validate_candidate_basic_info_valid_cases() {
        // 有效候选人信息
        assert!(matches!(
            validate_candidate_basic_info(
                "张三",
                "zhangsan@example.com",
                "+86-13812345678",
                Some("https://example.com/resume.pdf")
            ),
            ValidationResult::Valid
        ));

        // 边界情况 - 最小长度姓名
        assert!(matches!(
            validate_candidate_basic_info("A", "a@b.com", "1234567", None),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大长度姓名
        let max_name = "A".repeat(50);
        assert!(matches!(
            validate_candidate_basic_info(
                &max_name,
                "test@example.com",
                "+1-1234567890",
                Some("https://example.com")
            ),
            ValidationResult::Valid
        ));

        // 无简历URL
        assert!(matches!(
            validate_candidate_basic_info("李四", "lisi@example.com", "+86-13987654321", None),
            ValidationResult::Valid
        ));

        // 空简历URL字符串
        assert!(matches!(
            validate_candidate_basic_info("王五", "wangwu@example.com", "13812345678", Some("")),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_candidate_basic_info_invalid_cases() {
        // 空姓名
        assert!(matches!(
            validate_candidate_basic_info(
                "",
                "test@example.com",
                "13812345678",
                None
            ),
            ValidationResult::Invalid(msg) if msg.contains("Candidate name cannot be empty")
        ));

        // 姓名过长
        let long_name = "A".repeat(51);
        assert!(matches!(
            validate_candidate_basic_info(
                &long_name,
                "test@example.com",
                "13812345678",
                None
            ),
            ValidationResult::Invalid(msg) if msg.contains("Candidate name too long")
        ));

        // 无效邮箱格式
        let invalid_emails = [
            "invalid-email",
            "@example.com",
            "test@",
            "test.example.com",
            "test@.com",
            "test@example.",
        ];
        for email in invalid_emails {
            assert!(matches!(
                validate_candidate_basic_info(
                    "张三",
                    email,
                    "13812345678",
                    None
                ),
                ValidationResult::Invalid(msg) if msg.contains("Invalid email address")
            ));
        }

        // 空电话号码
        assert!(matches!(
            validate_candidate_basic_info(
                "张三",
                "test@example.com",
                "",
                None
            ),
            ValidationResult::Invalid(msg) if msg.contains("Phone number cannot be empty")
        ));

        // 电话号码太短
        assert!(matches!(
            validate_candidate_basic_info(
                "张三",
                "test@example.com",
                "123456",
                None
            ),
            ValidationResult::Invalid(msg) if msg.contains("Phone number length must be between 7 and 20")
        ));

        // 电话号码太长
        let long_phone = "123456789012345678901".to_string();
        assert!(matches!(
            validate_candidate_basic_info(
                "张三",
                "test@example.com",
                &long_phone,
                None
            ),
            ValidationResult::Invalid(msg) if msg.contains("Phone number length must be between 7 and 20")
        ));

        // 电话号码包含无效字符
        let invalid_phones = ["abc1234567", "123-456-7890#", "138 1234 5678"];
        for phone in invalid_phones {
            assert!(matches!(
                validate_candidate_basic_info(
                    "张三",
                    "test@example.com",
                    phone,
                    None
                ),
                ValidationResult::Invalid(msg) if msg.contains("Phone number contains invalid characters")
            ));
        }

        // 无效简历URL
        assert!(matches!(
            validate_candidate_basic_info(
                "张三",
                "test@example.com",
                "13812345678",
                Some("ftp://example.com/resume.pdf")
            ),
            ValidationResult::Invalid(msg) if msg.contains("Resume URL must start with http:// or https://")
        ));

        // 简历URL过长
        let long_url = "https://example.com/".to_string() + &"a".repeat(1000);
        assert!(matches!(
            validate_candidate_basic_info(
                "张三",
                "test@example.com",
                "13812345678",
                Some(&long_url)
            ),
            ValidationResult::Invalid(msg) if msg.contains("Resume URL too long")
        ));
    }

    // ========== validate_birthday 测试 ==========

    #[test]
    fn test_validate_birthday_valid_cases() {
        // 有效生日 - 各种年龄（使用固定日期避免时间依赖）
        let current_year = chrono::Utc::now().year();
        let valid_birthdays = [
            format!("{}-01-01", current_year - 20), // 20岁
            format!("{}-06-15", current_year - 30), // 30岁
            format!("{}-12-31", current_year - 40), // 40岁
            format!("{}-05-20", current_year - 50), // 50岁
            format!("{}-01-01", current_year - 68), // 68岁
        ];

        for birthday in valid_birthdays {
            assert!(
                matches!(validate_birthday(&birthday), ValidationResult::Valid),
                "Should be valid: {}",
                birthday
            );
        }

        // 18岁生日（边界）
        let birthday_18 = format!(
            "{}-{}",
            current_year - 18,
            chrono::Utc::now().format("%m-%d")
        );
        assert!(
            matches!(validate_birthday(&birthday_18), ValidationResult::Valid),
            "Should be valid (18 years): {}",
            birthday_18
        );

        // 70岁生日（边界）
        let birthday_70 = format!(
            "{}-{}",
            current_year - 70,
            chrono::Utc::now().format("%m-%d")
        );
        assert!(
            matches!(validate_birthday(&birthday_70), ValidationResult::Valid),
            "Should be valid (70 years): {}",
            birthday_70
        );

        // 测试单月/单日格式也是有效的（chrono接受这种格式）
        assert!(
            matches!(validate_birthday("1990-1-1"), ValidationResult::Valid),
            "Single digit month/day should be valid"
        );
    }

    #[test]
    fn test_validate_birthday_invalid_cases() {
        // 空生日
        assert!(matches!(
            validate_birthday(""),
            ValidationResult::Invalid(msg) if msg.contains("Birthday cannot be empty")
        ));

        // 无效格式
        let invalid_formats = [
            "1990/01/01",
            "90-01-01",
            "1990-13-01",
            "1990-01-32",
            "1990.01.01",
            "19900101",
            "01-01-1990",
        ];
        for birthday in invalid_formats {
            // Check each individually to see which one is actually valid
            let result = validate_birthday(birthday);
            match result {
                ValidationResult::Valid => {
                    // This should not happen - print debug info
                    panic!("Expected invalid format but got valid for: {}", birthday);
                }
                ValidationResult::Invalid(msg) => {
                    // This is expected, verify error message
                    assert!(
                        matches!(
                            validate_birthday(birthday),
                            ValidationResult::Invalid(err_msg) if
                                err_msg.contains("Invalid birthday format") ||
                                err_msg.contains("Age must be between") ||
                                err_msg.contains("Birthday cannot be earlier than 1900")
                        ),
                        "Should have validation error for: {} (got: {})",
                        birthday,
                        msg
                    );
                }
                ValidationResult::Sanitized(_) => {
                    // Handle sanitized results as valid
                    assert!(true); // Sanitized is acceptable
                } // ValidationResult::Warning(_) => {
                  //     // This should not happen - print debug info
                  //     panic!("Expected invalid format but got warning for: {}", birthday);
                  // }
            }
        }

        // 未来日期
        let current_year = chrono::Utc::now().year();
        let future_date = format!("{}-01-01", current_year + 1);
        assert!(matches!(
            validate_birthday(&future_date),
            ValidationResult::Invalid(msg) if msg.contains("Birthday cannot be in the future")
        ));

        // 日期太早
        assert!(matches!(
            validate_birthday("1899-12-31"),
            ValidationResult::Invalid(msg) if msg.contains("Birthday cannot be earlier than 1900-01-01")
        ));

        // 年龄太小（17岁）
        let current_year = chrono::Utc::now().year();
        let young_birthday = format!(
            "{}-{}",
            current_year - 17,
            chrono::Utc::now().format("%m-%d")
        );
        assert!(matches!(
            validate_birthday(&young_birthday),
            ValidationResult::Invalid(msg) if msg.contains("Age must be between 18 and 70 years")
        ));

        // 年龄太大（71岁）
        let old_birthday = format!(
            "{}-{}",
            current_year - 71,
            chrono::Utc::now().format("%m-%d")
        );
        assert!(matches!(
            validate_birthday(&old_birthday),
            ValidationResult::Invalid(msg) if msg.contains("Age must be between 18 and 70 years")
        ));
    }

    // ========== validate_education_background 测试 ==========

    #[test]
    fn test_validate_education_background_valid_cases() {
        // 有效教育背景 - 所有学位
        let valid_degrees = ["高中", "大专", "本科", "硕士", "博士", "博士后"];
        for degree in valid_degrees {
            assert!(matches!(
                validate_education_background("北京大学", degree, "计算机科学与技术", 2020),
                ValidationResult::Valid
            ));
        }

        // 边界情况 - 最小学校名长度
        assert!(matches!(
            validate_education_background("A", "本科", "B", 2020),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大学校名长度
        let max_school = "A".repeat(100);
        assert!(matches!(
            validate_education_background(&max_school, "本科", "计算机", 2020),
            ValidationResult::Valid
        ));

        // 边界情况 - 最小专业名长度
        assert!(matches!(
            validate_education_background("清华大学", "硕士", "A", 2020),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大专业名长度
        let max_major = "A".repeat(50);
        assert!(matches!(
            validate_education_background("复旦大学", "博士", &max_major, 2020),
            ValidationResult::Valid
        ));

        // 边界年份 - 最早
        assert!(matches!(
            validate_education_background("大学", "本科", "专业", 1950),
            ValidationResult::Valid
        ));

        // 边界年份 - 最晚（未来5年）
        let future_year = chrono::Utc::now().year() as u32 + 5;
        assert!(matches!(
            validate_education_background("大学", "本科", "专业", future_year),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_education_background_invalid_cases() {
        // 空学校名
        assert!(matches!(
            validate_education_background("", "本科", "计算机", 2020),
            ValidationResult::Invalid(msg) if msg.contains("School name cannot be empty")
        ));

        // 学校名过长
        let long_school = "A".repeat(101);
        assert!(matches!(
            validate_education_background(&long_school, "本科", "计算机", 2020),
            ValidationResult::Invalid(msg) if msg.contains("School name too long")
        ));

        // 无效学位
        assert!(matches!(
            validate_education_background("北京大学", "专科", "计算机", 2020),
            ValidationResult::Invalid(msg) if msg.contains("Invalid degree")
        ));

        // 空专业
        assert!(matches!(
            validate_education_background("清华大学", "硕士", "", 2020),
            ValidationResult::Invalid(msg) if msg.contains("Major cannot be empty")
        ));

        // 专业过长
        let long_major = "A".repeat(51);
        assert!(matches!(
            validate_education_background("复旦大学", "博士", &long_major, 2020),
            ValidationResult::Invalid(msg) if msg.contains("Major too long")
        ));

        // 毕业年份太早
        assert!(matches!(
            validate_education_background("大学", "本科", "专业", 1949),
            ValidationResult::Invalid(msg) if msg.contains("Invalid graduation year")
        ));

        // 毕业年份太晚
        let too_future_year = chrono::Utc::now().year() as u32 + 6;
        assert!(matches!(
            validate_education_background("大学", "本科", "专业", too_future_year),
            ValidationResult::Invalid(msg) if msg.contains("Invalid graduation year")
        ));
    }

    // ========== validate_interview_arrangement 测试 ==========

    #[test]
    fn test_validate_interview_arrangement_valid_cases() {
        // 有效面试安排 - 各种类型
        let valid_types = ["电话面试", "视频面试", "现场面试", "群面"];
        let interviewers = vec!["ou_12345678".to_string()];
        let future_time = chrono::Utc::now().timestamp() + 24 * 60 * 60;

        for interview_type in valid_types {
            assert!(matches!(
                validate_interview_arrangement(&interviewers, future_time, 60, interview_type),
                ValidationResult::Valid
            ));
        }

        // 边界情况 - 最少面试官数量
        assert!(matches!(
            validate_interview_arrangement(&interviewers, future_time, 30, "视频面试"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最多面试官数量
        let many_interviewers: Vec<String> = (0..10).map(|i| format!("ou_{}", i)).collect();
        assert!(matches!(
            validate_interview_arrangement(&many_interviewers, future_time, 60, "现场面试"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最短面试时长
        assert!(matches!(
            validate_interview_arrangement(&interviewers, future_time, 15, "电话面试"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最长面试时长
        assert!(matches!(
            validate_interview_arrangement(&interviewers, future_time, 480, "群面"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最远面试时间（1年后）
        let far_future_time = chrono::Utc::now().timestamp() + 365 * 24 * 60 * 60;
        assert!(matches!(
            validate_interview_arrangement(&interviewers, far_future_time, 60, "视频面试"),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_interview_arrangement_invalid_cases() {
        let interviewers = vec!["ou_12345678".to_string()];
        let future_time = chrono::Utc::now().timestamp() + 24 * 60 * 60;

        // 空面试官列表
        assert!(matches!(
            validate_interview_arrangement(&[], future_time, 60, "视频面试"),
            ValidationResult::Invalid(msg) if msg.contains("At least one interviewer is required")
        ));

        // 面试官数量过多
        let too_many_interviewers: Vec<String> = (0..11).map(|i| format!("ou_{}", i)).collect();
        assert!(matches!(
            validate_interview_arrangement(&too_many_interviewers, future_time, 60, "视频面试"),
            ValidationResult::Invalid(msg) if msg.contains("Too many interviewers")
        ));

        // 面试官ID格式错误
        let invalid_interviewers = vec!["user_123".to_string()];
        assert!(matches!(
            validate_interview_arrangement(&invalid_interviewers, future_time, 60, "视频面试"),
            ValidationResult::Invalid(msg) if msg.contains("must start with 'ou_'")
        ));

        // 面试时间已过
        let past_time = chrono::Utc::now().timestamp() - 24 * 60 * 60;
        assert!(matches!(
            validate_interview_arrangement(&interviewers, past_time, 60, "视频面试"),
            ValidationResult::Invalid(msg) if msg.contains("Interview time must be in the future")
        ));

        // 面试时间太远（超过1年）
        let too_far_time = chrono::Utc::now().timestamp() + 366 * 24 * 60 * 60;
        assert!(matches!(
            validate_interview_arrangement(&interviewers, too_far_time, 60, "视频面试"),
            ValidationResult::Invalid(msg) if msg.contains("Interview time cannot be more than 1 year")
        ));

        // 面试时长太短
        assert!(matches!(
            validate_interview_arrangement(&interviewers, future_time, 14, "视频面试"),
            ValidationResult::Invalid(msg) if msg.contains("Interview duration must be between 15 and 480")
        ));

        // 面试时长太长
        assert!(matches!(
            validate_interview_arrangement(&interviewers, future_time, 481, "视频面试"),
            ValidationResult::Invalid(msg) if msg.contains("Interview duration must be between 15 and 480")
        ));

        // 无效面试类型
        assert!(matches!(
            validate_interview_arrangement(&interviewers, future_time, 60, "线上面试"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid interview type")
        ));
    }

    // ========== validate_offer_info 测试 ==========

    #[test]
    fn test_validate_offer_info_valid_cases() {
        // 有效Offer信息
        let start_date = chrono::Utc::now().date_naive() + chrono::Duration::days(10);
        let expiry_date = start_date + chrono::Duration::days(20);

        assert!(matches!(
            validate_offer_info(
                "oc_12345678",
                "position_123",
                20000,
                &start_date.format("%Y-%m-%d").to_string(),
                &expiry_date.format("%Y-%m-%d").to_string()
            ),
            ValidationResult::Valid
        ));

        // 边界情况 - 最小薪资
        assert!(matches!(
            validate_offer_info(
                "oc_123",
                "pos_123",
                1,
                &start_date.format("%Y-%m-%d").to_string(),
                &expiry_date.format("%Y-%m-%d").to_string()
            ),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大薪资
        assert!(matches!(
            validate_offer_info(
                "oc_456",
                "pos_456",
                1_000_000,
                &start_date.format("%Y-%m-%d").to_string(),
                &expiry_date.format("%Y-%m-%d").to_string()
            ),
            ValidationResult::Valid
        ));

        // 边界情况 - 最大有效期（30天）
        let max_expiry_date = start_date + chrono::Duration::days(30);
        assert!(matches!(
            validate_offer_info(
                "oc_789",
                "pos_789",
                50000,
                &start_date.format("%Y-%m-%d").to_string(),
                &max_expiry_date.format("%Y-%m-%d").to_string()
            ),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_offer_info_invalid_cases() {
        let start_date = chrono::Utc::now().date_naive() + chrono::Duration::days(10);
        let expiry_date = start_date + chrono::Duration::days(20);
        let start_date_str = start_date.format("%Y-%m-%d").to_string();
        let expiry_date_str = expiry_date.format("%Y-%m-%d").to_string();

        // 空候选人ID
        assert!(matches!(
            validate_offer_info("", "position_123", 20000, &start_date_str, &expiry_date_str),
            ValidationResult::Invalid(msg) if msg.contains("Candidate ID cannot be empty")
        ));

        // 候选人ID格式错误
        assert!(matches!(
            validate_offer_info("candidate_123", "position_123", 20000, &start_date_str, &expiry_date_str),
            ValidationResult::Invalid(msg) if msg.contains("Candidate ID must start with 'oc_'")
        ));

        // 空职位ID
        assert!(matches!(
            validate_offer_info("oc_123", "", 20000, &start_date_str, &expiry_date_str),
            ValidationResult::Invalid(msg) if msg.contains("Position ID cannot be empty")
        ));

        // 薪资为零
        assert!(matches!(
            validate_offer_info("oc_123", "position_123", 0, &start_date_str, &expiry_date_str),
            ValidationResult::Invalid(msg) if msg.contains("Salary cannot be zero")
        ));

        // 薪资过高
        assert!(matches!(
            validate_offer_info("oc_123", "position_123", 1_000_001, &start_date_str, &expiry_date_str),
            ValidationResult::Invalid(msg) if msg.contains("Salary cannot exceed 1,000,000")
        ));

        // 无效开始日期格式
        assert!(matches!(
            validate_offer_info("oc_123", "position_123", 20000, "2024/12/01", &expiry_date_str),
            ValidationResult::Invalid(msg) if msg.contains("Invalid start date format")
        ));

        // 无效有效期格式
        assert!(matches!(
            validate_offer_info("oc_123", "position_123", 20000, &start_date_str, "2024/12/15"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid expiry date format")
        ));

        // 开始日期不是未来
        let today = chrono::Utc::now()
            .date_naive()
            .format("%Y-%m-%d")
            .to_string();
        let future_expiry = (chrono::Utc::now().date_naive() + chrono::Duration::days(20))
            .format("%Y-%m-%d")
            .to_string();
        assert!(matches!(
            validate_offer_info("oc_123", "position_123", 20000, &today, &future_expiry),
            ValidationResult::Invalid(msg) if msg.contains("Start date must be in the future")
        ));

        // 有效期不是未来
        let past_expiry = chrono::Utc::now()
            .date_naive()
            .format("%Y-%m-%d")
            .to_string();
        assert!(matches!(
            validate_offer_info("oc_123", "position_123", 20000, &start_date_str, &past_expiry),
            ValidationResult::Invalid(msg) if msg.contains("Expiry date must be in the future")
        ));

        // 有效期早于开始日期
        let early_expiry = (start_date - chrono::Duration::days(5))
            .format("%Y-%m-%d")
            .to_string();
        assert!(matches!(
            validate_offer_info("oc_123", "position_123", 20000, &start_date_str, &early_expiry),
            ValidationResult::Invalid(msg) if msg.contains("Expiry date must be after start date")
        ));

        // 有效期超过30天
        let late_expiry = (start_date + chrono::Duration::days(31))
            .format("%Y-%m-%d")
            .to_string();
        assert!(matches!(
            validate_offer_info("oc_123", "position_123", 20000, &start_date_str, &late_expiry),
            ValidationResult::Invalid(msg) if msg.contains("Offer expiry date cannot be more than 30 days")
        ));
    }

    // ========== validate_hiring_status_transition 测试 ==========

    #[test]
    fn test_validate_hiring_status_transition_valid_cases() {
        // 有效状态流转
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

        for (current, new) in valid_transitions {
            assert!(matches!(
                validate_hiring_status_transition(current, new),
                ValidationResult::Valid
            ));
        }
    }

    #[test]
    fn test_validate_hiring_status_transition_invalid_cases() {
        // 无效状态流转
        let invalid_transitions = [
            ("初试", "简历筛选"),     // 逆向流转
            ("复试", "初试"),         // 跨越阶段
            ("不合适", "初试"),       // 从不合适恢复
            ("Offer已发", "复试"),    // 逆向流转
            ("已接受", "Offer已发"),  // 逆向流转
            ("已拒绝", "初试"),       // 从拒绝恢复
            ("简历筛选", "简历筛选"), // 相同状态
            ("初试", "终试"),         // 跨越阶段
            ("Offer审批", "不合适"),  // 不合理的流转
        ];

        for (current, new) in invalid_transitions {
            assert!(matches!(
                validate_hiring_status_transition(current, new),
                ValidationResult::Invalid(msg) if msg.contains("Invalid status transition")
            ));
        }
    }

    // ========== validate_hiring_requirement 测试 ==========

    #[test]
    fn test_validate_hiring_requirement_valid_cases() {
        // 有效招聘需求
        assert!(matches!(
            validate_hiring_requirement(1, 1),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_hiring_requirement(10, 3),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_hiring_requirement(50, 5),
            ValidationResult::Valid
        ));

        // 边界情况
        assert!(matches!(
            validate_hiring_requirement(100, 1),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_hiring_requirement(1, 5),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_hiring_requirement_invalid_cases() {
        // 招聘人数为零
        assert!(matches!(
            validate_hiring_requirement(0, 3),
            ValidationResult::Invalid(msg) if msg.contains("Headcount cannot be zero")
        ));

        // 招聘人数过多
        assert!(matches!(
            validate_hiring_requirement(101, 3),
            ValidationResult::Invalid(msg) if msg.contains("Headcount cannot exceed 100")
        ));

        // 优先级过低
        assert!(matches!(
            validate_hiring_requirement(10, 0),
            ValidationResult::Invalid(msg) if msg.contains("Priority must be between 1 and 5")
        ));

        // 优先级过高
        assert!(matches!(
            validate_hiring_requirement(10, 6),
            ValidationResult::Invalid(msg) if msg.contains("Priority must be between 1 and 5")
        ));
    }

    // ========== validate_candidate_tags 测试 ==========

    #[test]
    fn test_validate_candidate_tags_valid_cases() {
        // 有效标签
        let valid_tags = vec![
            "Rust".to_string(),
            "后端开发".to_string(),
            "5年经验".to_string(),
        ];
        assert!(matches!(
            validate_candidate_tags(&valid_tags, 5),
            ValidationResult::Valid
        ));

        // 边界情况 - 空标签列表
        assert!(matches!(
            validate_candidate_tags(&[], 10),
            ValidationResult::Valid
        ));

        // 边界情况 - 达到最大标签数
        let max_tags: Vec<String> = (0..5).map(|i| format!("tag{}", i)).collect();
        assert!(matches!(
            validate_candidate_tags(&max_tags, 5),
            ValidationResult::Valid
        ));

        // 边界情况 - 最长标签
        let long_tag = "A".repeat(20);
        assert!(matches!(
            validate_candidate_tags(&[long_tag], 5),
            ValidationResult::Valid
        ));

        // 包含各种有效字符
        let special_tags = vec![
            "tag_with_underscore".to_string(),
            "tag-with-dash".to_string(),
            "tag with space".to_string(),
            "tag123".to_string(),
        ];
        assert!(matches!(
            validate_candidate_tags(&special_tags, 10),
            ValidationResult::Valid
        ));
    }

    #[test]
    fn test_validate_candidate_tags_invalid_cases() {
        // 标签数量过多
        let too_many_tags: Vec<String> = (0..6).map(|i| format!("tag{}", i)).collect();
        assert!(matches!(
            validate_candidate_tags(&too_many_tags, 5),
            ValidationResult::Invalid(msg) if msg.contains("Too many tags")
        ));

        // 空标签
        let tags_with_empty = vec![
            "valid_tag".to_string(),
            "".to_string(),
            "another_tag".to_string(),
        ];
        assert!(matches!(
            validate_candidate_tags(&tags_with_empty, 5),
            ValidationResult::Invalid(msg) if msg.contains("Tag at index 1 cannot be empty")
        ));

        // 标签过长
        let long_tag = "A".repeat(21);
        assert!(matches!(
            validate_candidate_tags(&[long_tag], 5),
            ValidationResult::Invalid(msg) if msg.contains("Tag at index 0 too long")
        ));

        // 包含无效字符
        let invalid_chars_tags = [
            "tag@symbol".to_string(),
            "tag#hash".to_string(),
            "tag*star".to_string(),
            "tag space!".to_string(),
        ];
        for tag in invalid_chars_tags.iter() {
            let tags = vec![tag.clone()];
            assert!(matches!(
                validate_candidate_tags(&tags, 5),
                ValidationResult::Invalid(msg) if msg.contains("Tag at index 0 contains invalid characters")
            ));
        }
    }

    // ========== validate_interview_feedback 测试 ==========

    #[test]
    fn test_validate_interview_feedback_valid_cases() {
        // 有效反馈
        assert!(matches!(
            validate_interview_feedback("候选人技术能力很强，沟通良好，推荐录用", 5, "初试"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最短反馈
        assert!(matches!(
            validate_interview_feedback("A", 1, "初试"),
            ValidationResult::Valid
        ));

        // 边界情况 - 最长反馈
        let long_feedback = "A".repeat(2000);
        assert!(matches!(
            validate_interview_feedback(&long_feedback, 3, "复试"),
            ValidationResult::Valid
        ));

        // 所有有效评分
        for rating in 1..=5 {
            assert!(matches!(
                validate_interview_feedback("候选人表现不错", rating, "终试"),
                ValidationResult::Valid
            ));
        }

        // 所有有效面试阶段
        let valid_phases = ["初试", "复试", "终试"];
        for phase in valid_phases {
            assert!(matches!(
                validate_interview_feedback("表现良好", 4, phase),
                ValidationResult::Valid
            ));
        }
    }

    #[test]
    fn test_validate_interview_feedback_invalid_cases() {
        // 空反馈
        assert!(matches!(
            validate_interview_feedback("", 4, "初试"),
            ValidationResult::Invalid(msg) if msg.contains("Feedback cannot be empty")
        ));

        // 反馈过长
        let long_feedback = "A".repeat(2001);
        assert!(matches!(
            validate_interview_feedback(&long_feedback, 4, "初试"),
            ValidationResult::Invalid(msg) if msg.contains("Feedback too long")
        ));

        // 评分过低
        assert!(matches!(
            validate_interview_feedback("表现不错", 0, "初试"),
            ValidationResult::Invalid(msg) if msg.contains("Rating must be between 1 and 5")
        ));

        // 评分过高
        assert!(matches!(
            validate_interview_feedback("表现不错", 6, "初试"),
            ValidationResult::Invalid(msg) if msg.contains("Rating must be between 1 and 5")
        ));

        // 无效面试阶段
        assert!(matches!(
            validate_interview_feedback("表现不错", 4, "笔试"),
            ValidationResult::Invalid(msg) if msg.contains("Invalid interview phase")
        ));
    }

    // ========== ValidateHireBuilder trait 测试 ==========

    #[test]
    fn test_validate_hire_builder_trait() {
        // 创建测试用的验证器
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

        // 测试 trait 方法
        assert!(matches!(
            validator.validate_job_position("测试职位", "测试描述", "od_123", "ou_123"),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validator.validate_candidate_basic_info(
                "张三",
                "test@example.com",
                "13812345678",
                None
            ),
            ValidationResult::Valid
        ));

        let interviewers = vec!["ou_123".to_string()];
        let future_time = chrono::Utc::now().timestamp() + 24 * 60 * 60;
        assert!(matches!(
            validator.validate_interview_arrangement(&interviewers, future_time, 60, "视频面试"),
            ValidationResult::Valid
        ));

        let future_date = (chrono::Utc::now().date_naive() + chrono::Duration::days(10))
            .format("%Y-%m-%d")
            .to_string();
        let expiry_date = (chrono::Utc::now().date_naive() + chrono::Duration::days(20))
            .format("%Y-%m-%d")
            .to_string();
        assert!(matches!(
            validator.validate_offer_info(
                "oc_123",
                "position_123",
                20000,
                &future_date,
                &expiry_date
            ),
            ValidationResult::Valid
        ));
    }

    // ========== 综合场景测试 ==========

    #[test]
    fn test_complete_hiring_workflow_validation() {
        // 测试完整的招聘流程验证场景

        // 1. 创建职位
        let position_result = validate_job_position(
            "高级Rust开发工程师",
            "负责分布式系统开发，要求精通Rust和系统编程",
            "od_6889b3c12345678",
            "ou_6889b3c12345678",
        );
        assert!(matches!(position_result, ValidationResult::Valid));

        // 2. 验证工作经验要求
        let experience_result = validate_work_experience(3, Some(8));
        assert!(matches!(experience_result, ValidationResult::Valid));

        // 3. 验证薪资范围
        let salary_result = validate_salary_range(25000, 35000, "CNY");
        assert!(matches!(salary_result, ValidationResult::Valid));

        // 4. 验证候选人信息
        let candidate_result = validate_candidate_basic_info(
            "李明",
            "liming@example.com",
            "+86-13812345678",
            Some("https://drive.google.com/resume.pdf"),
        );
        assert!(matches!(candidate_result, ValidationResult::Valid));

        // 5. 验证候选人年龄
        let birthday_result = validate_birthday("1992-06-15");
        assert!(matches!(birthday_result, ValidationResult::Valid));

        // 6. 验证教育背景
        let education_result =
            validate_education_background("清华大学", "硕士", "计算机科学与技术", 2018);
        assert!(matches!(education_result, ValidationResult::Valid));

        // 7. 验证面试安排
        let interviewers = vec![
            "ou_6889b3c12345678".to_string(),
            "ou_6889b3c87654321".to_string(),
        ];
        let interview_time = chrono::Utc::now().timestamp() + 3 * 24 * 60 * 60;
        let interview_result =
            validate_interview_arrangement(&interviewers, interview_time, 90, "视频面试");
        assert!(matches!(interview_result, ValidationResult::Valid));

        // 8. 验证状态流转
        let transition_result = validate_hiring_status_transition("初试", "复试");
        assert!(matches!(transition_result, ValidationResult::Valid));

        // 9. 验证面试反馈
        let feedback_result = validate_interview_feedback(
            "技术功底扎实，系统设计思路清晰，沟通表达能力强，符合高级工程师要求",
            5,
            "复试",
        );
        assert!(matches!(feedback_result, ValidationResult::Valid));

        // 10. 验证Offer信息
        let start_date = chrono::Utc::now().date_naive() + chrono::Duration::days(15);
        let expiry_date = start_date + chrono::Duration::days(25);
        let offer_result = validate_offer_info(
            "oc_6889b3c12345678",
            "position_rust_senior_001",
            30000,
            &start_date.format("%Y-%m-%d").to_string(),
            &expiry_date.format("%Y-%m-%d").to_string(),
        );
        assert!(matches!(offer_result, ValidationResult::Valid));

        // 11. 最终状态流转到Offer已发
        let final_transition = validate_hiring_status_transition("Offer审批", "Offer已发");
        assert!(matches!(final_transition, ValidationResult::Valid));
    }

    #[test]
    fn test_error_message_content() {
        // 测试错误消息的内容是否包含有用信息
        let result = validate_job_position("", "Valid description", "od_123", "ou_123");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("empty"));
            assert!(msg.contains("title"));
        }

        let result = validate_salary_range(5000, 3000, "CNY");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("greater"));
        }

        let result = validate_interview_arrangement(&[], 1234567890, 60, "视频面试");
        if let ValidationResult::Invalid(msg) = result {
            assert!(msg.contains("interviewer"));
        }

        let result = validate_candidate_basic_info("张三", "invalid-email", "abc", None);
        if let ValidationResult::Invalid(msg) = result {
            assert!(
                msg.contains("email") || msg.contains("Phone number contains invalid characters")
            );
        }
    }

    #[test]
    fn test_unicode_and_special_characters() {
        // 测试Unicode字符支持
        assert!(matches!(
            validate_job_position(
                "🚀 Rust开发工程师 🦀",
                "负责高性能系统开发，需要熟悉Unicode处理",
                "od_中文测试123",
                "ou_中文测试456"
            ),
            ValidationResult::Valid
        ));

        assert!(matches!(
            validate_candidate_basic_info(
                "张三 🌟",
                "zhangsan@example.com", // 使用ASCII邮箱避免Unicode问题
                "+86-13812345678",
                Some("https://example.com/resume.pdf") // 使用ASCII URL
            ),
            ValidationResult::Valid
        ));

        // 测试各种语言的姓名
        let names = ["John Smith", "張偉", "김철수", "田中太郎", "José García"];
        for name in names {
            assert!(
                matches!(
                    validate_candidate_basic_info(name, "test@example.com", "1234567890", None),
                    ValidationResult::Valid
                ),
                "Should be valid for name: {}",
                name
            );
        }
    }
}

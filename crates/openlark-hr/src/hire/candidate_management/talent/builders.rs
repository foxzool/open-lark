use std::collections::HashMap;

use openlark_core::{
        error::LarkAPIError,
        validation::{self, ValidateBuilder, ValidationResult},
        SDKResult,
    };
use crate::hire::models::TalentCreateRequest;

/// 人才创建请求构建器
///
/// 提供链式调用来构建人才创建请求，并内置数据验证功能。
/// 支持设置人才的各种属性，包括基本信息、联系方式、
/// 工作经历、教育背景、技能标签等。
///
/// # 使用示例
///
/// ```rust
/// use open_lark::service::hire::candidate_management::talent::TalentCreateRequestBuilder;
///
/// let request = TalentCreateRequestBuilder::default()
///     .with_name("张三")
///     .with_email("zhangsan@example.com")
///     .with_phone("13800138000")
///     .with_work_experience(5)
///     .with_education("本科")
///     .with_tags(vec!["Java".to_string(), "Spring".to_string()])
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct TalentCreateRequestBuilder {
    request: TalentCreateRequest,
}

impl TalentCreateRequestBuilder {
    /// 设置姓名（必填）
    ///
    /// # 参数
    /// - `name`: 人才姓名，长度必须在2-100字符之间
    pub fn with_name(mut self, name: &str) -> Self {
        self.request.name = name.to_string();
        self
    }

    /// 设置邮箱
    ///
    /// # 参数
    /// - `email`: 邮箱地址，格式必须正确
    pub fn with_email(mut self, email: &str) -> Self {
        self.request.email = Some(email.to_string());
        self
    }

    /// 设置电话
    ///
    /// # 参数
    /// - `phone`: 电话号码，支持各种格式
    pub fn with_phone(mut self, phone: &str) -> Self {
        self.request.phone = Some(phone.to_string());
        self
    }

    /// 设置性别
    ///
    /// # 参数
    /// - `gender`: 性别，如 "male", "female", "other"
    pub fn with_gender(mut self, gender: &str) -> Self {
        self.request.gender = Some(gender.to_string());
        self
    }

    /// 设置生日
    ///
    /// # 参数
    /// - `birthday`: 生日，格式建议为 YYYY-MM-DD
    pub fn with_birthday(mut self, birthday: &str) -> Self {
        self.request.birthday = Some(birthday.to_string());
        self
    }

    /// 设置工作年限
    ///
    /// # 参数
    /// - `years`: 工作年限，0-50年
    pub fn with_work_experience(mut self, years: u32) -> Self {
        self.request.work_experience = Some(years);
        self
    }

    /// 设置学历
    ///
    /// # 参数
    /// - `education`: 学历，如 "高中", "专科", "本科", "硕士", "博士"
    pub fn with_education(mut self, education: &str) -> Self {
        self.request.education = Some(education.to_string());
        self
    }

    /// 设置当前公司
    ///
    /// # 参数
    /// - `company`: 当前所在公司
    pub fn with_current_company(mut self, company: &str) -> Self {
        self.request.current_company = Some(company.to_string());
        self
    }

    /// 设置当前职位
    ///
    /// # 参数
    /// - `position`: 当前职位名称
    pub fn with_current_position(mut self, position: &str) -> Self {
        self.request.current_position = Some(position.to_string());
        self
    }

    /// 设置期望薪资
    ///
    /// # 参数
    /// - `salary`: 期望薪资，如 "10-15K", "20K以上"
    pub fn with_expected_salary(mut self, salary: &str) -> Self {
        self.request.expected_salary = Some(salary.to_string());
        self
    }

    /// 添加简历附件ID
    ///
    /// # 参数
    /// - `attachment_id`: 简历附件ID
    pub fn add_resume_attachment(mut self, attachment_id: &str) -> Self {
        self.request
            .resume_attachment_ids
            .push(attachment_id.to_string());
        self
    }

    /// 设置简历附件ID列表
    ///
    /// # 参数
    /// - `attachment_ids`: 简历附件ID列表
    pub fn with_resume_attachments(mut self, mut attachment_ids: Vec<String>) -> Self {
        self.request
            .resume_attachment_ids
            .append(&mut attachment_ids);
        self
    }

    /// 添加标签
    ///
    /// # 参数
    /// - `tag`: 标签内容
    pub fn add_tag(mut self, tag: &str) -> Self {
        self.request.tags.push(tag.to_string());
        self
    }

    /// 设置标签列表
    ///
    /// # 参数
    /// - `tags`: 标签列表
    pub fn with_tags(mut self, mut tags: Vec<String>) -> Self {
        self.request.tags.append(&mut tags);
        self
    }

    /// 添加自定义字段
    ///
    /// # 参数
    /// - `key`: 字段名
    /// - `value`: 字段值
    pub fn add_custom_field(mut self, key: &str, value: serde_json::Value) -> Self {
        self.request
            .custom_fields
            .get_or_insert_with(HashMap::new)
            .insert(key.to_string(), value);
        self
    }

    /// 设置自定义字段
    ///
    /// # 参数
    /// - `fields`: 自定义字段哈希表
    pub fn with_custom_fields(mut self, fields: HashMap<String, serde_json::Value>) -> Self {
        self.request.custom_fields = Some(fields);
        self
    }

    /// 构建人才创建请求
    ///
    /// 对所有设置的字段进行验证，确保数据符合要求。
    /// 验证通过后返回 TalentCreateRequest，验证失败则返回错误。
    ///
    /// # 验证规则
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/hire

    /// - 姓名：必填，2-100字符
    /// - 邮箱：格式必须正确
    /// - 电话：格式必须正确
    /// - 工作年限：0-50年
    /// - 生日：格式必须正确
    /// - 期望薪资：格式必须正确
    /// - 标签：每个标签1-50字符，最多20个
    /// - 简历附件：每个附件ID1-100字符
    /// - 自定义字段：键名1-100字符
    pub fn build(self) -> SDKResult<TalentCreateRequest> {
        // 1. 验证必填字段
        if self.request.name.is_empty() {
            return Err(LarkAPIError::illegal_param(
                "name is required for talent creation".to_string(),
            ));
        }

        // 2. 验证姓名
        match validation::validate_name(&self.request.name, "name") {
            ValidationResult::Valid => {}
            ValidationResult::Warning(msg) => {
                log::warn!("Name validation warning: {}", msg);
            }
            ValidationResult::Invalid(msg) => {
                return Err(LarkAPIError::illegal_param(format!(
                    "Invalid name: {}",
                    msg
                )));
            }
        }

        // 3. 清理姓名
        let sanitized_name = validation::sanitize_name(&self.request.name);

        // 4. 验证邮箱（如果提供）
        if let Some(ref email) = self.request.email {
            match validation::validate_email(email, "email") {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Email validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid email: {}",
                        msg
                    )));
                }
            }
        }

        // 5. 验证电话（如果提供）
        if let Some(ref phone) = self.request.phone {
            match validation::validate_phone(phone, "phone") {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Phone validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid phone: {}",
                        msg
                    )));
                }
            }
        }

        // 6. 验证工作年限（如果提供）
        if let Some(work_experience) = self.request.work_experience {
            match validation::validate_work_experience(work_experience, "work_experience") {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Work experience validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid work experience: {}",
                        msg
                    )));
                }
            }
        }

        // 7. 验证生日（如果提供）
        if let Some(ref birthday) = self.request.birthday {
            match validation::validate_birthday(&Some(birthday.clone()), "birthday") {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Birthday validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid birthday: {}",
                        msg
                    )));
                }
            }
        }

        // 8. 验证期望薪资（如果提供）
        if let Some(ref salary) = self.request.expected_salary {
            match validation::validate_expected_salary(&Some(salary.clone()), "expected_salary") {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Expected salary validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid expected salary: {}",
                        msg
                    )));
                }
            }
        }

        // 9. 验证标签
        for (index, tag) in self.request.tags.iter().enumerate() {
            match validation::validate_talent_tag(tag, &format!("tags[{}]", index)) {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Tag validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid tag at index {}: {}",
                        index, msg
                    )));
                }
            }
        }

        // 10. 验证标签数量
        match validation::validate_talent_tags(&self.request.tags) {
            ValidationResult::Valid => {}
            ValidationResult::Warning(msg) => {
                log::warn!("Tags count validation warning: {}", msg);
            }
            ValidationResult::Invalid(msg) => {
                return Err(LarkAPIError::illegal_param(format!(
                    "Invalid tags: {}",
                    msg
                )));
            }
        }

        // 11. 验证简历附件
        for (index, attachment_id) in self.request.resume_attachment_ids.iter().enumerate() {
            match validation::validate_resume_attachment(
                attachment_id,
                &format!("resume_attachment_ids[{}]", index),
            ) {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Resume attachment validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid resume attachment at index {}: {}",
                        index, msg
                    )));
                }
            }
        }

        // 12. 验证自定义字段（如果提供）
        if let Some(ref custom_fields) = self.request.custom_fields {
            match validation::validate_custom_fields(&Some(custom_fields.clone()), "custom_fields")
            {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Custom fields validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid custom fields: {}",
                        msg
                    )));
                }
            }
        }

        // 13. 清理标签
        let sanitized_tags: Vec<String> = self
            .request
            .tags
            .iter()
            .map(|tag| validation::sanitize_tag(tag))
            .collect();

        // 14. 返回验证通过的请求
        Ok(TalentCreateRequest {
            name: sanitized_name,
            email: self.request.email,
            phone: self.request.phone,
            gender: self.request.gender,
            birthday: self.request.birthday,
            work_experience: self.request.work_experience,
            education: self.request.education,
            current_company: self.request.current_company,
            current_position: self.request.current_position,
            expected_salary: self.request.expected_salary,
            resume_attachment_ids: self.request.resume_attachment_ids,
            tags: sanitized_tags,
            custom_fields: self.request.custom_fields,
        })
    }
}

impl ValidateBuilder for TalentCreateRequestBuilder {
    fn validate(&self) -> ValidationResult {
        // 验证必填字段
        if self.request.name.is_empty() {
            return ValidationResult::Invalid("name is required".to_string());
        }

        // 验证姓名
        if let ValidationResult::Invalid(msg) =
            validation::validate_name(&self.request.name, "name")
        {
            return ValidationResult::Invalid(msg);
        }

        // 验证邮箱（如果提供）
        if let Some(ref email) = self.request.email {
            if let ValidationResult::Invalid(msg) = validation::validate_email(email, "email") {
                return ValidationResult::Invalid(msg);
            }
        }

        // 验证电话（如果提供）
        if let Some(ref phone) = self.request.phone {
            if let ValidationResult::Invalid(msg) = validation::validate_phone(phone, "phone") {
                return ValidationResult::Invalid(msg);
            }
        }

        ValidationResult::Valid
    }
}

/// 人才列表请求构建器
///
/// 提供链式调用来构建人才列表查询请求，支持各种筛选条件。
#[derive(Debug, Clone, Default)]
pub struct TalentListRequestBuilder {
    request: crate::hire::candidate_management::talent::TalentListRequest,
}

impl TalentListRequestBuilder {
    /// 设置分页大小
    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn with_page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置姓名关键词
    pub fn with_name_keyword(mut self, name_keyword: &str) -> Self {
        self.request.name_keyword = Some(name_keyword.to_string());
        self
    }

    /// 设置邮箱关键词
    pub fn with_email_keyword(mut self, email_keyword: &str) -> Self {
        self.request.email_keyword = Some(email_keyword.to_string());
        self
    }

    /// 设置电话关键词
    pub fn with_phone_keyword(mut self, phone_keyword: &str) -> Self {
        self.request.phone_keyword = Some(phone_keyword.to_string());
        self
    }

    /// 设置工作年限筛选
    pub fn with_work_experience(mut self, work_experience: u32) -> Self {
        self.request.work_experience = Some(work_experience);
        self
    }

    /// 设置学历筛选
    pub fn with_education(mut self, education: &str) -> Self {
        self.request.education = Some(education.to_string());
        self
    }

    /// 添加标签筛选
    pub fn add_tag(mut self, tag: &str) -> Self {
        self.request.tags.push(tag.to_string());
        self
    }

    /// 设置标签列表筛选
    pub fn with_tags(mut self, mut tags: Vec<String>) -> Self {
        self.request.tags.append(&mut tags);
        self
    }

    /// 设置创建时间开始
    pub fn with_created_start_time(mut self, start_time: &str) -> Self {
        self.request.created_start_time = Some(start_time.to_string());
        self
    }

    /// 设置创建时间结束
    pub fn with_created_end_time(mut self, end_time: &str) -> Self {
        self.request.created_end_time = Some(end_time.to_string());
        self
    }

    /// 构建人才列表请求
    pub fn build(self) -> crate::hire::candidate_management::talent::TalentListRequest {
        self.request
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_talent_create_builder_valid() {
        let request = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .with_email("zhangsan@example.com")
            .with_phone("13800138000")
            .with_work_experience(5)
            .with_education("本科")
            .with_tags(vec!["Java".to_string(), "Rust".to_string()])
            .build()
            .unwrap();

        assert_eq!(request.name, "张三");
        assert_eq!(request.email, Some("zhangsan@example.com".to_string()));
        assert_eq!(request.work_experience, Some(5));
        assert_eq!(request.tags, vec!["java".to_string(), "rust".to_string()]);
    }

    #[test]
    fn test_talent_create_builder_missing_name() {
        let result = TalentCreateRequestBuilder::default()
            .with_email("test@example.com")
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("name is required"));
            }
            _ => panic!("Expected IllegalParam error"),
        }
    }

    #[test]
    fn test_talent_create_builder_invalid_email() {
        let result = TalentCreateRequestBuilder::default()
            .with_name("张三")
            .with_email("invalid-email")
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("Invalid email"));
            }
            _ => panic!("Expected IllegalParam error"),
        }
    }

    #[test]
    fn test_talent_create_builder_sanitizes_name() {
        let request = TalentCreateRequestBuilder::default()
            .with_name("  张三  ")
            .with_tags(vec!["  Java  ".to_string()])
            .build()
            .unwrap();

        assert_eq!(request.name, "张三");
        assert_eq!(request.tags, vec!["java".to_string()]);
    }

    #[test]
    fn test_talent_list_builder() {
        let request = TalentListRequestBuilder::default()
            .with_page_size(50)
            .with_name_keyword("张")
            .with_work_experience(5)
            .with_tags(vec!["Java".to_string()])
            .build();

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.name_keyword, Some("张".to_string()));
        assert_eq!(request.work_experience, Some(5));
        assert_eq!(request.tags, vec!["Java".to_string()]);
    }

    #[test]
    fn test_talent_create_builder_with_custom_fields() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert(
            "source".to_string(),
            serde_json::Value::String("招聘网站".to_string()),
        );
        custom_fields.insert(
            "rating".to_string(),
            serde_json::Value::Number(serde_json::Number::from(5)),
        );

        let request = TalentCreateRequestBuilder::default()
            .with_name("李四")
            .with_custom_fields(custom_fields)
            .build()
            .unwrap();

        assert!(request.custom_fields.is_some());
        let fields = request.custom_fields.as_ref().unwrap();
        assert_eq!(
            fields.get("source"),
            Some(&serde_json::Value::String("招聘网站".to_string()))
        );
        assert_eq!(
            fields.get("rating"),
            Some(&serde_json::Value::Number(serde_json::Number::from(5)))
        );
    }
}

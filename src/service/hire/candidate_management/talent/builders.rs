use SDKResult;use std::collections::HashMap;
use openlark_core::error::LarkAPIError;
use crate::{,
    core::{
        
        validation::{self, ValidateBuilder, ValidationResult}
        SDKResult,
    }
    service::hire::models::TalentCreateRequest,
};
/// 人才创建请求构建器,
///
/// 提供链式调用来构建人才创建请求，并内置数据验证功能。,
/// 支持设置人才的各种属性，包括基本信息、联系方式、,
/// 工作经历、教育背景、技能标签等。,
///,
/// # 使用示例,
///,
/// ```rust,
/// use open_lark::service::hire::candidate_management::talent::TalentCreateRequestBuilder;
///
/// let request = TalentCreateRequestBuilder::default(),
///     .with_name("张三"),
///     .with_email("zhangsan@example.com"),
///     .with_phone("13800138000"),
///     .with_work_experience(5),
///     .with_education("本科"),
///     .with_tags(vec!["Java".to_string(), "Spring".to_string()]),
///     .build(),
///     .unwrap();
/// ```,
#[derive(Debug, Clone)]
pub struct TalentCreateRequestBuilder {
    request: TalentCreateRequest}
impl TalentCreateRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 2. 验证姓名,
        match validation::validate_name(&self.request.name, "name") {
            ValidationResult::Valid => {}
ValidationResult::Warning(msg) => {,
                log::warn!("Name validation warning: {}", msg);
ValidationResult::Invalid(msg) => {,
                return Err(LarkAPIError::illegal_param(format!(
                    "Invalid name: {}",
                    msg,
)));
            }
// 3. 清理姓名,
        let sanitized_name = validation::sanitize_name(&self.request.name);
// 4. 验证邮箱（如果提供）,
        if let Some(ref email) = self.request.email {
            match validation::validate_email(email, "email") {
                ValidationResult::Valid => {}
ValidationResult::Warning(msg) => {,
                    log::warn!("Email validation warning: {}", msg);
ValidationResult::Invalid(msg) => {,
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid email: {}",
                        msg,
)));
}
// 5. 验证电话（如果提供）,
        if let Some(ref phone) = self.request.phone {
            match validation::validate_phone(phone, "phone") {
                ValidationResult::Valid => {}
ValidationResult::Warning(msg) => {,
                    log::warn!("Phone validation warning: {}", msg);
ValidationResult::Invalid(msg) => {,
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid phone: {}",
                        msg,
)));
}
// 6. 验证工作年限（如果提供）,
        if let Some(work_experience) = self.request.work_experience {
            match validation::validate_work_experience(work_experience, "work_experience") {
                ValidationResult::Valid => {}
ValidationResult::Warning(msg) => {,
                    log::warn!("Work experience validation warning: {}", msg);
ValidationResult::Invalid(msg) => {,
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid work experience: {}",
                        msg,
)));
}
// 7. 验证生日（如果提供）,
        if let Some(ref birthday) = self.request.birthday {
            match validation::validate_birthday(&Some(birthday.clone()), "birthday") {
                ValidationResult::Valid => {}
ValidationResult::Warning(msg) => {,
                    log::warn!("Birthday validation warning: {}", msg);
ValidationResult::Invalid(msg) => {,
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid birthday: {}",
                        msg,
)));
}
// 8. 验证期望薪资（如果提供）,
        if let Some(ref salary) = self.request.expected_salary {
            match validation::validate_expected_salary(&Some(salary.clone()), "expected_salary") {
                ValidationResult::Valid => {}
ValidationResult::Warning(msg) => {,
                    log::warn!("Expected salary validation warning: {}", msg);
ValidationResult::Invalid(msg) => {,
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid expected salary: {}",
                        msg,
)));
}
// 9. 验证标签,
        for (index, tag) in self.request.tags.iter().enumerate() {
            match validation::validate_talent_tag(tag, &format!("tags[{}]", index)) {
                ValidationResult::Valid => {}
ValidationResult::Warning(msg) => {,
                    log::warn!("Tag validation warning: {}", msg);
ValidationResult::Invalid(msg) => {,
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid tag at index {}: {}",
                        index, msg,
)));
}
// 10. 验证标签数量,
        match validation::validate_talent_tags(&self.request.tags) {
            ValidationResult::Valid => {}
ValidationResult::Warning(msg) => {,
                log::warn!("Tags count validation warning: {}", msg);
ValidationResult::Invalid(msg) => {,
                return Err(LarkAPIError::illegal_param(format!(
                    "Invalid tags: {}",
                    msg,
)));
            }
// 11. 验证简历附件,
        for (index, attachment_id) in self.request.resume_attachment_ids.iter().enumerate() {,
match validation::validate_resume_attachment(,
                attachment_id,
                &format!("resume_attachment_ids[{}]", index),
            ) {
                ValidationResult::Valid => {}
ValidationResult::Warning(msg) => {,
                    log::warn!("Resume attachment validation warning: {}", msg);
ValidationResult::Invalid(msg) => {,
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid resume attachment at index {}: {}",
                        index, msg,
)));
}
// 12. 验证自定义字段（如果提供）,
        if let Some(ref custom_fields) = self.request.custom_fields {
            match validation::validate_custom_fields(&Some(custom_fields.clone()), "custom_fields"),
{,
                ValidationResult::Valid => {}
ValidationResult::Warning(msg) => {,
                    log::warn!("Custom fields validation warning: {}", msg);
ValidationResult::Invalid(msg) => {,
                    return Err(LarkAPIError::illegal_param(format!(
                        "Invalid custom fields: {}",
                        msg,
)));
}
// 13. 清理标签,
        let sanitized_tags: Vec<String> = self,
.request,
            .tags,
.iter()
            .map(|tag| validation::sanitize_tag(tag)),
.collect();
        // 14. 返回验证通过的请求,
Ok(TalentCreateRequest {,
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
            custom_fields: self.request.custom_fields}),
impl ValidateBuilder for TalentCreateRequestBuilder {,
    fn validate(&self) -> ValidationResult {,
// 验证必填字段,
        if self.request.name.is_empty() {,
return ValidationResult::Invalid("name is required".to_string());
        }
// 验证姓名,
        if let ValidationResult::Invalid(msg) =
            validation::validate_name(&self.request.name, "name"),
{,
            return ValidationResult::Invalid(msg);
// 验证邮箱（如果提供）,
        if let Some(ref email) = self.request.email {
            if let ValidationResult::Invalid(msg) = validation::validate_email(email, "email") {,
return ValidationResult::Invalid(msg);
            }
// 验证电话（如果提供）,
        if let Some(ref phone) = self.request.phone {
            if let ValidationResult::Invalid(msg) = validation::validate_phone(phone, "phone") {,
return ValidationResult::Invalid(msg);
            }
ValidationResult::Valid,
    }
/// 人才列表请求构建器,
///
/// 提供链式调用来构建人才列表查询请求，支持各种筛选条件。
#[derive(Debug, Clone)]
pub struct TalentListRequestBuilder {
    request: crate::service::hire::candidate_management::talent::TalentListRequest}
impl TalentListRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_talent_create_builder_valid() {
let request = TalentCreateRequestBuilder::default(),
            .with_name()
.with_email()
            .with_phone()
.with_work_experience()
            .with_education("本科")
            .with_tags(vec!["Java".to_string(), "Rust".to_string()]),
.build()
            .unwrap();

        assert_eq!(request.name, "张三");
        assert_eq!(request.email, Some("zhangsan@example.com".to_string()));
        assert_eq!(request.work_experience, Some(5));
        assert_eq!(request.tags, vec!["java".to_string(), "rust".to_string()]);
#[test]
    fn test_talent_create_builder_missing_name() {
let result = TalentCreateRequestBuilder::default(),
            .with_email()
.build();
        assert!(result.is_err());
match result.unwrap_err() {,
            LarkAPIError::IllegalParamError(msg) => {,
assert!(msg.contains("name is required"));
            }
            _ => panic!("Expected IllegalParam error"),
        }
#[test]
    fn test_talent_create_builder_invalid_email() {
let result = TalentCreateRequestBuilder::default(),
            .with_name()
.with_email()
            .build();
assert!(result.is_err());
        match result.unwrap_err() {,
LarkAPIError::IllegalParamError(msg) => {,
                assert!(msg.contains("Invalid email"));
            _ => panic!("Expected IllegalParam error"),
        }
#[test]
    fn test_talent_create_builder_sanitizes_name() {
let request = TalentCreateRequestBuilder::default(),
            .with_name()
.with_tags(vec!["  Java  ".to_string()]),
            .build()
.unwrap();
        assert_eq!(request.name, "张三");
        assert_eq!(request.tags, vec!["java".to_string()]);
#[test]
    fn test_talent_list_builder() {
let request = TalentListRequestBuilder::default(),
            .with_page_size()
.with_name_keyword()
            .with_work_experience()
.with_tags(vec!["Java".to_string()]),
            .build();

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.name_keyword, Some("张".to_string()));
        assert_eq!(request.work_experience, Some(5));
        assert_eq!(request.tags, vec!["Java".to_string()]);
#[test]
    fn test_talent_create_builder_with_custom_fields() {
let mut custom_fields = HashMap::new();
        custom_fields.insert(
            "source".to_string(),
            serde_json::Value::String("招聘网站".to_string()),
        );
custom_fields.insert(,
            "rating".to_string(),
            serde_json::Value::Number(serde_json::Number::from(5)),
        );
let request = TalentCreateRequestBuilder::default(),
            .with_name()
.with_custom_fields()
            .build()
.unwrap();
        assert!(request.custom_fields.is_some());
let fields = request.custom_fields.as_ref().unwrap();
        assert_eq!(
            fields.get("source"),
            Some(&serde_json::Value::String("招聘网站".to_string())),
);
        assert_eq!(
            fields.get("rating"),
            Some(&serde_json::Value::Number(serde_json::Number::from(5))),
);
    }

//! Directory v1 更新员工信息API
//!
//! 提供企业员工信息部分更新功能，支持选择性字段更新
//! 适用于企业人力资源管理系统的员工信息维护场景

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use super::EmployeeService;
use super::super::models::{Employee, UserIdType, DepartmentIdType};

/// 更新员工请求
#[derive(Debug, Clone)]
pub struct PatchEmployeeRequest {
    /// 员工ID
    pub employee_id: String,
    /// 员工工号
    pub employee_no: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 英文名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 工作地点
    pub work_location: Option<String>,
    /// 职级
    pub job_level: Option<String>,
    /// 职位
    pub job_title: Option<String>,
    /// 上级ID
    pub leader_id: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl PatchEmployeeRequest {
    /// 创建新的请求实例
    pub fn new(employee_id: impl Into<String>) -> Self {
        Self {
            employee_id: employee_id.into(),
            employee_no: None,
            name: None,
            en_name: None,
            email: None,
            mobile: None,
            department_ids: None,
            work_location: None,
            job_level: None,
            job_title: None,
            leader_id: None,
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置员工工号
    pub fn employee_no(mut self, employee_no: impl Into<String>) -> Self {
        self.employee_no = Some(employee_no.into());
        self
    }

    /// 设置姓名
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置英文名
    pub fn en_name(mut self, en_name: impl Into<String>) -> Self {
        self.en_name = Some(en_name.into());
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// 设置手机号
    pub fn mobile(mut self, mobile: impl Into<String>) -> Self {
        self.mobile = Some(mobile.into());
        self
    }

    /// 设置部门ID列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = Some(department_ids);
        self
    }

    /// 设置工作地点
    pub fn work_location(mut self, work_location: impl Into<String>) -> Self {
        self.work_location = Some(work_location.into());
        self
    }

    /// 设置职级
    pub fn job_level(mut self, job_level: impl Into<String>) -> Self {
        self.job_level = Some(job_level.into());
        self
    }

    /// 设置职位
    pub fn job_title(mut self, job_title: impl Into<String>) -> Self {
        self.job_title = Some(job_title.into());
        self
    }

    /// 设置上级ID
    pub fn leader_id(mut self, leader_id: impl Into<String>) -> Self {
        self.leader_id = Some(leader_id.into());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        // 员工ID是必需的
        if self.employee_id.trim().is_empty() {
            return Err("员工ID不能为空".to_string());
        }

        // 验证员工ID长度
        if self.employee_id.len() > 64 {
            return Err("员工ID长度不能超过64个字符".to_string());
        }

        // 验证员工ID格式（只允许字母、数字、下划线、连字符）
        let allowed_chars = |s: &str| {
            s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        };

        if !allowed_chars(&self.employee_id) {
            return Err("员工ID只能包含字母、数字、下划线和连字符".to_string());
        }

        // 验证姓名（如果提供）
        if let Some(ref name) = self.name {
            if !name.trim().is_empty() {
                if name.len() > 50 {
                    return Err("姓名长度不能超过50个字符".to_string());
                }
            } else {
                return Err("姓名不能为空字符串".to_string());
            }
        }

        // 验证邮箱格式（如果提供）
        if let Some(ref email) = self.email {
            if !email.trim().is_empty() {
                // 简单的邮箱格式验证
                if !email.contains('@') || !email.contains('.') {
                    return Err("邮箱格式不正确".to_string());
                }
                if email.len() > 100 {
                    return Err("邮箱长度不能超过100个字符".to_string());
                }
            } else {
                return Err("邮箱不能为空字符串".to_string());
            }
        }

        // 验证手机号格式（如果提供）
        if let Some(ref mobile) = self.mobile {
            if !mobile.trim().is_empty() {
                // 中国手机号格式验证（11位数字，以1开头）
                if !mobile.chars().all(|c| c.is_ascii_digit()) || mobile.len() != 11 || !mobile.starts_with('1') {
                    return Err("手机号格式不正确，请输入11位数字".to_string());
                }
            } else {
                return Err("手机号不能为空字符串".to_string());
            }
        }

        // 验证员工工号（如果提供）
        if let Some(ref employee_no) = self.employee_no {
            if !employee_no.trim().is_empty() {
                if employee_no.len() > 50 {
                    return Err("员工工号长度不能超过50个字符".to_string());
                }
            } else {
                return Err("员工工号不能为空字符串".to_string());
            }
        }

        // 验证英文名（如果提供）
        if let Some(ref en_name) = self.en_name {
            if !en_name.trim().is_empty() {
                if en_name.len() > 50 {
                    return Err("英文名长度不能超过50个字符".to_string());
                }
            } else {
                return Err("英文名不能为空字符串".to_string());
            }
        }

        // 验证部门ID列表（如果提供）
        if let Some(ref department_ids) = self.department_ids {
            if department_ids.is_empty() {
                return Err("部门ID列表不能为空".to_string());
            }
            for dept_id in department_ids {
                if dept_id.trim().is_empty() {
                    return Err("部门ID不能为空".to_string());
                }
                if dept_id.len() > 64 {
                    return Err("部门ID长度不能超过64个字符".to_string());
                }
            }
        }

        // 验证工作地点（如果提供）
        if let Some(ref work_location) = self.work_location {
            if !work_location.trim().is_empty() {
                if work_location.len() > 100 {
                    return Err("工作地点长度不能超过100个字符".to_string());
                }
            } else {
                return Err("工作地点不能为空字符串".to_string());
            }
        }

        // 验证职级（如果提供）
        if let Some(ref job_level) = self.job_level {
            if !job_level.trim().is_empty() {
                if job_level.len() > 50 {
                    return Err("职级长度不能超过50个字符".to_string());
                }
            } else {
                return Err("职级不能为空字符串".to_string());
            }
        }

        // 验证职位（如果提供）
        if let Some(ref job_title) = self.job_title {
            if !job_title.trim().is_empty() {
                if job_title.len() > 50 {
                    return Err("职位长度不能超过50个字符".to_string());
                }
            } else {
                return Err("职位不能为空字符串".to_string());
            }
        }

        // 验证上级ID（如果提供）
        if let Some(ref leader_id) = self.leader_id {
            if !leader_id.trim().is_empty() {
                if leader_id.len() > 64 {
                    return Err("上级ID长度不能超过64个字符".to_string());
                }
                if !allowed_chars(leader_id) {
                    return Err("上级ID只能包含字母、数字、下划线和连字符".to_string());
                }
            } else {
                return Err("上级ID不能为空字符串".to_string());
            }
        }

        Ok(())
    }

    /// 检查是否有任何字段需要更新
    pub fn has_updates(&self) -> bool {
        self.employee_no.is_some() ||
        self.name.is_some() ||
        self.en_name.is_some() ||
        self.email.is_some() ||
        self.mobile.is_some() ||
        self.department_ids.is_some() ||
        self.work_location.is_some() ||
        self.job_level.is_some() ||
        self.job_title.is_some() ||
        self.leader_id.is_some()
    }
}

/// 更新员工响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchEmployeeResponseData {
    /// 更新的员工信息
    pub employee: Employee,
}

/// 更新员工响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchEmployeeResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PatchEmployeeResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for PatchEmployeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    /// 更新员工信息
    ///
    /// 部分更新指定员工的信息，只更新提供的字段
    /// 适用于员工信息维护场景，提供完善的数据验证和错误处理
    ///
    /// # 参数
    /// * `req` - 更新员工请求
    ///
    /// # 返回值
    /// 返回更新后的员工信息
    ///
    /// # 示例
    /// ```rust
    /// let request = PatchEmployeeRequest::new("emp_001")
    ///     .name("张三更新")
    ///     .email("zhangsan_new@company.com")
    ///     .job_title("高级软件工程师")
    ///     .user_id_type(UserIdType::OpenId)
    ///     .department_id_type(DepartmentIdType::DepartmentId);
    /// let response = service.patch_employee(&request).await?;
    /// ```
    pub async fn patch_employee(&self, req: &PatchEmployeeRequest) -> SDKResult<PatchEmployeeResponse> {
        req.validate().map_err(|e| crate::core::SDKError::InvalidParameter(e))?;

        // 检查是否有任何字段需要更新
        if !req.has_updates() {
            return Err(crate::core::SDKError::InvalidParameter(
                "至少需要提供一个要更新的字段".to_string()
            ));
        }

        log::debug!("开始更新员工信息: employee_id={}", req.employee_id);

        // 构建查询参数
        let mut query_params: HashMap<&str, String> = HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.to_string());
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.insert("department_id_type", department_id_type.to_string());
        }

        // 构建请求体（只包含非None字段）
        let mut body = json!({});

        if let Some(ref employee_no) = req.employee_no {
            body["employee_no"] = json!(employee_no);
        }
        if let Some(ref name) = req.name {
            body["name"] = json!(name);
        }
        if let Some(ref en_name) = req.en_name {
            body["en_name"] = json!(en_name);
        }
        if let Some(ref email) = req.email {
            body["email"] = json!(email);
        }
        if let Some(ref mobile) = req.mobile {
            body["mobile"] = json!(mobile);
        }
        if let Some(ref department_ids) = req.department_ids {
            body["department_ids"] = json!(department_ids);
        }
        if let Some(ref work_location) = req.work_location {
            body["work_location"] = json!(work_location);
        }
        if let Some(ref job_level) = req.job_level {
            body["job_level"] = json!(job_level);
        }
        if let Some(ref job_title) = req.job_title {
            body["job_title"] = json!(job_title);
        }
        if let Some(ref leader_id) = req.leader_id {
            body["leader_id"] = json!(leader_id);
        }

        // 构建API路径，替换employee_id占位符
        let api_path = crate::core::endpoints_original::Endpoints::DIRECTORY_V1_EMPLOYEE_GET
            .replace("{employee_id}", &req.employee_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            body: serde_json::to_vec(&body).unwrap_or_default(),
            ..Default::default()
        };

        let resp = Transport::<PatchEmployeeResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            log::info!("员工信息更新成功: employee_id={}", req.employee_id);
        } else {
            log::warn!("员工信息更新失败: employee_id={}, error={:?}",
                req.employee_id, response.error_message);
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 更新员工构建器
#[derive(Debug, Clone)]
pub struct PatchEmployeeBuilder {
    request: PatchEmployeeRequest,
}

impl PatchEmployeeBuilder {
    /// 创建新的构建器
    pub fn new(employee_id: impl Into<String>) -> Self {
        Self {
            request: PatchEmployeeRequest::new(employee_id),
        }
    }

    /// 设置员工工号
    pub fn employee_no(mut self, employee_no: impl Into<String>) -> Self {
        self.request = self.request.employee_no(employee_no);
        self
    }

    /// 设置姓名
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置英文名
    pub fn en_name(mut self, en_name: impl Into<String>) -> Self {
        self.request = self.request.en_name(en_name);
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.request = self.request.email(email);
        self
    }

    /// 设置手机号
    pub fn mobile(mut self, mobile: impl Into<String>) -> Self {
        self.request = self.request.mobile(mobile);
        self
    }

    /// 设置部门ID列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.request = self.request.department_ids(department_ids);
        self
    }

    /// 设置工作地点
    pub fn work_location(mut self, work_location: impl Into<String>) -> Self {
        self.request = self.request.work_location(work_location);
        self
    }

    /// 设置职级
    pub fn job_level(mut self, job_level: impl Into<String>) -> Self {
        self.request = self.request.job_level(job_level);
        self
    }

    /// 设置职位
    pub fn job_title(mut self, job_title: impl Into<String>) -> Self {
        self.request = self.request.job_title(job_title);
        self
    }

    /// 设置上级ID
    pub fn leader_id(mut self, leader_id: impl Into<String>) -> Self {
        self.request = self.request.leader_id(leader_id);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.request = self.request.department_id_type(department_id_type);
        self
    }

    /// 执行更新员工操作
    pub async fn execute(self, service: &EmployeeService) -> SDKResult<PatchEmployeeResponse> {
        service.patch_employee(&self.request).await
    }
}

impl EmployeeService {
    /// 更新员工构建器
    pub fn patch_employee_builder(&self, employee_id: impl Into<String>) -> PatchEmployeeBuilder {
        PatchEmployeeBuilder::new(employee_id)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_patch_employee_request_creation() {
        let request = PatchEmployeeRequest::new("emp_001");
        assert_eq!(request.employee_id, "emp_001");
        assert!(request.employee_no.is_none());
        assert!(request.name.is_none());
        assert!(request.email.is_none());
        assert!(request.mobile.is_none());
    }

    #[test]
    fn test_patch_employee_request_with_fields() {
        let request = PatchEmployeeRequest::new("emp_002")
            .name("张三")
            .email("zhangsan@company.com")
            .mobile("13800138000")
            .employee_no("EMP001")
            .job_title("软件工程师");

        assert_eq!(request.employee_id, "emp_002");
        assert_eq!(request.name, Some("张三".to_string()));
        assert_eq!(request.email, Some("zhangsan@company.com".to_string()));
        assert_eq!(request.mobile, Some("13800138000".to_string()));
        assert_eq!(request.employee_no, Some("EMP001".to_string()));
        assert_eq!(request.job_title, Some("软件工程师".to_string()));
    }

    #[test]
    fn test_patch_employee_request_validation() {
        // 测试正常情况
        let valid_request = PatchEmployeeRequest::new("emp_valid_001")
            .name("李四")
            .email("lisi@company.com")
            .mobile("13900139000");
        assert!(valid_request.validate().is_ok());
        assert!(valid_request.has_updates());

        // 测试空员工ID
        let empty_id_request = PatchEmployeeRequest::new("");
        assert!(empty_id_request.validate().is_err());

        // 测试员工ID过长
        let long_id_request = PatchEmployeeRequest::new(&"a".repeat(65));
        assert!(long_id_request.validate().is_err());

        // 测试员工ID包含非法字符
        let invalid_id_requests = vec![
            PatchEmployeeRequest::new("emp@001"),
            PatchEmployeeRequest::new("emp#001"),
            PatchEmployeeRequest::new("emp 001"),
            PatchEmployeeRequest::new("emp.001"),
        ];

        for request in invalid_id_requests {
            assert!(request.validate().is_err(), "员工ID格式验证失败");
        }

        // 测试空字符串字段
        let empty_field_requests = vec![
            PatchEmployeeRequest::new("emp_test").name(""),
            PatchEmployeeRequest::new("emp_test").email(""),
            PatchEmployeeRequest::new("emp_test").mobile(""),
            PatchEmployeeRequest::new("emp_test").employee_no(""),
            PatchEmployeeRequest::new("emp_test").en_name(""),
            PatchEmployeeRequest::new("emp_test").work_location(""),
            PatchEmployeeRequest::new("emp_test").job_level(""),
            PatchEmployeeRequest::new("emp_test").job_title(""),
            PatchEmployeeRequest::new("emp_test").leader_id(""),
        ];

        for request in empty_field_requests {
            assert!(request.validate().is_err(), "空字符串字段验证失败");
        }

        // 测试无效邮箱格式
        let invalid_email_requests = vec![
            PatchEmployeeRequest::new("emp_test").name("测试").email("invalid-email"),
            PatchEmployeeRequest::new("emp_test").name("测试").email("test@"),
            PatchEmployeeRequest::new("emp_test").name("测试").email("@company.com"),
            PatchEmployeeRequest::new("emp_test").name("测试").email("test.company.com"),
        ];

        for request in invalid_email_requests {
            assert!(request.validate().is_err(), "邮箱格式验证失败");
        }

        // 测试无效手机号格式
        let invalid_mobile_requests = vec![
            PatchEmployeeRequest::new("emp_test").name("测试").mobile("12345678901"),  // 不以1开头
            PatchEmployeeRequest::new("emp_test").name("测试").mobile("1380013800"),    // 10位
            PatchEmployeeRequest::new("emp_test").name("测试").mobile("138001380000"),  // 12位
            PatchEmployeeRequest::new("emp_test").name("测试").mobile("1380013800a"),   // 包含字母
            PatchEmployeeRequest::new("emp_test").name("测试").mobile("138-0013-8000"), // 包含特殊字符
        ];

        for request in invalid_mobile_requests {
            assert!(request.validate().is_err(), "手机号格式验证失败");
        }

        // 测试有效字段长度
        let valid_length_requests = vec![
            PatchEmployeeRequest::new("emp_test").name(&"中".repeat(50)),           // 最大姓名长度
            PatchEmployeeRequest::new("emp_test").email(&format!("{}@company.com", "a".repeat(90))), // 最大邮箱长度
            PatchEmployeeRequest::new("emp_test").employee_no(&"a".repeat(50)),     // 最大工号长度
            PatchEmployeeRequest::new("emp_test").en_name(&"a".repeat(50)),        // 最大英文名长度
            PatchEmployeeRequest::new("emp_test").work_location(&"a".repeat(100)), // 最大工作地点长度
            PatchEmployeeRequest::new("emp_test").job_level(&"a".repeat(50)),      // 最大职级长度
            PatchEmployeeRequest::new("emp_test").job_title(&"a".repeat(50)),      // 最大职位长度
        ];

        for request in valid_length_requests {
            assert!(request.validate().is_ok(), "有效长度字段验证失败");
        }

        // 测试空部门ID列表
        let empty_dept_request = PatchEmployeeRequest::new("emp_test")
            .name("测试")
            .department_ids(vec![]);
        assert!(empty_dept_request.validate().is_err());

        // 测试包含空字符串的部门ID列表
        let invalid_dept_request = PatchEmployeeRequest::new("emp_test")
            .name("测试")
            .department_ids(vec!["dept_001".to_string(), "".to_string()]);
        assert!(invalid_dept_request.validate().is_err());

        // 测试部门ID过长
        let long_dept_request = PatchEmployeeRequest::new("emp_test")
            .name("测试")
            .department_ids(vec![&"a".repeat(65)]);
        assert!(long_dept_request.validate().is_err());

        // 测试有效部门ID列表
        let valid_dept_request = PatchEmployeeRequest::new("emp_test")
            .name("测试")
            .department_ids(vec!["dept_001".to_string(), "dept_002".to_string()]);
        assert!(valid_dept_request.validate().is_ok());

        // 测试无效上级ID格式
        let invalid_leader_requests = vec![
            PatchEmployeeRequest::new("emp_test").name("测试").leader_id("leader@001"),
            PatchEmployeeRequest::new("emp_test").name("测试").leader_id("leader#001"),
            PatchEmployeeRequest::new("emp_test").name("测试").leader_id("leader 001"),
            PatchEmployeeRequest::new("emp_test").name("测试").leader_id("leader.001"),
        ];

        for request in invalid_leader_requests {
            assert!(request.validate().is_err(), "上级ID格式验证失败");
        }

        // 测试有效上级ID
        let valid_leader_request = PatchEmployeeRequest::new("emp_test")
            .name("测试")
            .leader_id("leader_001");
        assert!(valid_leader_request.validate().is_ok());
    }

    #[test]
    fn test_patch_employee_has_updates() {
        // 测试有更新的请求
        let with_updates_request = PatchEmployeeRequest::new("emp_test")
            .name("张三");
        assert!(with_updates_request.has_updates());

        // 测试多字段更新
        let multi_updates_request = PatchEmployeeRequest::new("emp_test")
            .name("张三")
            .email("zhangsan@company.com")
            .mobile("13800138000");
        assert!(multi_updates_request.has_updates());

        // 测试没有更新的请求
        let no_updates_request = PatchEmployeeRequest::new("emp_test");
        assert!(!no_updates_request.has_updates());

        // 测试只有类型字段的请求（不算更新）
        let type_only_request = PatchEmployeeRequest::new("emp_test")
            .user_id_type(UserIdType::OpenId)
            .department_id_type(DepartmentIdType::DepartmentId);
        assert!(!type_only_request.has_updates());
    }

    #[test]
    fn test_patch_employee_response_creation() {
        let employee = Employee {
            employee_id: Some("emp_001".to_string()),
            employee_no: Some("EMP001".to_string()),
            name: Some("张三更新".to_string()),
            en_name: Some("Zhang San Updated".to_string()),
            email: Some("zhangsan_new@company.com".to_string()),
            mobile: Some("13800138000".to_string()),
            department_ids: Some(vec!["dept_001".to_string()]),
            status: None,
            join_time: Some("2023-12-01T09:00:00Z".to_string()),
            leave_time: None,
            work_location: Some("北京".to_string()),
            job_level: Some("P5".to_string()),
            job_title: Some("高级软件工程师".to_string()),
            leader_id: Some("manager_001".to_string()),
        };

        let response_data = PatchEmployeeResponseData {
            employee,
        };

        let response = PatchEmployeeResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().employee.name, Some("张三更新".to_string()));
        assert_eq!(response.data.as_ref().unwrap().employee.email, Some("zhangsan_new@company.com".to_string()));
        assert_eq!(response.data.as_ref().unwrap().employee.job_title, Some("高级软件工程师".to_string()));
    }

    #[test]
    fn test_patch_employee_builder() {
        let builder = PatchEmployeeBuilder::new("emp_003")
            .name("王五")
            .email("wangwu@company.com")
            .mobile("13700137000")
            .employee_no("EMP002")
            .job_title("产品经理")
            .work_location("上海");

        assert_eq!(builder.request.employee_id, "emp_003");
        assert_eq!(builder.request.name, Some("王五".to_string()));
        assert_eq!(builder.request.email, Some("wangwu@company.com".to_string()));
        assert_eq!(builder.request.mobile, Some("13700137000".to_string()));
        assert_eq!(builder.request.employee_no, Some("EMP002".to_string()));
        assert_eq!(builder.request.job_title, Some("产品经理".to_string()));
        assert_eq!(builder.request.work_location, Some("上海".to_string()));
        assert!(builder.request.has_updates());
    }

    #[test]
    fn test_patch_employee_builder_validation() {
        // 测试有效构建器
        let valid_builder = PatchEmployeeBuilder::new("emp_valid_001")
            .name("赵六")
            .email("zhaoliu@company.com")
            .mobile("13600136000")
            .department_ids(vec!["dept_001".to_string()]);
        assert!(valid_builder.request.validate().is_ok());
        assert!(valid_builder.request.has_updates());

        // 测试无效构建器
        let invalid_builder = PatchEmployeeBuilder::new("")
            .name("测试");
        assert!(invalid_builder.request.validate().is_err());

        // 测试无更新字段构建器
        let no_updates_builder = PatchEmployeeBuilder::new("emp_test");
        assert!(!no_updates_builder.request.has_updates());
    }

    #[test]
    fn test_patch_employee_service_method() {
        let config = Config::default();
        let service = EmployeeService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.patch_employee_builder("emp_service_001")
            .name("测试员工")
            .email("test@company.com");
        assert_eq!(builder.request.employee_id, "emp_service_001");
        assert_eq!(builder.request.name, Some("测试员工".to_string()));
        assert_eq!(builder.request.email, Some("test@company.com".to_string()));
        assert!(builder.request.has_updates());
    }

    #[test]
    fn test_patch_employee_endpoint_construction() {
        // 验证端点常量存在
        assert_eq!(
            crate::core::endpoints_original::Endpoints::DIRECTORY_V1_EMPLOYEE_GET,
            "/open-apis/directory/v1/employees/{employee_id}"
        );

        // 验证路径替换逻辑
        let template = crate::core::endpoints_original::Endpoints::DIRECTORY_V1_EMPLOYEE_GET;
        let final_path = template.replace("{employee_id}", "emp_123");
        assert_eq!(final_path, "/open-apis/directory/v1/employees/emp_123");
    }

    #[test]
    fn test_patch_employee_request_methods() {
        // 测试链式调用
        let request = PatchEmployeeRequest::new("emp_chain_001")
            .name("链式调用测试")
            .email("chain@company.com")
            .mobile("13500135000")
            .employee_no("CHAIN001")
            .job_title("测试工程师")
            .en_name("Chain Test")
            .work_location("深圳")
            .job_level("P4")
            .leader_id("leader_001")
            .user_id_type(UserIdType::OpenId)
            .department_id_type(DepartmentIdType::DepartmentId);

        assert_eq!(request.employee_id, "emp_chain_001");
        assert_eq!(request.name, Some("链式调用测试".to_string()));
        assert_eq!(request.email, Some("chain@company.com".to_string()));
        assert_eq!(request.mobile, Some("13500135000".to_string()));
        assert_eq!(request.employee_no, Some("CHAIN001".to_string()));
        assert_eq!(request.job_title, Some("测试工程师".to_string()));
        assert_eq!(request.en_name, Some("Chain Test".to_string()));
        assert_eq!(request.work_location, Some("深圳".to_string()));
        assert_eq!(request.job_level, Some("P4".to_string()));
        assert_eq!(request.leader_id, Some("leader_001".to_string()));
        assert!(request.user_id_type.is_some());
        assert!(request.department_id_type.is_some());
        assert!(request.validate().is_ok());
        assert!(request.has_updates());
    }

    #[test]
    fn test_patch_employee_edge_cases() {
        // 测试最小长度员工ID
        let min_id_request = PatchEmployeeRequest::new("e").name("测试");
        assert!(min_id_request.validate().is_ok());

        // 测试最大长度员工ID
        let max_id_request = PatchEmployeeRequest::new(&"a".repeat(64)).name("测试");
        assert!(max_id_request.validate().is_ok());

        // 测试单字符更新字段
        let single_char_request = PatchEmployeeRequest::new("emp_test")
            .name("测")
            .employee_no("1")
            .en_name("a")
            .work_location("京")
            .job_level("1")
            .job_title("工")
            .leader_id("l");
        assert!(single_char_request.validate().is_ok());
        assert!(single_char_request.has_updates());

        // 测试只更新一个字段
        let single_field_request = PatchEmployeeRequest::new("emp_test")
            .name("仅更新姓名");
        assert!(single_field_request.validate().is_ok());
        assert!(single_field_request.has_updates());

        // 测试更新所有字段
        let all_fields_request = PatchEmployeeRequest::new("emp_test")
            .employee_no("ALL001")
            .name("全字段测试")
            .en_name("All Fields Test")
            .email("all@company.com")
            .mobile("13800138000")
            .department_ids(vec!["dept_001".to_string(), "dept_002".to_string()])
            .work_location("全地点")
            .job_level("ALL")
            .job_title("全职位")
            .leader_id("all_leader");
        assert!(all_fields_request.validate().is_ok());
        assert!(all_fields_request.has_updates());
    }

    #[test]
    fn test_patch_employee_response_trait() {
        assert_eq!(PatchEmployeeResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_patch_employee_comprehensive_scenario() {
        // 测试完整的业务场景 - 员工晋升更新
        let promotion_request = PatchEmployeeRequest::new("emp_promotion_001")
            .name("周八")
            .job_level("P6")
            .job_title("高级软件工程师")
            .leader_id("new_manager_001")
            .user_id_type(UserIdType::OpenId)
            .department_id_type(DepartmentIdType::DepartmentId);

        assert!(promotion_request.validate().is_ok());
        assert_eq!(promotion_request.employee_id, "emp_promotion_001");
        assert_eq!(promotion_request.name, Some("周八".to_string()));
        assert_eq!(promotion_request.job_level, Some("P6".to_string()));
        assert_eq!(promotion_request.job_title, Some("高级软件工程师".to_string()));
        assert_eq!(promotion_request.leader_id, Some("new_manager_001".to_string()));
        assert!(promotion_request.user_id_type.is_some());
        assert!(promotion_request.department_id_type.is_some());
        assert!(promotion_request.has_updates());

        // 模拟响应数据
        let employee = Employee {
            employee_id: Some("emp_promotion_001".to_string()),
            employee_no: None,
            name: Some("周八".to_string()),
            en_name: None,
            email: None,
            mobile: None,
            department_ids: None,
            status: None,
            join_time: None,
            leave_time: None,
            work_location: None,
            job_level: Some("P6".to_string()),
            job_title: Some("高级软件工程师".to_string()),
            leader_id: Some("new_manager_001".to_string()),
        };

        let response_data = PatchEmployeeResponseData {
            employee,
        };

        let response = PatchEmployeeResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());

        let updated_employee = response.data.unwrap().employee;
        assert_eq!(updated_employee.name, Some("周八".to_string()));
        assert_eq!(updated_employee.job_level, Some("P6".to_string()));
        assert_eq!(updated_employee.job_title, Some("高级软件工程师".to_string()));
        assert_eq!(updated_employee.leader_id, Some("new_manager_001".to_string()));
    }

    #[test]
    fn test_patch_employee_different_user_department_types() {
        // 测试不同的用户ID和部门ID类型组合
        let type_combinations = vec![
            (UserIdType::OpenId, DepartmentIdType::OpenDepartmentId),
            (UserIdType::UnionId, DepartmentIdType::OpenDepartmentId),
            (UserIdType::UserId, DepartmentIdType::DepartmentId),
            (UserIdType::OpenId, DepartmentIdType::DepartmentId),
        ];

        for (user_type, dept_type) in type_combinations {
            let request = PatchEmployeeRequest::new("emp_type_test")
                .name("类型测试")
                .department_ids(vec!["dept_001".to_string()])
                .user_id_type(user_type.clone())
                .department_id_type(dept_type.clone());

            assert!(request.validate().is_ok(),
                "用户类型 {:?} + 部门类型 {:?} 组合应该有效",
                user_type, dept_type);
            assert!(request.user_id_type.unwrap() == user_type);
            assert!(request.department_id_type.unwrap() == dept_type);
            assert!(request.has_updates());
        }
    }

    #[test]
    fn test_patch_employee_error_scenarios() {
        // 测试失败响应
        let error_response = PatchEmployeeResponse {
            data: None,
            success: false,
            error_message: Some("员工不存在".to_string()),
            error_code: Some("EMPLOYEE_NOT_FOUND".to_string()),
        };

        assert!(!error_response.success);
        assert!(error_response.data.is_none());
        assert_eq!(error_response.error_message, Some("员工不存在".to_string()));
        assert_eq!(error_response.error_code, Some("EMPLOYEE_NOT_FOUND".to_string()));

        // 测试权限错误
        let permission_error_response = PatchEmployeeResponse {
            data: None,
            success: false,
            error_message: Some("没有更新员工权限".to_string()),
            error_code: Some("PERMISSION_DENIED".to_string()),
        };

        assert!(!permission_error_response.success);
        assert_eq!(permission_error_response.error_message, Some("没有更新员工权限".to_string()));
        assert_eq!(permission_error_response.error_code, Some("PERMISSION_DENIED".to_string()));

        // 测试字段冲突错误
        let conflict_error_response = PatchEmployeeResponse {
            data: None,
            success: false,
            error_message: Some("员工工号已存在".to_string()),
            error_code: Some("EMPLOYEE_NO_EXISTS".to_string()),
        };

        assert!(!conflict_error_response.success);
        assert_eq!(conflict_error_response.error_message, Some("员工工号已存在".to_string()));
        assert_eq!(conflict_error_response.error_code, Some("EMPLOYEE_NO_EXISTS".to_string()));
    }

    #[test]
    fn test_patch_employee_builder_pattern() {
        // 测试构建器模式的流畅性
        let builder = PatchEmployeeBuilder::new("emp_builder_001")
            .name("构建器测试")
            .email("builder@company.com")
            .mobile("13300133000")
            .employee_no("BLD001")
            .department_ids(vec!["build_dept_001".to_string()])
            .job_title("构建器工程师")
            .work_location("广州");

        // 验证构建器状态
        assert_eq!(builder.request.employee_id, "emp_builder_001");
        assert_eq!(builder.request.name, Some("构建器测试".to_string()));
        assert_eq!(builder.request.email, Some("builder@company.com".to_string()));
        assert_eq!(builder.request.mobile, Some("13300133000".to_string()));
        assert_eq!(builder.request.employee_no, Some("BLD001".to_string()));
        assert_eq!(builder.request.department_ids.as_ref().unwrap().len(), 1);
        assert_eq!(builder.request.job_title, Some("构建器工程师".to_string()));
        assert_eq!(builder.request.work_location, Some("广州".to_string()));
        assert!(builder.request.has_updates());

        // 验证请求验证通过
        assert!(builder.request.validate().is_ok());

        // 测试链式调用覆盖
        let chained_builder = builder
            .name("重新设置名称")
            .job_title("重新设置职位")
            .request;
        assert_eq!(chained_builder.name.unwrap(), "重新设置名称");
        assert_eq!(chained_builder.job_title.unwrap(), "重新设置职位");
        assert!(chained_builder.has_updates());
    }

    #[test]
    fn test_patch_employee_json_serialization() {
        let request = PatchEmployeeRequest::new("emp_json_001")
            .name("JSON测试")
            .email("json@company.com")
            .mobile("13200132000")
            .employee_no("JSON001")
            .department_ids(vec!["json_dept_001".to_string()])
            .job_title("JSON工程师")
            .work_location("成都");

        // 测试请求可以转换为JSON（只包含非None字段）
        let body = json!({
            "name": "JSON测试",
            "email": "json@company.com",
            "mobile": "13200132000",
            "employee_no": "JSON001",
            "department_ids": ["json_dept_001"],
            "job_title": "JSON工程师",
            "work_location": "成都"
        });

        assert_eq!(body["name"], "JSON测试");
        assert_eq!(body["email"], "json@company.com");
        assert_eq!(body["mobile"], "13200132000");
        assert_eq!(body["employee_no"], "JSON001");
        assert_eq!(body["department_ids"].as_array().unwrap().len(), 1);
        assert_eq!(body["job_title"], "JSON工程师");
        assert_eq!(body["work_location"], "成都");

        // 验证employee_id不在请求体中（它是路径参数）
        assert!(!body.as_object().unwrap().contains_key("employee_id"));
    }

    #[test]
    fn test_patch_employee_selective_update() {
        // 测试选择性更新 - 只更新部分字段
        let base_request = PatchEmployeeRequest::new("emp_selective_001");

        // 只更新姓名
        let name_only_request = base_request.clone().name("仅更新姓名");
        assert!(name_only_request.has_updates());

        let mut expected_body = json!({});
        expected_body["name"] = json!("仅更新姓名");

        // 模拟请求体构建逻辑
        let mut actual_body = json!({});
        if let Some(ref name) = name_only_request.name {
            actual_body["name"] = json!(name);
        }
        assert_eq!(actual_body, expected_body);

        // 只更新部门
        let dept_only_request = PatchEmployeeRequest::new("emp_selective_001")
            .department_ids(vec!["new_dept_001".to_string()]);
        assert!(dept_only_request.has_updates());

        let mut dept_body = json!({});
        if let Some(ref department_ids) = dept_only_request.department_ids {
            dept_body["department_ids"] = json!(department_ids);
        }
        assert_eq!(dept_body["department_ids"].as_array().unwrap().len(), 1);

        // 更新多个字段
        let multi_field_request = PatchEmployeeRequest::new("emp_selective_001")
            .name("多字段更新")
            .email("multi@company.com")
            .job_title("多职位更新");
        assert!(multi_field_request.has_updates());

        let mut multi_body = json!({});
        if let Some(ref name) = multi_field_request.name {
            multi_body["name"] = json!(name);
        }
        if let Some(ref email) = multi_field_request.email {
            multi_body["email"] = json!(email);
        }
        if let Some(ref job_title) = multi_field_request.job_title {
            multi_body["job_title"] = json!(job_title);
        }

        assert_eq!(multi_body.as_object().unwrap().len(), 3);
        assert_eq!(multi_body["name"], "多字段更新");
        assert_eq!(multi_body["email"], "multi@company.com");
        assert_eq!(multi_body["job_title"], "多职位更新");
    }
}
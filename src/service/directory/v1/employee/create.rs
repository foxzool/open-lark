//! Directory v1 创建员工API
//!
//! 提供企业员工创建功能，支持完整的员工信息录入和验证
//! 适用于企业人力资源管理系统的员工入职流程

use crate::core::{
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

/// 创建员工请求
#[derive(Debug, Clone)]
pub struct CreateEmployeeRequest {
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
    /// 入职时间
    pub join_time: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>,
}

impl CreateEmployeeRequest {
    /// 创建新的请求实例
    pub fn new() -> Self {
        Self {
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
            join_time: None,
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

    /// 设置入职时间
    pub fn join_time(mut self, join_time: impl Into<String>) -> Self {
        self.join_time = Some(join_time.into());
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
        // 姓名是必需的
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("姓名不能为空".to_string());
            }
            if name.len() > 50 {
                return Err("姓名长度不能超过50个字符".to_string());
            }
        } else {
            return Err("姓名是必填字段".to_string());
        }

        // 验证邮箱格式
        if let Some(ref email) = self.email {
            if !email.trim().is_empty() {
                // 简单的邮箱格式验证
                if !email.contains('@') || !email.contains('.') {
                    return Err("邮箱格式不正确".to_string());
                }
                if email.len() > 100 {
                    return Err("邮箱长度不能超过100个字符".to_string());
                }
            }
        }

        // 验证手机号格式
        if let Some(ref mobile) = self.mobile {
            if !mobile.trim().is_empty() {
                // 中国手机号格式验证（11位数字，以1开头）
                if !mobile.chars().all(|c| c.is_ascii_digit()) || mobile.len() != 11 || !mobile.starts_with('1') {
                    return Err("手机号格式不正确，请输入11位数字".to_string());
                }
            }
        }

        // 验证员工工号
        if let Some(ref employee_no) = self.employee_no {
            if !employee_no.trim().is_empty() {
                if employee_no.len() > 50 {
                    return Err("员工工号长度不能超过50个字符".to_string());
                }
            }
        }

        // 验证英文名
        if let Some(ref en_name) = self.en_name {
            if !en_name.trim().is_empty() {
                if en_name.len() > 50 {
                    return Err("英文名长度不能超过50个字符".to_string());
                }
            }
        }

        // 验证部门ID列表
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

        // 验证工作地点
        if let Some(ref work_location) = self.work_location {
            if !work_location.trim().is_empty() {
                if work_location.len() > 100 {
                    return Err("工作地点长度不能超过100个字符".to_string());
                }
            }
        }

        // 验证职级
        if let Some(ref job_level) = self.job_level {
            if !job_level.trim().is_empty() {
                if job_level.len() > 50 {
                    return Err("职级长度不能超过50个字符".to_string());
                }
            }
        }

        // 验证职位
        if let Some(ref job_title) = self.job_title {
            if !job_title.trim().is_empty() {
                if job_title.len() > 50 {
                    return Err("职位长度不能超过50个字符".to_string());
                }
            }
        }

        // 验证上级ID
        if let Some(ref leader_id) = self.leader_id {
            if !leader_id.trim().is_empty() {
                if leader_id.len() > 64 {
                    return Err("上级ID长度不能超过64个字符".to_string());
                }
            }
        }

        // 验证入职时间格式（简单验证ISO 8601格式）
        if let Some(ref join_time) = self.join_time {
            if !join_time.trim().is_empty() {
                // 验证格式：YYYY-MM-DD或YYYY-MM-DDTHH:MM:SSZ
                if !join_time.starts_with("20") && !join_time.starts_with("19") {
                    return Err("入职时间格式不正确，请使用YYYY-MM-DD格式".to_string());
                }
            }
        }

        Ok(())
    }
}

/// 创建员工响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployeeResponseData {
    /// 创建的员工信息
    pub employee: Employee,
}

/// 创建员工响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateEmployeeResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateEmployeeResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for CreateEmployeeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl EmployeeService {
    /// 创建员工
    ///
    /// 在企业中创建新员工，支持设置完整的员工信息
    /// 适用于员工入职场景，提供完善的数据验证和错误处理
    ///
    /// # 参数
    /// * `req` - 创建员工请求
    ///
    /// # 返回值
    /// 返回创建的员工信息
    ///
    /// # 示例
    /// ```rust
    /// let request = CreateEmployeeRequest::new()
    ///     .name("张三")
    ///     .email("zhangsan@company.com")
    ///     .mobile("13800138000")
    ///     .employee_no("EMP001")
    ///     .department_ids(vec!["dept_001".to_string()])
    ///     .job_title("软件工程师")
    ///     .user_id_type(UserIdType::OpenId)
    ///     .department_id_type(DepartmentIdType::DepartmentId);
    /// let response = service.create_employee(&request).await?;
    /// ```
    pub async fn create_employee(&self, req: &CreateEmployeeRequest) -> SDKResult<CreateEmployeeResponse> {
        req.validate().map_err(|e| crate::core::SDKError::InvalidParameter(e))?;
        log::debug!("开始创建员工: name={:?}", req.name);

        // 构建查询参数
        let mut query_params: HashMap<&str, String> = HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.to_string());
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.insert("department_id_type", department_id_type.to_string());
        }

        // 构建请求体
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
        if let Some(ref join_time) = req.join_time {
            body["join_time"] = json!(join_time);
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::DIRECTORY_V1_EMPLOYEES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            body: serde_json::to_vec(&body).unwrap_or_default(),
            ..Default::default()
        };

        let resp = Transport::<CreateEmployeeResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            log::info!("员工创建成功: name={:?}", req.name);
        } else {
            log::warn!("员工创建失败: name={:?}, error={:?}",
                req.name, response.error_message);
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 创建员工构建器
#[derive(Debug, Clone)]
pub struct CreateEmployeeBuilder {
    request: CreateEmployeeRequest,
}

impl CreateEmployeeBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            request: CreateEmployeeRequest::new(),
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

    /// 设置入职时间
    pub fn join_time(mut self, join_time: impl Into<String>) -> Self {
        self.request = self.request.join_time(join_time);
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

    /// 执行创建员工操作
    pub async fn execute(self, service: &EmployeeService) -> SDKResult<CreateEmployeeResponse> {
        service.create_employee(&self.request).await
    }
}

impl EmployeeService {
    /// 创建员工构建器
    pub fn create_employee_builder(&self) -> CreateEmployeeBuilder {
        CreateEmployeeBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    #[test]
    fn test_create_employee_request_creation() {
        let request = CreateEmployeeRequest::new();
        assert!(request.name.is_none());
        assert!(request.email.is_none());
        assert!(request.mobile.is_none());
        assert!(request.employee_no.is_none());
    }

    #[test]
    fn test_create_employee_request_with_fields() {
        let request = CreateEmployeeRequest::new()
            .name("张三")
            .email("zhangsan@company.com")
            .mobile("13800138000")
            .employee_no("EMP001")
            .job_title("软件工程师");

        assert_eq!(request.name, Some("张三".to_string()));
        assert_eq!(request.email, Some("zhangsan@company.com".to_string()));
        assert_eq!(request.mobile, Some("13800138000".to_string()));
        assert_eq!(request.employee_no, Some("EMP001".to_string()));
        assert_eq!(request.job_title, Some("软件工程师".to_string()));
    }

    #[test]
    fn test_create_employee_request_validation() {
        // 测试正常情况
        let valid_request = CreateEmployeeRequest::new()
            .name("李四")
            .email("lisi@company.com")
            .mobile("13900139000");
        assert!(valid_request.validate().is_ok());

        // 测试姓名为空
        let empty_name_request = CreateEmployeeRequest::new()
            .name("");
        assert!(empty_name_request.validate().is_err());

        // 测试无姓名
        let no_name_request = CreateEmployeeRequest::new();
        assert!(no_name_request.validate().is_err());

        // 测试姓名过长
        let long_name_request = CreateEmployeeRequest::new()
            .name(&"a".repeat(51));
        assert!(long_name_request.validate().is_err());

        // 测试无效邮箱格式
        let invalid_email_requests = vec![
            CreateEmployeeRequest::new().name("测试").email("invalid-email"),
            CreateEmployeeRequest::new().name("测试").email("test@"),
            CreateEmployeeRequest::new().name("测试").email("@company.com"),
            CreateEmployeeRequest::new().name("测试").email("test.company.com"),
        ];

        for request in invalid_email_requests {
            assert!(request.validate().is_err(), "邮箱格式验证失败");
        }

        // 测试邮箱过长
        let long_email_request = CreateEmployeeRequest::new()
            .name("测试")
            .email(&format!("{}@company.com", "a".repeat(90)));
        assert!(long_email_request.validate().is_err());

        // 测试无效手机号格式
        let invalid_mobile_requests = vec![
            CreateEmployeeRequest::new().name("测试").mobile("12345678901"),  // 不以1开头
            CreateEmployeeRequest::new().name("测试").mobile("1380013800"),    // 10位
            CreateEmployeeRequest::new().name("测试").mobile("138001380000"),  // 12位
            CreateEmployeeRequest::new().name("测试").mobile("1380013800a"),   // 包含字母
            CreateEmployeeRequest::new().name("测试").mobile("138-0013-8000"), // 包含特殊字符
        ];

        for request in invalid_mobile_requests {
            assert!(request.validate().is_err(), "手机号格式验证失败");
        }

        // 测试有效手机号
        let valid_mobile_request = CreateEmployeeRequest::new()
            .name("测试")
            .mobile("13800138000");
        assert!(valid_mobile_request.validate().is_ok());

        // 测试空部门ID列表
        let empty_dept_request = CreateEmployeeRequest::new()
            .name("测试")
            .department_ids(vec![]);
        assert!(empty_dept_request.validate().is_err());

        // 测试包含空字符串的部门ID列表
        let invalid_dept_request = CreateEmployeeRequest::new()
            .name("测试")
            .department_ids(vec!["dept_001".to_string(), "".to_string()]);
        assert!(invalid_dept_request.validate().is_err());

        // 测试部门ID过长
        let long_dept_request = CreateEmployeeRequest::new()
            .name("测试")
            .department_ids(vec![&"a".repeat(65)]);
        assert!(long_dept_request.validate().is_err());

        // 测试有效部门ID列表
        let valid_dept_request = CreateEmployeeRequest::new()
            .name("测试")
            .department_ids(vec!["dept_001".to_string(), "dept_002".to_string()]);
        assert!(valid_dept_request.validate().is_ok());

        // 测试员工工号过长
        let long_employee_no_request = CreateEmployeeRequest::new()
            .name("测试")
            .employee_no(&"a".repeat(51));
        assert!(long_employee_no_request.validate().is_err());

        // 测试英文名过长
        let long_en_name_request = CreateEmployeeRequest::new()
            .name("测试")
            .en_name(&"a".repeat(51));
        assert!(long_en_name_request.validate().is_err());

        // 测试工作地点过长
        let long_work_location_request = CreateEmployeeRequest::new()
            .name("测试")
            .work_location(&"a".repeat(101));
        assert!(long_work_location_request.validate().is_err());

        // 测试职级过长
        let long_job_level_request = CreateEmployeeRequest::new()
            .name("测试")
            .job_level(&"a".repeat(51));
        assert!(long_job_level_request.validate().is_err());

        // 测试职位过长
        let long_job_title_request = CreateEmployeeRequest::new()
            .name("测试")
            .job_title(&"a".repeat(51));
        assert!(long_job_title_request.validate().is_err());

        // 测试上级ID过长
        let long_leader_id_request = CreateEmployeeRequest::new()
            .name("测试")
            .leader_id(&"a".repeat(65));
        assert!(long_leader_id_request.validate().is_err());

        // 测试无效入职时间格式
        let invalid_join_time_requests = vec![
            CreateEmployeeRequest::new().name("测试").join_time("2023-13-01"),    // 无效月份
            CreateEmployeeRequest::new().name("测试").join_time("2023-02-30"),    // 无效日期
            CreateEmployeeRequest::new().name("测试").join_time("18-01-01"),       // 不以19或20开头
            CreateEmployeeRequest::new().name("测试").join_time("invalid-date"),
        ];

        for request in invalid_join_time_requests {
            assert!(request.validate().is_err(), "入职时间格式验证失败");
        }

        // 测试有效入职时间格式
        let valid_join_time_requests = vec![
            CreateEmployeeRequest::new().name("测试").join_time("2023-01-01"),
            CreateEmployeeRequest::new().name("测试").join_time("1999-12-31"),
            CreateEmployeeRequest::new().name("测试").join_time("2023-12-01T15:30:00Z"),
        ];

        for request in valid_join_time_requests {
            assert!(request.validate().is_ok(), "有效入职时间格式验证失败");
        }
    }

    #[test]
    fn test_create_employee_response_creation() {
        let employee = Employee {
            employee_id: Some("emp_001".to_string()),
            employee_no: Some("EMP001".to_string()),
            name: Some("张三".to_string()),
            en_name: Some("Zhang San".to_string()),
            email: Some("zhangsan@company.com".to_string()),
            mobile: Some("13800138000".to_string()),
            department_ids: Some(vec!["dept_001".to_string()]),
            status: None,
            join_time: Some("2023-12-01T09:00:00Z".to_string()),
            leave_time: None,
            work_location: Some("北京".to_string()),
            job_level: Some("P5".to_string()),
            job_title: Some("软件工程师".to_string()),
            leader_id: Some("manager_001".to_string()),
        };

        let response_data = CreateEmployeeResponseData {
            employee,
        };

        let response = CreateEmployeeResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().employee.name, Some("张三".to_string()));
        assert_eq!(response.data.as_ref().unwrap().employee.email, Some("zhangsan@company.com".to_string()));
    }

    #[test]
    fn test_create_employee_builder() {
        let builder = CreateEmployeeBuilder::new()
            .name("王五")
            .email("wangwu@company.com")
            .mobile("13700137000")
            .employee_no("EMP002")
            .job_title("产品经理")
            .work_location("上海");

        assert_eq!(builder.request.name, Some("王五".to_string()));
        assert_eq!(builder.request.email, Some("wangwu@company.com".to_string()));
        assert_eq!(builder.request.mobile, Some("13700137000".to_string()));
        assert_eq!(builder.request.employee_no, Some("EMP002".to_string()));
        assert_eq!(builder.request.job_title, Some("产品经理".to_string()));
        assert_eq!(builder.request.work_location, Some("上海".to_string()));
    }

    #[test]
    fn test_create_employee_builder_validation() {
        // 测试有效构建器
        let valid_builder = CreateEmployeeBuilder::new()
            .name("赵六")
            .email("zhaoliu@company.com")
            .mobile("13600136000")
            .department_ids(vec!["dept_001".to_string()]);
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = CreateEmployeeBuilder::new()
            .name("");
        assert!(invalid_builder.request.validate().is_err());

        // 测试无姓名构建器
        let no_name_builder = CreateEmployeeBuilder::new();
        assert!(no_name_builder.request.validate().is_err());
    }

    #[test]
    fn test_create_employee_service_method() {
        let config = Config::default();
        let service = EmployeeService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.create_employee_builder()
            .name("测试员工")
            .email("test@company.com");
        assert_eq!(builder.request.name, Some("测试员工".to_string()));
        assert_eq!(builder.request.email, Some("test@company.com".to_string()));
    }

    #[test]
    fn test_create_employee_endpoint_construction() {
        // 验证端点常量存在
        assert_eq!(
            crate::core::endpoints_original::Endpoints::DIRECTORY_V1_EMPLOYEES,
            "/open-apis/directory/v1/employees"
        );
    }

    #[test]
    fn test_create_employee_request_methods() {
        // 测试链式调用
        let request = CreateEmployeeRequest::new()
            .name("链式调用测试")
            .email("chain@company.com")
            .mobile("13500135000")
            .employee_no("CHAIN001")
            .job_title("测试工程师")
            .en_name("Chain Test")
            .work_location("深圳")
            .job_level("P4")
            .leader_id("leader_001")
            .join_time("2023-12-01");

        assert_eq!(request.name, Some("链式调用测试".to_string()));
        assert_eq!(request.email, Some("chain@company.com".to_string()));
        assert_eq!(request.mobile, Some("13500135000".to_string()));
        assert_eq!(request.employee_no, Some("CHAIN001".to_string()));
        assert_eq!(request.job_title, Some("测试工程师".to_string()));
        assert_eq!(request.en_name, Some("Chain Test".to_string()));
        assert_eq!(request.work_location, Some("深圳".to_string()));
        assert_eq!(request.job_level, Some("P4".to_string()));
        assert_eq!(request.leader_id, Some("leader_001".to_string()));
        assert_eq!(request.join_time, Some("2023-12-01".to_string()));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_employee_edge_cases() {
        // 测试最小长度姓名
        let min_name_request = CreateEmployeeRequest::new()
            .name("张");
        assert!(min_name_request.validate().is_ok());

        // 测试最大长度姓名
        let max_name_request = CreateEmployeeRequest::new()
            .name(&"中".repeat(50));
        assert!(max_name_request.validate().is_ok());

        // 测试边界手机号
        let boundary_mobile_requests = vec![
            ("10000000000", false), // 以1开头但第二位是0
            ("19999999999", true),  // 有效最大手机号
            ("11000000000", false), // 以1开头但第二位是1（特殊号码段）
            ("13800138000", true),  // 常见有效号码
        ];

        for (mobile, should_be_valid) in boundary_mobile_requests {
            let request = CreateEmployeeRequest::new()
                .name("测试")
                .mobile(mobile);

            if should_be_valid {
                assert!(request.validate().is_ok(), "手机号 {} 应该有效", mobile);
            } else {
                assert!(request.validate().is_err(), "手机号 {} 应该无效", mobile);
            }
        }

        // 测试空值字段
        let empty_fields_request = CreateEmployeeRequest::new()
            .name("测试")
            .email("")
            .mobile("")
            .employee_no("")
            .en_name("")
            .work_location("")
            .job_level("")
            .job_title("")
            .leader_id("")
            .join_time("");
        assert!(empty_fields_request.validate().is_ok());

        // 测试单字符字段
        let single_char_request = CreateEmployeeRequest::new()
            .name("测")
            .employee_no("1")
            .en_name("a")
            .work_location("京")
            .job_level("1")
            .job_title("工")
            .leader_id("l")
            .join_time("2023-01-01");
        assert!(single_char_request.validate().is_ok());
    }

    #[test]
    fn test_create_employee_response_trait() {
        assert_eq!(CreateEmployeeResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_employee_comprehensive_scenario() {
        // 测试完整的业务场景 - 创建一个完整的员工档案
        let comprehensive_request = CreateEmployeeRequest::new()
            .name("周八")
            .en_name("Zhou Ba")
            .email("zhouba@company.com")
            .mobile("13400134000")
            .employee_no("COM008")
            .department_ids(vec![
                "tech_dept_001".to_string(),
                "project_team_002".to_string()
            ])
            .work_location("杭州")
            .job_level("P6")
            .job_title("高级软件工程师")
            .leader_id("tech_lead_001")
            .join_time("2023-12-01T09:00:00Z")
            .user_id_type(UserIdType::OpenId)
            .department_id_type(DepartmentIdType::DepartmentId);

        assert!(comprehensive_request.validate().is_ok());
        assert_eq!(comprehensive_request.name, Some("周八".to_string()));
        assert_eq!(comprehensive_request.en_name, Some("Zhou Ba".to_string()));
        assert_eq!(comprehensive_request.email, Some("zhouba@company.com".to_string()));
        assert_eq!(comprehensive_request.mobile, Some("13400134000".to_string()));
        assert_eq!(comprehensive_request.employee_no, Some("COM008".to_string()));
        assert!(comprehensive_request.department_ids.is_some());
        assert_eq!(comprehensive_request.department_ids.as_ref().unwrap().len(), 2);
        assert_eq!(comprehensive_request.work_location, Some("杭州".to_string()));
        assert_eq!(comprehensive_request.job_level, Some("P6".to_string()));
        assert_eq!(comprehensive_request.job_title, Some("高级软件工程师".to_string()));
        assert_eq!(comprehensive_request.leader_id, Some("tech_lead_001".to_string()));
        assert_eq!(comprehensive_request.join_time, Some("2023-12-01T09:00:00Z".to_string()));
        assert!(comprehensive_request.user_id_type.is_some());
        assert!(comprehensive_request.department_id_type.is_some());

        // 模拟响应数据
        let employee = Employee {
            employee_id: Some("emp_comprehensive_008".to_string()),
            employee_no: Some("COM008".to_string()),
            name: Some("周八".to_string()),
            en_name: Some("Zhou Ba".to_string()),
            email: Some("zhouba@company.com".to_string()),
            mobile: Some("13400134000".to_string()),
            department_ids: Some(vec![
                "tech_dept_001".to_string(),
                "project_team_002".to_string()
            ]),
            status: None,
            join_time: Some("2023-12-01T09:00:00Z".to_string()),
            leave_time: None,
            work_location: Some("杭州".to_string()),
            job_level: Some("P6".to_string()),
            job_title: Some("高级软件工程师".to_string()),
            leader_id: Some("tech_lead_001".to_string()),
        };

        let response_data = CreateEmployeeResponseData {
            employee,
        };

        let response = CreateEmployeeResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());

        let created_employee = response.data.unwrap().employee;
        assert_eq!(created_employee.name, Some("周八".to_string()));
        assert_eq!(created_employee.employee_no, Some("COM008".to_string()));
        assert_eq!(created_employee.job_title, Some("高级软件工程师".to_string()));
        assert_eq!(created_employee.work_location, Some("杭州".to_string()));
    }

    #[test]
    fn test_create_employee_different_user_department_types() {
        // 测试不同的用户ID和部门ID类型组合
        let type_combinations = vec![
            (UserIdType::OpenId, DepartmentIdType::OpenDepartmentId),
            (UserIdType::UnionId, DepartmentIdType::OpenDepartmentId),
            (UserIdType::UserId, DepartmentIdType::DepartmentId),
            (UserIdType::OpenId, DepartmentIdType::DepartmentId),
        ];

        for (user_type, dept_type) in type_combinations {
            let request = CreateEmployeeRequest::new()
                .name("类型测试")
                .department_ids(vec!["dept_001".to_string()])
                .user_id_type(user_type.clone())
                .department_id_type(dept_type.clone());

            assert!(request.validate().is_ok(),
                "用户类型 {:?} + 部门类型 {:?} 组合应该有效",
                user_type, dept_type);
            assert!(request.user_id_type.unwrap() == user_type);
            assert!(request.department_id_type.unwrap() == dept_type);
        }
    }

    #[test]
    fn test_create_employee_error_scenarios() {
        // 测试失败响应
        let error_response = CreateEmployeeResponse {
            data: None,
            success: false,
            error_message: Some("员工已存在".to_string()),
            error_code: Some("EMPLOYEE_EXISTS".to_string()),
        };

        assert!(!error_response.success);
        assert!(error_response.data.is_none());
        assert_eq!(error_response.error_message, Some("员工已存在".to_string()));
        assert_eq!(error_response.error_code, Some("EMPLOYEE_EXISTS".to_string()));

        // 测试权限错误
        let permission_error_response = CreateEmployeeResponse {
            data: None,
            success: false,
            error_message: Some("没有创建员工权限".to_string()),
            error_code: Some("PERMISSION_DENIED".to_string()),
        };

        assert!(!permission_error_response.success);
        assert_eq!(permission_error_response.error_message, Some("没有创建员工权限".to_string()));
        assert_eq!(permission_error_response.error_code, Some("PERMISSION_DENIED".to_string()));

        // 测试部门不存在错误
        let dept_error_response = CreateEmployeeResponse {
            data: None,
            success: false,
            error_message: Some("指定部门不存在".to_string()),
            error_code: Some("DEPARTMENT_NOT_FOUND".to_string()),
        };

        assert!(!dept_error_response.success);
        assert_eq!(dept_error_response.error_message, Some("指定部门不存在".to_string()));
        assert_eq!(dept_error_response.error_code, Some("DEPARTMENT_NOT_FOUND".to_string()));
    }

    #[test]
    fn test_create_employee_builder_pattern() {
        // 测试构建器模式的流畅性
        let builder = CreateEmployeeBuilder::new()
            .name("构建器测试")
            .email("builder@company.com")
            .mobile("13300133000")
            .employee_no("BLD001")
            .department_ids(vec!["build_dept_001".to_string()])
            .job_title("构建器工程师")
            .work_location("广州");

        // 验证构建器状态
        assert_eq!(builder.request.name, Some("构建器测试".to_string()));
        assert_eq!(builder.request.email, Some("builder@company.com".to_string()));
        assert_eq!(builder.request.mobile, Some("13300133000".to_string()));
        assert_eq!(builder.request.employee_no, Some("BLD001".to_string()));
        assert_eq!(builder.request.department_ids.as_ref().unwrap().len(), 1);
        assert_eq!(builder.request.job_title, Some("构建器工程师".to_string()));
        assert_eq!(builder.request.work_location, Some("广州".to_string()));

        // 验证请求验证通过
        assert!(builder.request.validate().is_ok());

        // 测试链式调用覆盖
        let chained_builder = builder
            .name("重新设置名称")
            .job_title("重新设置职位")
            .request;
        assert_eq!(chained_builder.name.unwrap(), "重新设置名称");
        assert_eq!(chained_builder.job_title.unwrap(), "重新设置职位");
    }

    #[test]
    fn test_create_employee_json_serialization() {
        let request = CreateEmployeeRequest::new()
            .name("JSON测试")
            .email("json@company.com")
            .mobile("13200132000")
            .employee_no("JSON001")
            .department_ids(vec!["json_dept_001".to_string()])
            .job_title("JSON工程师")
            .work_location("成都");

        // 测试请求可以转换为JSON
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
    }
}
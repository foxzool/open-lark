use crate::config::Config;
use crate::error::SDKError;
use crate::response::SDKResult;
use crate::service_trait::Service;
use crate::transport::Transport;
use crate::service::directory::v1::employee::regular::{EmployeeService, ENDPOINT_RESURRECT};
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 恢复离职员工请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResurrectEmployeeRequest {
    /// 恢复时间，Unix 时间戳（毫秒）
    /// 员工恢复的时间，用于记录员工重新入职的时间点
    pub restore_time: u64,
    /// 恢复原因
    /// 1: 重新聘用
    /// 2: 误操作恢复
    /// 3: 其他原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_reason: Option<i32>,
    /// 恢复备注
    /// 恢复原因的详细说明，帮助HR了解员工恢复的具体情况
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_remark: Option<String>,
    /// 上级ID
    /// 恢复后的直属上级，用于组织架构调整
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_id: Option<String>,
    /// 部门ID列表
    /// 恢复后所属的部门，支持多部门分配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 职位
    /// 恢复后的职位名称，用于角色定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 工作地点
    /// 恢复后的工作地点，用于办公地点管理
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
}

impl ResurrectEmployeeRequest {
    /// 创建新的恢复员工请求
    ///
    /// # 参数
    /// * `restore_time` - 恢复时间（Unix时间戳，毫秒）
    pub fn new(restore_time: u64) -> Self {
        Self {
            restore_time,
            restore_reason: None,
            restore_remark: None,
            leader_id: None,
            department_ids: None,
            job_title: None,
            work_location: None,
        }
    }

    /// 创建恢复员工请求的构建器
    pub fn builder() -> ResurrectEmployeeBuilder {
        ResurrectEmployeeBuilder::default()
    }
}

/// 恢复员工请求构建器
///
/// 提供流畅的API来构建恢复员工请求，支持方法链调用
#[derive(Debug, Clone, Default)]
pub struct ResurrectEmployeeBuilder {
    restore_time: u64,
    restore_reason: Option<i32>,
    restore_remark: Option<String>,
    leader_id: Option<String>,
    department_ids: Option<Vec<String>>,
    job_title: Option<String>,
    work_location: Option<String>,
}

impl ResurrectEmployeeBuilder {
    /// 设置恢复时间
    ///
    /// # 参数
    /// * `restore_time` - 恢复时间（Unix时间戳，毫秒）
    ///
    /// # 示例
    /// ```rust
    /// let builder = ResurrectEmployeeBuilder::default()
    ///     .restore_time(1704067200000); // 2024-01-01 00:00:00
    /// ```
    pub fn restore_time(mut self, restore_time: u64) -> Self {
        self.restore_time = restore_time;
        self
    }

    /// 设置恢复原因
    ///
    /// # 参数
    /// * `restore_reason` - 恢复原因（1: 重新聘用，2: 误操作恢复，3: 其他原因）
    ///
    /// # 示例
    /// ```rust
    /// let builder = ResurrectEmployeeBuilder::default()
    ///     .restore_reason(1); // 重新聘用
    /// ```
    pub fn restore_reason(mut self, restore_reason: i32) -> Self {
        self.restore_reason = Some(restore_reason);
        self
    }

    /// 设置恢复备注
    ///
    /// # 参数
    /// * `restore_remark` - 恢复原因的详细说明
    ///
    /// # 示例
    /// ```rust
    /// let builder = ResurrectEmployeeBuilder::default()
    ///     .restore_remark("员工重新加入公司，担任高级工程师职位");
    /// ```
    pub fn restore_remark(mut self, restore_remark: impl Into<String>) -> Self {
        self.restore_remark = Some(restore_remark.into());
        self
    }

    /// 设置上级ID
    ///
    /// # 参数
    /// * `leader_id` - 直属上级的员工ID
    ///
    /// # 示例
    /// ```rust
    /// let builder = ResurrectEmployeeBuilder::default()
    ///     .leader_id("manager_123456789");
    /// ```
    pub fn leader_id(mut self, leader_id: impl Into<String>) -> Self {
        self.leader_id = Some(leader_id.into());
        self
    }

    /// 设置部门ID列表
    ///
    /// # 参数
    /// * `department_ids` - 部门ID列表，支持多部门分配
    ///
    /// # 示例
    /// ```rust
    /// let builder = ResurrectEmployeeBuilder::default()
    ///     .department_ids(vec!["dept_001".to_string(), "dept_002".to_string()]);
    /// ```
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = Some(department_ids);
        self
    }

    /// 设置职位
    ///
    /// # 参数
    /// * `job_title` - 职位名称
    ///
    /// # 示例
    /// ```rust
    /// let builder = ResurrectEmployeeBuilder::default()
    ///     .job_title("高级工程师");
    /// ```
    pub fn job_title(mut self, job_title: impl Into<String>) -> Self {
        self.job_title = Some(job_title.into());
        self
    }

    /// 设置工作地点
    ///
    /// # 参数
    /// * `work_location` - 工作地点
    ///
    /// # 示例
    /// ```rust
    /// let builder = ResurrectEmployeeBuilder::default()
    ///     .work_location("北京办公室");
    /// ```
    pub fn work_location(mut self, work_location: impl Into<String>) -> Self {
        self.work_location = Some(work_location.into());
        self
    }

    /// 构建恢复员工请求
    ///
    /// # 返回
    /// 成功返回恢复员工请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果恢复时间为0，返回错误
    /// * 如果恢复时间超出合理范围，返回警告但允许继续
    /// * 如果恢复原因不在有效范围内，返回错误
    /// * 如果部门ID列表为空，返回错误
    pub fn build(self) -> SDKResult<ResurrectEmployeeRequest> {
        // 验证恢复时间
        if self.restore_time == 0 {
            return Err(SDKError::ValidationError("恢复时间不能为空".to_string()));
        }

        // 验证恢复时间是否在合理范围内（2020-2050年）
        let min_time = 1577836800000; // 2020-01-01 00:00:00
        let max_time = 2524608000000; // 2050-01-01 00:00:00
        if self.restore_time < min_time {
            return Err(SDKError::ValidationError("恢复时间过早".to_string()));
        }
        if self.restore_time > max_time {
            return Err(SDKError::ValidationError("恢复时间过晚".to_string()));
        }

        // 验证恢复原因
        if let Some(reason) = self.restore_reason {
            if reason < 1 || reason > 3 {
                return Err(SDKError::ValidationError(
                    "恢复原因必须为1（重新聘用）、2（误操作恢复）或3（其他原因）".to_string(),
                ));
            }
        }

        // 验证恢复备注长度
        if let Some(remark) = &self.restore_remark {
            if remark.len() > 500 {
                return Err(SDKError::ValidationError(
                    "恢复备注不能超过500个字符".to_string(),
                ));
            }
        }

        // 验证部门ID列表
        if let Some(dept_ids) = &self.department_ids {
            if dept_ids.is_empty() {
                return Err(SDKError::ValidationError(
                    "部门ID列表不能为空".to_string(),
                ));
            }
            if dept_ids.len() > 10 {
                return Err(SDKError::ValidationError(
                    "部门数量不能超过10个".to_string(),
                ));
            }
        }

        // 验证职位长度
        if let Some(title) = &self.job_title {
            if title.len() > 100 {
                return Err(SDKError::ValidationError(
                    "职位名称不能超过100个字符".to_string(),
                ));
            }
        }

        // 验证工作地点长度
        if let Some(location) = &self.work_location {
            if location.len() > 200 {
                return Err(SDKError::ValidationError(
                    "工作地点不能超过200个字符".to_string(),
                ));
            }
        }

        Ok(ResurrectEmployeeRequest {
            restore_time: self.restore_time,
            restore_reason: self.restore_reason,
            restore_remark: self.restore_remark,
            leader_id: self.leader_id,
            department_ids: self.department_ids,
            job_title: self.job_title,
            work_location: self.work_location,
        })
    }
}

/// 恢复离职员工响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResurrectEmployeeResponse {
    /// 操作是否成功
    /// true表示员工成功恢复，false表示操作失败
    pub success: bool,
    /// 员工ID
    /// 被恢复的员工ID，用于确认操作的对象
    pub employee_id: String,
    /// 恢复时间
    /// 员工的实际恢复时间，用于记录员工状态变更的时间点
    pub restore_time: u64,
    /// 操作时间
    /// 系统处理恢复请求的时间，用于审计和追踪
    pub operation_time: u64,
    /// 恢复后的员工状态
    /// 员工恢复后的状态信息，如在职、待入职等
    pub employee_status: String,
    /// 部门信息
    /// 恢复后员工所属的部门信息
    pub department_info: Option<String>,
}

impl ResurrectEmployeeResponse {
    /// 创建新的恢复员工响应
    ///
    /// # 参数
    /// * `employee_id` - 员工ID
    /// * `restore_time` - 恢复时间
    /// * `operation_time` - 操作时间
    /// * `employee_status` - 员工状态
    pub fn new(
        employee_id: String,
        restore_time: u64,
        operation_time: u64,
        employee_status: String,
    ) -> Self {
        Self {
            success: true,
            employee_id,
            restore_time,
            operation_time,
            employee_status,
            department_info: None,
        }
    }

    /// 检查操作是否成功
    ///
    /// # 返回
    /// true表示员工成功恢复
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// 获取员工ID
    ///
    /// # 返回
    /// 员工ID字符串
    pub fn employee_id(&self) -> &str {
        &self.employee_id
    }

    /// 获取恢复时间
    ///
    /// # 返回
    /// 恢复时间（Unix时间戳，毫秒）
    pub fn restore_time(&self) -> u64 {
        self.restore_time
    }

    /// 获取操作时间
    ///
    /// # 返回
    /// 操作时间（Unix时间戳，毫秒）
    pub fn operation_time(&self) -> u64 {
        self.operation_time
    }

    /// 获取员工状态
    ///
    /// # 返回
    /// 员工状态字符串
    pub fn employee_status(&self) -> &str {
        &self.employee_status
    }

    /// 获取部门信息
    ///
    /// # 返回
    /// 部门信息字符串
    pub fn department_info(&self) -> Option<&str> {
        self.department_info.as_deref()
    }

    /// 设置部门信息
    ///
    /// # 参数
    /// * `department_info` - 部门信息
    pub fn set_department_info(&mut self, department_info: String) {
        self.department_info = Some(department_info);
    }
}

/// 恢复员工构建器
///
/// 提供流畅的API来恢复员工，支持方法链调用和完整的错误处理
#[derive(Debug, Clone)]
pub struct ResurrectEmployeeBuilder {
    service: Arc<EmployeeService>,
    employee_id: String,
    request: ResurrectEmployeeRequest,
}

impl ResurrectEmployeeBuilder {
    /// 创建新的恢复员工构建器
    ///
    /// # 参数
    /// * `service` - 员工服务实例
    /// * `employee_id` - 员工ID
    /// * `request` - 恢复员工请求
    pub(crate) fn new(
        service: Arc<EmployeeService>,
        employee_id: String,
        request: ResurrectEmployeeRequest,
    ) -> Self {
        Self {
            service,
            employee_id,
            request,
        }
    }

    /// 执行恢复员工操作
    ///
    /// 向飞书API发送POST请求来恢复指定员工
    ///
    /// # 返回
    /// * `Ok(ResurrectEmployeeResponse)` - 恢复成功，返回恢复结果
    /// * `Err(SDKError)` - 恢复失败，返回错误信息
    ///
    /// # 错误类型
    /// * `SDKError::NetworkError` - 网络请求失败
    /// * `SDKError::ApiError` - API调用失败，包含错误码和消息
    /// * `SDKError::SerializationError` - 响应序列化失败
    /// * `SDKError::AuthenticationError` - 身份验证失败
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::directory::v1::employee::resurrect::{ResurrectEmployeeRequest, ResurrectEmployeeResponse};
    ///
    /// async fn resurrect_employee_example(
    ///     service: Arc<EmployeeService>,
    ///     employee_id: &str
    /// ) -> Result<ResurrectEmployeeResponse, Box<dyn std::error::Error>> {
    ///     let request = ResurrectEmployeeRequest::builder()
    ///         .restore_time(1704067200000) // 2024-01-01 00:00:00
    ///         .restore_reason(1) // 重新聘用
    ///         .restore_remark("员工重新加入公司，担任高级工程师职位")
    ///         .leader_id("manager_123456789")
    ///         .department_ids(vec!["dept_001".to_string()])
    ///         .job_title("高级工程师")
    ///         .work_location("北京办公室")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .resurrect_employee_builder(&employee_id, request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("员工 {} 恢复成功，操作时间: {}", response.employee_id(), response.operation_time());
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<ResurrectEmployeeResponse> {
        let endpoint = ENDPOINT_RESURRECT.replace("{employee_id}", &self.employee_id);
        let url = self.service.config().build_url(&endpoint);

        // 构建请求体
        let body = serde_json::to_value(&self.request)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        // 发送HTTP POST请求
        let response = self
            .service
            .transport()
            .post(&url, Some(body))
            .await?;

        // 解析响应体
        let response_data: ResurrectEmployeeResponse = serde_json::from_value(response)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        Ok(response_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::transport::MockTransport;
    use std::sync::Arc;

    fn create_test_service() -> Arc<EmployeeService> {
        let config = Config::new("test_app_id", "test_app_secret");
        let transport = MockTransport::new();
        Arc::new(EmployeeService::new_with_transport(config, transport))
    }

    #[test]
    fn test_resurrect_employee_request_builder() {
        // 测试正常构建
        let request = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .restore_reason(1)
            .restore_remark("测试恢复备注")
            .leader_id("manager_123")
            .department_ids(vec!["dept_001".to_string(), "dept_002".to_string()])
            .job_title("高级工程师")
            .work_location("北京办公室")
            .build()
            .unwrap();

        assert_eq!(request.restore_time, 1704067200000);
        assert_eq!(request.restore_reason, Some(1));
        assert_eq!(request.restore_remark, Some("测试恢复备注".to_string()));
        assert_eq!(request.leader_id, Some("manager_123".to_string()));
        assert_eq!(request.department_ids, Some(vec!["dept_001".to_string(), "dept_002".to_string()]));
        assert_eq!(request.job_title, Some("高级工程师".to_string()));
        assert_eq!(request.work_location, Some("北京办公室".to_string()));
    }

    #[test]
    fn test_resurrect_employee_request_builder_only_required() {
        let request = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .build()
            .unwrap();

        assert_eq!(request.restore_time, 1704067200000);
        assert_eq!(request.restore_reason, None);
        assert_eq!(request.restore_remark, None);
        assert_eq!(request.leader_id, None);
        assert_eq!(request.department_ids, None);
        assert_eq!(request.job_title, None);
        assert_eq!(request.work_location, None);
    }

    #[test]
    fn test_resurrect_employee_request_validation() {
        // 测试空恢复时间
        let result = ResurrectEmployeeRequest::builder().build();
        assert!(result.is_err());

        // 测试恢复时间过早
        let result = ResurrectEmployeeRequest::builder()
            .restore_time(1000000000000) // 2001年
            .build();
        assert!(result.is_err());

        // 测试恢复时间过晚
        let result = ResurrectEmployeeRequest::builder()
            .restore_time(3000000000000) // 2065年
            .build();
        assert!(result.is_err());

        // 测试无效恢复原因
        let result = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .restore_reason(5) // 无效原因
            .build();
        assert!(result.is_err());

        // 测试恢复备注过长
        let long_remark = "a".repeat(501);
        let result = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .restore_remark(long_remark)
            .build();
        assert!(result.is_err());

        // 测试空部门ID列表
        let result = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .department_ids(vec![])
            .build();
        assert!(result.is_err());

        // 测试部门数量过多
        let many_depts = (0..11).map(|i| format!("dept_{:03}", i)).collect();
        let result = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .department_ids(many_depts)
            .build();
        assert!(result.is_err());

        // 测试职位名称过长
        let long_title = "a".repeat(101);
        let result = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .job_title(long_title)
            .build();
        assert!(result.is_err());

        // 测试工作地点过长
        let long_location = "a".repeat(201);
        let result = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .work_location(long_location)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_resurrect_employee_request_valid_restore_reasons() {
        let valid_reasons = [1, 2, 3];

        for reason in valid_reasons.iter() {
            let request = ResurrectEmployeeRequest::builder()
                .restore_time(1704067200000)
                .restore_reason(*reason)
                .build()
                .unwrap();

            assert_eq!(request.restore_reason, Some(*reason));
        }
    }

    #[test]
    fn test_resurrect_employee_request_edge_cases() {
        // 测试最小有效时间
        let request = ResurrectEmployeeRequest::builder()
            .restore_time(1577836800000) // 2020-01-01 00:00:00
            .build();
        assert!(request.is_ok());

        // 测试最大有效时间
        let request = ResurrectEmployeeRequest::builder()
            .restore_time(2524608000000) // 2050-01-01 00:00:00
            .build();
        assert!(request.is_ok());

        // 测试空字符串恢复备注
        let request = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .restore_remark("")
            .build();
        assert!(request.is_ok());
        assert_eq!(request.unwrap().restore_remark, Some("".to_string()));

        // 测试最大部门数量
        let max_depts = (0..10).map(|i| format!("dept_{:03}", i)).collect();
        let request = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .department_ids(max_depts)
            .build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_resurrect_employee_response() {
        let mut response = ResurrectEmployeeResponse::new(
            "test_employee_id".to_string(),
            1704067200000,
            1704067201000,
            "active".to_string(),
        );

        assert!(response.is_success());
        assert_eq!(response.employee_id(), "test_employee_id");
        assert_eq!(response.restore_time(), 1704067200000);
        assert_eq!(response.operation_time(), 1704067201000);
        assert_eq!(response.employee_status(), "active");
        assert_eq!(response.department_info(), None);

        // 测试设置部门信息
        response.set_department_info("技术部".to_string());
        assert_eq!(response.department_info(), Some("技术部"));
    }

    #[tokio::test]
    async fn test_resurrect_employee_builder() {
        let service = create_test_service();
        let request = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .restore_reason(1)
            .restore_remark("测试恢复员工")
            .build()
            .unwrap();

        let builder = service.resurrect_employee_builder("test_employee_id", request);
        assert_eq!(builder.employee_id, "test_employee_id");
        assert_eq!(builder.request.restore_time, 1704067200000);
    }

    #[test]
    fn test_resurrect_employee_request_new() {
        let request = ResurrectEmployeeRequest::new(1704067200000);
        assert_eq!(request.restore_time, 1704067200000);
        assert_eq!(request.restore_reason, None);
        assert_eq!(request.restore_remark, None);
        assert_eq!(request.leader_id, None);
        assert_eq!(request.department_ids, None);
        assert_eq!(request.job_title, None);
        assert_eq!(request.work_location, None);
    }

    #[test]
    fn test_resurrect_employee_request_serialization() {
        let request = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .restore_reason(2)
            .restore_remark("误操作恢复")
            .leader_id("manager_123")
            .department_ids(vec!["dept_001".to_string()])
            .job_title("工程师")
            .work_location("上海")
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: ResurrectEmployeeRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.restore_time, request.restore_time);
        assert_eq!(parsed.restore_reason, request.restore_reason);
        assert_eq!(parsed.restore_remark, request.restore_remark);
        assert_eq!(parsed.leader_id, request.leader_id);
        assert_eq!(parsed.department_ids, request.department_ids);
        assert_eq!(parsed.job_title, request.job_title);
        assert_eq!(parsed.work_location, request.work_location);
    }

    #[test]
    fn test_resurrect_employee_response_serialization() {
        let mut response = ResurrectEmployeeResponse::new(
            "emp_123".to_string(),
            1704067200000,
            1704067201000,
            "active".to_string(),
        );
        response.set_department_info("技术部".to_string());

        let json = serde_json::to_string(&response).unwrap();
        let parsed: ResurrectEmployeeResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.employee_id, response.employee_id);
        assert_eq!(parsed.restore_time, response.restore_time);
        assert_eq!(parsed.operation_time, response.operation_time);
        assert_eq!(parsed.success, response.success);
        assert_eq!(parsed.employee_status, response.employee_status);
        assert_eq!(parsed.department_info, response.department_info);
    }

    #[test]
    fn test_multiple_valid_requests() {
        let test_cases = vec![
            (1704067200000, Some(1), Some("重新聘用".to_string())),
            (1704067200000, Some(2), Some("误操作恢复".to_string())),
            (1704067200000, Some(3), Some("其他原因".to_string())),
            (1704067200000, None, None),
            (1704067200000, Some(1), None),
            (1704067200000, None, Some("特殊情况恢复".to_string())),
        ];

        for (restore_time, restore_reason, restore_remark) in test_cases {
            let mut builder = ResurrectEmployeeRequest::builder().restore_time(restore_time);

            if let Some(reason) = restore_reason {
                builder = builder.restore_reason(reason);
            }

            if let Some(remark) = restore_remark {
                builder = builder.restore_remark(remark);
            }

            let request = builder.build().unwrap();
            assert_eq!(request.restore_time, restore_time);
            assert_eq!(request.restore_reason, restore_reason);
            assert_eq!(request.restore_remark, restore_remark);
        }
    }

    #[test]
    fn test_employee_id_validation() {
        // 测试不同的员工ID格式
        let valid_employee_ids = vec![
            "emp_123456789",
            "open_123456789",
            "user_123456789",
            "employee_test_001",
        ];

        for employee_id in valid_employee_ids {
            let service = create_test_service();
            let request = ResurrectEmployeeRequest::builder()
                .restore_time(1704067200000)
                .build()
                .unwrap();

            let builder = service.resurrect_employee_builder(employee_id, request);
            assert_eq!(builder.employee_id, employee_id);
        }
    }

    #[test]
    fn test_complex_restore_request() {
        // 测试包含所有字段的复杂恢复请求
        let request = ResurrectEmployeeRequest::builder()
            .restore_time(1704067200000)
            .restore_reason(1)
            .restore_remark("员工因个人原因离职，现重新加入公司担任更高级职位")
            .leader_id("manager_123456789")
            .department_ids(vec![
                "dept_001".to_string(),
                "dept_002".to_string(),
                "dept_003".to_string(),
            ])
            .job_title("高级技术专家")
            .work_location("深圳总部办公室")
            .build()
            .unwrap();

        // 验证所有字段都正确设置
        assert_eq!(request.restore_time, 1704067200000);
        assert_eq!(request.restore_reason, Some(1));
        assert_eq!(request.restore_remark, Some("员工因个人原因离职，现重新加入公司担任更高级职位".to_string()));
        assert_eq!(request.leader_id, Some("manager_123456789".to_string()));
        assert_eq!(request.department_ids, Some(vec![
            "dept_001".to_string(),
            "dept_002".to_string(),
            "dept_003".to_string(),
        ]));
        assert_eq!(request.job_title, Some("高级技术专家".to_string()));
        assert_eq!(request.work_location, Some("深圳总部办公室".to_string()));
    }
}
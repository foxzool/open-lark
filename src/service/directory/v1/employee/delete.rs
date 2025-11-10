use crate::config::Config;
use crate::error::SDKError;
use crate::response::SDKResult;
use crate::service_trait::Service;
use crate::transport::Transport;
use crate::service::directory::v1::employee::regular::{EmployeeService, ENDPOINT_DELETE};
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 删除员工请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEmployeeRequest {
    /// 离职时间，Unix 时间戳（毫秒）
    /// 员工离职的时间，用于记录员工的离职时间点
    pub leave_time: u64,
    /// 离职原因
    /// 1: 个人原因
    /// 2: 公司原因
    /// 3: 其他原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_reason: Option<i32>,
    /// 离职备注
    /// 离职原因的详细说明，帮助HR了解员工离职的具体情况
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_remark: Option<String>,
}

impl DeleteEmployeeRequest {
    /// 创建新的删除员工请求
    ///
    /// # 参数
    /// * `leave_time` - 离职时间（Unix时间戳，毫秒）
    pub fn new(leave_time: u64) -> Self {
        Self {
            leave_time,
            leave_reason: None,
            leave_remark: None,
        }
    }

    /// 创建删除员工请求的构建器
    pub fn builder() -> DeleteEmployeeBuilder {
        DeleteEmployeeBuilder::default()
    }
}

/// 删除员工请求构建器
///
/// 提供流畅的API来构建删除员工请求，支持方法链调用
#[derive(Debug, Clone, Default)]
pub struct DeleteEmployeeBuilder {
    leave_time: u64,
    leave_reason: Option<i32>,
    leave_remark: Option<String>,
}

impl DeleteEmployeeBuilder {
    /// 设置离职时间
    ///
    /// # 参数
    /// * `leave_time` - 离职时间（Unix时间戳，毫秒）
    ///
    /// # 示例
    /// ```rust
    /// let builder = DeleteEmployeeBuilder::default()
    ///     .leave_time(1704067200000); // 2024-01-01 00:00:00
    /// ```
    pub fn leave_time(mut self, leave_time: u64) -> Self {
        self.leave_time = leave_time;
        self
    }

    /// 设置离职原因
    ///
    /// # 参数
    /// * `leave_reason` - 离职原因（1: 个人原因，2: 公司原因，3: 其他原因）
    ///
    /// # 示例
    /// ```rust
    /// let builder = DeleteEmployeeBuilder::default()
    ///     .leave_reason(1); // 个人原因
    /// ```
    pub fn leave_reason(mut self, leave_reason: i32) -> Self {
        self.leave_reason = Some(leave_reason);
        self
    }

    /// 设置离职备注
    ///
    /// # 参数
    /// * `leave_remark` - 离职原因的详细说明
    ///
    /// # 示例
    /// ```rust
    /// let builder = DeleteEmployeeBuilder::default()
    ///     .leave_remark("寻求更好的职业发展机会");
    /// ```
    pub fn leave_remark(mut self, leave_remark: impl Into<String>) -> Self {
        self.leave_remark = Some(leave_remark.into());
        self
    }

    /// 构建删除员工请求
    ///
    /// # 返回
    /// 成功返回删除员工请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果离职时间为0，返回错误
    /// * 如果离职时间超出合理范围，返回警告但允许继续
    /// * 如果离职原因不在有效范围内，返回错误
    pub fn build(self) -> SDKResult<DeleteEmployeeRequest> {
        // 验证离职时间
        if self.leave_time == 0 {
            return Err(SDKError::ValidationError("离职时间不能为空".to_string()));
        }

        // 验证离职时间是否在合理范围内（2020-2050年）
        let min_time = 1577836800000; // 2020-01-01 00:00:00
        let max_time = 2524608000000; // 2050-01-01 00:00:00
        if self.leave_time < min_time {
            return Err(SDKError::ValidationError("离职时间过早".to_string()));
        }
        if self.leave_time > max_time {
            return Err(SDKError::ValidationError("离职时间过晚".to_string()));
        }

        // 验证离职原因
        if let Some(reason) = self.leave_reason {
            if reason < 1 || reason > 3 {
                return Err(SDKError::ValidationError(
                    "离职原因必须为1（个人原因）、2（公司原因）或3（其他原因）".to_string(),
                ));
            }
        }

        // 验证离职备注长度
        if let Some(remark) = &self.leave_remark {
            if remark.len() > 500 {
                return Err(SDKError::ValidationError(
                    "离职备注不能超过500个字符".to_string(),
                ));
            }
        }

        Ok(DeleteEmployeeRequest {
            leave_time: self.leave_time,
            leave_reason: self.leave_reason,
            leave_remark: self.leave_remark,
        })
    }
}

/// 删除员工响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteEmployeeResponse {
    /// 操作是否成功
    /// true表示员工成功离职，false表示操作失败
    pub success: bool,
    /// 员工ID
    /// 被删除的员工ID，用于确认操作的对象
    pub employee_id: String,
    /// 离职时间
    /// 员工的实际离职时间，用于记录员工状态变更的时间点
    pub leave_time: u64,
    /// 操作时间
    /// 系统处理删除请求的时间，用于审计和追踪
    pub operation_time: u64,
}

impl DeleteEmployeeResponse {
    /// 创建新的删除员工响应
    ///
    /// # 参数
    /// * `employee_id` - 员工ID
    /// * `leave_time` - 离职时间
    /// * `operation_time` - 操作时间
    pub fn new(employee_id: String, leave_time: u64, operation_time: u64) -> Self {
        Self {
            success: true,
            employee_id,
            leave_time,
            operation_time,
        }
    }

    /// 检查操作是否成功
    ///
    /// # 返回
    /// true表示员工成功离职
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

    /// 获取离职时间
    ///
    /// # 返回
    /// 离职时间（Unix时间戳，毫秒）
    pub fn leave_time(&self) -> u64 {
        self.leave_time
    }

    /// 获取操作时间
    ///
    /// # 返回
    /// 操作时间（Unix时间戳，毫秒）
    pub fn operation_time(&self) -> u64 {
        self.operation_time
    }
}

/// 删除员工构建器
///
/// 提供流畅的API来删除员工，支持方法链调用和完整的错误处理
#[derive(Debug, Clone)]
pub struct DeleteEmployeeBuilder {
    service: Arc<EmployeeService>,
    employee_id: String,
    request: DeleteEmployeeRequest,
}

impl DeleteEmployeeBuilder {
    /// 创建新的删除员工构建器
    ///
    /// # 参数
    /// * `service` - 员工服务实例
    /// * `employee_id` - 员工ID
    /// * `request` - 删除员工请求
    pub(crate) fn new(
        service: Arc<EmployeeService>,
        employee_id: String,
        request: DeleteEmployeeRequest,
    ) -> Self {
        Self {
            service,
            employee_id,
            request,
        }
    }

    /// 执行删除员工操作
    ///
    /// 向飞书API发送DELETE请求来删除指定员工
    ///
    /// # 返回
    /// * `Ok(DeleteEmployeeResponse)` - 删除成功，返回删除结果
    /// * `Err(SDKError)` - 删除失败，返回错误信息
    ///
    /// # 错误类型
    /// * `SDKError::NetworkError` - 网络请求失败
    /// * `SDKError::ApiError` - API调用失败，包含错误码和消息
    /// * `SDKError::SerializationError` - 响应序列化失败
    /// * `SDKError::AuthenticationError` - 身份验证失败
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::directory::v1::employee::delete::{DeleteEmployeeRequest, DeleteEmployeeResponse};
    ///
    /// async fn delete_employee_example(
    ///     service: Arc<EmployeeService>,
    ///     employee_id: &str
    /// ) -> Result<DeleteEmployeeResponse, Box<dyn std::error::Error>> {
    ///     let request = DeleteEmployeeRequest::builder()
    ///         .leave_time(1704067200000) // 2024-01-01 00:00:00
    ///         .leave_reason(1) // 个人原因
    ///         .leave_remark("寻求更好的职业发展机会")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .delete_employee_builder(&employee_id, request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("员工 {} 删除成功，操作时间: {}", response.employee_id(), response.operation_time());
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<DeleteEmployeeResponse> {
        let endpoint = ENDPOINT_DELETE.replace("{employee_id}", &self.employee_id);
        let url = self.service.config().build_url(&endpoint);

        // 构建请求体
        let body = serde_json::to_value(&self.request)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        // 发送HTTP DELETE请求
        let response = self
            .service
            .transport()
            .delete(&url, Some(body))
            .await?;

        // 解析响应体
        let response_data: DeleteEmployeeResponse = serde_json::from_value(response)
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
    fn test_delete_employee_request_builder() {
        // 测试正常构建
        let request = DeleteEmployeeRequest::builder()
            .leave_time(1704067200000)
            .leave_reason(1)
            .leave_remark("测试离职备注")
            .build()
            .unwrap();

        assert_eq!(request.leave_time, 1704067200000);
        assert_eq!(request.leave_reason, Some(1));
        assert_eq!(request.leave_remark, Some("测试离职备注".to_string()));
    }

    #[test]
    fn test_delete_employee_request_builder_only_required() {
        let request = DeleteEmployeeRequest::builder()
            .leave_time(1704067200000)
            .build()
            .unwrap();

        assert_eq!(request.leave_time, 1704067200000);
        assert_eq!(request.leave_reason, None);
        assert_eq!(request.leave_remark, None);
    }

    #[test]
    fn test_delete_employee_request_validation() {
        // 测试空离职时间
        let result = DeleteEmployeeRequest::builder().build();
        assert!(result.is_err());

        // 测试离职时间过早
        let result = DeleteEmployeeRequest::builder()
            .leave_time(1000000000000) // 2001年
            .build();
        assert!(result.is_err());

        // 测试离职时间过晚
        let result = DeleteEmployeeRequest::builder()
            .leave_time(3000000000000) // 2065年
            .build();
        assert!(result.is_err());

        // 测试无效离职原因
        let result = DeleteEmployeeRequest::builder()
            .leave_time(1704067200000)
            .leave_reason(5) // 无效原因
            .build();
        assert!(result.is_err());

        // 测试离职备注过长
        let long_remark = "a".repeat(501);
        let result = DeleteEmployeeRequest::builder()
            .leave_time(1704067200000)
            .leave_remark(long_remark)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_employee_request_valid_leave_reasons() {
        let valid_reasons = [1, 2, 3];

        for reason in valid_reasons.iter() {
            let request = DeleteEmployeeRequest::builder()
                .leave_time(1704067200000)
                .leave_reason(*reason)
                .build()
                .unwrap();

            assert_eq!(request.leave_reason, Some(*reason));
        }
    }

    #[test]
    fn test_delete_employee_request_edge_cases() {
        // 测试最小有效时间
        let request = DeleteEmployeeRequest::builder()
            .leave_time(1577836800000) // 2020-01-01 00:00:00
            .build();
        assert!(request.is_ok());

        // 测试最大有效时间
        let request = DeleteEmployeeRequest::builder()
            .leave_time(2524608000000) // 2050-01-01 00:00:00
            .build();
        assert!(request.is_ok());

        // 测试空字符串离职备注
        let request = DeleteEmployeeRequest::builder()
            .leave_time(1704067200000)
            .leave_remark("")
            .build();
        assert!(request.is_ok());
        assert_eq!(request.unwrap().leave_remark, Some("".to_string()));
    }

    #[test]
    fn test_delete_employee_response() {
        let response = DeleteEmployeeResponse::new(
            "test_employee_id".to_string(),
            1704067200000,
            1704067201000,
        );

        assert!(response.is_success());
        assert_eq!(response.employee_id(), "test_employee_id");
        assert_eq!(response.leave_time(), 1704067200000);
        assert_eq!(response.operation_time(), 1704067201000);
    }

    #[tokio::test]
    async fn test_delete_employee_builder() {
        let service = create_test_service();
        let request = DeleteEmployeeRequest::builder()
            .leave_time(1704067200000)
            .leave_reason(1)
            .leave_remark("测试删除员工")
            .build()
            .unwrap();

        let builder = service.delete_employee_builder("test_employee_id", request);
        assert_eq!(builder.employee_id, "test_employee_id");
        assert_eq!(builder.request.leave_time, 1704067200000);
    }

    #[test]
    fn test_delete_employee_request_new() {
        let request = DeleteEmployeeRequest::new(1704067200000);
        assert_eq!(request.leave_time, 1704067200000);
        assert_eq!(request.leave_reason, None);
        assert_eq!(request.leave_remark, None);
    }

    #[test]
    fn test_delete_employee_request_serialization() {
        let request = DeleteEmployeeRequest::builder()
            .leave_time(1704067200000)
            .leave_reason(2)
            .leave_remark("公司架构调整")
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: DeleteEmployeeRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.leave_time, request.leave_time);
        assert_eq!(parsed.leave_reason, request.leave_reason);
        assert_eq!(parsed.leave_remark, request.leave_remark);
    }

    #[test]
    fn test_delete_employee_response_serialization() {
        let response = DeleteEmployeeResponse::new(
            "emp_123".to_string(),
            1704067200000,
            1704067201000,
        );

        let json = serde_json::to_string(&response).unwrap();
        let parsed: DeleteEmployeeResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.employee_id, response.employee_id);
        assert_eq!(parsed.leave_time, response.leave_time);
        assert_eq!(parsed.operation_time, response.operation_time);
        assert_eq!(parsed.success, response.success);
    }

    #[test]
    fn test_multiple_valid_requests() {
        let test_cases = vec![
            (1704067200000, Some(1), Some("个人原因".to_string())),
            (1704067200000, Some(2), Some("公司原因".to_string())),
            (1704067200000, Some(3), Some("其他原因".to_string())),
            (1704067200000, None, None),
            (1704067200000, Some(1), None),
            (1704067200000, None, Some("无具体原因".to_string())),
        ];

        for (leave_time, leave_reason, leave_remark) in test_cases {
            let mut builder = DeleteEmployeeRequest::builder().leave_time(leave_time);

            if let Some(reason) = leave_reason {
                builder = builder.leave_reason(reason);
            }

            if let Some(remark) = leave_remark {
                builder = builder.leave_remark(remark);
            }

            let request = builder.build().unwrap();
            assert_eq!(request.leave_time, leave_time);
            assert_eq!(request.leave_reason, leave_reason);
            assert_eq!(request.leave_remark, leave_remark);
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
            let request = DeleteEmployeeRequest::builder()
                .leave_time(1704067200000)
                .build()
                .unwrap();

            let builder = service.delete_employee_builder(employee_id, request);
            assert_eq!(builder.employee_id, employee_id);
        }
    }
}
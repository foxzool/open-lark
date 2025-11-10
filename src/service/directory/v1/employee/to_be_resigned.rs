use open_lark_core::config::Config;
use open_lark_core::error::SDKError;
use crate::response::SDKResult;
use crate::service_trait::Service;
use crate::transport::Transport;
use crate::service::directory::v1::employee::regular::{EmployeeService, ENDPOINT_TO_BE_RESIGNED};
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新在职员工为待离职请求体
///
/// 用于将现有在职员工的状态更新为"待离职"，设置离职相关信息
/// 这是员工生命周期管理中的重要环节，介于在职和离职状态之间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToBeResignedRequest {
    /// 离职时间
    /// 预计离职的Unix时间戳（毫秒），用于记录员工计划离职的时间点
    /// 必须为将来的时间，不能早于当前时间
    pub resign_time: u64,
    /// 离职原因
    /// 员工离职的原因分类，用于HR数据分析和管理决策
    /// 1: 个人原因
    /// 2: 公司原因
    /// 3: 其他原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resign_reason: Option<i32>,
    /// 离职备注
    /// 对离职原因的详细说明，帮助HR了解具体情况
    /// 最大长度为500个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resign_remark: Option<String>,
}

impl ToBeResignedRequest {
    /// 创建新的更新在职员工为待离职请求
    ///
    /// # 参数
    /// * `resign_time` - 离职时间（Unix时间戳，毫秒）
    ///
    /// # 示例
    /// ```rust
    /// let request = ToBeResignedRequest::new(1704067200000); // 2024-01-01 00:00:00
    /// ```
    pub fn new(resign_time: u64) -> Self {
        Self {
            resign_time,
            resign_reason: None,
            resign_remark: None,
        }
    }

    /// 创建更新在职员工为待离职请求的构建器
    pub fn builder() -> ToBeResignedBuilder {
        ToBeResignedBuilder::default()
    }
}

/// 更新在职员工为待离职请求构建器
///
/// 提供流畅的API来构建待离职请求，支持方法链调用和完整的参数验证
#[derive(Debug, Clone, Default)]
pub struct ToBeResignedBuilder {
    resign_time: u64,
    resign_reason: Option<i32>,
    resign_remark: Option<String>,
}

impl ToBeResignedBuilder {
    /// 设置离职时间
    ///
    /// # 参数
    /// * `resign_time` - 离职时间（Unix时间戳，毫秒）
    ///
    /// # 示例
    /// ```rust
    /// let builder = ToBeResignedBuilder::default()
    ///     .resign_time(1704067200000); // 2024-01-01 00:00:00
    /// ```
    pub fn resign_time(mut self, resign_time: u64) -> Self {
        self.resign_time = resign_time;
        self
    }

    /// 设置离职原因
    ///
    /// # 参数
    /// * `resign_reason` - 离职原因（1: 个人原因，2: 公司原因，3: 其他原因）
    ///
    /// # 示例
    /// ```rust
    /// let builder = ToBeResignedBuilder::default()
    ///     .resign_reason(1); // 个人原因
    /// ```
    pub fn resign_reason(mut self, resign_reason: i32) -> Self {
        self.resign_reason = Some(resign_reason);
        self
    }

    /// 设置离职备注
    ///
    /// # 参数
    /// * `resign_remark` - 离职原因的详细说明
    ///
    /// # 示例
    /// ```rust
    /// let builder = ToBeResignedBuilder::default()
    ///     .resign_remark("寻求更好的职业发展机会");
    /// ```
    pub fn resign_remark(mut self, resign_remark: impl Into<String>) -> Self {
        self.resign_remark = Some(resign_remark.into());
        self
    }

    /// 构建更新在职员工为待离职请求
    ///
    /// # 返回
    /// 成功返回更新在职员工为待离职请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果离职时间为0，返回错误
    /// * 如果离职时间早于当前时间，返回错误
    /// * 如果离职时间超出合理范围，返回警告但允许继续
    /// * 如果离职原因不在有效范围内，返回错误
    /// * 如果离职备注过长，返回错误
    pub fn build(self) -> SDKResult<ToBeResignedRequest> {
        // 验证离职时间
        if self.resign_time == 0 {
            return Err(SDKError::ValidationError("离职时间不能为空".to_string()));
        }

        // 获取当前时间戳（毫秒）
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| SDKError::ValidationError(format!("时间获取失败: {}", e)))?
            .as_millis() as u64;

        // 验证离职时间不能早于当前时间
        if self.resign_time < current_time {
            return Err(SDKError::ValidationError(
                "离职时间不能早于当前时间".to_string(),
            ));
        }

        // 验证离职时间是否在合理范围内（当前时间到2030年）
        let max_time = 1893456000000; // 2030-01-01 00:00:00
        if self.resign_time > max_time {
            return Err(SDKError::ValidationError(
                "离职时间超出合理范围".to_string(),
            ));
        }

        // 验证离职原因
        if let Some(reason) = self.resign_reason {
            if reason < 1 || reason > 3 {
                return Err(SDKError::ValidationError(
                    "离职原因必须为1（个人原因）、2（公司原因）或3（其他原因）".to_string(),
                ));
            }
        }

        // 验证离职备注长度
        if let Some(remark) = &self.resign_remark {
            if remark.len() > 500 {
                return Err(SDKError::ValidationError(
                    "离职备注不能超过500个字符".to_string(),
                ));
            }
        }

        Ok(ToBeResignedRequest {
            resign_time: self.resign_time,
            resign_reason: self.resign_reason,
            resign_remark: self.resign_remark,
        })
    }
}

/// 更新在职员工为待离职响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToBeResignedResponse {
    /// 员工ID
    /// 被更新为待离职状态的员工ID，用于确认操作的对象
    pub employee_id: String,
    /// 员工状态
    /// 更新后的员工状态，应该为"待离职"或相应的状态码
    pub status: String,
    /// 离职时间
    /// 系统记录的预计离职时间，用于确认时间设置正确
    pub resign_time: u64,
    /// 操作时间
    /// 系统处理待离职设置请求的时间，用于审计和追踪
    pub operation_time: u64,
    /// 离职原因
    /// 记录的离职原因，用于后续的HR分析
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resign_reason: Option<i32>,
    /// 离职备注
    /// 记录的离职备注，提供详细的离职信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resign_remark: Option<String>,
}

impl ToBeResignedResponse {
    /// 创建新的更新在职员工为待离职响应
    ///
    /// # 参数
    /// * `employee_id` - 员工ID
    /// * `status` - 员工状态
    /// * `resign_time` - 离职时间
    /// * `operation_time` - 操作时间
    pub fn new(employee_id: String, status: String, resign_time: u64, operation_time: u64) -> Self {
        Self {
            employee_id,
            status,
            resign_time,
            operation_time,
            resign_reason: None,
            resign_remark: None,
        }
    }

    /// 创建包含离职原因的响应
    ///
    /// # 参数
    /// * `employee_id` - 员工ID
    /// * `status` - 员工状态
    /// * `resign_time` - 离职时间
    /// * `operation_time` - 操作时间
    /// * `resign_reason` - 离职原因
    /// * `resign_remark` - 离职备注
    pub fn new_with_details(
        employee_id: String,
        status: String,
        resign_time: u64,
        operation_time: u64,
        resign_reason: Option<i32>,
        resign_remark: Option<String>,
    ) -> Self {
        Self {
            employee_id,
            status,
            resign_time,
            operation_time,
            resign_reason,
            resign_remark,
        }
    }

    /// 检查员工是否已设置为待离职状态
    ///
    /// # 返回
    /// true表示员工状态为待离职
    pub fn is_to_be_resigned(&self) -> bool {
        self.status == "待离职" || self.status == "to_be_resigned"
    }

    /// 获取员工ID
    ///
    /// # 返回
    /// 员工ID字符串
    pub fn employee_id(&self) -> &str {
        &self.employee_id
    }

    /// 获取员工状态
    ///
    /// # 返回
    /// 员工状态字符串
    pub fn status(&self) -> &str {
        &self.status
    }

    /// 获取离职时间
    ///
    /// # 返回
    /// 离职时间（Unix时间戳，毫秒）
    pub fn resign_time(&self) -> u64 {
        self.resign_time
    }

    /// 获取操作时间
    ///
    /// # 返回
    /// 操作时间（Unix时间戳，毫秒）
    pub fn operation_time(&self) -> u64 {
        self.operation_time
    }

    /// 获取离职原因
    ///
    /// # 返回
    /// 离职原因选项
    pub fn resign_reason(&self) -> Option<i32> {
        self.resign_reason
    }

    /// 获取离职原因描述
    ///
    /// # 返回
    /// 离职原因的中文描述
    pub fn resign_reason_description(&self) -> &'static str {
        match self.resign_reason {
            Some(1) => "个人原因",
            Some(2) => "公司原因",
            Some(3) => "其他原因",
            _ => "未指定原因",
        }
    }

    /// 获取离职备注
    ///
    /// # 返回
    /// 离职备注选项
    pub fn resign_remark(&self) -> Option<&String> {
        self.resign_remark.as_ref()
    }

    /// 格式化离职时间为可读字符串
    ///
    /// # 返回
    /// 格式化的离职时间字符串
    pub fn formatted_resign_time(&self) -> String {
        // 简单的时间戳格式化，实际应用中可以使用chrono库
        let seconds = self.resign_time / 1000;
        let datetime = std::time::UNIX_EPOCH + std::time::Duration::from_secs(seconds);

        // 这里使用简单的格式化，实际项目中可以使用更专业的时间库
        format!("{}", datetime.elapsed().unwrap_or_default().as_secs())
    }
}

/// 更新在职员工为待离职构建器
///
/// 提供流畅的API来更新员工状态为待离职，支持方法链调用和完整的错误处理
#[derive(Debug, Clone)]
pub struct ToBeResignedBuilder {
    service: Arc<EmployeeService>,
    employee_id: String,
    request: ToBeResignedRequest,
}

impl ToBeResignedBuilder {
    /// 创建新的更新在职员工为待离职构建器
    ///
    /// # 参数
    /// * `service` - 员工服务实例
    /// * `employee_id` - 员工ID
    /// * `request` - 更新在职员工为待离职请求
    pub(crate) fn new(
        service: Arc<EmployeeService>,
        employee_id: String,
        request: ToBeResignedRequest,
    ) -> Self {
        Self {
            service,
            employee_id,
            request,
        }
    }

    /// 执行更新在职员工为待离职操作
    ///
    /// 向飞书API发送PATCH请求来更新指定员工的状态为待离职
    ///
    /// # 返回
    /// * `Ok(ToBeResignedResponse)` - 更新成功，返回更新结果
    /// * `Err(SDKError)` - 更新失败，返回错误信息
    ///
    /// # 错误类型
    /// * `SDKError::NetworkError` - 网络请求失败
    /// * `SDKError::ApiError` - API调用失败，包含错误码和消息
    /// * `SDKError::SerializationError` - 响应序列化失败
    /// * `SDKError::AuthenticationError` - 身份验证失败
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::directory::v1::employee::to_be_resigned::{ToBeResignedRequest, ToBeResignedResponse};
    ///
    /// async fn to_be_resigned_employee_example(
    ///     service: Arc<EmployeeService>,
    ///     employee_id: &str
    /// ) -> Result<ToBeResignedResponse, Box<dyn std::error::Error>> {
    ///     // 设置30天后离职
    ///     let thirty_days_later = std::time::SystemTime::now()
    ///         .duration_since(std::time::UNIX_EPOCH)?
    ///         .as_millis() as u64 + (30 * 24 * 60 * 60 * 1000);
    ///
    ///     let request = ToBeResignedRequest::builder()
    ///         .resign_time(thirty_days_later)
    ///         .resign_reason(1) // 个人原因
    ///         .resign_remark("寻求更好的职业发展机会")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .to_be_resigned_employee_builder(&employee_id, request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("员工 {} 已设置为待离职状态，预计离职时间: {}",
    ///              response.employee_id(),
    ///              response.formatted_resign_time());
    ///
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<ToBeResignedResponse> {
        let endpoint = ENDPOINT_TO_BE_RESIGNED.replace("{employee_id}", &self.employee_id);
        let url = self.service.config().build_url(&endpoint);

        // 构建请求体
        let body = serde_json::to_value(&self.request)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        // 发送HTTP PATCH请求
        let response = self
            .service
            .transport()
            .patch(&url, Some(body))
            .await?;

        // 解析响应体
        let response_data: ToBeResignedResponse = serde_json::from_value(response)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        Ok(response_data)
    }

    /// 设置离职时间（链式调用）
    ///
    /// # 参数
    /// * `resign_time` - 离职时间（Unix时间戳，毫秒）
    pub fn resign_time(mut self, resign_time: u64) -> Self {
        self.request.resign_time = resign_time;
        self
    }

    /// 设置离职原因（链式调用）
    ///
    /// # 参数
    /// * `resign_reason` - 离职原因（1: 个人原因，2: 公司原因，3: 其他原因）
    pub fn resign_reason(mut self, resign_reason: i32) -> Self {
        self.request.resign_reason = Some(resign_reason);
        self
    }

    /// 设置离职备注（链式调用）
    ///
    /// # 参数
    /// * `resign_remark` - 离职原因的详细说明
    pub fn resign_remark(mut self, resign_remark: impl Into<String>) -> Self {
        self.request.resign_remark = Some(resign_remark.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark_core::config::Config;
    use crate::transport::MockTransport;
    use std::sync::Arc;

    fn create_test_service() -> Arc<EmployeeService> {
        let config = Config::new("test_app_id", "test_app_secret");
        let transport = MockTransport::new();
        Arc::new(EmployeeService::new_with_transport(config, transport))
    }

    #[test]
    fn test_to_be_resigned_request_builder() {
        // 测试正常构建
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 + (24 * 60 * 60 * 1000); // 24小时后

        let request = ToBeResignedRequest::builder()
            .resign_time(future_time)
            .resign_reason(1)
            .resign_remark("测试离职备注")
            .build()
            .unwrap();

        assert_eq!(request.resign_time, future_time);
        assert_eq!(request.resign_reason, Some(1));
        assert_eq!(request.resign_remark, Some("测试离职备注".to_string()));
    }

    #[test]
    fn test_to_be_resigned_request_builder_only_required() {
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 + (24 * 60 * 60 * 1000); // 24小时后

        let request = ToBeResignedRequest::builder()
            .resign_time(future_time)
            .build()
            .unwrap();

        assert_eq!(request.resign_time, future_time);
        assert_eq!(request.resign_reason, None);
        assert_eq!(request.resign_remark, None);
    }

    #[test]
    fn test_to_be_resigned_request_validation() {
        // 测试空离职时间
        let result = ToBeResignedRequest::builder().build();
        assert!(result.is_err());

        // 测试过去的时间
        let past_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 - (24 * 60 * 60 * 1000); // 24小时前

        let result = ToBeResignedRequest::builder()
            .resign_time(past_time)
            .build();
        assert!(result.is_err());

        // 测试无效离职原因
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 + (24 * 60 * 60 * 1000);

        let result = ToBeResignedRequest::builder()
            .resign_time(future_time)
            .resign_reason(5) // 无效原因
            .build();
        assert!(result.is_err());

        // 测试离职备注过长
        let long_remark = "a".repeat(501);
        let result = ToBeResignedRequest::builder()
            .resign_time(future_time)
            .resign_remark(long_remark)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_to_be_resigned_request_valid_resign_reasons() {
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 + (24 * 60 * 60 * 1000);

        let valid_reasons = [1, 2, 3];

        for reason in valid_reasons.iter() {
            let request = ToBeResignedRequest::builder()
                .resign_time(future_time)
                .resign_reason(*reason)
                .build()
                .unwrap();

            assert_eq!(request.resign_reason, Some(*reason));
        }
    }

    #[test]
    fn test_to_be_resigned_request_new() {
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 + (24 * 60 * 60 * 1000);

        let request = ToBeResignedRequest::new(future_time);
        assert_eq!(request.resign_time, future_time);
        assert_eq!(request.resign_reason, None);
        assert_eq!(request.resign_remark, None);
    }

    #[test]
    fn test_to_be_resigned_response() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let response = ToBeResignedResponse::new(
            "test_employee_id".to_string(),
            "待离职".to_string(),
            1704067200000,
            current_time,
        );

        assert!(response.is_to_be_resigned());
        assert_eq!(response.employee_id(), "test_employee_id");
        assert_eq!(response.status(), "待离职");
        assert_eq!(response.resign_time(), 1704067200000);
        assert_eq!(response.operation_time(), current_time);
    }

    #[test]
    fn test_to_be_resigned_response_with_details() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let response = ToBeResignedResponse::new_with_details(
            "test_employee_id".to_string(),
            "to_be_resigned".to_string(),
            1704067200000,
            current_time,
            Some(1),
            Some("个人发展原因".to_string()),
        );

        assert!(response.is_to_be_resigned());
        assert_eq!(response.resign_reason(), Some(1));
        assert_eq!(response.resign_reason_description(), "个人原因");
        assert_eq!(response.resign_remark(), Some(&"个人发展原因".to_string()));
    }

    #[test]
    fn test_to_be_resigned_request_serialization() {
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 + (24 * 60 * 60 * 1000);

        let request = ToBeResignedRequest::builder()
            .resign_time(future_time)
            .resign_reason(2)
            .resign_remark("公司架构调整")
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: ToBeResignedRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.resign_time, request.resign_time);
        assert_eq!(parsed.resign_reason, request.resign_reason);
        assert_eq!(parsed.resign_remark, request.resign_remark);
    }

    #[test]
    fn test_to_be_resigned_response_serialization() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let response = ToBeResignedResponse::new_with_details(
            "emp_123".to_string(),
            "待离职".to_string(),
            1704067200000,
            current_time,
            Some(1),
            Some("测试离职".to_string()),
        );

        let json = serde_json::to_string(&response).unwrap();
        let parsed: ToBeResignedResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.employee_id, response.employee_id);
        assert_eq!(parsed.status, response.status);
        assert_eq!(parsed.resign_time, response.resign_time);
        assert_eq!(parsed.operation_time, response.operation_time);
        assert_eq!(parsed.resign_reason, response.resign_reason);
        assert_eq!(parsed.resign_remark, response.resign_remark);
    }

    #[test]
    fn test_to_be_resigned_builder_chain_methods() {
        let service = create_test_service();
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 + (24 * 60 * 60 * 1000);

        let request = ToBeResignedRequest::builder()
            .resign_time(future_time)
            .resign_reason(1)
            .resign_remark("测试设置待离职")
            .build()
            .unwrap();

        let builder = service.to_be_resigned_employee_builder("test_employee_id", request);

        // 测试链式调用不会panic
        let _chained_builder = builder
            .resign_time(future_time + 1000)
            .resign_reason(2)
            .resign_remark("链式调用测试");
    }

    #[test]
    fn test_multiple_valid_requests() {
        let base_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let test_cases = vec![
            (base_time + 86400000, Some(1), Some("个人原因".to_string())),      // 1天后
            (base_time + 172800000, Some(2), Some("公司原因".to_string())),     // 2天后
            (base_time + 259200000, Some(3), Some("其他原因".to_string())),     // 3天后
            (base_time + 604800000, None, None),                              // 1周后，无详细信息
            (base_time + 86400000, Some(1), None),                            // 只有原因
            (base_time + 86400000, None, Some("无具体原因".to_string())),      // 只有备注
        ];

        for (resign_time, resign_reason, resign_remark) in test_cases {
            let mut builder = ToBeResignedRequest::builder().resign_time(resign_time);

            if let Some(reason) = resign_reason {
                builder = builder.resign_reason(reason);
            }

            if let Some(remark) = resign_remark {
                builder = builder.resign_remark(remark);
            }

            let request = builder.build().unwrap();
            assert_eq!(request.resign_time, resign_time);
            assert_eq!(request.resign_reason, resign_reason);
            assert_eq!(request.resign_remark, resign_remark);
        }
    }

    #[tokio::test]
    async fn test_to_be_resigned_builder_execute_structure() {
        let service = create_test_service();
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 + (24 * 60 * 60 * 1000);

        let request = ToBeResignedRequest::builder()
            .resign_time(future_time)
            .resign_reason(1)
            .build()
            .unwrap();

        let builder = service.to_be_resigned_employee_builder("test_employee_id", request);

        // 验证builder结构正确
        assert_eq!(builder.employee_id, "test_employee_id");
        assert_eq!(builder.request.resign_time, future_time);
        assert_eq!(builder.request.resign_reason, Some(1));
    }

    #[test]
    fn test_edge_cases() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // 测试最小有效时间（当前时间+1毫秒）
        let min_valid_time = current_time + 1;
        let request = ToBeResignedRequest::builder()
            .resign_time(min_valid_time)
            .build();
        assert!(request.is_ok());

        // 测试最大有效时间（2030年）
        let max_valid_time = 1893456000000;
        let request = ToBeResignedRequest::builder()
            .resign_time(max_valid_time)
            .build();
        assert!(request.is_ok());

        // 测试超出最大时间
        let invalid_time = 2000000000000; // 超过2030年
        let request = ToBeResignedRequest::builder()
            .resign_time(invalid_time)
            .build();
        assert!(request.is_err());

        // 测试空字符串离职备注
        let valid_time = current_time + 86400000;
        let request = ToBeResignedRequest::builder()
            .resign_time(valid_time)
            .resign_remark("")
            .build();
        assert!(request.is_ok());
        assert_eq!(request.unwrap().resign_remark, Some("".to_string()));
    }

    #[test]
    fn test_employee_id_validation() {
        let service = create_test_service();
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 + (24 * 60 * 60 * 1000);

        let request = ToBeResignedRequest::builder()
            .resign_time(future_time)
            .build()
            .unwrap();

        // 测试不同格式的员工ID
        let valid_employee_ids = vec![
            "emp_123456789",
            "open_123456789",
            "user_123456789",
            "employee_test_001",
        ];

        for employee_id in valid_employee_ids {
            let builder = service.to_be_resigned_employee_builder(employee_id, request.clone());
            assert_eq!(builder.employee_id, employee_id);
        }
    }

    #[test]
    fn test_resign_reason_descriptions() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // 测试所有离职原因的描述
        let reasons = vec![
            (Some(1), "个人原因"),
            (Some(2), "公司原因"),
            (Some(3), "其他原因"),
            (None, "未指定原因"),
        ];

        for (reason, expected_description) in reasons {
            let response = ToBeResignedResponse::new_with_details(
                "test_emp".to_string(),
                "待离职".to_string(),
                1704067200000,
                current_time,
                reason,
                None,
            );

            assert_eq!(response.resign_reason_description(), expected_description);
        }
    }

    #[test]
    fn test_time_validation_edge_cases() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // 测试当前时间（应该失败，因为不能等于当前时间）
        let request = ToBeResignedRequest::builder()
            .resign_time(current_time)
            .build();
        assert!(request.is_err());

        // 测试当前时间+1毫秒（应该成功）
        let request = ToBeResignedRequest::builder()
            .resign_time(current_time + 1)
            .build();
        assert!(request.is_ok());

        // 测试当前时间+1秒（应该成功）
        let request = ToBeResignedRequest::builder()
            .resign_time(current_time + 1000)
            .build();
        assert!(request.is_ok());

        // 测试合理范围的未来时间（1年后）
        let one_year_later = current_time + (365 * 24 * 60 * 60 * 1000);
        let request = ToBeResignedRequest::builder()
            .resign_time(one_year_later)
            .build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_response_status_checking() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // 测试中文状态
        let response_chinese = ToBeResignedResponse::new(
            "emp_123".to_string(),
            "待离职".to_string(),
            1704067200000,
            current_time,
        );
        assert!(response_chinese.is_to_be_resigned());

        // 测试英文状态
        let response_english = ToBeResignedResponse::new(
            "emp_456".to_string(),
            "to_be_resigned".to_string(),
            1704067200000,
            current_time,
        );
        assert!(response_english.is_to_be_resigned());

        // 测试其他状态（不是待离职）
        let response_other = ToBeResignedResponse::new(
            "emp_789".to_string(),
            "在职".to_string(),
            1704067200000,
            current_time,
        );
        assert!(!response_other.is_to_be_resigned());
    }

    #[test]
    fn test_complete_resignation_workflow() {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let future_time = current_time + (30 * 24 * 60 * 60 * 1000); // 30天后

        // 模拟完整的待离职设置流程
        let request = ToBeResignedRequest::builder()
            .resign_time(future_time)
            .resign_reason(1)
            .resign_remark("经过深思熟虑，决定寻求新的职业挑战。感谢公司的培养和支持。")
            .build()
            .unwrap();

        // 验证请求数据
        assert_eq!(request.resign_time, future_time);
        assert_eq!(request.resign_reason, Some(1));
        assert_eq!(request.resign_remark.unwrap().len(), 30); // 备注长度检查

        // 模拟响应数据
        let response = ToBeResignedResponse::new_with_details(
            "emp_workflow_test".to_string(),
            "待离职".to_string(),
            future_time,
            current_time + 5000, // 操作时间稍晚于请求创建时间
            Some(1),
            Some("经过深思熟虑，决定寻求新的职业挑战。感谢公司的培养和支持。".to_string()),
        );

        // 验证响应数据
        assert!(response.is_to_be_resigned());
        assert_eq!(response.employee_id(), "emp_workflow_test");
        assert_eq!(response.resign_time(), future_time);
        assert_eq!(response.resign_reason_description(), "个人原因");
        assert!(response.resign_remark().unwrap().contains("职业挑战"));
    }
}
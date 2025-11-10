use crate::config::Config;
use crate::error::SDKError;
use crate::response::SDKResult;
use crate::service_trait::Service;
use crate::transport::Transport;
use crate::service::directory::v1::employee::regular::{EmployeeService, ENDPOINT_MGET};
use crate::service::directory::v1::models::UserIdType;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量获取员工信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MGetEmployeeRequest {
    /// 员工ID列表
    /// 支持批量查询多个员工信息，最多支持100个员工ID
    pub employee_ids: Vec<String>,
    /// 用户ID类型
    /// 指定员工ID的类型，支持open_id、user_id、union_id等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

impl MGetEmployeeRequest {
    /// 创建新的批量获取员工请求
    ///
    /// # 参数
    /// * `employee_ids` - 员工ID列表
    ///
    /// # 示例
    /// ```rust
    /// let request = MGetEmployeeRequest::new(vec![
    ///     "emp_123456789".to_string(),
    ///     "emp_987654321".to_string(),
    /// ]);
    /// ```
    pub fn new(employee_ids: Vec<String>) -> Self {
        Self {
            employee_ids,
            user_id_type: None,
        }
    }

    /// 创建批量获取员工请求的构建器
    pub fn builder() -> MGetEmployeeBuilder {
        MGetEmployeeBuilder::default()
    }
}

/// 批量获取员工信息请求构建器
///
/// 提供流畅的API来构建批量获取员工请求，支持方法链调用
#[derive(Debug, Clone, Default)]
pub struct MGetEmployeeBuilder {
    employee_ids: Vec<String>,
    user_id_type: Option<UserIdType>,
}

impl MGetEmployeeBuilder {
    /// 添加员工ID
    ///
    /// # 参数
    /// * `employee_id` - 员工ID
    ///
    /// # 示例
    /// ```rust
    /// let builder = MGetEmployeeBuilder::default()
    ///     .employee_id("emp_123456789");
    /// ```
    pub fn employee_id(mut self, employee_id: impl Into<String>) -> Self {
        self.employee_ids.push(employee_id.into());
        self
    }

    /// 批量添加员工ID
    ///
    /// # 参数
    /// * `employee_ids` - 员工ID列表
    ///
    /// # 示例
    /// ```rust
    /// let builder = MGetEmployeeBuilder::default()
    ///     .employee_ids(vec!["emp_123".to_string(), "emp_456".to_string()]);
    /// ```
    pub fn employee_ids(mut self, employee_ids: Vec<String>) -> Self {
        self.employee_ids.extend(employee_ids);
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// * `user_id_type` - 用户ID类型
    ///
    /// # 示例
    /// ```rust
    /// let builder = MGetEmployeeBuilder::default()
    ///     .employee_id("emp_123456789")
    ///     .user_id_type(UserIdType::OpenId);
    /// ```
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 构建批量获取员工请求
    ///
    /// # 返回
    /// 成功返回批量获取员工请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果员工ID列表为空，返回错误
    /// * 如果员工ID数量超过100个，返回错误
    /// * 如果员工ID格式无效，返回错误
    pub fn build(self) -> SDKResult<MGetEmployeeRequest> {
        // 验证员工ID列表
        if self.employee_ids.is_empty() {
            return Err(SDKError::ValidationError("员工ID列表不能为空".to_string()));
        }

        // 验证员工ID数量限制
        if self.employee_ids.len() > 100 {
            return Err(SDKError::ValidationError(
                "员工ID数量不能超过100个".to_string(),
            ));
        }

        // 验证员工ID格式
        for employee_id in &self.employee_ids {
            if employee_id.is_empty() {
                return Err(SDKError::ValidationError("员工ID不能为空".to_string()));
            }
            if employee_id.len() > 64 {
                return Err(SDKError::ValidationError(
                    "员工ID长度不能超过64个字符".to_string(),
                ));
            }
        }

        // 验证员工ID重复
        let mut unique_ids = std::collections::HashSet::new();
        for employee_id in &self.employee_ids {
            if !unique_ids.insert(employee_id) {
                return Err(SDKError::ValidationError(
                    format!("员工ID重复: {}", employee_id),
                ));
            }
        }

        Ok(MGetEmployeeRequest {
            employee_ids: self.employee_ids,
            user_id_type: self.user_id_type,
        })
    }
}

/// 员工详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeInfo {
    /// 员工ID
    pub employee_id: String,
    /// 用户ID
    pub user_id: String,
    /// 姓名
    pub name: String,
    /// 别名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 性别
    pub gender: Option<String>,
    /// 头像
    pub avatar: Option<String>,
    /// 员工状态
    /// active: 在职, deleted: 已删除, resigned: 已离职
    pub status: String,
    /// 入职时间
    pub join_time: Option<u64>,
    /// 创建时间
    pub create_time: u64,
    /// 更新时间
    pub update_time: u64,
    /// 部门信息
    pub department_ids: Option<Vec<String>>,
    /// 职位信息
    pub orders: Option<Vec<serde_json::Value>>,
    /// 自定义字段
    pub custom_fields: Option<serde_json::Value>,
}

impl EmployeeInfo {
    /// 创建新的员工信息
    ///
    /// # 参数
    /// * `employee_id` - 员工ID
    /// * `user_id` - 用户ID
    /// * `name` - 姓名
    /// * `status` - 员工状态
    /// * `create_time` - 创建时间
    /// * `update_time` - 更新时间
    pub fn new(
        employee_id: String,
        user_id: String,
        name: String,
        status: String,
        create_time: u64,
        update_time: u64,
    ) -> Self {
        Self {
            employee_id,
            user_id,
            name,
            en_name: None,
            email: None,
            mobile: None,
            gender: None,
            avatar: None,
            status,
            join_time: None,
            create_time,
            update_time,
            department_ids: None,
            orders: None,
            custom_fields: None,
        }
    }

    /// 检查员工是否在职
    ///
    /// # 返回
    /// true表示员工在职
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }

    /// 检查员工是否已删除
    ///
    /// # 返回
    /// true表示员工已删除
    pub fn is_deleted(&self) -> bool {
        self.status == "deleted"
    }

    /// 检查员工是否已离职
    ///
    /// # 返回
    /// true表示员工已离职
    pub fn is_resigned(&self) -> bool {
        self.status == "resigned"
    }

    /// 获取员工ID
    pub fn employee_id(&self) -> &str {
        &self.employee_id
    }

    /// 获取用户ID
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// 获取姓名
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取英文名
    pub fn en_name(&self) -> Option<&str> {
        self.en_name.as_deref()
    }

    /// 获取邮箱
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    /// 获取手机号
    pub fn mobile(&self) -> Option<&str> {
        self.mobile.as_deref()
    }

    /// 获取性别
    pub fn gender(&self) -> Option<&str> {
        self.gender.as_deref()
    }

    /// 获取头像
    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }

    /// 获取员工状态
    pub fn status(&self) -> &str {
        &self.status
    }

    /// 获取入职时间
    pub fn join_time(&self) -> Option<u64> {
        self.join_time
    }

    /// 获取创建时间
    pub fn create_time(&self) -> u64 {
        self.create_time
    }

    /// 获取更新时间
    pub fn update_time(&self) -> u64 {
        self.update_time
    }

    /// 获取部门ID列表
    pub fn department_ids(&self) -> Option<&Vec<String>> {
        self.department_ids.as_ref()
    }
}

/// 批量获取员工信息响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MGetEmployeeResponse {
    /// 操作是否成功
    /// true表示所有员工信息都成功获取，false表示部分或全部失败
    pub success: bool,
    /// 员工信息列表
    /// 成功获取的员工信息列表
    pub employees: Vec<EmployeeInfo>,
    /// 失败的员工ID列表
    /// 未能成功获取的员工ID及错误原因
    pub failed_items: Option<Vec<FailedItem>>,
    /// 总数
    /// 请求的员工总数
    pub total: u32,
    /// 成功数量
    /// 成功获取的员工数量
    pub success_count: u32,
    /// 失败数量
    /// 获取失败的员工数量
    pub failed_count: u32,
}

impl MGetEmployeeResponse {
    /// 创建新的批量获取员工响应
    ///
    /// # 参数
    /// * `employees` - 员工信息列表
    /// * `failed_items` - 失败项列表
    /// * `total` - 总数
    pub fn new(
        employees: Vec<EmployeeInfo>,
        failed_items: Option<Vec<FailedItem>>,
        total: u32,
    ) -> Self {
        let success_count = employees.len() as u32;
        let failed_count = failed_items.as_ref().map_or(0, |items| items.len() as u32);
        let success = failed_count == 0;

        Self {
            success,
            employees,
            failed_items,
            total,
            success_count,
            failed_count,
        }
    }

    /// 检查操作是否完全成功
    ///
    /// # 返回
    /// true表示所有员工信息都成功获取
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// 检查是否有部分失败
    ///
    /// # 返回
    /// true表示有部分员工信息获取失败
    pub fn has_failures(&self) -> bool {
        self.failed_count > 0
    }

    /// 获取员工信息列表
    ///
    /// # 返回
    /// 员工信息列表
    pub fn employees(&self) -> &Vec<EmployeeInfo> {
        &self.employees
    }

    /// 获取失败项列表
    ///
    /// # 返回
    /// 失败项列表
    pub fn failed_items(&self) -> Option<&Vec<FailedItem>> {
        self.failed_items.as_ref()
    }

    /// 获取总数
    ///
    /// # 返回
    /// 请求的员工总数
    pub fn total(&self) -> u32 {
        self.total
    }

    /// 获取成功数量
    ///
    /// # 返回
    /// 成功获取的员工数量
    pub fn success_count(&self) -> u32 {
        self.success_count
    }

    /// 获取失败数量
    ///
    /// # 返回
    /// 获取失败的员工数量
    pub fn failed_count(&self) -> u32 {
        self.failed_count
    }

    /// 获取成功率
    ///
    /// # 返回
    /// 成功率（0.0 - 1.0）
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.success_count as f64 / self.total as f64
        }
    }
}

/// 失败项信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedItem {
    /// 员工ID
    pub employee_id: String,
    /// 错误码
    pub error_code: i32,
    /// 错误消息
    pub error_message: String,
}

impl FailedItem {
    /// 创建新的失败项
    ///
    /// # 参数
    /// * `employee_id` - 员工ID
    /// * `error_code` - 错误码
    /// * `error_message` - 错误消息
    pub fn new(employee_id: String, error_code: i32, error_message: String) -> Self {
        Self {
            employee_id,
            error_code,
            error_message,
        }
    }

    /// 获取员工ID
    pub fn employee_id(&self) -> &str {
        &self.employee_id
    }

    /// 获取错误码
    pub fn error_code(&self) -> i32 {
        self.error_code
    }

    /// 获取错误消息
    pub fn error_message(&self) -> &str {
        &self.error_message
    }
}

/// 批量获取员工信息构建器
///
/// 提供流畅的API来批量获取员工信息，支持方法链调用和完整的错误处理
#[derive(Debug, Clone)]
pub struct MGetEmployeeBuilder {
    service: Arc<EmployeeService>,
    request: MGetEmployeeRequest,
}

impl MGetEmployeeBuilder {
    /// 创建新的批量获取员工信息构建器
    ///
    /// # 参数
    /// * `service` - 员工服务实例
    /// * `request` - 批量获取员工信息请求
    pub(crate) fn new(service: Arc<EmployeeService>, request: MGetEmployeeRequest) -> Self {
        Self { service, request }
    }

    /// 执行批量获取员工信息操作
    ///
    /// 向飞书API发送POST请求来批量获取员工信息
    ///
    /// # 返回
    /// * `Ok(MGetEmployeeResponse)` - 获取成功，返回员工信息列表
    /// * `Err(SDKError)` - 获取失败，返回错误信息
    ///
    /// # 错误类型
    /// * `SDKError::NetworkError` - 网络请求失败
    /// * `SDKError::ApiError` - API调用失败，包含错误码和消息
    /// * `SDKError::SerializationError` - 响应序列化失败
    /// * `SDKError::AuthenticationError` - 身份验证失败
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::directory::v1::employee::mget::{MGetEmployeeRequest, MGetEmployeeResponse};
    ///
    /// async fn mget_employee_example(
    ///     service: Arc<EmployeeService>
    /// ) -> Result<MGetEmployeeResponse, Box<dyn std::error::Error>> {
    ///     let request = MGetEmployeeRequest::builder()
    ///         .employee_id("emp_123456789")
    ///         .employee_id("emp_987654321")
    ///         .user_id_type(UserIdType::OpenId)
    ///         .build()?;
    ///
    ///     let response = service
    ///         .mget_employee_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("成功获取 {} 个员工信息", response.success_count());
    ///     if response.has_failures() {
    ///         println!("失败 {} 个员工", response.failed_count());
    ///     }
    ///
    ///     for employee in response.employees() {
    ///         println!("员工: {} ({})", employee.name(), employee.employee_id());
    ///     }
    ///
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<MGetEmployeeResponse> {
        let url = self.service.config().build_url(ENDPOINT_MGET);

        // 构建查询参数
        let mut query_params = Vec::new();
        if let Some(id_type) = &self.request.user_id_type {
            query_params.push(("user_id_type", id_type.to_string()));
        }

        // 构建请求体
        let body = serde_json::json!({
            "employee_ids": self.request.employee_ids
        });

        // 发送HTTP POST请求
        let response = self
            .service
            .transport()
            .post_with_query(&url, Some(body), &query_params)
            .await?;

        // 解析响应体
        let response_data: MGetEmployeeResponse = serde_json::from_value(response)
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
    fn test_mget_employee_request_builder() {
        // 测试正常构建
        let request = MGetEmployeeRequest::builder()
            .employee_id("emp_123456789")
            .employee_id("emp_987654321")
            .user_id_type(UserIdType::OpenId)
            .build()
            .unwrap();

        assert_eq!(request.employee_ids.len(), 2);
        assert_eq!(request.employee_ids[0], "emp_123456789");
        assert_eq!(request.employee_ids[1], "emp_987654321");
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_mget_employee_request_batch_ids() {
        let ids = vec![
            "emp_001".to_string(),
            "emp_002".to_string(),
            "emp_003".to_string(),
        ];

        let request = MGetEmployeeRequest::builder()
            .employee_ids(ids.clone())
            .build()
            .unwrap();

        assert_eq!(request.employee_ids, ids);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_mget_employee_request_validation() {
        // 测试空员工ID列表
        let result = MGetEmployeeRequest::builder().build();
        assert!(result.is_err());

        // 测试超过100个员工ID
        let many_ids: Vec<String> = (0..101).map(|i| format!("emp_{:03}", i)).collect();
        let result = MGetEmployeeRequest::builder()
            .employee_ids(many_ids)
            .build();
        assert!(result.is_err());

        // 测试空员工ID
        let result = MGetEmployeeRequest::builder()
            .employee_id("")
            .build();
        assert!(result.is_err());

        // 测试员工ID过长
        let long_id = "a".repeat(65);
        let result = MGetEmployeeRequest::builder()
            .employee_id(long_id)
            .build();
        assert!(result.is_err());

        // 测试重复员工ID
        let result = MGetEmployeeRequest::builder()
            .employee_id("emp_123")
            .employee_id("emp_123") // 重复
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_mget_employee_request_edge_cases() {
        // 测试单个员工ID
        let request = MGetEmployeeRequest::builder()
            .employee_id("emp_123")
            .build()
            .unwrap();
        assert_eq!(request.employee_ids.len(), 1);

        // 测试最大员工ID数量（100个）
        let max_ids: Vec<String> = (0..100).map(|i| format!("emp_{:03}", i)).collect();
        let request = MGetEmployeeRequest::builder()
            .employee_ids(max_ids)
            .build();
        assert!(request.is_ok());

        // 测试不同用户ID类型
        let user_types = [
            UserIdType::UserId,
            UserIdType::OpenId,
            UserIdType::UnionId,
        ];

        for user_type in user_types.iter() {
            let request = MGetEmployeeRequest::builder()
                .employee_id("emp_123")
                .user_id_type(*user_type)
                .build()
                .unwrap();

            assert_eq!(request.user_id_type, Some(*user_type));
        }
    }

    #[test]
    fn test_employee_info() {
        let employee = EmployeeInfo::new(
            "emp_123456789".to_string(),
            "user_123456789".to_string(),
            "张三".to_string(),
            "active".to_string(),
            1704067200000,
            1704067201000,
        );

        assert_eq!(employee.employee_id(), "emp_123456789");
        assert_eq!(employee.user_id(), "user_123456789");
        assert_eq!(employee.name(), "张三");
        assert!(employee.is_active());
        assert!(!employee.is_deleted());
        assert!(!employee.is_resigned());
        assert_eq!(employee.status(), "active");
    }

    #[test]
    fn test_employee_info_status() {
        let active_employee = EmployeeInfo::new(
            "emp_001".to_string(),
            "user_001".to_string(),
            "在职员工".to_string(),
            "active".to_string(),
            1704067200000,
            1704067201000,
        );

        let deleted_employee = EmployeeInfo::new(
            "emp_002".to_string(),
            "user_002".to_string(),
            "已删除员工".to_string(),
            "deleted".to_string(),
            1704067200000,
            1704067201000,
        );

        let resigned_employee = EmployeeInfo::new(
            "emp_003".to_string(),
            "user_003".to_string(),
            "已离职员工".to_string(),
            "resigned".to_string(),
            1704067200000,
            1704067201000,
        );

        assert!(active_employee.is_active());
        assert!(!active_employee.is_deleted());
        assert!(!active_employee.is_resigned());

        assert!(!deleted_employee.is_active());
        assert!(deleted_employee.is_deleted());
        assert!(!deleted_employee.is_resigned());

        assert!(!resigned_employee.is_active());
        assert!(!resigned_employee.is_deleted());
        assert!(resigned_employee.is_resigned());
    }

    #[test]
    fn test_mget_employee_response() {
        let employees = vec![
            EmployeeInfo::new(
                "emp_001".to_string(),
                "user_001".to_string(),
                "员工1".to_string(),
                "active".to_string(),
                1704067200000,
                1704067201000,
            ),
            EmployeeInfo::new(
                "emp_002".to_string(),
                "user_002".to_string(),
                "员工2".to_string(),
                "active".to_string(),
                1704067200000,
                1704067201000,
            ),
        ];

        let response = MGetEmployeeResponse::new(employees.clone(), None, 2);

        assert!(response.is_success());
        assert!(!response.has_failures());
        assert_eq!(response.total(), 2);
        assert_eq!(response.success_count(), 2);
        assert_eq!(response.failed_count(), 0);
        assert_eq!(response.success_rate(), 1.0);
        assert_eq!(response.employees().len(), 2);
        assert!(response.failed_items().is_none());
    }

    #[test]
    fn test_mget_employee_response_with_failures() {
        let employees = vec![EmployeeInfo::new(
            "emp_001".to_string(),
            "user_001".to_string(),
            "员工1".to_string(),
            "active".to_string(),
            1704067200000,
            1704067201000,
        )];

        let failed_items = vec![
            FailedItem::new("emp_002".to_string(), 404, "员工不存在".to_string()),
            FailedItem::new("emp_003".to_string(), 403, "无权限".to_string()),
        ];

        let response = MGetEmployeeResponse::new(employees, Some(failed_items), 3);

        assert!(!response.is_success());
        assert!(response.has_failures());
        assert_eq!(response.total(), 3);
        assert_eq!(response.success_count(), 1);
        assert_eq!(response.failed_count(), 2);
        assert!((response.success_rate() - 0.333).abs() < 0.001);
        assert_eq!(response.employees().len(), 1);
        assert!(response.failed_items().is_some());
    }

    #[test]
    fn test_failed_item() {
        let failed_item = FailedItem::new(
            "emp_123".to_string(),
            404,
            "员工不存在".to_string(),
        );

        assert_eq!(failed_item.employee_id(), "emp_123");
        assert_eq!(failed_item.error_code(), 404);
        assert_eq!(failed_item.error_message(), "员工不存在");
    }

    #[test]
    fn test_mget_employee_request_new() {
        let ids = vec![
            "emp_001".to_string(),
            "emp_002".to_string(),
        ];

        let request = MGetEmployeeRequest::new(ids.clone());
        assert_eq!(request.employee_ids, ids);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_mget_employee_request_serialization() {
        let request = MGetEmployeeRequest::builder()
            .employee_id("emp_123")
            .employee_id("emp_456")
            .user_id_type(UserIdType::OpenId)
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: MGetEmployeeRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.employee_ids, request.employee_ids);
        assert_eq!(parsed.user_id_type, request.user_id_type);
    }

    #[test]
    fn test_mget_employee_response_serialization() {
        let employees = vec![EmployeeInfo::new(
            "emp_123".to_string(),
            "user_123".to_string(),
            "测试员工".to_string(),
            "active".to_string(),
            1704067200000,
            1704067201000,
        )];

        let response = MGetEmployeeResponse::new(employees, None, 1);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: MGetEmployeeResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.success, response.success);
        assert_eq!(parsed.total, response.total);
        assert_eq!(parsed.success_count, response.success_count);
        assert_eq!(parsed.failed_count, response.failed_count);
    }

    #[tokio::test]
    async fn test_mget_employee_builder() {
        let service = create_test_service();
        let request = MGetEmployeeRequest::builder()
            .employee_id("emp_123456789")
            .employee_id("emp_987654321")
            .user_id_type(UserIdType::OpenId)
            .build()
            .unwrap();

        let builder = service.mget_employee_builder(request);
        assert_eq!(builder.request.employee_ids.len(), 2);
        assert_eq!(builder.request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_multiple_valid_requests() {
        let test_cases = vec![
            vec!["emp_001"],
            vec!["emp_001", "emp_002"],
            vec!["emp_001", "emp_002", "emp_003", "emp_004", "emp_005"],
        ];

        for ids in test_cases {
            let mut builder = MGetEmployeeRequest::builder();
            for id in ids {
                builder = builder.employee_id(id);
            }

            let request = builder.build().unwrap();
            assert_eq!(request.employee_ids.len(), ids.len());
        }
    }

    #[test]
    fn test_employee_id_formats() {
        // 测试不同的员工ID格式
        let valid_ids = vec![
            "emp_123456789",
            "open_123456789",
            "user_123456789",
            "employee_test_001",
            "cli_678c5c3a6c8b9e8b",
        ];

        for employee_id in valid_ids {
            let request = MGetEmployeeRequest::builder()
                .employee_id(employee_id)
                .build();
            assert!(request.is_ok(), "应该接受有效的员工ID: {}", employee_id);
        }
    }

    #[test]
    fn test_complex_mget_request() {
        // 测试包含所有字段的复杂批量请求
        let ids = vec![
            "emp_manager_001".to_string(),
            "emp_dev_002".to_string(),
            "emp_hr_003".to_string(),
            "emp_sales_004".to_string(),
            "emp_admin_005".to_string(),
        ];

        let request = MGetEmployeeRequest::builder()
            .employee_ids(ids.clone())
            .user_id_type(UserIdType::UnionId)
            .build()
            .unwrap();

        // 验证所有字段都正确设置
        assert_eq!(request.employee_ids, ids);
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(request.employee_ids.len(), 5);
    }

    #[test]
    fn test_response_statistics() {
        // 测试响应统计信息的准确性
        let employees = vec![
            EmployeeInfo::new("emp_001".to_string(), "user_001".to_string(), "员工1".to_string(), "active".to_string(), 1, 1),
            EmployeeInfo::new("emp_002".to_string(), "user_002".to_string(), "员工2".to_string(), "active".to_string(), 1, 1),
            EmployeeInfo::new("emp_003".to_string(), "user_003".to_string(), "员工3".to_string(), "active".to_string(), 1, 1),
        ];

        let failed_items = vec![
            FailedItem::new("emp_004".to_string(), 404, "不存在".to_string()),
        ];

        let response = MGetEmployeeResponse::new(employees, Some(failed_items), 4);

        assert_eq!(response.total(), 4);
        assert_eq!(response.success_count(), 3);
        assert_eq!(response.failed_count(), 1);
        assert_eq!(response.success_rate(), 0.75);
        assert!(response.has_failures());
        assert!(!response.is_success());
    }
}
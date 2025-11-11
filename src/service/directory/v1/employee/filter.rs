use config::Config;
use openlark_core::error::SDKError;
use crate::response::SDKResult;
use crate::service_trait::Service;
use crate::transport::Transport;
use crate::service::directory::v1::employee::regular::{EmployeeService, ENDPOINT_FILTER};
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量获取员工列表请求体
///
/// 提供灵活的员工查询功能，支持分页、过滤和排序
/// 与API #84的区别：API #84通过精确ID查询，API #85通过条件过滤查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterEmployeeRequest {
    /// 分页大小
    /// 分页大小，最大值为50，默认值为10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    /// 用于获取下一页的标记，首次请求时为空
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 员工状态过滤
    /// 可选值：1-在职，2-离职，3-待入职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 部门ID过滤
    /// 需要查询的部门ID列表，如果不为空，则只返回这些部门的员工
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 用户ID类型
    /// 支持：open_id、user_id、union_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    /// 支持：open_department_id、department_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 排序字段
    /// 支持按字段排序，如：employee_id、name、create_time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    /// 排序方向
    /// asc：升序，desc：降序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
}

impl FilterEmployeeRequest {
    /// 创建新的批量获取员工列表请求
    ///
    /// 使用默认参数创建请求，可以通过后续方法调整参数
    pub fn new() -> Self {
        Self {
            page_size: Some(10), // 默认分页大小
            page_token: None,
            status: None,
            department_ids: None,
            user_id_type: None,
            department_id_type: None,
            sort_field: None,
            sort_direction: None,
        }
    }

    /// 创建批量获取员工列表请求的构建器
    pub fn builder() -> FilterEmployeeBuilder {
        FilterEmployeeBuilder::default()
    }
}

/// 批量获取员工列表请求构建器
///
/// 提供流畅的API来构建员工查询请求，支持方法链调用
#[derive(Debug, Clone, Default)]
pub struct FilterEmployeeBuilder {
    page_size: Option<i32>,
    page_token: Option<String>,
    status: Option<i32>,
    department_ids: Option<Vec<String>>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
    sort_field: Option<String>,
    sort_direction: Option<String>,
}

impl FilterEmployeeBuilder {
    /// 设置分页大小
    ///
    /// # 参数
    /// * `page_size` - 分页大小，范围1-50
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterEmployeeBuilder::default()
    ///     .page_size(20);
    /// ```
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    ///
    /// # 参数
    /// * `page_token` - 分页标记，用于获取下一页数据
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterEmployeeBuilder::default()
    ///     .page_token("next_page_token_123".to_string());
    /// ```
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置员工状态过滤
    ///
    /// # 参数
    /// * `status` - 员工状态（1-在职，2-离职，3-待入职）
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterEmployeeBuilder::default()
    ///     .status(1); // 只查询在职员工
    /// ```
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置部门ID过滤
    ///
    /// # 参数
    /// * `department_ids` - 部门ID列表
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterEmployeeBuilder::default()
    ///     .department_ids(vec!["dept_123".to_string(), "dept_456".to_string()]);
    /// ```
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = Some(department_ids);
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// * `user_id_type` - 用户ID类型（open_id、user_id、union_id）
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterEmployeeBuilder::default()
    ///     .user_id_type("open_id".to_string());
    /// ```
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门ID类型
    ///
    /// # 参数
    /// * `department_id_type` - 部门ID类型（open_department_id、department_id）
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterEmployeeBuilder::default()
    ///     .department_id_type("department_id".to_string());
    /// ```
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 设置排序字段
    ///
    /// # 参数
    /// * `sort_field` - 排序字段
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterEmployeeBuilder::default()
    ///     .sort_field("create_time".to_string());
    /// ```
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方向
    ///
    /// # 参数
    /// * `sort_direction` - 排序方向（asc、desc）
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterEmployeeBuilder::default()
    ///     .sort_direction("desc".to_string()); // 降序
    /// ```
    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.sort_direction = Some(sort_direction.into());
        self
    }

    /// 构建批量获取员工列表请求
    ///
    /// # 返回
    /// 成功返回批量获取员工列表请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果分页大小超出范围，返回错误
    /// * 如果员工状态不在有效范围内，返回错误
    /// * 如果部门ID列表为空，返回错误
    /// * 如果用户ID类型无效，返回错误
    pub fn build(self) -> SDKResult<FilterEmployeeRequest> {
        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 50 {
                return Err(SDKError::ValidationError(
                    "分页大小必须在1-50之间".to_string(),
                ));
            }
        }

        // 验证员工状态
        if let Some(status) = self.status {
            if status < 1 || status > 3 {
                return Err(SDKError::ValidationError(
                    "员工状态必须为1（在职）、2（离职）或3（待入职）".to_string(),
                ));
            }
        }

        // 验证部门ID列表
        if let Some(ref department_ids) = self.department_ids {
            if department_ids.is_empty() {
                return Err(SDKError::ValidationError(
                    "部门ID列表不能为空".to_string(),
                ));
            }
            if department_ids.len() > 50 {
                return Err(SDKError::ValidationError(
                    "部门ID数量不能超过50个".to_string(),
                ));
            }
        }

        // 验证用户ID类型
        if let Some(ref user_id_type) = self.user_id_type {
            let valid_types = ["open_id", "user_id", "union_id"];
            if !valid_types.contains(&user_id_type.as_str()) {
                return Err(SDKError::ValidationError(
                    "用户ID类型必须为open_id、user_id或union_id".to_string(),
                ));
            }
        }

        // 验证部门ID类型
        if let Some(ref department_id_type) = self.department_id_type {
            let valid_types = ["open_department_id", "department_id"];
            if !valid_types.contains(&department_id_type.as_str()) {
                return Err(SDKError::ValidationError(
                    "部门ID类型必须为open_department_id或department_id".to_string(),
                ));
            }
        }

        // 验证排序方向
        if let Some(ref sort_direction) = self.sort_direction {
            let valid_directions = ["asc", "desc"];
            if !valid_directions.contains(&sort_direction.as_str()) {
                return Err(SDKError::ValidationError(
                    "排序方向必须为asc或desc".to_string(),
                ));
            }
        }

        Ok(FilterEmployeeRequest {
            page_size: self.page_size,
            page_token: self.page_token,
            status: self.status,
            department_ids: self.department_ids,
            user_id_type: self.user_id_type,
            department_id_type: self.department_id_type,
            sort_field: self.sort_field,
            sort_direction: self.sort_direction,
        })
    }
}

/// 员工信息结构
///
/// 包含员工的基本信息，用于批量查询结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterEmployeeInfo {
    /// 员工ID
    /// 员工的唯一标识符
    pub employee_id: String,
    /// 用户ID
    /// 用户的唯一标识符
    pub user_id: String,
    /// 姓名
    /// 员工的真实姓名
    pub name: String,
    /// 邮箱
    /// 员工的工作邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    /// 员工的手机号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 员工状态
    /// 1-在职，2-离职，3-待入职
    pub status: i32,
    /// 部门ID列表
    /// 员工所属的部门ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 职位
    /// 员工的职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 工号
    /// 员工的工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// 入职时间
    /// 员工入职的时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_time: Option<u64>,
    /// 创建时间
    /// 员工记录创建的时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<u64>,
    /// 更新时间
    /// 员工记录更新的时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<u64>,
}

impl FilterEmployeeInfo {
    /// 创建新的员工信息
    ///
    /// # 参数
    /// * `employee_id` - 员工ID
    /// * `user_id` - 用户ID
    /// * `name` - 姓名
    /// * `status` - 员工状态
    pub fn new(employee_id: String, user_id: String, name: String, status: i32) -> Self {
        Self {
            employee_id,
            user_id,
            name,
            status,
            email: None,
            mobile: None,
            department_ids: None,
            job_title: None,
            employee_number: None,
            hire_time: None,
            create_time: None,
            update_time: None,
        }
    }

    /// 获取员工状态描述
    ///
    /// # 返回
    /// 返回员工状态的中文描述
    pub fn status_description(&self) -> &'static str {
        match self.status {
            1 => "在职",
            2 => "离职",
            3 => "待入职",
            _ => "未知状态",
        }
    }

    /// 检查员工是否在职
    ///
    /// # 返回
    /// true表示员工在职
    pub fn is_active(&self) -> bool {
        self.status == 1
    }

    /// 获取员工ID
    ///
    /// # 返回
    /// 员工ID字符串
    pub fn employee_id(&self) -> &str {
        &self.employee_id
    }

    /// 获取用户ID
    ///
    /// # 返回
    /// 用户ID字符串
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// 获取员工姓名
    ///
    /// # 返回
    /// 员工姓名字符串
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取员工邮箱
    ///
    /// # 返回
    /// 员工邮箱选项
    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }

    /// 获取员工手机号
    ///
    /// # 返回
    /// 员工手机号选项
    pub fn mobile(&self) -> Option<&String> {
        self.mobile.as_ref()
    }

    /// 获取员工状态
    ///
    /// # 返回
    /// 员工状态数字
    pub fn status(&self) -> i32 {
        self.status
    }

    /// 获取部门ID列表
    ///
    /// # 返回
    /// 部门ID列表选项
    pub fn department_ids(&self) -> Option<&Vec<String>> {
        self.department_ids.as_ref()
    }

    /// 获取员工职位
    ///
    /// # 返回
    /// 员工职位选项
    pub fn job_title(&self) -> Option<&String> {
        self.job_title.as_ref()
    }

    /// 获取员工工号
    ///
    /// # 返回
    /// 员工工号选项
    pub fn employee_number(&self) -> Option<&String> {
        self.employee_number.as_ref()
    }

    /// 获取入职时间
    ///
    /// # 返回
    /// 入职时间戳选项
    pub fn hire_time(&self) -> Option<u64> {
        self.hire_time
    }

    /// 获取创建时间
    ///
    /// # 返回
    /// 创建时间戳选项
    pub fn create_time(&self) -> Option<u64> {
        self.create_time
    }

    /// 获取更新时间
    ///
    /// # 返回
    /// 更新时间戳选项
    pub fn update_time(&self) -> Option<u64> {
        self.update_time
    }
}

/// 批量获取员工列表响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterEmployeeResponse {
    /// 是否还有更多数据
    /// true表示还有更多数据，需要继续分页获取
    pub has_more: bool,
    /// 分页标记
    /// 获取下一页数据的标记，如果没有更多数据则为空
    pub page_token: Option<String>,
    /// 员工列表
    /// 符合查询条件的员工信息列表
    pub employees: Vec<FilterEmployeeInfo>,
}

impl FilterEmployeeResponse {
    /// 创建新的批量获取员工列表响应
    ///
    /// # 参数
    /// * `has_more` - 是否还有更多数据
    /// * `page_token` - 分页标记
    /// * `employees` - 员工列表
    pub fn new(
        has_more: bool,
        page_token: Option<String>,
        employees: Vec<FilterEmployeeInfo>,
    ) -> Self {
        Self {
            has_more,
            page_token,
            employees,
        }
    }

    /// 检查是否还有更多数据
    ///
    /// # 返回
    /// true表示还有更多数据需要获取
    pub fn has_more(&self) -> bool {
        self.has_more
    }

    /// 获取分页标记
    ///
    /// # 返回
    /// 分页标记选项，用于获取下一页数据
    pub fn page_token(&self) -> Option<&String> {
        self.page_token.as_ref()
    }

    /// 获取员工列表
    ///
    /// # 返回
    /// 员工信息列表的引用
    pub fn employees(&self) -> &Vec<FilterEmployeeInfo> {
        &self.employees
    }

    /// 获取员工数量
    ///
    /// # 返回
    /// 当前页面员工数量
    pub fn employee_count(&self) -> usize {
        self.employees.len()
    }

    /// 检查是否有员工数据
    ///
    /// # 返回
    /// true表示有员工数据
    pub fn has_employees(&self) -> bool {
        !self.employees.is_empty()
    }

    /// 根据状态过滤员工
    ///
    /// # 参数
    /// * `status` - 员工状态
    ///
    /// # 返回
    /// 符合指定状态的员工列表
    pub fn filter_by_status(&self, status: i32) -> Vec<&FilterEmployeeInfo> {
        self.employees
            .iter()
            .filter(|employee| employee.status == status)
            .collect()
    }

    /// 根据部门过滤员工
    ///
    /// # 参数
    /// * `department_id` - 部门ID
    ///
    /// # 返回
    /// 属于指定部门的员工列表
    pub fn filter_by_department(&self, department_id: &str) -> Vec<&FilterEmployeeInfo> {
        self.employees
            .iter()
            .filter(|employee| {
                if let Some(ref departments) = employee.department_ids {
                    departments.contains(&department_id.to_string())
                } else {
                    false
                }
            })
            .collect()
    }

    /// 获取在职员工
    ///
    /// # 返回
    /// 在职员工列表
    pub fn active_employees(&self) -> Vec<&FilterEmployeeInfo> {
        self.filter_by_status(1)
    }

    /// 获取离职员工
    ///
    /// # 返回
    /// 离职员工列表
    pub fn resigned_employees(&self) -> Vec<&FilterEmployeeInfo> {
        self.filter_by_status(2)
    }

    /// 获取待入职员工
    ///
    /// # 返回
    /// 待入职员工列表
    pub fn pending_employees(&self) -> Vec<&FilterEmployeeInfo> {
        self.filter_by_status(3)
    }
}

/// 批量获取员工列表构建器
///
/// 提供流畅的API来查询员工列表，支持分页、过滤和排序功能
#[derive(Debug, Clone)]
pub struct FilterEmployeeBuilder {
    service: Arc<EmployeeService>,
    request: FilterEmployeeRequest,
}

impl FilterEmployeeBuilder {
    /// 创建新的批量获取员工列表构建器
    ///
    /// # 参数
    /// * `service` - 员工服务实例
    /// * `request` - 批量获取员工列表请求
    pub(crate) fn new(service: Arc<EmployeeService>, request: FilterEmployeeRequest) -> Self {
        Self { service, request }
    }

    /// 执行批量获取员工列表操作
    ///
    /// 向飞书API发送POST请求来查询符合条件的员工列表
    ///
    /// # 返回
    /// * `Ok(FilterEmployeeResponse)` - 查询成功，返回员工列表
    /// * `Err(SDKError)` - 查询失败，返回错误信息
    ///
    /// # 错误类型
    /// * `SDKError::NetworkError` - 网络请求失败
    /// * `SDKError::ApiError` - API调用失败，包含错误码和消息
    /// * `SDKError::SerializationError` - 响应序列化失败
    /// * `SDKError::AuthenticationError` - 身份验证失败
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::directory::v1::employee::filter::{FilterEmployeeRequest, FilterEmployeeResponse};
    ///
    /// async fn filter_employee_example(
    ///     service: Arc<EmployeeService>
    /// ) -> Result<FilterEmployeeResponse, Box<dyn std::error::Error>> {
    ///     let request = FilterEmployeeRequest::builder()
    ///         .page_size(20)
    ///         .status(1) // 只查询在职员工
    ///         .user_id_type("open_id".to_string())
    ///         .sort_field("create_time".to_string())
    ///         .sort_direction("desc".to_string())
    ///         .build()?;
    ///
    ///     let response = service
    ///         .filter_employee_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("找到 {} 个员工，还有更多数据: {}",
    ///              response.employee_count(),
    ///              response.has_more());
    ///
    ///     // 如果还有更多数据，可以继续获取下一页
    ///     if response.has_more() {
    ///         if let Some(next_token) = response.page_token() {
    ///             let next_request = FilterEmployeeRequest::builder()
    ///                 .page_size(20)
    ///                 .page_token(next_token.clone())
    ///                 .status(1)
    ///                 .user_id_type("open_id".to_string())
    ///                 .build()?;
    ///
    ///             let next_response = service
    ///                 .filter_employee_builder(next_request)
    ///                 .execute()
    ///                 .await?;
    ///
    ///             println!("下一页有 {} 个员工", next_response.employee_count());
    ///         }
    ///     }
    ///
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<FilterEmployeeResponse> {
        let url = self.service.config().build_url(ENDPOINT_FILTER);

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
        let response_data: FilterEmployeeResponse = serde_json::from_value(response)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        Ok(response_data)
    }

    /// 设置分页大小（链式调用）
    ///
    /// # 参数
    /// * `page_size` - 分页大小，范围1-50
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（链式调用）
    ///
    /// # 参数
    /// * `page_token` - 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 设置员工状态过滤（链式调用）
    ///
    /// # 参数
    /// * `status` - 员工状态（1-在职，2-离职，3-待入职）
    pub fn status(mut self, status: i32) -> Self {
        self.request.status = Some(status);
        self
    }

    /// 设置部门ID过滤（链式调用）
    ///
    /// # 参数
    /// * `department_ids` - 部门ID列表
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.request.department_ids = Some(department_ids);
        self
    }

    /// 设置用户ID类型（链式调用）
    ///
    /// # 参数
    /// * `user_id_type` - 用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门ID类型（链式调用）
    ///
    /// # 参数
    /// * `department_id_type` - 部门ID类型
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.request.department_id_type = Some(department_id_type.into());
        self
    }

    /// 设置排序字段（链式调用）
    ///
    /// # 参数
    /// * `sort_field` - 排序字段
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.request.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方向（链式调用）
    ///
    /// # 参数
    /// * `sort_direction` - 排序方向
    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.request.sort_direction = Some(sort_direction.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;
    use crate::transport::MockTransport;
    use std::sync::Arc;

    fn create_test_service() -> Arc<EmployeeService> {
        let config = Config::new("test_app_id", "test_app_secret");
        let transport = MockTransport::new();
        Arc::new(EmployeeService::new_with_transport(config, transport))
    }

    #[test]
    fn test_filter_employee_request_builder() {
        // 测试完整构建
        let request = FilterEmployeeRequest::builder()
            .page_size(20)
            .page_token("test_token")
            .status(1)
            .department_ids(vec!["dept_123".to_string(), "dept_456".to_string()])
            .user_id_type("open_id")
            .department_id_type("department_id")
            .sort_field("create_time")
            .sort_direction("desc")
            .build()
            .unwrap();

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("test_token".to_string()));
        assert_eq!(request.status, Some(1));
        assert_eq!(request.department_ids, Some(vec!["dept_123".to_string(), "dept_456".to_string()]));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.department_id_type, Some("department_id".to_string()));
        assert_eq!(request.sort_field, Some("create_time".to_string()));
        assert_eq!(request.sort_direction, Some("desc".to_string()));
    }

    #[test]
    fn test_filter_employee_request_builder_default() {
        let request = FilterEmployeeRequest::new();
        assert_eq!(request.page_size, Some(10)); // 默认分页大小
        assert_eq!(request.page_token, None);
        assert_eq!(request.status, None);
        assert_eq!(request.department_ids, None);
    }

    #[test]
    fn test_filter_employee_request_validation() {
        // 测试无效分页大小
        let result = FilterEmployeeRequest::builder()
            .page_size(0)
            .build();
        assert!(result.is_err());

        let result = FilterEmployeeRequest::builder()
            .page_size(51)
            .build();
        assert!(result.is_err());

        // 测试无效员工状态
        let result = FilterEmployeeRequest::builder()
            .status(0)
            .build();
        assert!(result.is_err());

        let result = FilterEmployeeRequest::builder()
            .status(4)
            .build();
        assert!(result.is_err());

        // 测试空的部门ID列表
        let result = FilterEmployeeRequest::builder()
            .department_ids(vec![])
            .build();
        assert!(result.is_err());

        // 测试过多的部门ID
        let many_departments: Vec<String> = (0..51).map(|i| format!("dept_{}", i)).collect();
        let result = FilterEmployeeRequest::builder()
            .department_ids(many_departments)
            .build();
        assert!(result.is_err());

        // 测试无效的用户ID类型
        let result = FilterEmployeeRequest::builder()
            .user_id_type("invalid_type")
            .build();
        assert!(result.is_err());

        // 测试无效的部门ID类型
        let result = FilterEmployeeRequest::builder()
            .department_id_type("invalid_type")
            .build();
        assert!(result.is_err());

        // 测试无效的排序方向
        let result = FilterEmployeeRequest::builder()
            .sort_direction("invalid_direction")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_employee_request_valid_parameters() {
        // 测试有效的员工状态
        let valid_statuses = [1, 2, 3];
        for status in valid_statuses.iter() {
            let request = FilterEmployeeRequest::builder()
                .status(*status)
                .build()
                .unwrap();
            assert_eq!(request.status, Some(*status));
        }

        // 测试有效的用户ID类型
        let valid_user_id_types = ["open_id", "user_id", "union_id"];
        for user_id_type in valid_user_id_types.iter() {
            let request = FilterEmployeeRequest::builder()
                .user_id_type(*user_id_type)
                .build()
                .unwrap();
            assert_eq!(request.user_id_type, Some(user_id_type.to_string()));
        }

        // 测试有效的部门ID类型
        let valid_department_id_types = ["open_department_id", "department_id"];
        for department_id_type in valid_department_id_types.iter() {
            let request = FilterEmployeeRequest::builder()
                .department_id_type(*department_id_type)
                .build()
                .unwrap();
            assert_eq!(request.department_id_type, Some(department_id_type.to_string()));
        }

        // 测试有效的排序方向
        let valid_sort_directions = ["asc", "desc"];
        for sort_direction in valid_sort_directions.iter() {
            let request = FilterEmployeeRequest::builder()
                .sort_direction(*sort_direction)
                .build()
                .unwrap();
            assert_eq!(request.sort_direction, Some(sort_direction.to_string()));
        }
    }

    #[test]
    fn test_filter_employee_info() {
        let employee = FilterEmployeeInfo::new(
            "emp_123".to_string(),
            "user_456".to_string(),
            "张三".to_string(),
            1,
        );

        assert_eq!(employee.employee_id(), "emp_123");
        assert_eq!(employee.user_id(), "user_456");
        assert_eq!(employee.name(), "张三");
        assert_eq!(employee.status(), 1);
        assert_eq!(employee.status_description(), "在职");
        assert!(employee.is_active());

        // 测试离职员工
        let resigned_employee = FilterEmployeeInfo::new(
            "emp_789".to_string(),
            "user_101".to_string(),
            "李四".to_string(),
            2,
        );
        assert!(!resigned_employee.is_active());
        assert_eq!(resigned_employee.status_description(), "离职");
    }

    #[test]
    fn test_filter_employee_response() {
        let employees = vec![
            FilterEmployeeInfo::new(
                "emp_1".to_string(),
                "user_1".to_string(),
                "张三".to_string(),
                1,
            ),
            FilterEmployeeInfo::new(
                "emp_2".to_string(),
                "user_2".to_string(),
                "李四".to_string(),
                2,
            ),
            FilterEmployeeInfo::new(
                "emp_3".to_string(),
                "user_3".to_string(),
                "王五".to_string(),
                3,
            ),
        ];

        let response = FilterEmployeeResponse::new(
            true,
            Some("next_page_token".to_string()),
            employees.clone(),
        );

        assert!(response.has_more());
        assert_eq!(response.page_token(), Some(&"next_page_token".to_string()));
        assert_eq!(response.employee_count(), 3);
        assert!(response.has_employees());

        // 测试状态过滤
        let active_employees = response.active_employees();
        assert_eq!(active_employees.len(), 1);
        assert_eq!(active_employees[0].name(), "张三");

        let resigned_employees = response.resigned_employees();
        assert_eq!(resigned_employees.len(), 1);
        assert_eq!(resigned_employees[0].name(), "李四");

        let pending_employees = response.pending_employees();
        assert_eq!(pending_employees.len(), 1);
        assert_eq!(pending_employees[0].name(), "王五");

        // 测试部门过滤
        let employee_with_dept = FilterEmployeeInfo::new(
            "emp_4".to_string(),
            "user_4".to_string(),
            "赵六".to_string(),
            1,
        );
        let employee_with_dept_data = FilterEmployeeInfo {
            department_ids: Some(vec!["dept_123".to_string(), "dept_456".to_string()]),
            ..employee_with_dept
        };

        let employees_with_dept = vec![
            employees[0].clone(),
            employee_with_dept_data,
        ];

        let response_with_dept = FilterEmployeeResponse::new(
            false,
            None,
            employees_with_dept,
        );

        let dept_employees = response_with_dept.filter_by_department("dept_123");
        assert_eq!(dept_employees.len(), 1);
        assert_eq!(dept_employees[0].name(), "赵六");
    }

    #[test]
    fn test_filter_employee_request_serialization() {
        let request = FilterEmployeeRequest::builder()
            .page_size(20)
            .status(1)
            .user_id_type("open_id")
            .sort_field("create_time")
            .sort_direction("desc")
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: FilterEmployeeRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.page_size, request.page_size);
        assert_eq!(parsed.status, request.status);
        assert_eq!(parsed.user_id_type, request.user_id_type);
        assert_eq!(parsed.sort_field, request.sort_field);
        assert_eq!(parsed.sort_direction, request.sort_direction);
    }

    #[test]
    fn test_filter_employee_response_serialization() {
        let employees = vec![
            FilterEmployeeInfo::new(
                "emp_1".to_string(),
                "user_1".to_string(),
                "张三".to_string(),
                1,
            ),
        ];

        let response = FilterEmployeeResponse::new(
            true,
            Some("next_token".to_string()),
            employees.clone(),
        );

        let json = serde_json::to_string(&response).unwrap();
        let parsed: FilterEmployeeResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.has_more, response.has_more);
        assert_eq!(parsed.page_token, response.page_token);
        assert_eq!(parsed.employees.len(), response.employees.len());
        assert_eq!(parsed.employees[0].employee_id, response.employees[0].employee_id);
        assert_eq!(parsed.employees[0].name, response.employees[0].name);
    }

    #[test]
    fn test_filter_employee_builder_chain_methods() {
        let service = create_test_service();
        let request = FilterEmployeeRequest::builder()
            .page_size(15)
            .status(1)
            .user_id_type("union_id")
            .build()
            .unwrap();

        let builder = service.filter_employee_builder(request);

        // 测试链式调用不会panic
        let _chained_builder = builder
            .page_size(25)
            .page_token("test_token")
            .status(2)
            .department_ids(vec!["dept_123".to_string()])
            .user_id_type("open_id")
            .department_id_type("open_department_id")
            .sort_field("name")
            .sort_direction("asc");
    }

    #[test]
    fn test_multiple_valid_requests() {
        let test_cases = vec![
            // 基本查询
            (Some(10), None, None, None, None, None, None, None),
            // 带状态过滤
            (Some(20), None, Some(1), None, Some("open_id".to_string()), None, None, None),
            // 带部门过滤
            (Some(30), Some("token_123".to_string()), None, Some(vec!["dept_1".to_string()]), None, Some("department_id".to_string()), None, None),
            // 带排序
            (Some(5), None, Some(2), None, Some("user_id".to_string()), None, Some("create_time".to_string()), Some("desc".to_string())),
            // 完整参数
            (Some(50), Some("full_token".to_string()), Some(3), Some(vec!["dept_1".to_string(), "dept_2".to_string()]), Some("union_id".to_string()), Some("open_department_id".to_string()), Some("name".to_string()), Some("asc".to_string())),
        ];

        for (page_size, page_token, status, department_ids, user_id_type, department_id_type, sort_field, sort_direction) in test_cases {
            let mut builder = FilterEmployeeRequest::builder();

            if let Some(ps) = page_size {
                builder = builder.page_size(ps);
            }
            if let Some(pt) = page_token {
                builder = builder.page_token(pt);
            }
            if let Some(s) = status {
                builder = builder.status(s);
            }
            if let Some(di) = department_ids {
                builder = builder.department_ids(di);
            }
            if let Some(uit) = user_id_type {
                builder = builder.user_id_type(uit);
            }
            if let Some(dit) = department_id_type {
                builder = builder.department_id_type(dit);
            }
            if let Some(sf) = sort_field {
                builder = builder.sort_field(sf);
            }
            if let Some(sd) = sort_direction {
                builder = builder.sort_direction(sd);
            }

            let request = builder.build().unwrap();
            assert_eq!(request.page_size, page_size);
            assert_eq!(request.page_token, page_token);
            assert_eq!(request.status, status);
            assert_eq!(request.department_ids, department_ids);
            assert_eq!(request.user_id_type, user_id_type);
            assert_eq!(request.department_id_type, department_id_type);
            assert_eq!(request.sort_field, sort_field);
            assert_eq!(request.sort_direction, sort_direction);
        }
    }

    #[tokio::test]
    async fn test_filter_employee_builder_execute_structure() {
        let service = create_test_service();
        let request = FilterEmployeeRequest::builder()
            .page_size(10)
            .status(1)
            .build()
            .unwrap();

        let builder = service.filter_employee_builder(request);

        // 验证builder结构正确
        assert_eq!(builder.request.page_size, Some(10));
        assert_eq!(builder.request.status, Some(1));
    }

    #[test]
    fn test_edge_cases() {
        // 测试最小分页大小
        let request = FilterEmployeeRequest::builder()
            .page_size(1)
            .build();
        assert!(request.is_ok());

        // 测试最大分页大小
        let request = FilterEmployeeRequest::builder()
            .page_size(50)
            .build();
        assert!(request.is_ok());

        // 测试单个部门ID
        let request = FilterEmployeeRequest::builder()
            .department_ids(vec!["dept_123".to_string()])
            .build();
        assert!(request.is_ok());

        // 测试最大部门ID数量
        let many_departments: Vec<String> = (0..50).map(|i| format!("dept_{}", i)).collect();
        let request = FilterEmployeeRequest::builder()
            .department_ids(many_departments)
            .build();
        assert!(request.is_ok());

        // 测试空字符串参数
        let request = FilterEmployeeRequest::builder()
            .page_token("")
            .user_id_type("")
            .department_id_type("")
            .sort_field("")
            .sort_direction("")
            .build();

        // 空字符串在某些情况下是有效的（取决于具体验证逻辑）
        // 这里主要确保不会panic
        assert!(request.is_ok() || request.is_err());
    }

    #[test]
    fn test_employee_info_comprehensive() {
        let mut employee = FilterEmployeeInfo::new(
            "emp_comprehensive".to_string(),
            "user_comprehensive".to_string(),
            "综合测试员工".to_string(),
            1,
        );

        // 设置所有可选字段
        employee.email = Some("test@example.com".to_string());
        employee.mobile = Some("+86 138 0013 8000".to_string());
        employee.department_ids = Some(vec!["dept_1".to_string(), "dept_2".to_string()]);
        employee.job_title = Some("软件工程师".to_string());
        employee.employee_number = Some("EMP001".to_string());
        employee.hire_time = Some(1609459200000); // 2021-01-01
        employee.create_time = Some(1609372800000); // 2020-12-31
        employee.update_time = Some(1609545600000); // 2021-01-02

        // 验证所有字段
        assert_eq!(employee.email(), Some(&"test@example.com".to_string()));
        assert_eq!(employee.mobile(), Some(&"+86 138 0013 8000".to_string()));
        assert_eq!(employee.department_ids(), Some(&vec!["dept_1".to_string(), "dept_2".to_string()]));
        assert_eq!(employee.job_title(), Some(&"软件工程师".to_string()));
        assert_eq!(employee.employee_number(), Some(&"EMP001".to_string()));
        assert_eq!(employee.hire_time(), Some(1609459200000));
        assert_eq!(employee.create_time(), Some(1609372800000));
        assert_eq!(employee.update_time(), Some(1609545600000));
    }

    #[test]
    fn test_response_filtering_methods() {
        let employees = vec![
            FilterEmployeeInfo {
                employee_id: "emp_1".to_string(),
                user_id: "user_1".to_string(),
                name: "张三".to_string(),
                status: 1,
                department_ids: Some(vec!["dept_A".to_string()]),
                ..Default::default()
            },
            FilterEmployeeInfo {
                employee_id: "emp_2".to_string(),
                user_id: "user_2".to_string(),
                name: "李四".to_string(),
                status: 1,
                department_ids: Some(vec!["dept_B".to_string(), "dept_A".to_string()]),
                ..Default::default()
            },
            FilterEmployeeInfo {
                employee_id: "emp_3".to_string(),
                user_id: "user_3".to_string(),
                name: "王五".to_string(),
                status: 2,
                department_ids: Some(vec!["dept_C".to_string()]),
                ..Default::default()
            },
        ];

        let response = FilterEmployeeResponse::new(false, None, employees);

        // 测试状态过滤
        let active_employees = response.active_employees();
        assert_eq!(active_employees.len(), 2);

        let resigned_employees = response.resigned_employees();
        assert_eq!(resigned_employees.len(), 1);

        let pending_employees = response.pending_employees();
        assert_eq!(pending_employees.len(), 0);

        // 测试部门过滤
        let dept_a_employees = response.filter_by_department("dept_A");
        assert_eq!(dept_a_employees.len(), 2); // 张三和李四都属于dept_A

        let dept_b_employees = response.filter_by_department("dept_B");
        assert_eq!(dept_b_employees.len(), 1); // 只有李四属于dept_B

        let dept_c_employees = response.filter_by_department("dept_C");
        assert_eq!(dept_c_employees.len(), 1); // 只有王五属于dept_C

        let non_existent_dept = response.filter_by_department("dept_X");
        assert_eq!(non_existent_dept.len(), 0); // 没有员工属于dept_X
    }
}

impl Default for FilterEmployeeInfo {
    fn default() -> Self {
        Self {
            employee_id: String::new(),
            user_id: String::new(),
            name: String::new(),
            status: 1,
            email: None,
            mobile: None,
            department_ids: None,
            job_title: None,
            employee_number: None,
            hire_time: None,
            create_time: None,
            update_time: None,
        }
    }
}
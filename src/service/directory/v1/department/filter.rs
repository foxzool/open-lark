use crate::config::Config;
use crate::error::SDKError;
use crate::response::SDKResult;
use crate::service_trait::Service;
use crate::transport::Transport;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取部门列表API端点
pub const ENDPOINT_FILTER: &str = "/open-apis/directory/v1/departments/filter";

/// 获取部门列表请求体
///
/// 提供灵活的部门查询功能，支持分页、过滤和排序
/// 与API #90的区别：API #90通过精确ID查询，API #91通过条件过滤查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterDepartmentRequest {
    /// 分页大小
    /// 分页大小，最大值为50，默认值为10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    /// 用于获取下一页的标记，首次请求时为空
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 父部门ID过滤
    /// 需要查询的父部门ID，如果不为空，则只返回该部门的子部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 用户ID类型
    /// 支持：open_id、user_id、union_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    /// 支持：open_department_id、department_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 部门状态过滤
    /// 可选值：active（激活）、deleted（已删除）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_status: Option<String>,
    /// 排序字段
    /// 支持按字段排序，如：department_id、name、create_time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    /// 排序方向
    /// asc：升序，desc：降序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
}

impl FilterDepartmentRequest {
    /// 创建新的获取部门列表请求
    pub fn new() -> Self {
        Self {
            page_size: None,
            page_token: None,
            parent_department_id: None,
            user_id_type: None,
            department_id_type: None,
            department_status: None,
            sort_field: None,
            sort_direction: None,
        }
    }

    /// 创建获取部门列表请求的构建器
    pub fn builder() -> FilterDepartmentBuilder {
        FilterDepartmentBuilder::default()
    }
}

/// 获取部门列表请求构建器
///
/// 提供流畅的API来构建获取部门列表请求，支持方法链调用
#[derive(Debug, Clone, Default)]
pub struct FilterDepartmentBuilder {
    page_size: Option<i32>,
    page_token: Option<String>,
    parent_department_id: Option<String>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
    department_status: Option<String>,
    sort_field: Option<String>,
    sort_direction: Option<String>,
}

impl FilterDepartmentBuilder {
    /// 设置分页大小
    ///
    /// # 参数
    /// * `page_size` - 分页大小，范围：1-50
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterDepartmentBuilder::default()
    ///     .page_size(20);
    /// ```
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    ///
    /// # 参数
    /// * `page_token` - 分页标记，用于获取下一页
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterDepartmentBuilder::default()
    ///     .page_token("next_page_token".to_string());
    /// ```
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置父部门ID过滤
    ///
    /// # 参数
    /// * `parent_department_id` - 父部门ID
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterDepartmentBuilder::default()
    ///     .parent_department_id("od_parent".to_string());
    /// ```
    pub fn parent_department_id(mut self, parent_department_id: impl Into<String>) -> Self {
        self.parent_department_id = Some(parent_department_id.into());
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// * `user_id_type` - 用户ID类型
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterDepartmentBuilder::default()
    ///     .user_id_type("open_id".to_string());
    /// ```
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门ID类型
    ///
    /// # 参数
    /// * `department_id_type` - 部门ID类型
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterDepartmentBuilder::default()
    ///     .department_id_type("open_department_id".to_string());
    /// ```
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 设置部门状态过滤
    ///
    /// # 参数
    /// * `department_status` - 部门状态
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterDepartmentBuilder::default()
    ///     .department_status("active".to_string());
    /// ```
    pub fn department_status(mut self, department_status: impl Into<String>) -> Self {
        self.department_status = Some(department_status.into());
        self
    }

    /// 设置排序字段
    ///
    /// # 参数
    /// * `sort_field` - 排序字段
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterDepartmentBuilder::default()
    ///     .sort_field("name".to_string());
    /// ```
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方向
    ///
    /// # 参数
    /// * `sort_direction` - 排序方向
    ///
    /// # 示例
    /// ```rust
    /// let builder = FilterDepartmentBuilder::default()
    ///     .sort_direction("asc".to_string());
    /// ```
    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.sort_direction = Some(sort_direction.into());
        self
    }

    /// 构建获取部门列表请求
    ///
    /// # 返回
    /// 成功返回获取部门列表请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果分页大小超出范围，返回错误
    /// * 如果用户ID类型无效，返回错误
    /// * 如果部门ID类型无效，返回错误
    /// * 如果部门状态无效，返回错误
    /// * 如果排序字段无效，返回错误
    /// * 如果排序方向无效，返回错误
    pub fn build(self) -> SDKResult<FilterDepartmentRequest> {
        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 50 {
                return Err(SDKError::ValidationError("分页大小必须在1-50之间".to_string()));
            }
        }

        // 验证用户ID类型
        if let Some(user_id_type) = &self.user_id_type {
            let valid_types = ["open_id", "user_id", "union_id"];
            if !valid_types.contains(&user_id_type.as_str()) {
                return Err(SDKError::ValidationError(
                    "用户ID类型必须为open_id、user_id或union_id".to_string(),
                ));
            }
        }

        // 验证部门ID类型
        if let Some(department_id_type) = &self.department_id_type {
            let valid_types = ["open_department_id", "department_id"];
            if !valid_types.contains(&department_id_type.as_str()) {
                return Err(SDKError::ValidationError(
                    "部门ID类型必须为open_department_id或department_id".to_string(),
                ));
            }
        }

        // 验证部门状态
        if let Some(department_status) = &self.department_status {
            let valid_statuses = ["active", "deleted"];
            if !valid_statuses.contains(&department_status.as_str()) {
                return Err(SDKError::ValidationError(
                    "部门状态必须为active或deleted".to_string(),
                ));
            }
        }

        // 验证排序字段
        if let Some(sort_field) = &self.sort_field {
            let valid_fields = [
                "department_id", "name", "en_name", "parent_department_id",
                "leader_user_id", "order", "status", "create_time", "update_time"
            ];
            if !valid_fields.contains(&sort_field.as_str()) {
                return Err(SDKError::ValidationError(
                    "排序字段无效".to_string(),
                ));
            }
        }

        // 验证排序方向
        if let Some(sort_direction) = &self.sort_direction {
            let valid_directions = ["asc", "desc"];
            if !valid_directions.contains(&sort_direction.as_str()) {
                return Err(SDKError::ValidationError(
                    "排序方向必须为asc或desc".to_string(),
                ));
            }
        }

        Ok(FilterDepartmentRequest {
            page_size: self.page_size,
            page_token: self.page_token,
            parent_department_id: self.parent_department_id,
            user_id_type: self.user_id_type,
            department_id_type: self.department_id_type,
            department_status: self.department_status,
            sort_field: self.sort_field,
            sort_direction: self.sort_direction,
        })
    }
}

/// 获取部门列表响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterDepartmentResponse {
    /// 部门信息列表
    pub department: Vec<DepartmentInfo>,
    /// 分页标记
    /// 用于获取下一页的标记，为空表示没有下一页
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有下一页
    pub has_more: bool,
}

/// 部门信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentInfo {
    /// 部门ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 英文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 父部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门主管用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    /// 部门排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// 部门状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 部门描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 成员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
}

impl FilterDepartmentResponse {
    /// 创建新的获取部门列表响应
    ///
    /// # 参数
    /// * `department` - 部门信息列表
    /// * `page_token` - 分页标记
    /// * `has_more` - 是否有下一页
    pub fn new(department: Vec<DepartmentInfo>, page_token: Option<String>, has_more: bool) -> Self {
        Self {
            department,
            page_token,
            has_more,
        }
    }

    /// 获取部门信息列表
    ///
    /// # 返回
    /// 部门信息列表
    pub fn department(&self) -> &Vec<DepartmentInfo> {
        &self.department
    }

    /// 获取部门数量
    ///
    /// # 返回
    /// 返回的部门数量
    pub fn department_count(&self) -> usize {
        self.department.len()
    }

    /// 获取分页标记
    ///
    /// # 返回
    /// 分页标记
    pub fn page_token(&self) -> Option<&str> {
        self.page_token.as_deref()
    }

    /// 是否有下一页
    ///
    /// # 返回
    /// true表示有下一页，false表示没有下一页
    pub fn has_more(&self) -> bool {
        self.has_more
    }
}

/// 获取部门列表构建器
///
/// 提供流畅的API来获取部门列表，支持方法链调用和完整的错误处理
#[derive(Debug, Clone)]
pub struct FilterDepartmentBuilder {
    service: Arc<DepartmentService>,
    request: FilterDepartmentRequest,
}

impl FilterDepartmentBuilder {
    /// 创建新的获取部门列表构建器
    ///
    /// # 参数
    /// * `service` - 部门服务实例
    /// * `request` - 获取部门列表请求
    pub(crate) fn new(service: Arc<DepartmentService>, request: FilterDepartmentRequest) -> Self {
        Self { service, request }
    }

    /// 执行获取部门列表操作
    ///
    /// 向飞书API发送POST请求来获取部门列表
    ///
    /// # 返回
    /// * `Ok(FilterDepartmentResponse)` - 获取成功，返回部门列表
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
    /// use open_lark::service::directory::v1::department::filter::{FilterDepartmentRequest, FilterDepartmentResponse};
    ///
    /// async fn filter_department_example(
    ///     service: Arc<DepartmentService>,
    /// ) -> Result<FilterDepartmentResponse, Box<dyn std::error::Error>> {
    ///     let request = FilterDepartmentRequest::builder()
    ///         .page_size(20)
    ///         .parent_department_id("od_parent")
    ///         .user_id_type("open_id")
    ///         .department_id_type("open_department_id")
    ///         .department_status("active")
    ///         .sort_field("name")
    ///         .sort_direction("asc")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .filter_department_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("成功获取{}个部门信息", response.department_count());
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<FilterDepartmentResponse> {
        let url = self.service.config().build_url(ENDPOINT_FILTER);

        // 构建查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &self.request.user_id_type {
            query_params.push(("user_id_type", user_id_type.clone()));
        }
        if let Some(department_id_type) = &self.request.department_id_type {
            query_params.push(("department_id_type", department_id_type.clone()));
        }

        // 构建请求体
        let mut body = serde_json::Map::new();
        if let Some(page_size) = self.request.page_size {
            body.insert("page_size".to_string(), serde_json::Value::Number(page_size.into()));
        }
        if let Some(page_token) = self.request.page_token {
            body.insert("page_token".to_string(), serde_json::Value::String(page_token));
        }
        if let Some(parent_department_id) = self.request.parent_department_id {
            body.insert("parent_department_id".to_string(), serde_json::Value::String(parent_department_id));
        }
        if let Some(department_status) = self.request.department_status {
            body.insert("department_status".to_string(), serde_json::Value::String(department_status));
        }
        if let Some(sort_field) = self.request.sort_field {
            body.insert("sort_field".to_string(), serde_json::Value::String(sort_field));
        }
        if let Some(sort_direction) = self.request.sort_direction {
            body.insert("sort_direction".to_string(), serde_json::Value::String(sort_direction));
        }

        // 发送HTTP POST请求
        let response = self
            .service
            .transport()
            .post_with_query(&url, Some(serde_json::Value::Object(body)), &query_params)
            .await?;

        // 解析响应体
        let response_data: FilterDepartmentResponse = serde_json::from_value(response)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        Ok(response_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_department_request_builder() {
        // 测试正常构建
        let request = FilterDepartmentRequest::builder()
            .page_size(20)
            .parent_department_id("od_parent".to_string())
            .user_id_type("open_id")
            .department_id_type("open_department_id")
            .department_status("active")
            .sort_field("name")
            .sort_direction("asc")
            .build()
            .unwrap();

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.parent_department_id, Some("od_parent".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.department_id_type, Some("open_department_id".to_string()));
        assert_eq!(request.department_status, Some("active".to_string()));
        assert_eq!(request.sort_field, Some("name".to_string()));
        assert_eq!(request.sort_direction, Some("asc".to_string()));
    }

    #[test]
    fn test_filter_department_request_builder_default() {
        let request = FilterDepartmentRequest::builder().build().unwrap();

        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.parent_department_id, None);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
        assert_eq!(request.department_status, None);
        assert_eq!(request.sort_field, None);
        assert_eq!(request.sort_direction, None);
    }

    #[test]
    fn test_filter_department_request_validation() {
        // 测试分页大小过小
        let result = FilterDepartmentRequest::builder()
            .page_size(0)
            .build();
        assert!(result.is_err());

        // 测试分页大小过大
        let result = FilterDepartmentRequest::builder()
            .page_size(51)
            .build();
        assert!(result.is_err());

        // 测试无效用户ID类型
        let result = FilterDepartmentRequest::builder()
            .user_id_type("invalid_type")
            .build();
        assert!(result.is_err());

        // 测试无效部门ID类型
        let result = FilterDepartmentRequest::builder()
            .department_id_type("invalid_type")
            .build();
        assert!(result.is_err());

        // 测试无效部门状态
        let result = FilterDepartmentRequest::builder()
            .department_status("invalid_status")
            .build();
        assert!(result.is_err());

        // 测试无效排序字段
        let result = FilterDepartmentRequest::builder()
            .sort_field("invalid_field")
            .build();
        assert!(result.is_err());

        // 测试无效排序方向
        let result = FilterDepartmentRequest::builder()
            .sort_direction("invalid_direction")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_department_request_edge_cases() {
        // 测试最小有效分页大小
        let result = FilterDepartmentRequest::builder()
            .page_size(1)
            .build();
        assert!(result.is_ok());

        // 测试最大有效分页大小
        let result = FilterDepartmentRequest::builder()
            .page_size(50)
            .build();
        assert!(result.is_ok());

        // 测试空字符串page_token
        let request = FilterDepartmentRequest::builder()
            .page_token("")
            .build()
            .unwrap();
        assert_eq!(request.page_token, Some("".to_string()));
    }

    #[test]
    fn test_filter_department_response() {
        let departments = vec![
            DepartmentInfo {
                department_id: "od_123".to_string(),
                name: "技术部".to_string(),
                en_name: Some("Tech Department".to_string()),
                parent_department_id: Some("od_root".to_string()),
                leader_user_id: Some("user_456".to_string()),
                order: Some("1".to_string()),
                status: Some("active".to_string()),
                create_time: Some("2024-01-01T00:00:00Z".to_string()),
                update_time: Some("2024-01-02T00:00:00Z".to_string()),
                description: Some("负责技术研发".to_string()),
                member_count: Some(10),
            },
        ];

        let response = FilterDepartmentResponse::new(
            departments.clone(),
            Some("next_page_token".to_string()),
            true,
        );

        assert_eq!(response.department_count(), 1);
        assert_eq!(response.page_token(), Some("next_page_token"));
        assert!(response.has_more());
        assert_eq!(response.department()[0].name, "技术部");
        assert_eq!(response.department()[0].member_count, Some(10));
    }

    #[test]
    fn test_filter_department_request_new() {
        let request = FilterDepartmentRequest::new();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.parent_department_id, None);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
        assert_eq!(request.department_status, None);
        assert_eq!(request.sort_field, None);
        assert_eq!(request.sort_direction, None);
    }

    #[test]
    fn test_multiple_valid_requests() {
        let test_cases = vec![
            // 最小请求
            vec![],
            // 分页请求
            vec![("page_size", "20")],
            // 过滤请求
            vec![("parent_department_id", "od_parent"), ("department_status", "active")],
            // 排序请求
            vec![("sort_field", "name"), ("sort_direction", "desc")],
            // 完整请求
            vec![
                ("page_size", "10"),
                ("user_id_type", "open_id"),
                ("department_id_type", "open_department_id"),
                ("department_status", "active"),
                ("sort_field", "create_time"),
                ("sort_direction", "asc"),
            ],
        ];

        for params in test_cases {
            let mut builder = FilterDepartmentRequest::builder();

            for (key, value) in params {
                match key {
                    "page_size" => builder = builder.page_size(value.parse().unwrap()),
                    "parent_department_id" => builder = builder.parent_department_id(value),
                    "user_id_type" => builder = builder.user_id_type(value),
                    "department_id_type" => builder = builder.department_id_type(value),
                    "department_status" => builder = builder.department_status(value),
                    "sort_field" => builder = builder.sort_field(value),
                    "sort_direction" => builder = builder.sort_direction(value),
                    _ => {}
                }
            }

            let request = builder.build().unwrap();
            // 验证请求构建成功
            assert!(true);
        }
    }

    #[test]
    fn test_department_info_creation() {
        let department = DepartmentInfo {
            department_id: "od_test".to_string(),
            name: "测试部门".to_string(),
            en_name: Some("Test Department".to_string()),
            parent_department_id: None,
            leader_user_id: None,
            order: None,
            status: Some("active".to_string()),
            create_time: None,
            update_time: None,
            description: Some("测试用部门".to_string()),
            member_count: Some(5),
        };

        assert_eq!(department.department_id, "od_test");
        assert_eq!(department.name, "测试部门");
        assert_eq!(department.en_name, Some("Test Department".to_string()));
        assert_eq!(department.status, Some("active".to_string()));
        assert_eq!(department.description, Some("测试用部门".to_string()));
        assert_eq!(department.member_count, Some(5));
    }

    #[test]
    fn test_filter_department_request_serialization() {
        let request = FilterDepartmentRequest::builder()
            .page_size(20)
            .parent_department_id("od_parent".to_string())
            .user_id_type("open_id")
            .department_id_type("open_department_id")
            .department_status("active")
            .sort_field("name")
            .sort_direction("asc")
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: FilterDepartmentRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.page_size, request.page_size);
        assert_eq!(parsed.parent_department_id, request.parent_department_id);
        assert_eq!(parsed.user_id_type, request.user_id_type);
        assert_eq!(parsed.department_id_type, request.department_id_type);
        assert_eq!(parsed.department_status, request.department_status);
        assert_eq!(parsed.sort_field, request.sort_field);
        assert_eq!(parsed.sort_direction, request.sort_direction);
    }

    #[test]
    fn test_filter_department_response_serialization() {
        let departments = vec![
            DepartmentInfo {
                department_id: "od_123".to_string(),
                name: "技术部".to_string(),
                en_name: None,
                parent_department_id: None,
                leader_user_id: None,
                order: None,
                status: None,
                create_time: None,
                update_time: None,
                description: None,
                member_count: None,
            },
        ];

        let response = FilterDepartmentResponse::new(
            departments.clone(),
            Some("token".to_string()),
            false,
        );

        let json = serde_json::to_string(&response).unwrap();
        let parsed: FilterDepartmentResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.department_count(), 1);
        assert_eq!(parsed.page_token(), Some("token"));
        assert!(!parsed.has_more());
        assert_eq!(parsed.department()[0].department_id, "od_123");
        assert_eq!(parsed.department()[0].name, "技术部");
    }

    #[test]
    fn test_valid_sort_fields() {
        let valid_fields = [
            "department_id", "name", "en_name", "parent_department_id",
            "leader_user_id", "order", "status", "create_time", "update_time"
        ];

        for field in valid_fields.iter() {
            let request = FilterDepartmentRequest::builder()
                .sort_field(*field)
                .build()
                .unwrap();

            assert_eq!(request.sort_field, Some(field.to_string()));
        }
    }

    #[test]
    fn test_valid_sort_directions() {
        let valid_directions = ["asc", "desc"];

        for direction in valid_directions.iter() {
            let request = FilterDepartmentRequest::builder()
                .sort_direction(*direction)
                .build()
                .unwrap();

            assert_eq!(request.sort_direction, Some(direction.to_string()));
        }
    }
}
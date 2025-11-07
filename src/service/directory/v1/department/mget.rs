use crate::core::config::Config;
use crate::core::error::SDKError;
use crate::core::response::SDKResult;
use crate::core::service_trait::Service;
use crate::core::transport::Transport;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量获取部门信息API端点
pub const ENDPOINT_MGET: &str = "/open-apis/directory/v1/departments/mget";

/// 批量获取部门信息请求体
///
/// 根据部门ID列表批量获取部门详细信息
/// 支持一次性查询多个部门的基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MGetDepartmentRequest {
    /// 部门ID列表
    /// 必填参数，最多支持50个部门ID
    pub department_ids: Vec<String>,
    /// 用户ID类型
    /// 支持：open_id、user_id、union_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    /// 支持：open_department_id、department_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl MGetDepartmentRequest {
    /// 创建新的批量获取部门信息请求
    ///
    /// # 参数
    /// * `department_ids` - 部门ID列表
    pub fn new(department_ids: Vec<String>) -> Self {
        Self {
            department_ids,
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 创建批量获取部门信息请求的构建器
    pub fn builder() -> MGetDepartmentBuilder {
        MGetDepartmentBuilder::default()
    }
}

/// 批量获取部门信息请求构建器
///
/// 提供流畅的API来构建批量获取部门信息请求，支持方法链调用
#[derive(Debug, Clone, Default)]
pub struct MGetDepartmentBuilder {
    department_ids: Vec<String>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
}

impl MGetDepartmentBuilder {
    /// 设置部门ID列表
    ///
    /// # 参数
    /// * `department_ids` - 部门ID列表，最多50个
    ///
    /// # 示例
    /// ```rust
    /// let builder = MGetDepartmentBuilder::default()
    ///     .department_ids(vec!["od_123".to_string(), "od_456".to_string()]);
    /// ```
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = department_ids;
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// * `user_id_type` - 用户ID类型
    ///
    /// # 示例
    /// ```rust
    /// let builder = MGetDepartmentBuilder::default()
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
    /// let builder = MGetDepartmentBuilder::default()
    ///     .department_id_type("open_department_id".to_string());
    /// ```
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 构建批量获取部门信息请求
    ///
    /// # 返回
    /// 成功返回批量获取部门信息请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果部门ID列表为空，返回错误
    /// * 如果部门ID数量超过50个，返回错误
    /// * 如果用户ID类型无效，返回错误
    /// * 如果部门ID类型无效，返回错误
    pub fn build(self) -> SDKResult<MGetDepartmentRequest> {
        // 验证部门ID列表
        if self.department_ids.is_empty() {
            return Err(SDKError::ValidationError("部门ID列表不能为空".to_string()));
        }

        // 验证部门ID数量限制
        if self.department_ids.len() > 50 {
            return Err(SDKError::ValidationError("部门ID数量不能超过50个".to_string()));
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

        Ok(MGetDepartmentRequest {
            department_ids: self.department_ids,
            user_id_type: self.user_id_type,
            department_id_type: self.department_id_type,
        })
    }
}

/// 批量获取部门信息响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MGetDepartmentResponse {
    /// 部门信息列表
    /// 返回查询到的部门详细信息
    pub department: Vec<DepartmentInfo>,
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
}

impl MGetDepartmentResponse {
    /// 创建新的批量获取部门信息响应
    ///
    /// # 参数
    /// * `department` - 部门信息列表
    pub fn new(department: Vec<DepartmentInfo>) -> Self {
        Self { department }
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
}

/// 批量获取部门信息构建器
///
/// 提供流畅的API来批量获取部门信息，支持方法链调用和完整的错误处理
#[derive(Debug, Clone)]
pub struct MGetDepartmentBuilder {
    service: Arc<DepartmentService>,
    request: MGetDepartmentRequest,
}

impl MGetDepartmentBuilder {
    /// 创建新的批量获取部门信息构建器
    ///
    /// # 参数
    /// * `service` - 部门服务实例
    /// * `request` - 批量获取部门信息请求
    pub(crate) fn new(service: Arc<DepartmentService>, request: MGetDepartmentRequest) -> Self {
        Self { service, request }
    }

    /// 执行批量获取部门信息操作
    ///
    /// 向飞书API发送POST请求来批量获取部门信息
    ///
    /// # 返回
    /// * `Ok(MGetDepartmentResponse)` - 获取成功，返回部门信息列表
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
    /// use open_lark::service::directory::v1::department::mget::{MGetDepartmentRequest, MGetDepartmentResponse};
    ///
    /// async fn mget_department_example(
    ///     service: Arc<DepartmentService>,
    /// ) -> Result<MGetDepartmentResponse, Box<dyn std::error::Error>> {
    ///     let request = MGetDepartmentRequest::builder()
    ///         .department_ids(vec!["od_123".to_string(), "od_456".to_string()])
    ///         .user_id_type("open_id")
    ///         .department_id_type("open_department_id")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .mget_department_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("成功获取{}个部门信息", response.department_count());
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<MGetDepartmentResponse> {
        let url = self.service.config().build_url(ENDPOINT_MGET);

        // 构建查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &self.request.user_id_type {
            query_params.push(("user_id_type", user_id_type.clone()));
        }
        if let Some(department_id_type) = &self.request.department_id_type {
            query_params.push(("department_id_type", department_id_type.clone()));
        }

        // 构建请求体
        let body = serde_json::json!({
            "department_ids": self.request.department_ids
        });

        // 发送HTTP POST请求
        let response = self
            .service
            .transport()
            .post_with_query(&url, Some(body), &query_params)
            .await?;

        // 解析响应体
        let response_data: MGetDepartmentResponse = serde_json::from_value(response)
            .map_err(|e| SDKError::SerializationError(e.to_string()))?;

        Ok(response_data)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mget_department_request_builder() {
        // 测试正常构建
        let request = MGetDepartmentRequest::builder()
            .department_ids(vec!["od_123".to_string(), "od_456".to_string()])
            .user_id_type("open_id")
            .department_id_type("open_department_id")
            .build()
            .unwrap();

        assert_eq!(request.department_ids.len(), 2);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.department_id_type, Some("open_department_id".to_string()));
    }

    #[test]
    fn test_mget_department_request_builder_only_required() {
        let request = MGetDepartmentRequest::builder()
            .department_ids(vec!["od_123".to_string()])
            .build()
            .unwrap();

        assert_eq!(request.department_ids.len(), 1);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_mget_department_request_validation() {
        // 测试空部门ID列表
        let result = MGetDepartmentRequest::builder().build();
        assert!(result.is_err());

        // 测试部门ID数量超过限制
        let many_ids: Vec<String> = (0..51).map(|i| format!("od_{}", i)).collect();
        let result = MGetDepartmentRequest::builder()
            .department_ids(many_ids)
            .build();
        assert!(result.is_err());

        // 测试无效用户ID类型
        let result = MGetDepartmentRequest::builder()
            .department_ids(vec!["od_123".to_string()])
            .user_id_type("invalid_type")
            .build();
        assert!(result.is_err());

        // 测试无效部门ID类型
        let result = MGetDepartmentRequest::builder()
            .department_ids(vec!["od_123".to_string()])
            .department_id_type("invalid_type")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_mget_department_request_valid_id_types() {
        let valid_user_types = ["open_id", "user_id", "union_id"];
        let valid_department_types = ["open_department_id", "department_id"];

        for user_type in valid_user_types.iter() {
            let request = MGetDepartmentRequest::builder()
                .department_ids(vec!["od_123".to_string()])
                .user_id_type(*user_type)
                .build()
                .unwrap();

            assert_eq!(request.user_id_type, Some(user_type.to_string()));
        }

        for dept_type in valid_department_types.iter() {
            let request = MGetDepartmentRequest::builder()
                .department_ids(vec!["od_123".to_string()])
                .department_id_type(*dept_type)
                .build()
                .unwrap();

            assert_eq!(request.department_id_type, Some(dept_type.to_string()));
        }
    }

    #[test]
    fn test_mget_department_request_edge_cases() {
        // 测试最大有效部门ID数量（50个）
        let max_ids: Vec<String> = (0..50).map(|i| format!("od_{}", i)).collect();
        let result = MGetDepartmentRequest::builder()
            .department_ids(max_ids)
            .build();
        assert!(result.is_ok());

        // 测试单个部门ID
        let request = MGetDepartmentRequest::builder()
            .department_ids(vec!["od_single".to_string()])
            .build()
            .unwrap();
        assert_eq!(request.department_ids.len(), 1);
    }

    #[test]
    fn test_mget_department_response() {
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
            },
        ];

        let response = MGetDepartmentResponse::new(departments.clone());

        assert_eq!(response.department_count(), 1);
        assert_eq!(response.department()[0].name, "技术部");
        assert_eq!(response.department()[0].en_name, Some("Tech Department".to_string()));
    }

    #[test]
    fn test_mget_department_request_new() {
        let ids = vec!["od_123".to_string(), "od_456".to_string()];
        let request = MGetDepartmentRequest::new(ids.clone());
        assert_eq!(request.department_ids, ids);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_multiple_valid_requests() {
        let test_cases = vec![
            (vec!["od_123"], Some("open_id"), Some("open_department_id")),
            (vec!["od_456", "od_789"], Some("user_id"), None),
            (vec!["od_001"], None, Some("department_id")),
        ];

        for (ids, user_type, dept_type) in test_cases {
            let mut builder = MGetDepartmentRequest::builder()
                .department_ids(ids.into_iter().map(|s| s.to_string()).collect());

            if let Some(user_type) = user_type {
                builder = builder.user_id_type(user_type);
            }

            if let Some(dept_type) = dept_type {
                builder = builder.department_id_type(dept_type);
            }

            let request = builder.build().unwrap();
            assert!(!request.department_ids.is_empty());
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
        };

        assert_eq!(department.department_id, "od_test");
        assert_eq!(department.name, "测试部门");
        assert_eq!(department.en_name, Some("Test Department".to_string()));
        assert_eq!(department.status, Some("active".to_string()));
        assert_eq!(department.description, Some("测试用部门".to_string()));
    }

    #[test]
    fn test_mget_department_request_serialization() {
        let request = MGetDepartmentRequest::builder()
            .department_ids(vec!["od_123".to_string(), "od_456".to_string()])
            .user_id_type("open_id")
            .department_id_type("open_department_id")
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let parsed: MGetDepartmentRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.department_ids, request.department_ids);
        assert_eq!(parsed.user_id_type, request.user_id_type);
        assert_eq!(parsed.department_id_type, request.department_id_type);
    }

    #[test]
    fn test_mget_department_response_serialization() {
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
            },
        ];

        let response = MGetDepartmentResponse::new(departments.clone());

        let json = serde_json::to_string(&response).unwrap();
        let parsed: MGetDepartmentResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.department_count(), 1);
        assert_eq!(parsed.department()[0].department_id, "od_123");
        assert_eq!(parsed.department()[0].name, "技术部");
    }
}
use crate::core::config::Config;
use crate::core::error::SDKError;
use crate::core::response::SDKResult;
use crate::core::service_trait::Service;
use crate::core::transport::Transport;
use crate::core::endpoints_original::Endpoints;
use open_lark_core::core::api_req::ApiRequest;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询云文档事件订阅状态API端点
pub const ENDPOINT_GET_FILE_SUBSCRIBE: &str = "/open-apis/drive/v1/files/{}/get_subscribe";

/// 查询云文档事件订阅状态请求体
///
/// 用于查询指定文件的文档事件订阅状态，包括订阅类型、订阅者信息、
/// 订阅时间等详细状态信息
#[derive(Debug, Clone)]
pub struct GetFileSubscriptionRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 文件令牌
    /// 要查询订阅状态的文件的唯一标识符
    pub file_token: String,
}

impl Default for GetFileSubscriptionRequest {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::default(),
            file_token: String::new(),
        }
    }
}

impl GetFileSubscriptionRequest {
    /// 创建新的查询云文档事件订阅状态请求
    ///
    /// # 参数
    /// * `file_token` - 文件令牌
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            api_req: ApiRequest::default(),
            file_token: file_token.into(),
        }
    }

    /// 创建查询云文档事件订阅状态请求的构建器
    pub fn builder() -> GetFileSubscriptionBuilder {
        GetFileSubscriptionBuilder::default()
    }

    /// 构建请求验证
    ///
    /// 验证请求参数的有效性
    ///
    /// # 返回
    /// 成功返回空值，失败返回错误信息
    pub fn build(&self) -> SDKResult<()> {
        // 验证文件令牌
        if self.file_token.trim().is_empty() {
            return Err(SDKError::ValidationError("文件令牌不能为空".to_string()));
        }

        // 验证文件令牌长度（飞书文件令牌通常有一定长度要求）
        if self.file_token.len() < 10 {
            return Err(SDKError::ValidationError("文件令牌长度无效".to_string()));
        }

        // 验证文件令牌字符（仅允许字母、数字、连字符、下划线）
        if !self.file_token.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(SDKError::ValidationError("文件令牌包含无效字符".to_string()));
        }

        Ok(())
    }
}

/// 查询云文档事件订阅状态请求构建器
///
/// 提供流畅的API来构建查询云文档事件订阅状态请求，支持方法链调用
#[derive(Debug, Clone, Default)]
pub struct GetFileSubscriptionBuilder {
    file_token: Option<String>,
}

impl GetFileSubscriptionBuilder {
    /// 创建新的查询云文档事件订阅状态构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件令牌
    ///
    /// # 参数
    /// * `file_token` - 文件令牌
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 构建请求对象
    ///
    /// # 返回
    /// 成功返回请求对象，失败返回错误信息
    pub fn build(self) -> SDKResult<GetFileSubscriptionRequest> {
        let request = GetFileSubscriptionRequest {
            api_req: ApiRequest::default(),
            file_token: self.file_token.ok_or_else(|| {
                SDKError::ValidationError("文件令牌是必需的".to_string())
            })?,
        };

        // 执行请求验证
        request.build()?;
        Ok(request)
    }
}

/// 查询云文档事件订阅状态响应体
///
/// 包含文件的文档事件订阅状态信息，包括是否已订阅、订阅类型、
/// 订阅者详情、订阅时间等完整信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFileSubscriptionResponse {
    /// 响应码
    /// 0表示成功，非0表示失败
    pub code: i32,
    /// 响应消息
    /// 成功时返回"success"，失败时返回错误描述
    pub msg: String,
    /// 订阅状态数据
    /// 包含详细的订阅状态信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FileSubscriptionData>,
}

/// 文件订阅状态数据
///
/// 包含文件的文档事件订阅详细信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSubscriptionData {
    /// 文件令牌
    /// 查询的文件的唯一标识符
    pub file_token: String,
    /// 是否已订阅
    /// true表示已订阅事件，false表示未订阅
    pub subscribed: bool,
    /// 订阅类型
    /// 订阅的事件类型列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_types: Option<Vec<SubscriptionType>>,
    /// 订阅者信息
    /// 订阅该文件事件的用户或应用信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<SubscriberInfo>>,
    /// 订阅创建时间
    /// 格式：YYYY-MM-DD HH:mm:ss
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 订阅更新时间
    /// 格式：YYYY-MM-DD HH:mm:ss
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    /// 订阅状态
    /// 当前订阅的状态（活跃、暂停等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatus>,
}

/// 订阅类型枚举
///
/// 支持的文档事件订阅类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionType {
    /// 文件内容变更事件
    FileContentChanged,
    /// 文件元数据变更事件
    FileMetaChanged,
    /// 文件权限变更事件
    FilePermissionChanged,
    /// 文件分享状态变更事件
    FileShareChanged,
    /// 文件评论事件
    FileComment,
    /// 文件点赞事件
    FileLike,
    /// 文件访问事件
    FileAccess,
    /// 所有事件
    AllEvents,
}

/// 订阅者信息
///
/// 包含订阅者的详细信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriberInfo {
    /// 订阅者ID
    /// 用户ID或应用ID
    pub subscriber_id: String,
    /// 订阅者类型
    /// 用户或应用
    pub subscriber_type: SubscriberType,
    /// 订阅者名称
    /// 用户名或应用名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_name: Option<String>,
    /// 订阅权限
    /// 订阅者的权限级别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<SubscriptionPermission>,
    /// 订阅时间
    /// 格式：YYYY-MM-DD HH:mm:ss
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_time: Option<String>,
}

/// 订阅者类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriberType {
    /// 用户订阅者
    User,
    /// 应用订阅者
    Application,
}

/// 订阅权限枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPermission {
    /// 只读权限
    Read,
    /// 读写权限
    ReadWrite,
    /// 管理权限
    Admin,
}

/// 订阅状态枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    /// 活跃状态
    Active,
    /// 暂停状态
    Paused,
    /// 已过期
    Expired,
    /// 已取消
    Cancelled,
}

/// 查询云文档事件订阅状态构建器
///
/// 提供流畅的API来执行查询操作，支持方法链调用
#[derive(Debug, Clone)]
pub struct GetFileSubscriptionBuilder {
    service: Arc<crate::service::cloud_docs::drive::v1::DriveServiceV1>,
    request: GetFileSubscriptionRequest,
}

impl GetFileSubscriptionBuilder {
    /// 创建新的查询构建器
    ///
    /// # 参数
    /// * `service` - Drive服务实例
    /// * `request` - 查询请求
    pub fn new(service: Arc<crate::service::cloud_docs::drive::v1::DriveServiceV1>, request: GetFileSubscriptionRequest) -> Self {
        Self {
            service,
            request,
        }
    }

    /// 执行查询操作
    ///
    /// 发送HTTP GET请求查询文件的文档事件订阅状态
    ///
    /// # 返回
    /// 成功返回订阅状态信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::drive::v1::subscription::{GetFileSubscriptionRequest, GetFileSubscriptionResponse};
    ///
    /// async fn get_file_subscription_example(
    ///     service: std::sync::Arc<DriveServiceV1>,
    /// ) -> Result<GetFileSubscriptionResponse, Box<dyn std::error::Error>> {
    ///     let request = GetFileSubscriptionRequest::builder()
    ///         .file_token("file_token_123")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .get_file_subscription_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("查询订阅状态成功，订阅状态: {:?}", response.data);
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<GetFileSubscriptionResponse> {
        // 构建API请求
        let mut api_req = self.request.api_req.clone();
        api_req.method = Method::GET;

        // 设置端点URL并替换文件令牌占位符
        api_req.endpoint = Endpoints::DRIVE_V1_FILE_GET_SUBSCRIBE.replace("{}", &self.request.file_token);

        // 设置请求头
        api_req.headers.insert("Content-Type".parse().unwrap(), "application/json".parse().unwrap());

        // 执行HTTP请求
        let response = self.service.transport().request(&api_req).await?;

        // 解析响应
        let subscription_response: GetFileSubscriptionResponse = response.json().await
            .map_err(|e| SDKError::NetworkError(format!("解析响应失败: {}", e)))?;

        // 验证响应状态
        if subscription_response.code != 0 {
            return Err(SDKError::ApiError(
                subscription_response.code,
                subscription_response.msg,
            ));
        }

        Ok(subscription_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::transport::MockTransport;

    fn create_test_request() -> GetFileSubscriptionRequest {
        GetFileSubscriptionRequest::new("test_file_token_123")
    }

    fn create_test_subscription_data() -> FileSubscriptionData {
        FileSubscriptionData {
            file_token: "test_file_token_123".to_string(),
            subscribed: true,
            subscription_types: Some(vec![
                SubscriptionType::FileContentChanged,
                SubscriptionType::FileMetaChanged,
            ]),
            subscribers: Some(vec![
                SubscriberInfo {
                    subscriber_id: "user_001".to_string(),
                    subscriber_type: SubscriberType::User,
                    subscriber_name: Some("张三".to_string()),
                    permission: Some(SubscriptionPermission::Read),
                    subscribed_time: Some("2024-01-01 10:00:00".to_string()),
                },
            ]),
            created_time: Some("2024-01-01 09:00:00".to_string()),
            updated_time: Some("2024-01-01 10:00:00".to_string()),
            status: Some(SubscriptionStatus::Active),
        }
    }

    #[test]
    fn test_get_file_subscription_request_new() {
        let request = create_test_request();
        assert_eq!(request.file_token, "test_file_token_123");
    }

    #[test]
    fn test_get_file_subscription_request_default() {
        let request = GetFileSubscriptionRequest::default();
        assert!(request.file_token.is_empty());
    }

    #[test]
    fn test_get_file_subscription_request_builder() {
        let builder = GetFileSubscriptionRequest::builder();
        assert!(builder.file_token.is_none());
    }

    #[test]
    fn test_get_file_subscription_request_build_success() {
        let request = create_test_request();
        assert!(request.build().is_ok());
    }

    #[test]
    fn test_get_file_subscription_request_build_empty_file_token() {
        let request = GetFileSubscriptionRequest::new("");
        assert!(request.build().is_err());
    }

    #[test]
    fn test_get_file_subscription_request_build_short_file_token() {
        let request = GetFileSubscriptionRequest::new("short");
        assert!(request.build().is_err());
    }

    #[test]
    fn test_get_file_subscription_request_build_invalid_characters() {
        let request = GetFileSubscriptionRequest::new("token@123");
        assert!(request.build().is_err());
    }

    #[test]
    fn test_get_file_subscription_builder_with_file_token() {
        let builder = GetFileSubscriptionRequest::builder()
            .file_token("test_file_token_456");

        assert_eq!(builder.file_token, Some("test_file_token_456".to_string()));
    }

    #[test]
    fn test_get_file_subscription_builder_build_success() {
        let builder = GetFileSubscriptionRequest::builder()
            .file_token("test_file_token_789");

        let request = builder.build();
        assert!(request.is_ok());

        if let Ok(req) = request {
            assert_eq!(req.file_token, "test_file_token_789");
        }
    }

    #[test]
    fn test_get_file_subscription_builder_build_missing_file_token() {
        let builder = GetFileSubscriptionRequest::builder();
        let request = builder.build();
        assert!(request.is_err());
    }

    #[test]
    fn test_file_subscription_data_creation() {
        let data = create_test_subscription_data();

        assert_eq!(data.file_token, "test_file_token_123");
        assert!(data.subscribed);
        assert!(data.subscription_types.is_some());
        assert!(data.subscribers.is_some());
        assert!(data.created_time.is_some());
        assert!(data.updated_time.is_some());
        assert!(data.status.is_some());
    }

    #[test]
    fn test_subscription_type_serialization() {
        let subscription_type = SubscriptionType::FileContentChanged;
        let serialized = serde_json::to_string(&subscription_type).unwrap();
        assert_eq!(serialized, "\"file_content_changed\"");
    }

    #[test]
    fn test_subscription_type_deserialization() {
        let json = "\"file_meta_changed\"";
        let subscription_type: SubscriptionType = serde_json::from_str(json).unwrap();
        assert_eq!(subscription_type, SubscriptionType::FileMetaChanged);
    }

    #[test]
    fn test_subscriber_type_serialization() {
        let subscriber_type = SubscriberType::Application;
        let serialized = serde_json::to_string(&subscriber_type).unwrap();
        assert_eq!(serialized, "\"application\"");
    }

    #[test]
    fn test_subscriber_type_deserialization() {
        let json = "\"user\"";
        let subscriber_type: SubscriberType = serde_json::from_str(json).unwrap();
        assert_eq!(subscriber_type, SubscriberType::User);
    }

    #[test]
    fn test_subscription_permission_serialization() {
        let permission = SubscriptionPermission::ReadWrite;
        let serialized = serde_json::to_string(&permission).unwrap();
        assert_eq!(serialized, "\"read_write\"");
    }

    #[test]
    fn test_subscription_permission_deserialization() {
        let json = "\"admin\"";
        let permission: SubscriptionPermission = serde_json::from_str(json).unwrap();
        assert_eq!(permission, SubscriptionPermission::Admin);
    }

    #[test]
    fn test_subscription_status_serialization() {
        let status = SubscriptionStatus::Active;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"active\"");
    }

    #[test]
    fn test_subscription_status_deserialization() {
        let json = "\"paused\"";
        let status: SubscriptionStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, SubscriptionStatus::Paused);
    }

    #[test]
    fn test_subscriber_info_creation() {
        let subscriber = SubscriberInfo {
            subscriber_id: "user_002".to_string(),
            subscriber_type: SubscriberType::User,
            subscriber_name: Some("李四".to_string()),
            permission: Some(SubscriptionPermission::Admin),
            subscribed_time: Some("2024-01-02 15:30:00".to_string()),
        };

        assert_eq!(subscriber.subscriber_id, "user_002");
        assert_eq!(subscriber.subscriber_type, SubscriberType::User);
        assert_eq!(subscriber.subscriber_name, Some("李四".to_string()));
        assert_eq!(subscriber.permission, Some(SubscriptionPermission::Admin));
        assert_eq!(subscriber.subscribed_time, Some("2024-01-02 15:30:00".to_string()));
    }

    #[test]
    fn test_get_file_subscription_response_creation() {
        let data = create_test_subscription_data();
        let response = GetFileSubscriptionResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        };

        assert_eq!(response.code, 0);
        assert_eq!(response.msg, "success");
        assert!(response.data.is_some());
    }

    #[test]
    fn test_get_file_subscription_response_error() {
        let response = GetFileSubscriptionResponse {
            code: 1001,
            msg: "文件未找到".to_string(),
            data: None,
        };

        assert_eq!(response.code, 1001);
        assert_eq!(response.msg, "文件未找到");
        assert!(response.data.is_none());
    }

    #[test]
    fn test_all_subscription_types() {
        let types = vec![
            SubscriptionType::FileContentChanged,
            SubscriptionType::FileMetaChanged,
            SubscriptionType::FilePermissionChanged,
            SubscriptionType::FileShareChanged,
            SubscriptionType::FileComment,
            SubscriptionType::FileLike,
            SubscriptionType::FileAccess,
            SubscriptionType::AllEvents,
        ];

        assert_eq!(types.len(), 8);

        // 测试序列化和反序列化
        for subscription_type in &types {
            let serialized = serde_json::to_string(subscription_type).unwrap();
            let deserialized: SubscriptionType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(*subscription_type, deserialized);
        }
    }

    #[test]
    fn test_complete_subscription_workflow() {
        // 测试完整的构建和验证流程
        let request = GetFileSubscriptionRequest::builder()
            .file_token("valid_file_token_123")
            .build()
            .expect("构建请求应该成功");

        // 验证请求参数
        assert!(request.build().is_ok());
        assert_eq!(request.file_token, "valid_file_token_123");

        // 创建模拟响应数据
        let subscription_data = create_test_subscription_data();
        let response = GetFileSubscriptionResponse {
            code: 0,
            msg: "success".to_string(),
            data: Some(subscription_data),
        };

        // 验证响应数据
        assert_eq!(response.code, 0);
        assert!(response.data.is_some());
        let data = response.data.unwrap();
        assert!(data.subscribed);
        assert_eq!(data.file_token, "test_file_token_123");
    }
}
//! 知识空间成员管理服务
//!
//! 提供飞书知识库空间成员的管理功能，包括：
//! - 删除空间成员
//! - 成员权限管理
//! - 成员信息查询

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除知识空间成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSpaceMemberRequest {
    /// 知识空间ID
    pub space_id: String,
    /// 成员ID
    pub member_id: String,
}

impl DeleteSpaceMemberRequest {
    /// 创建新的删除成员请求实例
    ///
    /// # 参数
    /// - `space_id`: 知识空间ID
    /// - `member_id`: 成员ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_member::DeleteSpaceMemberRequest;
    ///
    /// let request = DeleteSpaceMemberRequest::new("space_123", "member_456");
    /// ```
    pub fn new(space_id: impl Into<String>, member_id: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            member_id: member_id.into(),
        }
    }

    /// 验证请求参数
    ///
    /// # 返回值
    /// - `Ok(())`: 参数验证通过
    /// - `Err(String)`: 参数验证失败，返回错误信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_member::DeleteSpaceMemberRequest;
    ///
    /// let request = DeleteSpaceMemberRequest::new("space_123", "member_456");
    /// assert!(request.validate().is_ok());
    ///
    /// let empty_request = DeleteSpaceMemberRequest::new("", "member_456");
    /// assert!(empty_request.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.space_id.trim().is_empty() {
            return Err("知识空间ID不能为空".to_string());
        }
        if self.member_id.trim().is_empty() {
            return Err("成员ID不能为空".to_string());
        }
        if self.space_id.len() > 100 {
            return Err("知识空间ID长度不能超过100个字符".to_string());
        }
        if self.member_id.len() > 100 {
            return Err("成员ID长度不能超过100个字符".to_string());
        }
        Ok(())
    }
}

/// 删除知识空间成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteSpaceMemberResponse {
    /// 操作结果状态
    pub success: bool,
    /// 错误消息（如果有）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ApiResponseTrait for DeleteSpaceMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 知识空间成员管理服务
#[derive(Clone, Debug)]
pub struct SpaceMemberService {
    config: Config,
}

impl SpaceMemberService {
    /// 创建空间成员管理服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::wiki::v2::space_member::SpaceMemberService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = SpaceMemberService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 删除知识空间成员
    ///
    /// 从指定的知识空间中删除指定成员。删除后，该成员将无法访问该知识空间。
    ///
    /// # 参数
    /// * `req` - 删除成员请求
    ///
    /// # 返回值
    /// 返回删除操作的结果状态
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_member::{SpaceMemberService, DeleteSpaceMemberRequest};
    ///
    /// let service = SpaceMemberService::new(config);
    /// let request = DeleteSpaceMemberRequest::new("space_123", "member_456");
    ///
    /// let result = service.delete(&request).await?;
    /// if result.success {
    ///     println!("成员删除成功");
    /// } else {
    ///     println!("成员删除失败: {:?}", result.error_message);
    /// }
    /// ```
    pub async fn delete(
        &self,
        req: &DeleteSpaceMemberRequest,
    ) -> SDKResult<DeleteSpaceMemberResponse> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!(
            "开始删除知识空间成员: space_id={}, member_id={}",
            req.space_id,
            req.member_id
        );

        // 构建动态端点路径
        let endpoint = openlark_core::endpoints::Endpoints::WIKI_V2_SPACE_MEMBER_DELETE
            .replace("{}", &req.space_id)
            .replace("{}", &req.member_id);

        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Delete,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None, // DELETE请求无body
            
        };

        let resp =
            Transport::<DeleteSpaceMemberResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "知识空间成员删除完成: space_id={}, member_id={}, success={}",
            req.space_id,
            req.member_id,
            response.success
        );

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 删除知识空间成员构建器
#[derive(Clone, Debug)]
pub struct DeleteSpaceMemberBuilder {
    request: DeleteSpaceMemberRequest,
}

impl Default for DeleteSpaceMemberBuilder {
    fn default() -> Self {
        Self {
            request: DeleteSpaceMemberRequest {
                space_id: String::new(),
                member_id: String::new(),
            },
        }
    }
}

impl DeleteSpaceMemberBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置知识空间ID
    ///
    /// # 参数
    /// - `space_id`: 知识空间ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_member::DeleteSpaceMemberBuilder;
    ///
    /// let builder = DeleteSpaceMemberBuilder::new().space_id("space_123");
    /// ```
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.request.space_id = space_id.into();
        self
    }

    /// 设置成员ID
    ///
    /// # 参数
    /// - `member_id`: 成员ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_member::DeleteSpaceMemberBuilder;
    ///
    /// let builder = DeleteSpaceMemberBuilder::new().member_id("member_456");
    /// ```
    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.request.member_id = member_id.into();
        self
    }

    /// 执行删除知识空间成员操作
    ///
    /// # 参数
    /// - `service`: 空间成员管理服务实例
    ///
    /// # 返回值
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_member::{SpaceMemberService, DeleteSpaceMemberBuilder};
    ///
    /// let service = SpaceMemberService::new(config);
    ///
    /// let result = DeleteSpaceMemberBuilder::new()
    ///     .space_id("space_123")
    ///     .member_id("member_456")
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(
        self,
        service: &SpaceMemberService,
    ) -> SDKResult<DeleteSpaceMemberResponse> {
        service.delete(&self.request).await
    }
}

impl SpaceMemberService {
    /// 创建删除知识空间成员构建器
    ///
    /// # 返回值
    /// 返回删除成员构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::wiki::v2::space_member::SpaceMemberService;
    ///
    /// let service = SpaceMemberService::new(config);
    /// let builder = service.delete_space_member_builder();
    /// ```
    pub fn delete_space_member_builder(&self) -> DeleteSpaceMemberBuilder {
        DeleteSpaceMemberBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_member_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = SpaceMemberService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_delete_space_member_request() {
        let request = DeleteSpaceMemberRequest::new("space_123", "member_456");

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.member_id, "member_456");
    }

    #[test]
    fn test_delete_space_member_request_validation() {
        // 测试正常情况
        let valid_request = DeleteSpaceMemberRequest::new("space_123", "member_456");
        assert!(valid_request.validate().is_ok());

        // 测试空space_id
        let empty_space_request = DeleteSpaceMemberRequest::new("", "member_456");
        assert!(empty_space_request.validate().is_err());

        // 测试空member_id
        let empty_member_request = DeleteSpaceMemberRequest::new("space_123", "");
        assert!(empty_member_request.validate().is_err());

        // 测试空白字符
        let whitespace_request = DeleteSpaceMemberRequest::new("   ", "member_456");
        assert!(whitespace_request.validate().is_err());

        // 测试长度超限
        let long_space_request = DeleteSpaceMemberRequest::new(&"a".repeat(101), "member_456");
        assert!(long_space_request.validate().is_err());

        let long_member_request = DeleteSpaceMemberRequest::new("space_123", &"a".repeat(101));
        assert!(long_member_request.validate().is_err());

        // 测试长度边界
        let boundary_space_request = DeleteSpaceMemberRequest::new(&"a".repeat(100), "member_456");
        assert!(boundary_space_request.validate().is_ok());

        let boundary_member_request = DeleteSpaceMemberRequest::new("space_123", &"a".repeat(100));
        assert!(boundary_member_request.validate().is_ok());
    }

    #[test]
    fn test_delete_space_member_builder() {
        let builder = DeleteSpaceMemberBuilder::new()
            .space_id("space_123")
            .member_id("member_456");

        assert_eq!(builder.request.space_id, "space_123");
        assert_eq!(builder.request.member_id, "member_456");
    }

    #[test]
    fn test_delete_space_member_builder_default() {
        let builder = DeleteSpaceMemberBuilder::default();
        assert_eq!(builder.request.space_id, "");
        assert_eq!(builder.request.member_id, "");
    }

    #[test]
    fn test_response_default_creation() {
        let response = DeleteSpaceMemberResponse::default();
        assert!(!response.success);
        assert_eq!(response.error_message, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut response = DeleteSpaceMemberResponse::default();
        response.success = true;
        response.error_message = Some("测试错误".to_string());

        assert!(response.success);
        assert_eq!(response.error_message, Some("测试错误".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            DeleteSpaceMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_request_serialization() {
        let request = DeleteSpaceMemberRequest::new("space_123", "member_456");

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: DeleteSpaceMemberRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.space_id, deserialized.space_id);
        assert_eq!(request.member_id, deserialized.member_id);
    }

    #[test]
    fn test_response_serialization() {
        let mut response = DeleteSpaceMemberResponse::default();
        response.success = true;
        response.error_message = Some("测试消息".to_string());

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: DeleteSpaceMemberResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.success, deserialized.success);
        assert_eq!(response.error_message, deserialized.error_message);
    }

    #[test]
    fn test_request_with_special_characters() {
        // 测试包含特殊字符的ID
        let special_request = DeleteSpaceMemberRequest::new("space_abc-123", "member_xyz-456");
        assert!(special_request.validate().is_ok());

        // 测试包含数字的ID
        let numeric_request = DeleteSpaceMemberRequest::new("123456", "789012");
        assert!(numeric_request.validate().is_ok());
    }

    #[test]
    fn test_builder_chain_calls() {
        let builder = DeleteSpaceMemberBuilder::new()
            .space_id("space_123")
            .member_id("member_456")
            .space_id("space_789") // 覆盖之前的值
            .member_id("member_012"); // 覆盖之前的值

        assert_eq!(builder.request.space_id, "space_789");
        assert_eq!(builder.request.member_id, "member_012");
    }

    #[test]
    fn test_request_validation_edge_cases() {
        // 测试仅包含空白字符的ID
        let whitespace_space_request = DeleteSpaceMemberRequest::new("  \t\n  ", "member_456");
        assert!(whitespace_space_request.validate().is_err());

        let whitespace_member_request = DeleteSpaceMemberRequest::new("space_123", "  \t\n  ");
        assert!(whitespace_member_request.validate().is_err());

        // 测试中文字符（虽然可能不常见，但应该能处理）
        let chinese_request = DeleteSpaceMemberRequest::new("空间_123", "成员_456");
        assert!(chinese_request.validate().is_ok());
    }

    #[test]
    fn test_endpoint_constant() {
        // 测试端点常量是否正确定义
        assert_eq!(
            openlark_core::endpoints::Endpoints::WIKI_V2_SPACE_MEMBER_DELETE,
            "/open-apis/wiki/v2/spaces/{}/members/{}"
        );
    }

    #[test]
    fn test_error_message_handling() {
        let mut response = DeleteSpaceMemberResponse::default();

        // 测试成功情况
        response.success = true;
        assert!(response.success);
        assert_eq!(response.error_message, None);

        // 测试失败情况
        response.success = false;
        response.error_message = Some("权限不足".to_string());
        assert!(!response.success);
        assert_eq!(response.error_message, Some("权限不足".to_string()));
    }
}

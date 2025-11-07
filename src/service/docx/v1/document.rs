//! 文档管理服务
//!
//! 提供飞书协作文档的创建、查询、管理等基础功能，包括：
//! - 创建新文档
//! - 获取文档信息
//! - 删除文档
//! - 文档权限管理

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 文档信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    /// 文档ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文档URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 文档版本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /// 所在文件夹信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 文档状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            document_id: None,
            title: None,
            url: None,
            version: None,
            create_time: None,
            update_time: None,
            creator: None,
            folder_token: None,
            status: None,
        }
    }
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Creator {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl Default for Creator {
    fn default() -> Self {
        Self {
            user_id: None,
            name: None,
            avatar: None,
        }
    }
}

/// 创建文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    /// 文档标题
    pub title: String,
    /// 所在文件夹token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

/// 获取文档请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRequest {
    /// 文档ID
    pub document_id: String,
}

impl GetDocumentRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::GetDocumentRequest;
    ///
    /// let request = GetDocumentRequest::new("doc_123");
    /// ```
    pub fn new(document_id: impl Into<String>) -> Self {
        Self {
            document_id: document_id.into(),
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
    /// use open_lark::service::docx::v1::document::GetDocumentRequest;
    ///
    /// let request = GetDocumentRequest::new("doc_123");
    /// assert!(request.validate().is_ok());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.trim().is_empty() {
            return Err("文档ID不能为空".to_string());
        }
        if self.document_id.len() > 200 {
            return Err("文档ID长度不能超过200个字符".to_string());
        }
        Ok(())
    }
}

impl CreateDocumentRequest {
    /// 创建新的请求实例
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            folder_token: None,
        }
    }

    /// 设置所在文件夹
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("标题不能为空".to_string());
        }
        if self.title.len() > 200 {
            return Err("标题长度不能超过200个字符".to_string());
        }
        Ok(())
    }
}

/// 创建文档响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDocumentResponse {
    /// 创建的文档信息
    pub document: Document,
}

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetDocumentResponse {
    /// 文档信息
    pub document: Document,
}

impl ApiResponseTrait for GetDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文档管理服务
#[derive(Debug, Clone)]
pub struct DocumentService {
    config: Config,
}

impl DocumentService {
    /// 创建文档管理服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::docx::v1::document::DocumentService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = DocumentService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建文档
    ///
    /// 创建一个新的协作文档，支持指定标题和所在文件夹
    ///
    /// # 参数
    /// * `req` - 创建文档请求
    ///
    /// # 返回值
    /// 返回创建的文档信息
    pub async fn create(&self, req: &CreateDocumentRequest) -> SDKResult<CreateDocumentResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始创建文档: title={:?}", req.title);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::DOCX_V1_DOCUMENTS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateDocumentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档创建成功: title={}, document_id={:?}",
            req.title,
            response.document.document_id
        );

        Ok(response)
    }

    /// 获取文档信息
    ///
    /// 获取指定文档的详细信息，包括标题、版本、创建者、
    /// 更新时间等元数据信息。
    ///
    /// # 参数
    /// * `req` - 获取文档请求
    ///
    /// # 返回值
    /// 返回文档的详细信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::{DocumentService, GetDocumentRequest};
    ///
    /// let service = DocumentService::new(config);
    /// let request = GetDocumentRequest::new("doc_123");
    ///
    /// let result = service.get(&request).await?;
    /// println!("文档标题: {:?}", result.document.title);
    /// println!("文档版本: {:?}", result.document.version);
    /// ```
    pub async fn get(&self, req: &GetDocumentRequest) -> SDKResult<GetDocumentResponse> {
        req.validate()
            .map_err(|msg| crate::core::error::LarkAPIError::illegal_param(msg))?;
        log::debug!("开始获取文档信息: document_id={}", req.document_id);

        // 构建动态端点路径
        let endpoint = crate::core::endpoints_original::Endpoints::DOCX_V1_DOCUMENT_GET
            .replace("{}", &req.document_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: endpoint,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(), // GET请求无body
            ..Default::default()
        };

        let resp = Transport::<GetDocumentResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档信息获取完成: document_id={}, title={:?}",
            req.document_id,
            response.document.title
        );

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 创建文档构建器
#[derive(Debug, Clone)]
pub struct CreateDocumentBuilder {
    request: CreateDocumentRequest,
}

impl Default for CreateDocumentBuilder {
    fn default() -> Self {
        Self {
            request: CreateDocumentRequest {
                title: String::new(),
                folder_token: None,
            },
        }
    }
}

impl CreateDocumentBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文档标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = title.into();
        self
    }

    /// 设置所在文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = Some(folder_token.into());
        self
    }

    /// 执行创建文档操作
    pub async fn execute(self, service: &DocumentService) -> SDKResult<CreateDocumentResponse> {
        service.create(&self.request).await
    }
}

/// 获取文档构建器
#[derive(Debug, Clone)]
pub struct GetDocumentBuilder {
    request: GetDocumentRequest,
}

impl Default for GetDocumentBuilder {
    fn default() -> Self {
        Self {
            request: GetDocumentRequest {
                document_id: String::new(),
            },
        }
    }
}

impl GetDocumentBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文档ID
    ///
    /// # 参数
    /// - `document_id`: 文档ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::GetDocumentBuilder;
    ///
    /// let builder = GetDocumentBuilder::new().document_id("doc_123");
    /// ```
    pub fn document_id(mut self, document_id: impl Into<String>) -> Self {
        self.request.document_id = document_id.into();
        self
    }

    /// 执行获取文档操作
    ///
    /// # 参数
    /// - `service`: 文档管理服务实例
    ///
    /// # 返回值
    /// 返回文档的详细信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::{DocumentService, GetDocumentBuilder};
    ///
    /// let service = DocumentService::new(config);
    ///
    /// let result = GetDocumentBuilder::new()
    ///     .document_id("doc_123")
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(self, service: &DocumentService) -> SDKResult<GetDocumentResponse> {
        service.get(&self.request).await
    }
}

impl DocumentService {
    /// 创建文档构建器
    pub fn create_document_builder(&self) -> CreateDocumentBuilder {
        CreateDocumentBuilder::new()
    }

    /// 创建获取文档构建器
    ///
    /// # 返回值
    /// 返回获取文档构建器实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::docx::v1::document::DocumentService;
    ///
    /// let service = DocumentService::new(config);
    /// let builder = service.get_document_builder();
    /// ```
    pub fn get_document_builder(&self) -> GetDocumentBuilder {
        GetDocumentBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_service_creation() {
        let config = Config::default();
        let service = DocumentService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_document_default_creation() {
        let document = Document::default();
        assert_eq!(document.document_id, None);
        assert_eq!(document.title, None);
        assert_eq!(document.url, None);
        assert_eq!(document.version, None);
        assert_eq!(document.create_time, None);
        assert_eq!(document.update_time, None);
        assert_eq!(document.creator, None);
        assert_eq!(document.folder_token, None);
        assert_eq!(document.status, None);
    }

    #[test]
    fn test_document_with_data() {
        let creator = Creator {
            user_id: Some("user_123".to_string()),
            name: Some("张三".to_string()),
            avatar: Some("avatar_url".to_string()),
        };

        let document = Document {
            document_id: Some("doc_456".to_string()),
            title: Some("项目计划".to_string()),
            url: Some("https://example.com/doc".to_string()),
            version: Some(1),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
            creator: Some(creator),
            folder_token: Some("folder_789".to_string()),
            status: Some("active".to_string()),
        };

        assert_eq!(document.document_id, Some("doc_456".to_string()));
        assert_eq!(document.title, Some("项目计划".to_string()));
        assert_eq!(document.url, Some("https://example.com/doc".to_string()));
        assert_eq!(document.version, Some(1));
        assert_eq!(
            document.creator.as_ref().unwrap().user_id,
            Some("user_123".to_string())
        );
        assert_eq!(
            document.creator.as_ref().unwrap().name,
            Some("张三".to_string())
        );
        assert_eq!(document.folder_token, Some("folder_789".to_string()));
        assert_eq!(document.status, Some("active".to_string()));
    }

    #[test]
    fn test_creator_default_creation() {
        let creator = Creator::default();
        assert_eq!(creator.user_id, None);
        assert_eq!(creator.name, None);
        assert_eq!(creator.avatar, None);
    }

    #[test]
    fn test_create_document_request() {
        let request = CreateDocumentRequest::new("测试文档").folder_token("folder_token");

        assert_eq!(request.title, "测试文档");
        assert_eq!(request.folder_token, Some("folder_token".to_string()));
    }

    #[test]
    fn test_create_document_request_validation() {
        // 测试正常情况
        let valid_request = CreateDocumentRequest::new("有效标题");
        assert!(valid_request.validate().is_ok());

        // 测试空标题
        let empty_title_request = CreateDocumentRequest::new("");
        assert!(empty_title_request.validate().is_err());

        // 测试空白标题
        let whitespace_title_request = CreateDocumentRequest::new("   ");
        assert!(whitespace_title_request.validate().is_err());

        // 测试标题过长
        let long_title_request = CreateDocumentRequest::new(&"a".repeat(201));
        assert!(long_title_request.validate().is_err());

        // 测试标题长度边界
        let boundary_title_request = CreateDocumentRequest::new(&"a".repeat(200));
        assert!(boundary_title_request.validate().is_ok());
    }

    #[test]
    fn test_create_document_builder() {
        let builder = CreateDocumentBuilder::new()
            .title("构建器测试")
            .folder_token("test_folder");

        assert_eq!(builder.request.title, "构建器测试");
        assert_eq!(
            builder.request.folder_token,
            Some("test_folder".to_string())
        );
    }

    #[test]
    fn test_create_document_builder_default() {
        let builder = CreateDocumentBuilder::default();
        assert_eq!(builder.request.title, "");
        assert_eq!(builder.request.folder_token, None);
    }

    #[test]
    fn test_response_default_creation() {
        let response = CreateDocumentResponse::default();
        assert_eq!(response.document.document_id, None);
        assert_eq!(response.document.title, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut response = CreateDocumentResponse::default();
        response.document = Document {
            document_id: Some("doc_abc".to_string()),
            title: Some("响应测试".to_string()),
            ..Default::default()
        };

        assert_eq!(response.document.document_id, Some("doc_abc".to_string()));
        assert_eq!(response.document.title, Some("响应测试".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(CreateDocumentResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = CreateDocumentRequest::new("序列化测试").folder_token("test_folder");

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: CreateDocumentRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.title, deserialized.title);
        assert_eq!(request.folder_token, deserialized.folder_token);
    }

    #[test]
    fn test_endpoint_constant() {
        // Test that the endpoint constant is properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::DOCX_V1_DOCUMENTS,
            "/open-apis/docx/v1/documents"
        );
    }

    #[test]
    fn test_document_title_variations() {
        // Test different document titles
        let project_doc = Document {
            title: Some("项目计划文档".to_string()),
            ..Default::default()
        };

        let meeting_doc = Document {
            title: Some("会议纪要".to_string()),
            ..Default::default()
        };

        let report_doc = Document {
            title: Some("月度报告".to_string()),
            ..Default::default()
        };

        assert_eq!(project_doc.title, Some("项目计划文档".to_string()));
        assert_eq!(meeting_doc.title, Some("会议纪要".to_string()));
        assert_eq!(report_doc.title, Some("月度报告".to_string()));
    }

    #[test]
    fn test_comprehensive_document_data() {
        // Test comprehensive document data with all fields
        let comprehensive_creator = Creator {
            user_id: Some("creator_001".to_string()),
            name: Some("李四".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
        };

        let comprehensive_document = Document {
            document_id: Some("comprehensive_doc_001".to_string()),
            title: Some("2023年度工作总结".to_string()),
            url: Some("https://docs.example.com/comprehensive_doc_001".to_string()),
            version: Some(3),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
            creator: Some(comprehensive_creator),
            folder_token: Some("reports_folder_2023".to_string()),
            status: Some("published".to_string()),
        };

        assert_eq!(
            comprehensive_document.document_id,
            Some("comprehensive_doc_001".to_string())
        );
        assert_eq!(
            comprehensive_document.title,
            Some("2023年度工作总结".to_string())
        );
        assert_eq!(
            comprehensive_document.url,
            Some("https://docs.example.com/comprehensive_doc_001".to_string())
        );
        assert_eq!(comprehensive_document.version, Some(3));
        assert_eq!(
            comprehensive_document.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_document.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_document.creator.as_ref().unwrap().user_id,
            Some("creator_001".to_string())
        );
        assert_eq!(
            comprehensive_document.creator.as_ref().unwrap().name,
            Some("李四".to_string())
        );
        assert_eq!(
            comprehensive_document.folder_token,
            Some("reports_folder_2023".to_string())
        );
        assert_eq!(comprehensive_document.status, Some("published".to_string()));
    }

    #[test]
    fn test_request_validation_edge_cases() {
        // Test with whitespace-only title
        let whitespace_request = CreateDocumentRequest::new("  \t\n  ");
        assert!(whitespace_request.validate().is_err());

        // Test with special characters in title
        let special_chars_request = CreateDocumentRequest::new("项目计划-Q1_2023.docx");
        assert!(special_chars_request.validate().is_ok());

        // Test with Unicode characters
        let unicode_request = CreateDocumentRequest::new("📊 项目数据 📈");
        assert!(unicode_request.validate().is_ok());
    }

    #[test]
    fn test_document_version_handling() {
        // Test document version
        let versioned_doc = Document {
            document_id: Some("doc_versioned".to_string()),
            title: Some("版本化文档".to_string()),
            version: Some(5),
            ..Default::default()
        };

        assert_eq!(versioned_doc.version, Some(5));

        let unversioned_doc = Document {
            document_id: Some("doc_unversioned".to_string()),
            title: Some("无版本文档".to_string()),
            version: None,
            ..Default::default()
        };

        assert_eq!(unversioned_doc.version, None);
    }

    #[test]
    fn test_get_document_request() {
        let request = GetDocumentRequest::new("doc_123");
        assert_eq!(request.document_id, "doc_123");
    }

    #[test]
    fn test_get_document_request_validation() {
        // 测试正常情况
        let valid_request = GetDocumentRequest::new("doc_123");
        assert!(valid_request.validate().is_ok());

        // 测试空document_id
        let empty_request = GetDocumentRequest::new("");
        assert!(empty_request.validate().is_err());

        // 测试空白字符
        let whitespace_request = GetDocumentRequest::new("   ");
        assert!(whitespace_request.validate().is_err());

        // 测试长度超限
        let long_request = GetDocumentRequest::new(&"a".repeat(201));
        assert!(long_request.validate().is_err());

        // 测试长度边界
        let boundary_request = GetDocumentRequest::new(&"a".repeat(200));
        assert!(boundary_request.validate().is_ok());
    }

    #[test]
    fn test_get_document_builder() {
        let builder = GetDocumentBuilder::new().document_id("doc_123");
        assert_eq!(builder.request.document_id, "doc_123");
    }

    #[test]
    fn test_get_document_builder_default() {
        let builder = GetDocumentBuilder::default();
        assert_eq!(builder.request.document_id, "");
    }

    #[test]
    fn test_get_document_response_default_creation() {
        let response = GetDocumentResponse::default();
        assert_eq!(response.document.document_id, None);
        assert_eq!(response.document.title, None);
    }

    #[test]
    fn test_get_document_response_with_data() {
        let mut response = GetDocumentResponse::default();
        response.document = Document {
            document_id: Some("doc_abc".to_string()),
            title: Some("获取测试文档".to_string()),
            ..Default::default()
        };

        assert_eq!(response.document.document_id, Some("doc_abc".to_string()));
        assert_eq!(response.document.title, Some("获取测试文档".to_string()));
    }

    #[test]
    fn test_get_document_response_api_trait() {
        assert_eq!(GetDocumentResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_document_request_serialization() {
        let request = GetDocumentRequest::new("doc_123");
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetDocumentRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.document_id, deserialized.document_id);
    }

    #[test]
    fn test_get_document_response_serialization() {
        let mut response = GetDocumentResponse::default();
        response.document = Document {
            document_id: Some("doc_xyz".to_string()),
            title: Some("序列化测试".to_string()),
            version: Some(2),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetDocumentResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            response.document.document_id,
            deserialized.document.document_id
        );
        assert_eq!(response.document.title, deserialized.document.title);
        assert_eq!(response.document.version, deserialized.document.version);
    }

    #[test]
    fn test_get_document_builder_chain_calls() {
        let builder = GetDocumentBuilder::new()
            .document_id("doc_123")
            .document_id("doc_456"); // 覆盖之前的值

        assert_eq!(builder.request.document_id, "doc_456");
    }

    #[test]
    fn test_get_document_request_validation_edge_cases() {
        // 测试仅包含空白字符的document_id
        let whitespace_request = GetDocumentRequest::new("  \t\n  ");
        assert!(whitespace_request.validate().is_err());

        // 测试中文字符（虽然可能不常见，但应该能处理）
        let chinese_request = GetDocumentRequest::new("文档_123");
        assert!(chinese_request.validate().is_ok());

        // 测试包含特殊字符的document_id
        let special_chars_request = GetDocumentRequest::new("doc_abc-123_xyz");
        assert!(special_chars_request.validate().is_ok());
    }

    #[test]
    fn test_get_document_endpoint_constant() {
        // 测试端点常量是否正确定义
        assert_eq!(
            crate::core::endpoints_original::Endpoints::DOCX_V1_DOCUMENT_GET,
            "/open-apis/docx/v1/documents/{}"
        );
    }

    #[test]
    fn test_get_document_response_comprehensive_data() {
        // 测试包含完整数据的文档响应
        let comprehensive_creator = Creator {
            user_id: Some("user_001".to_string()),
            name: Some("测试用户".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
        };

        let comprehensive_response = GetDocumentResponse {
            document: Document {
                document_id: Some("doc_comprehensive".to_string()),
                title: Some("综合测试文档".to_string()),
                url: Some("https://docs.example.com/doc_comprehensive".to_string()),
                version: Some(5),
                create_time: Some("2023-01-01T08:00:00Z".to_string()),
                update_time: Some("2023-12-31T16:00:00Z".to_string()),
                creator: Some(comprehensive_creator),
                folder_token: Some("folder_123".to_string()),
                status: Some("published".to_string()),
            },
        };

        assert_eq!(
            comprehensive_response.document.document_id,
            Some("doc_comprehensive".to_string())
        );
        assert_eq!(
            comprehensive_response.document.title,
            Some("综合测试文档".to_string())
        );
        assert_eq!(
            comprehensive_response.document.url,
            Some("https://docs.example.com/doc_comprehensive".to_string())
        );
        assert_eq!(comprehensive_response.document.version, Some(5));
        assert_eq!(
            comprehensive_response.document.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_response.document.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_response
                .document
                .creator
                .as_ref()
                .unwrap()
                .user_id,
            Some("user_001".to_string())
        );
        assert_eq!(
            comprehensive_response
                .document
                .creator
                .as_ref()
                .unwrap()
                .name,
            Some("测试用户".to_string())
        );
        assert_eq!(
            comprehensive_response.document.folder_token,
            Some("folder_123".to_string())
        );
        assert_eq!(
            comprehensive_response.document.status,
            Some("published".to_string())
        );
    }
}

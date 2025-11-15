//! 获取旧版文档内容服务
//!
//! 提供获取旧版文档内容的功能，包括纯文本和富文本内容。

use std::collections::HashMap;
use openlark_core::{
    
    
    constants::AccessTokenType,
    http::Transport,
    api_req::ApiRequest,
    SDKResult,
};

use super::{
    requests::{GetDocRawContentV2Request, GetDocContentV2Request},
    responses::{GetDocRawContentV2Response, GetDocContentV2Response},
};

/// 文档内容服务
#[derive(Clone)]
pub struct ContentDocService {
    config: Config,
}

impl ContentDocService {
    /// 创建内容服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取旧版文档纯文本内容
    ///
    /// 获取文档的纯文本内容，不包含富文本格式信息。
    /// 适用于需要处理纯文本内容的场景，如搜索、分析等。
    ///
    /// # 参数
    /// - `req`: 获取文档纯文本内容请求
    ///
    /// # 返回
    /// - `SDKResult<GetDocRawContentV2Response>`: 文档纯文本内容响应
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::{DocV2Service, GetDocRawContentV2Request};
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = DocV2Service::new(config);
    ///
    ///     let request = GetDocRawContentV2Request {
    ///         doc_token: "doc_token_123".to_string(),
    ///         user_id_type: Some("open_id".to_string()),
    ///     };
    ///
    ///     let response = service.content.get_raw(&request).await?;
    ///     if let Some(content) = response.content {
    ///         println!("纯文本内容: {}", content.content.as_ref().unwrap_or(&"".to_string()));
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_raw(&self, req: &GetDocRawContentV2Request) -> SDKResult<GetDocRawContentV2Response> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始获取文档纯文本内容: doc_token={}", req.doc_token);

          // 构建动态端点路径，替换doc_token参数
        let endpoint = format!("/open-apis/doc/v2/{}/raw_content", req.doc_token);

        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: endpoint,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp = Transport::<GetDocRawContentV2Response>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档纯文本内容获取成功: doc_token={}, content_length={}",
            req.doc_token,
            response.content.as_ref()
                .and_then(|c| c.content.as_ref())
                .map(|s| s.len())
                .unwrap_or(0)
        );

        Ok(response)
    }

    /// 获取旧版文档富文本内容
    ///
    /// 获取文档的结构化富文本内容，保留格式信息。
    /// 适用于需要显示或编辑文档内容的场景。
    ///
    /// # 参数
    /// - `req`: 获取文档富文本内容请求
    ///
    /// # 返回
    /// - `SDKResult<GetDocContentV2Response>`: 文档富文本内容响应
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::{DocV2Service, GetDocContentV2Request};
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = DocV2Service::new(config);
    ///
    ///     let request = GetDocContentV2Request {
    ///         doc_token: "doc_token_123".to_string(),
    ///         user_id_type: Some("open_id".to_string()),
    ///     };
    ///
    ///     let response = service.content.get(&request).await?;
    ///     if let Some(content) = response.content {
    ///         println!("富文本内容: {}", content.content.as_ref().unwrap_or(&"".to_string()));
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, req: &GetDocContentV2Request) -> SDKResult<GetDocContentV2Response> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始获取文档富文本内容: doc_token={}", req.doc_token);

        // 构建动态端点路径，替换doc_token参数
        let endpoint = format!("/open-apis/doc/v2/{}/content", req.doc_token);

        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: endpoint,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp = Transport::<GetDocContentV2Response>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档富文本内容获取成功: doc_token={}, content_length={}",
            req.doc_token,
            response.content.as_ref()
                .and_then(|c| c.content.as_ref())
                .map(|s| s.len())
                .unwrap_or(0)
        );

        Ok(response)
    }

    /// 获取文档纯文本内容构建器
    ///
    /// 提供构建器模式获取文档纯文本内容，支持链式调用。
    ///
    /// # 参数
    /// - `doc_token`: 文档Token
    ///
    /// # 返回
    /// - `GetDocRawContentBuilder`: 文档纯文本内容获取构建器
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::DocV2Service;
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = DocV2Service::new(config);
    ///
    ///     let response = service.content
    ///         .get_raw_content_builder("doc_token_123")
    ///         .user_id_type("open_id")
    ///         .execute(&service.content)
    ///         .await?;
    ///
    ///     if let Some(content) = response.content {
    ///         println!("纯文本内容: {}", content.content.as_ref().unwrap_or(&"".to_string()));
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn get_raw_content_builder(&self, doc_token: impl Into<String>) -> GetDocRawContentBuilder {
        GetDocRawContentBuilder::new(doc_token)
    }

    /// 获取文档富文本内容构建器
    ///
    /// 提供构建器模式获取文档富文本内容，支持链式调用。
    ///
    /// # 参数
    /// - `doc_token`: 文档Token
    ///
    /// # 返回
    /// - `GetDocContentBuilder`: 文档富文本内容获取构建器
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::DocV2Service;
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = DocV2Service::new(config);
    ///
    ///     let response = service.content
    ///         .get_content_builder("doc_token_123")
    ///         .user_id_type("open_id")
    ///         .execute(&service.content)
    ///         .await?;
    ///
    ///     if let Some(content) = response.content {
    ///         println!("富文本内容: {}", content.content.as_ref().unwrap_or(&"".to_string()));
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn get_content_builder(&self, doc_token: impl Into<String>) -> GetDocContentBuilder {
        GetDocContentBuilder::new(doc_token)
    }
}

/// 文档纯文本内容获取构建器
#[derive(Debug, Clone)]
pub struct GetDocRawContentBuilder {
    doc_token: Option<String>,
    user_id_type: Option<String>,
}

impl GetDocRawContentBuilder {
    /// 创建新的构建器实例
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            doc_token: Some(doc_token.into()),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行获取操作
    pub async fn execute(
        self,
        service: &ContentDocService,
    ) -> SDKResult<GetDocRawContentV2Response> {
        let doc_token = self.doc_token.ok_or_else(|| {
            openlark_core::error::LarkAPIError::illegal_param("文档Token是必需的")
        })?;

        let request = GetDocRawContentV2Request {
            doc_token,
            user_id_type: self.user_id_type,
        };

        service.get_raw(&request).await
    }
}

/// 文档富文本内容获取构建器
#[derive(Debug, Clone)]
pub struct GetDocContentBuilder {
    doc_token: Option<String>,
    user_id_type: Option<String>,
}

impl GetDocContentBuilder {
    /// 创建新的构建器实例
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            doc_token: Some(doc_token.into()),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行获取操作
    pub async fn execute(
        self,
        service: &ContentDocService,
    ) -> SDKResult<GetDocContentV2Response> {
        let doc_token = self.doc_token.ok_or_else(|| {
            openlark_core::error::LarkAPIError::illegal_param("文档Token是必需的")
        })?;

        let request = GetDocContentV2Request {
            doc_token,
            user_id_type: self.user_id_type,
        };

        service.get(&request).await
    }
}
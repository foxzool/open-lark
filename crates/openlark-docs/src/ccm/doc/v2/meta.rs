//! 获取旧版文档元信息服务
//!
//! 提供获取旧版文档元数据的功能。

use openlark_core::{
    api_req::ApiRequest, config::Config, constants::AccessTokenType, http::Transport, SDKResult,
};
use std::collections::HashMap;

use super::{requests::GetDocMetaV2Request, responses::GetDocMetaV2Response};

/// 文档元信息服务
#[derive(Clone)]
pub struct MetaDocService {
    config: Config,
}

impl MetaDocService {
    /// 创建元信息服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取旧版文档元信息
    ///
    /// 根据文档Token获取文档的元数据信息，包括标题、类型、状态、
    /// 创建时间、更新时间、创建者等基本信息。
    ///
    /// # 参数
    /// - `req`: 获取文档元信息请求
    ///
    /// # 返回
    /// - `SDKResult<GetDocMetaV2Response>`: 文档元信息响应
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::{DocV2Service, GetDocMetaV2Request};
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = DocV2Service::new(config);
    ///
    ///     let request = GetDocMetaV2Request {
    ///         doc_token: "doc_token_123".to_string(),
    ///         user_id_type: Some("open_id".to_string()),
    ///     };
    ///
    ///     let response = service.meta.get(&request).await?;
    ///     println!("文档元信息: {:?}", response.document);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, req: &GetDocMetaV2Request) -> SDKResult<GetDocMetaV2Response> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始获取文档元信息: doc_token={}", req.doc_token);

        // 构建动态端点路径，替换doc_token参数
        let endpoint = req.doc_token.clone();
        let endpoint = format!("/open-apis/doc/v2/meta/{}", endpoint);

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

        let resp = Transport::<GetDocMetaV2Response>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档元信息获取成功: doc_token={}, title={:?}",
            req.doc_token,
            response.document.as_ref().and_then(|d| d.title.as_ref())
        );

        Ok(response)
    }

    /// 获取文档元信息构建器
    ///
    /// 提供构建器模式获取文档元信息，支持链式调用。
    ///
    /// # 参数
    /// - `doc_token`: 文档Token
    ///
    /// # 返回
    /// - `GetDocMetaBuilder`: 文档元信息获取构建器
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
    ///     let response = service.meta
    ///         .get_doc_meta_builder("doc_token_123")
    ///         .user_id_type("open_id")
    ///         .execute(&service.meta)
    ///         .await?;
    ///
    ///     println!("文档元信息: {:?}", response.document);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_doc_meta_builder(&self, doc_token: impl Into<String>) -> GetDocMetaBuilder {
        GetDocMetaBuilder::new(doc_token)
    }
}

/// 文档元信息获取构建器
///
/// 提供链式调用方式获取文档元信息。
#[derive(Debug, Clone)]
pub struct GetDocMetaBuilder {
    doc_token: Option<String>,
    user_id_type: Option<String>,
}

impl GetDocMetaBuilder {
    /// 创建新的构建器实例
    ///
    /// # 参数
    /// - `doc_token`: 文档Token
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            doc_token: Some(doc_token.into()),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型，如 "open_id", "user_id", "union_id"
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行获取操作
    ///
    /// # 参数
    /// - `service`: 文档元信息服务实例
    ///
    /// # 返回
    /// - `SDKResult<GetDocMetaV2Response>`: 文档元信息响应
    ///
    /// # 错误
    /// - 如果缺少必需参数（如doc_token），会返回错误
    pub async fn execute(self, service: &MetaDocService) -> SDKResult<GetDocMetaV2Response> {
        let doc_token = self.doc_token.ok_or_else(|| {
            openlark_core::error::LarkAPIError::illegal_param("文档Token是必需的")
        })?;

        let request = GetDocMetaV2Request {
            doc_token,
            user_id_type: self.user_id_type,
        };

        service.get(&request).await
    }
}

impl Default for GetDocMetaBuilder {
    fn default() -> Self {
        // 注意：默认情况下doc_token是必需的，所以这里不提供默认值
        Self {
            doc_token: None,
            user_id_type: None,
        }
    }
}

//! 创建旧版文档服务
//!
//! 提供创建和初始化旧版文档的功能。

use openlark_core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    api_req::ApiRequest,
    SDKResult,
};

use super::{
    requests::CreateDocV2Request,
    responses::CreateDocV2Response,
    models::DocType,
};

/// 文档创建服务
#[derive(Clone)]
pub struct CreateDocService {
    config: Config,
}

impl CreateDocService {
    /// 创建文档服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建旧版文档
    ///
    /// 创建并初始化一个新的旧版文档。
    ///
    /// # 参数
    /// - `req`: 创建文档请求
    ///
    /// # 返回
    /// - `SDKResult<CreateDocV2Response>`: 创建结果，包含文档基本信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::{DocV2Service, CreateDocV2Request};
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = DocV2Service::new(config);
    ///
    ///     let request = CreateDocV2Request {
    ///         title: "我的新文档".to_string(),
    ///         doc_type: Some(DocType::Doc),
    ///         folder_token: None,
    ///         template_id: None,
    ///     };
    ///
    ///     let response = service.create.create(&request).await?;
    ///     println!("文档创建成功: {:?}", response.document);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&self, req: &CreateDocV2Request) -> SDKResult<CreateDocV2Response> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始创建旧版文档: title={:?}", req.title);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/doc/v2/create".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateDocV2Response>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "旧版文档创建成功: title={}, doc_token={:?}",
            req.title,
            response.document.as_ref().and_then(|d| d.doc_token.as_ref())
        );

        Ok(response)
    }

    /// 创建文档构建器
    ///
    /// 提供构建器模式创建文档，支持链式调用。
    ///
    /// # 返回
    /// - `CreateDocBuilder`: 文档创建构建器
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
    ///     let response = service.create
    ///         .create_doc_builder()
    ///         .title("项目文档")
    ///         .doc_type(DocType::Doc)
    ///         .folder_token("folder_token_123")
    ///         .execute(&service.create)
    ///         .await?;
    ///
    ///     println!("文档创建成功: {:?}", response.document);
    ///     Ok(())
    /// }
    /// ```
    pub fn create_doc_builder(&self) -> CreateDocBuilder {
        CreateDocBuilder::new()
    }
}

/// 文档创建构建器
///
/// 提供链式调用方式创建文档，使API使用更加灵活和直观。
#[derive(Debug, Clone)]
pub struct CreateDocBuilder {
    title: Option<String>,
    doc_type: Option<DocType>,
    folder_token: Option<String>,
    template_id: Option<String>,
}

impl CreateDocBuilder {
    /// 创建新的构建器实例
    pub fn new() -> Self {
        Self {
            title: None,
            doc_type: None,
            folder_token: None,
            template_id: None,
        }
    }

    /// 设置文档标题
    ///
    /// # 参数
    /// - `title`: 文档标题，不能为空且长度不超过255个字符
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置文档类型
    ///
    /// # 参数
    /// - `doc_type`: 文档类型（doc、sheet、slide、mindnote）
    pub fn doc_type(mut self, doc_type: DocType) -> Self {
        self.doc_type = Some(doc_type);
        self
    }

    /// 设置文件夹Token
    ///
    /// # 参数
    /// - `folder_token`: 目标文件夹Token，指定文档创建位置
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 设置模板ID
    ///
    /// # 参数
    /// - `template_id`: 模板ID，使用指定模板创建文档
    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.template_id = Some(template_id.into());
        self
    }

    /// 执行创建操作
    ///
    /// # 参数
    /// - `service`: 文档创建服务实例
    ///
    /// # 返回
    /// - `SDKResult<CreateDocV2Response>`: 创建结果
    ///
    /// # 错误
    /// - 如果缺少必需参数（如标题），会返回错误
    pub async fn execute(
        self,
        service: &CreateDocService,
    ) -> SDKResult<CreateDocV2Response> {
        let title = self.title.ok_or_else(|| {
            openlark_core::error::LarkAPIError::illegal_param("文档标题是必需的")
        })?;

        let request = CreateDocV2Request {
            title,
            doc_type: self.doc_type,
            folder_token: self.folder_token,
            template_id: self.template_id,
        };

        service.create(&request).await
    }
}

impl Default for CreateDocBuilder {
    fn default() -> Self {
        Self::new()
    }
}
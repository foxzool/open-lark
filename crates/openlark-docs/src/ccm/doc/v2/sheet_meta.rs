//! 获取电子表格元数据服务
//!
//! 提供获取文档中电子表格元数据的功能，包括工作表信息、
//! 行列数量等。

use crate::prelude::*;
use openlark_core::{api::ApiRequest, constants::AccessTokenType, http::Transport, SDKResult};

use super::{requests::GetDocSheetMetaV2Request, responses::GetDocSheetMetaV2Response};

/// 电子表格元数据服务
#[derive(Clone, Debug)]
pub struct SheetMetaDocService {
    config: Config,
}

impl SheetMetaDocService {
    /// 创建电子表格元数据服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取旧版文档中的电子表格元数据
    ///
    /// 根据文档Token获取文档中电子表格的元数据信息。
    /// 仅适用于文档类型为电子表格的文档。
    ///
    /// # 参数
    /// - `req`: 获取电子表格元数据请求
    ///
    /// # 返回
    /// - `SDKResult<GetDocSheetMetaV2Response>`: 电子表格元数据响应
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::{DocV2Service, GetDocSheetMetaV2Request};
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret",
    ///     let service = DocV2Service::new(config,
    ///
    ///     let request = GetDocSheetMetaV2Request {
    ///         doc_token: "doc_token_123".to_string(),
    ///         user_id_type: Some("open_id".to_string()),
    ///     };
    ///
    ///     let response = service.sheet_meta.get(&request).await?;
    ///     if let Some(sheet_meta) = response.sheet_meta {
    ///         println!("工作表数量: {:?}", sheet_meta.sheet_count,
    ///         if let Some(sheets) = &sheet_meta.sheets {
    ///             for sheet in sheets {
    ///                 println!("工作表: {:?}", sheet.title,
    ///                 println!("行列数: {} x {}",
    ///                     sheet.row_count.unwrap_or(0),
    ///                     sheet.col_count.unwrap_or(0),
    ///             }
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(
        &self,
        req: &GetDocSheetMetaV2Request,
    ) -> SDKResult<GetDocSheetMetaV2Response> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!("开始获取电子表格元数据: doc_token={}", req.doc_token,

        // 构建动态端点路径，替换doc_token参数
        let endpoint = format!("/open-apis/doc/v2/{}/sheet_meta", req.doc_token,

        let mut query = std::collections::HashMap::new(,
        if let Some(user_id_type) = &req.user_id_type {
            .query(insert("user_id_type", user_id_type.clone()
        }

        
            query: std::collections::HashMap::new(),
            timeout: None,
            _phantom: std::marker::PhantomData,
            method: openlark_core::api::HttpMethod::Get,
            url: endpoint,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query,
            
        };

        let resp =
            Transport::<GetDocSheetMetaV2Response>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default(,

        log::info!(
            "电子表格元数据获取成功: doc_token={}, sheet_count={:?}",
            req.doc_token,
            response.sheet_meta.as_ref().and_then(|s| s.sheet_count)
        ,

        Ok(response)
    }

    /// 获取电子表格元数据构建器
    ///
    /// 提供构建器模式获取电子表格元数据，支持链式调用。
    ///
    /// # 参数
    /// - `doc_token`: 文档Token
    ///
    /// # 返回
    /// - `GetDocSheetMetaBuilder`: 电子表格元数据获取构建器
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::DocV2Service;
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret",
    ///     let service = DocV2Service::new(config,
    ///
    ///     let response = service.sheet_meta
    ///         .get_sheet_meta_builder("doc_token_123")
    ///         .user_id_type("open_id")
    ///         .execute(&service.sheet_meta)
    ///         .await?;
    ///
    ///     if let Some(sheet_meta) = response.sheet_meta {
    ///         println!("工作表数量: {:?}", sheet_meta.sheet_count,
    ///         if let Some(sheets) = &sheet_meta.sheets {
    ///             for sheet in sheets {
    ///                 println!("工作表: {:?}", sheet.title,
    ///             }
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn get_sheet_meta_builder(&self, doc_token: impl Into<String>) -> GetDocSheetMetaBuilder {
        GetDocSheetMetaBuilder::new(doc_token)
    }
}

/// 电子表格元数据获取构建器
///
/// 提供链式调用方式获取电子表格元数据。
#[derive(Debug, Clone)]
pub struct GetDocSheetMetaBuilder {
    doc_token: Option<String>,
    user_id_type: Option<String>,
}

impl GetDocSheetMetaBuilder {
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
        self.user_id_type = Some(user_id_type.into(),
        self
    }

    /// 执行获取操作
    ///
    /// # 参数
    /// - `service`: 电子表格元数据服务实例
    ///
    /// # 返回
    /// - `SDKResult<GetDocSheetMetaV2Response>`: 电子表格元数据响应
    ///
    /// # 错误
    /// - 如果缺少必需参数（如doc_token），会返回错误
    /// - 如果文档类型不是电子表格，可能会返回相应错误
    pub async fn execute(
        self,
        service: &SheetMetaDocService,
    ) -> SDKResult<GetDocSheetMetaV2Response> {
        let doc_token = self.doc_token.ok_or_else(|| {
            openlark_core::error::LarkAPIError::illegal_param("文档Token是必需的")
        })?;

        let request = GetDocSheetMetaV2Request {
            doc_token,
            user_id_type: self.user_id_type,
        };

        service.get(&request).await
    }
}

impl Default for GetDocSheetMetaBuilder {
    fn default() -> Self {
        // 注意：默认情况下doc_token是必需的，所以这里不提供默认值
        Self {
            doc_token: None,
            user_id_type: None,
        }
    }
}

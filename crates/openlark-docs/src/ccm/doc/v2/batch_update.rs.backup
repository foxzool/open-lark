//! 编辑旧版文档内容服务
//!
//! 提供批量编辑更新文档内容的功能，包括更新标题、
//! 范围删除、插入内容等操作。

use crate::prelude::*;
use openlark_core::{api::ApiRequest, constants::AccessTokenType, http::Transport, SDKResult};

use super::{
    models::{BatchUpdateOperation, BatchUpdateOperationType},
    requests::UpdateDocBatchV2Request,
    responses::UpdateDocBatchV2Response,
};

/// 文档批量更新服务
#[derive(Clone)]
pub struct BatchUpdateDocService {
    config: Config,
}

impl BatchUpdateDocService {
    /// 创建批量更新服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 编辑旧版文档内容
    ///
    /// 批量编辑更新文档内容，支持多种操作类型：
    /// - 更新文档标题
    /// - 插入内容到指定位置
    /// - 删除指定范围的内容
    /// - 替换指定范围的内容
    ///
    /// # 参数
    /// - `req`: 批量更新文档请求
    ///
    /// # 返回
    /// - `SDKResult<UpdateDocBatchV2Response>`: 批量更新结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::{DocV2Service, UpdateDocBatchV2Request};
    /// use open_lark::service::docs::v2::models::{BatchUpdateOperation, BatchUpdateOperationType};
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = DocV2Service::new(config);
    ///
    ///     let operations = vec![
    ///         BatchUpdateOperation {
    ///             operation_type: Some(BatchUpdateOperationType::UpdateTitle),
    ///             new_title: Some("新标题".to_string()),
    ///             ..Default::default()
    ///         },
    ///         BatchUpdateOperation {
    ///             operation_type: Some(BatchUpdateOperationType::Insert),
    ///             start_index: Some(0),
    ///             content: Some("插入的内容".to_string()),
    ///             ..Default::default()
    ///         },
    ///     ];
    ///
    ///     let request = UpdateDocBatchV2Request {
    ///         doc_token: "doc_token_123".to_string(),
    ///         operations,
    ///         user_id_type: Some("open_id".to_string()),
    ///     };
    ///
    ///     let response = service.batch_update.update(&request).await?;
    ///     println!("批量更新完成: {:?}", response.batch_update_result);
    ///     Ok(())
    /// }
    /// ```
    pub async fn update(
        &self,
        req: &UpdateDocBatchV2Request,
    ) -> SDKResult<UpdateDocBatchV2Response> {
        req.validate()
            .map_err(|msg| openlark_core::error::LarkAPIError::illegal_param(msg))?;

        log::debug!(
            "开始批量更新文档: doc_token={}, 操作数量={}",
            req.doc_token,
            req.operations.len()
        );

        // 构建动态端点路径，替换doc_token参数
        let endpoint = format!("/open-apis/doc/v2/{}/batch_update", req.doc_token);

        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: endpoint,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateDocBatchV2Response>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "文档批量更新完成: doc_token={}, 新版本={:?}",
            req.doc_token,
            response
                .batch_update_result
                .as_ref()
                .and_then(|r| r.new_revision)
        );

        Ok(response)
    }

    /// 批量更新文档构建器
    ///
    /// 提供构建器模式批量更新文档，支持链式调用。
    ///
    /// # 参数
    /// - `doc_token`: 文档Token
    ///
    /// # 返回
    /// - `BatchUpdateDocBuilder`: 批量更新文档构建器
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::docs::v2::DocV2Service;
    /// use open_lark::service::docs::v2::models::BatchUpdateOperationType;
    ///
    /// #[tokio::main]
    /// async fn main() -> SDKResult<()> {
    ///     let config = Config::new("app_id", "app_secret");
    ///     let service = DocV2Service::new(config);
    ///
    ///     let response = service.batch_update
    ///         .batch_update_builder("doc_token_123")
    ///         .user_id_type("open_id")
    ///         .update_title("新标题")
    ///         .insert_content(0, "插入的内容")
    ///         .replace_content(10, 20, "替换的内容")
    ///         .execute(&service.batch_update)
    ///         .await?;
    ///
    ///     println!("批量更新完成: {:?}", response.batch_update_result);
    ///     Ok(())
    /// }
    /// ```
    pub fn batch_update_builder(&self, doc_token: impl Into<String>) -> BatchUpdateDocBuilder {
        BatchUpdateDocBuilder::new(doc_token)
    }
}

/// 批量更新文档构建器
///
/// 提供链式调用方式批量更新文档内容。
#[derive(Debug, Clone)]
pub struct BatchUpdateDocBuilder {
    doc_token: Option<String>,
    user_id_type: Option<String>,
    operations: Vec<BatchUpdateOperation>,
}

impl BatchUpdateDocBuilder {
    /// 创建新的构建器实例
    ///
    /// # 参数
    /// - `doc_token`: 文档Token
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            doc_token: Some(doc_token.into()),
            user_id_type: None,
            operations: Vec::new(),
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

    /// 更新文档标题
    ///
    /// # 参数
    /// - `title`: 新的文档标题
    pub fn update_title(mut self, title: impl Into<String>) -> Self {
        self.operations.push(BatchUpdateOperation {
            operation_type: Some(BatchUpdateOperationType::UpdateTitle),
            new_title: Some(title.into()),
            ..Default::default()
        });
        self
    }

    /// 插入内容
    ///
    /// # 参数
    /// - `index`: 插入位置（从0开始）
    /// - `content`: 要插入的内容
    pub fn insert_content(mut self, index: i32, content: impl Into<String>) -> Self {
        self.operations.push(BatchUpdateOperation {
            operation_type: Some(BatchUpdateOperationType::Insert),
            start_index: Some(index),
            content: Some(content.into()),
            ..Default::default()
        });
        self
    }

    /// 删除内容
    ///
    /// # 参数
    /// - `start_index`: 删除起始位置（从0开始）
    /// - `end_index`: 删除结束位置（不包含）
    pub fn delete_content(mut self, start_index: i32, end_index: i32) -> Self {
        self.operations.push(BatchUpdateOperation {
            operation_type: Some(BatchUpdateOperationType::Delete),
            start_index: Some(start_index),
            end_index: Some(end_index),
            ..Default::default()
        });
        self
    }

    /// 替换内容
    ///
    /// # 参数
    /// - `start_index`: 替换起始位置（从0开始）
    /// - `end_index`: 替换结束位置（不包含）
    /// - `content`: 新的内容
    pub fn replace_content(
        mut self,
        start_index: i32,
        end_index: i32,
        content: impl Into<String>,
    ) -> Self {
        self.operations.push(BatchUpdateOperation {
            operation_type: Some(BatchUpdateOperationType::Replace),
            start_index: Some(start_index),
            end_index: Some(end_index),
            content: Some(content.into()),
            ..Default::default()
        });
        self
    }

    /// 添加自定义操作
    ///
    /// # 参数
    /// - `operation`: 自定义的批量更新操作
    pub fn add_operation(mut self, operation: BatchUpdateOperation) -> Self {
        self.operations.push(operation);
        self
    }

    /// 添加多个操作
    ///
    /// # 参数
    /// - `operations`: 批量更新操作列表
    pub fn add_operations(mut self, operations: Vec<BatchUpdateOperation>) -> Self {
        self.operations.extend(operations);
        self
    }

    /// 执行批量更新操作
    ///
    /// # 参数
    /// - `service`: 批量更新服务实例
    ///
    /// # 返回
    /// - `SDKResult<UpdateDocBatchV2Response>`: 批量更新结果
    ///
    /// # 错误
    /// - 如果缺少必需参数（如doc_token），会返回错误
    /// - 如果操作列表为空或超过限制，会返回错误
    pub async fn execute(
        self,
        service: &BatchUpdateDocService,
    ) -> SDKResult<UpdateDocBatchV2Response> {
        let doc_token = self.doc_token.ok_or_else(|| {
            openlark_core::error::LarkAPIError::illegal_param("文档Token是必需的")
        })?;

        if self.operations.is_empty() {
            return Err(openlark_core::error::LarkAPIError::illegal_param(
                "至少需要指定一个更新操作",
            ));
        }

        if self.operations.len() > 100 {
            return Err(openlark_core::error::LarkAPIError::illegal_param(
                "操作数量不能超过100个",
            ));
        }

        let request = UpdateDocBatchV2Request {
            doc_token,
            operations: self.operations,
            user_id_type: self.user_id_type,
        };

        service.update(&request).await
    }
}

impl Default for BatchUpdateDocBuilder {
    fn default() -> Self {
        // 注意：默认情况下doc_token是必需的，所以这里不提供默认值
        Self {
            doc_token: None,
            user_id_type: None,
            operations: Vec::new(),
        }
    }
}

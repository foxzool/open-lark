//! 编辑旧版文档内容服务
//!
//! 提供批量编辑更新文档内容的功能，包括更新标题、
//! 范围删除、插入内容等操作。

use crate::prelude::*;
use openlark_core::{api::ApiRequest, http::Transport, SDKResult};

use super::{
    models::{BatchUpdateOperation, BatchUpdateOperationType},
    requests::UpdateDocBatchV2Request,
    responses::UpdateDocBatchV2Response,
};

/// 文档批量更新服务
#[derive(Clone, Debug)]
pub struct BatchUpdateDocService {
    config: Config,
}

impl BatchUpdateDocService {
    /// 创建批量更新服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 编辑旧版文档内容
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

        let mut query = std::collections::HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query.insert("user_id_type".to_string(), user_id_type.clone());
        }

        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Post,
            url: endpoint,
            headers: std::collections::HashMap::new(),
            query,
            body: Some(openlark_core::api::RequestData::Json(serde_json::to_value(
                req,
            )?)),
            timeout: None,
            _phantom: std::marker::PhantomData,
        };

        let resp =
            Transport::<UpdateDocBatchV2Response>::request(api_req, &self.config, None).await?;

        log::info!(
            "批量更新文档完成: doc_token={}, 操作数量={}",
            req.doc_token,
            req.operations.len()
        );

        Ok(resp.data.unwrap_or_default())
    }
}

/// 批量更新文档构建器
#[derive(Debug, Clone)]
pub struct UpdateDocBatchV2Builder {
    request: UpdateDocBatchV2Request,
}

impl UpdateDocBatchV2Builder {
    /// 创建新的批量更新构建器
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            request: UpdateDocBatchV2Request {
                doc_token: doc_token.into(),
                operations: Vec::new(),
                user_id_type: None,
            },
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 更新文档标题
    pub fn update_title(mut self, title: impl Into<String>) -> Self {
        self.request.operations.push(BatchUpdateOperation {
            operation_type: Some(BatchUpdateOperationType::UpdateTitle),
            new_title: Some(title.into()),
            start_index: None,
            end_index: None,
            content: None,
        });
        self
    }

    /// 插入内容
    pub fn insert_content(mut self, index: i32, content: impl Into<String>) -> Self {
        self.request.operations.push(BatchUpdateOperation {
            operation_type: Some(BatchUpdateOperationType::Insert),
            new_title: None,
            start_index: Some(index),
            end_index: None,
            content: Some(content.into()),
        });
        self
    }

    /// 删除内容
    pub fn delete_content(mut self, start_index: i32, end_index: i32) -> Self {
        self.request.operations.push(BatchUpdateOperation {
            operation_type: Some(BatchUpdateOperationType::Delete),
            new_title: None,
            start_index: Some(start_index),
            end_index: Some(end_index),
            content: None,
        });
        self
    }

    /// 替换内容
    pub fn replace_content(
        mut self,
        start_index: i32,
        end_index: i32,
        content: impl Into<String>,
    ) -> Self {
        self.request.operations.push(BatchUpdateOperation {
            operation_type: Some(BatchUpdateOperationType::Replace),
            new_title: None,
            start_index: Some(start_index),
            end_index: Some(end_index),
            content: Some(content.into()),
        });
        self
    }

    /// 添加操作
    pub fn add_operation(mut self, operation: BatchUpdateOperation) -> Self {
        self.request.operations.push(operation);
        self
    }

    /// 添加多个操作
    pub fn add_operations(mut self, operations: Vec<BatchUpdateOperation>) -> Self {
        self.request.operations.extend(operations);
        self
    }

    /// 构建请求
    pub fn build(self) -> SDKResult<UpdateDocBatchV2Request> {
        if self.request.operations.is_empty() {
            return Err(openlark_core::error::LarkAPIError::illegal_param(
                "至少需要指定一个操作".to_string(),
            ));
        }

        if self.request.operations.len() > 100 {
            return Err(openlark_core::error::LarkAPIError::illegal_param(
                "操作数量不能超过100个".to_string(),
            ));
        }

        Ok(self.request)
    }
}

impl Default for UpdateDocBatchV2Builder {
    fn default() -> Self {
        Self::new("")
    }
}

//! Bitable V2 批量操作 API
//!
//! 提供高性能的批量数据操作功能，包括：
//! - 超大规模批量增删改
//! - 事务性批量操作
//! - 异步批量处理
//! - 操作结果追踪
use std::collections::HashMap;

use openlark_core::{
use serde_json::Value;
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 数据模型
pub mod models {
    use super::*;

    /// 批量操作请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct BulkOperationRequest {
        /// 应用token
        pub app_token: String,
        /// 数据表ID
        pub table_id: String,
        /// 操作类型
        pub operation_type: BulkOperationType,
        /// 操作数据
        pub data: Vec<Value>,
        /// 是否使用事务
        #[serde(skip_serializing_if = "Option::is_none")]
        pub use_transaction: Option<bool>,
        /// 回调配置（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        pub callback: Option<CallbackConfig>,
    }

    /// 批量操作类型
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum BulkOperationType {
        /// 批量插入
        #[serde(rename = "batch_insert")]
        BatchInsert,
        /// 批量更新
        #[serde(rename = "batch_update")]
        BatchUpdate,
        /// 批量删除
        #[serde(rename = "batch_delete")]
        BatchDelete,
        /// 批量插入或更新（upsert）
        #[serde(rename = "batch_upsert")]
        BatchUpsert,
    }

    /// 回调配置
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct CallbackConfig {
        /// 回调URL
        pub url: String,
        /// 回调头信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<std::collections::HashMap<String, String>>,
        /// 重试次数
        #[serde(skip_serializing_if = "Option::is_none")]
        pub retry_count: Option<i32>,
    }

    /// 批量操作响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct BulkOperationResponse {
        /// 操作结果
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<BulkOperationData>,
    }

    impl ApiResponseTrait for BulkOperationResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    /// 操作结果数据
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct BulkOperationData {
        /// 任务ID
        #[serde(skip_serializing_if = "Option::is_none")]
        pub task_id: Option<String>,
        /// 操作状态
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<BulkOperationStatus>,
        /// 成功数量
        #[serde(skip_serializing_if = "Option::is_none")]
        pub success_count: Option<i32>,
        /// 失败数量
        #[serde(skip_serializing_if = "Option::is_none")]
        pub failure_count: Option<i32>,
        /// 总数量
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total_count: Option<i32>,
        /// 错误详情
        #[serde(skip_serializing_if = "Option::is_none")]
        pub errors: Option<Vec<BulkOperationError>>,
    }

    /// 批量操作状态
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum BulkOperationStatus {
        /// 排队中
        #[serde(rename = "pending")]
        Pending,
        /// 处理中
        #[serde(rename = "processing")]
        Processing,
        /// 已完成
        #[serde(rename = "completed")]
        Completed,
        /// 失败
        #[serde(rename = "failed")]
        Failed,
    }

    /// 批量操作错误
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct BulkOperationError {
        /// 数据索引
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data_index: Option<i32>,
        /// 错误码
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_code: Option<String>,
        /// 错误信息
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_message: Option<String>,
        /// 原始数据
        #[serde(skip_serializing_if = "Option::is_none")]
        pub raw_data: Option<Value>,
    }

    /// 查询批量操作状态请求
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct GetBulkOperationStatusRequest {
        /// 应用token
        pub app_token: String,
        /// 任务ID
        pub task_id: String,
    }

    /// 查询批量操作状态响应
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    pub struct GetBulkOperationStatusResponse {
        /// 操作状态
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<BulkOperationData>,
    }

    impl ApiResponseTrait for GetBulkOperationStatusResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }
}

/// 服务实现
pub mod services {
    use super::*;

    /// 批量操作服务
    #[derive(Debug, Clone)]
    pub struct BatchOperationsService {
        config: Config,
    }

    impl BatchOperationsService {
        /// 创建批量操作服务实例
        pub fn new(config: Config) -> Self {
            Self { config }
        }

        /// 执行批量操作
        ///
        /// 执行大规模的批量数据操作
        ///
        /// # 参数
        /// * `request` - 批量操作请求
        ///
        /// # 返回
        /// 返回批量操作任务ID和初始状态
        pub async fn execute_bulk_operation(
            &self,
            request: &models::BulkOperationRequest,
        ) -> SDKResult<models::BulkOperationResponse> {
            // 验证请求参数
            request
                .validate()
                .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

            log::info!(
                "执行批量操作: app_token={}, table_id={}, operation_type={:?}, count={}",
                request.app_token,
                request.table_id,
                request.operation_type,
                request.data.len()
            );

            // 构建请求体
            let body = serde_json::to_value(request)?;

            // 构建API请求
            let api_req = ApiRequest::
                url: format!(
                url: format!(
                    "/open-apis/bitable/v2/apps/{}/tables/{}/bulk_operations",
                    request.app_token, request.table_id
                ),
                body: Some(serde_json::to_string(body: serde_json::to_vec(&body)?,body)?.into()),
                .query(HashMap::new())
                .query(HashMap::new())
            };

            // 发送请求
            let resp =
                Transport::<models::BulkOperationResponse>::request(api_req, &self.config, None)
                    .await?;
            let response = resp.data.unwrap_or_default();

            if let Some(ref data) = response.data {
                log::info!(
                    "批量操作任务已创建: task_id={}, status={:?}",
                    data.task_id.as_deref().unwrap_or("unknown"),
                    data.status
                );
            }

            Ok(response)
        }

        /// 查询批量操作状态
        ///
        /// 查询异步批量操作的任务状态和结果
        ///
        /// # 参数
        /// * `request` - 查询状态请求
        ///
        /// # 返回
        /// 返回操作状态和结果详情
        pub async fn get_bulk_operation_status(
            &self,
            request: &models::GetBulkOperationStatusRequest,
        ) -> SDKResult<models::GetBulkOperationStatusResponse> {
            // 验证请求参数
            request
                .validate()
                .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

            log::info!(
                "查询批量操作状态: app_token={}, task_id={}",
                request.app_token,
                request.task_id
            );

            // 构建API请求
            let api_req = ApiRequest::
                url: format!(
                url: format!(
                    "/open-apis/bitable/v2/apps/{}/bulk_operations/{}",
                    request.app_token, request.task_id
                ),
                body: None,
                .query(HashMap::new())
                .query(HashMap::new())
            };

            // 发送请求
            let resp = Transport::<models::GetBulkOperationStatusResponse>::request(
                api_req,
                &self.config,
                None,
            )
            .await?;
            let response = resp.data.unwrap_or_default();

            if let Some(ref data) = response.data {
                log::info!("批量操作状态查询完成: task_id={}, status={:?}, success_count={}, failure_count={}",
                          request.task_id,
                          data.status,
                          data.success_count.unwrap_or(0),
                          data.failure_count.unwrap_or(0));
            }

            Ok(response)
        }
    }
}

// 为请求结构体添加验证方法
impl models::BulkOperationRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.table_id.trim().is_empty() {
            return Err("数据表ID不能为空".to_string());
        }
        if self.data.is_empty() {
            return Err("操作数据不能为空".to_string());
        }
        if self.data.len() > 10000 {
            return Err("批量操作数据量不能超过10000条".to_string());
        }

        // 验证回调URL格式
        if let Some(ref callback) = self.callback {
            if callback.url.trim().is_empty() {
                return Err("回调URL不能为空".to_string());
            }
            if !callback.url.starts_with("http://") && !callback.url.starts_with("https://") {
                return Err("回调URL格式不正确".to_string());
            }
        }

        Ok(())
    }
}

impl models::GetBulkOperationStatusRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.app_token.trim().is_empty() {
            return Err("应用token不能为空".to_string());
        }
        if self.task_id.trim().is_empty() {
            return Err("任务ID不能为空".to_string());
        }
        Ok(())
    }
}

// 重新导出
pub use models::*;
pub use services::BatchOperationsService;

// 构建器模式实现
pub struct BulkOperationRequestBuilder {
    request: models::BulkOperationRequest,
}

impl BulkOperationRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        table_id: impl Into<String>,
        operation_type: models::BulkOperationType,
        data: Vec<Value>,
    ) -> Self {
        Self {
            request: models::BulkOperationRequest {
                app_token: app_token.into(),
                table_id: table_id.into(),
                operation_type,
                data,
                use_transaction: None,
                callback: None,
            },
        }
    }

    pub fn use_transaction(mut self, use_transaction: bool) -> Self {
        self.request.use_transaction = Some(use_transaction);
        self
    }

    pub fn callback(mut self, callback: models::CallbackConfig) -> Self {
        self.request.callback = Some(callback);
        self
    }

    pub async fn execute(
        self,
        service: &BatchOperationsService,
    ) -> SDKResult<models::BulkOperationResponse> {
        service.execute_bulk_operation(&self.request).await
    }
}

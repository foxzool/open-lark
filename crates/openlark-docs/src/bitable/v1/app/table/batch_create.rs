//! Bitable 批量创建数据表API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/batchCreate

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

// 导入通用批量操作模块
use crate::common::batch::BatchOperationResult;
use crate::common::types::{AppToken, TableId};

// 导入 TableData 类型
use super::create::TableData;

/// 批量新增数据表请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BatchCreateTableRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<BatchCreateTableResponse>,
    /// 多维表格的 app_token
    app_token: AppToken,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 客户端令牌，用于幂等操作
    client_token: Option<String>,
    /// 要新增的数据表列表
    tables: Vec<TableData>,
}

impl BatchCreateTableRequest {
    /// 创建批量新增数据表请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post(""),
            app_token: String::new(),
            user_id_type: None,
            client_token: None,
            tables: Vec::new(),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置客户端令牌，用于幂等操作
    pub fn client_token(mut self, client_token: String) -> Self {
        self.client_token = Some(client_token);
        self
    }

    /// 设置要新增的数据表列表
    pub fn tables(mut self, tables: Vec<TableData>) -> Self {
        self.tables = tables;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchCreateTableResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.tables.is_empty() {
            return Err(validation_error("tables", "数据表列表不能为空"));
        }

        if self.tables.len() > 50 {
            return Err(validation_error("tables", "批量创建数据表数量不能超过50个"));
        }

        // 验证每个数据表
        for (index, table) in self.tables.iter().enumerate() {
            if table.name.trim().is_empty() {
                return Err(validation_error(
                    &format!("tables[{}].name", index),
                    "数据表名称不能为空",
                ));
            }
            if table.name.len() > 100 {
                return Err(validation_error(
                    &format!("tables[{}].name", index),
                    "数据表名称长度不能超过100个字符",
                ));
            }
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables:batch_create",
            self.config.base_url, self.app_token
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 构建请求体
        let request_body = BatchCreateTableRequestBody {
            tables: self.tables,
            user_id_type: self.user_id_type,
            client_token: self.client_token,
        };

        // 设置请求体
        api_request.body = Some(RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::post(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 解析响应数据
        let response_data: BatchCreateTableResponse = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析批量创建数据表响应失败", "响应数据格式不正确"))?;

        Ok(BatchCreateTableResponse {
            tables: response_data.tables,
            success: response.raw_response.is_success(),
        })
    }
}

/// 批量新增数据表Builder
pub struct BatchCreateTableRequestBuilder {
    request: BatchCreateTableRequest,
}

impl BatchCreateTableRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchCreateTableRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置客户端令牌，用于幂等操作
    pub fn client_token(mut self, client_token: String) -> Self {
        self.request = self.request.client_token(client_token);
        self
    }

    /// 设置要新增的数据表列表
    pub fn tables(mut self, tables: Vec<TableData>) -> Self {
        self.request = self.request.tables(tables);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchCreateTableRequest {
        self.request
    }
}

// 执行批量创建数据表操作的实现将在后续版本中完成

/// 批量新增数据表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateTableResponse {
    /// 新增的数据表列表
    pub tables: Vec<BatchCreateTableResult>,
    /// 操作结果
    pub success: bool,
}

/// 批量创建数据表的结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateTableResult {
    /// 数据表 ID
    #[serde(skip)]
    pub table_id: TableId,
    /// 数据表名称
    pub name: String,
    /// 数据表的默认视图 ID
    pub default_view_id: String,
    /// 批量操作结果
    #[serde(flatten)]
    pub operation: BatchOperationResult,
}

impl BatchCreateTableResult {
    /// 创建成功的结果
    pub fn success(table_id: impl Into<TableId>, name: String, default_view_id: String) -> Self {
        Self {
            table_id: table_id.into(),
            name,
            default_view_id,
            operation: BatchOperationResult::success(),
        }
    }

    /// 创建失败的结果
    pub fn failure(table_id: impl Into<TableId>, name: String, error: impl ToString) -> Self {
        Self {
            table_id: table_id.into(),
            name,
            default_view_id: String::new(),
            operation: BatchOperationResult::failure(error),
        }
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.operation.success
    }

    /// 获取错误信息
    pub fn get_error(&self) -> Option<&str> {
        self.operation.error.as_deref()
    }

    /// 获取表格ID
    pub fn get_table_id(&self) -> &TableId {
        &self.table_id
    }
}

/// 批量新增数据表请求体（内部使用）
#[derive(Serialize)]
struct BatchCreateTableRequestBody {
    /// 要新增的数据表列表
    tables: Vec<TableData>,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
    /// 客户端令牌，用于幂等操作
    #[serde(skip_serializing_if = "Option::is_none")]
    client_token: Option<String>,
}

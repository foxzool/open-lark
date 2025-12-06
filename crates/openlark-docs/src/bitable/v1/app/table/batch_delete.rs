//! Bitable 批量删除数据表API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/batchDelete

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 批量删除数据表请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BatchDeleteTableRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<BatchDeleteTableResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    client_token: Option<String>,
    /// 要删除的数据表 ID 列表
    table_ids: Vec<String>,
}

impl BatchDeleteTableRequest {
    /// 创建批量删除数据表请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::delete(""),
            app_token: String::new(),
            user_id_type: None,
            client_token: None,
            table_ids: Vec::new(),
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

    /// 设置要删除的数据表ID列表
    pub fn table_ids(mut self, table_ids: Vec<String>) -> Self {
        self.table_ids = table_ids;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchDeleteTableResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_ids.is_empty() {
            return Err(validation_error("table_ids", "数据表ID列表不能为空"));
        }

        if self.table_ids.len() > 50 {
            return Err(validation_error(
                "table_ids",
                "批量删除数据表数量不能超过50个",
            ));
        }

        // 验证每个数据表ID
        for (index, table_id) in self.table_ids.iter().enumerate() {
            if table_id.trim().is_empty() {
                return Err(validation_error(
                    &format!("table_ids[{}]", index),
                    "数据表ID不能为空",
                ));
            }
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables:batch_delete",
            self.config.base_url, self.app_token
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 构建请求体
        let request_body = BatchDeleteTableRequestBody {
            table_ids: self.table_ids,
            user_id_type: self.user_id_type,
            client_token: self.client_token,
        };

        // 设置请求体
        api_request.body = Some(RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::delete(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 解析响应数据
        let response_data: BatchDeleteTableResponse = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析批量删除数据表响应失败", "响应数据格式不正确"))?;

        Ok(BatchDeleteTableResponse {
            results: response_data.results,
            success: response.raw_response.is_success(),
        })
    }
}

/// 批量删除数据表Builder
pub struct BatchDeleteTableRequestBuilder {
    request: BatchDeleteTableRequest,
}

impl BatchDeleteTableRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchDeleteTableRequest::new(config),
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

    /// 设置要删除的数据表ID列表
    pub fn table_ids(mut self, table_ids: Vec<String>) -> Self {
        self.request = self.request.table_ids(table_ids);
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchDeleteTableRequest {
        self.request
    }
}

// 执行批量删除数据表操作的实现将在后续版本中完成

/// 批量删除数据表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteTableResponse {
    /// 删除的数据表结果列表
    pub results: Vec<BatchDeleteTableResult>,
    /// 操作结果
    pub success: bool,
}

/// 批量删除数据表的结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteTableResult {
    /// 数据表 ID
    pub table_id: String,
    /// 是否删除成功
    pub success: bool,
    /// 错误信息（如果删除失败）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 批量删除数据表请求体（内部使用）
#[derive(Serialize)]
struct BatchDeleteTableRequestBody {
    table_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_token: Option<String>,
}

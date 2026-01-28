//! 执行SQL
//!
//! URL: POST:/open-apis/apaas/v1/workspaces/:workspace_id/sql_commands

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 执行SQL Builder
#[derive(Debug, Clone)]
pub struct SqlCommandsBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// SQL 语句
    sql: String,
}

impl SqlCommandsBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, workspace_id: impl Into<String>, sql: impl Into<String>) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            sql: sql.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SqlCommandsResponse> {
        let url = format!("/open-apis/apaas/v1/workspaces/{}/sql_commands", self.workspace_id);

        let request = SqlCommandsRequest {
            sql: self.sql,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, None::<&()>).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<SqlCommandsResponse> {
        let url = format!("/open-apis/apaas/v1/workspaces/{}/sql_commands", self.workspace_id);

        let request = SqlCommandsRequest {
            sql: self.sql,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, Some(option)).await
    }
}

/// 执行SQL请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct SqlCommandsRequest {
    /// SQL 语句
    #[serde(rename = "sql")]
    sql: String,
}

/// SQL 执行结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SqlResult {
    /// 结果数据
    #[serde(rename = "data")]
    data: Vec<serde_json::Value>,
    /// 影响行数
    #[serde(rename = "affected_rows")]
    affected_rows: u32,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

/// 执行SQL响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SqlCommandsResponse {
    /// SQL 执行结果
    #[serde(rename = "result")]
    result: SqlResult,
}

impl ApiResponseTrait for SqlCommandsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

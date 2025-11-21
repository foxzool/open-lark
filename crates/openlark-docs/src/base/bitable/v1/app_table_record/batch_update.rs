#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
    api::ApiRequest,
    core::{BaseResponse, ResponseFormat, api::ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    service::bitable::v1::Record,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量更新记录请求
#[derive(Clone)]
pub struct BatchUpdateRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    #[serde(skip)]
    client_token: Option<String>,
    /// 更新的记录列表
    pub records: Vec<Record>,
}

impl BatchUpdateRecordRequest {
    /// 创建批量更新记录请求
    pub fn new(app_token: String, table_id: String, records: Vec<Record>) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(reqwest::Method::POST)
                .path("/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_update"),
            app_token,
            table_id,
            user_id_type: None,
            client_token: None,
            records,
        }
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置客户端令牌
    pub fn client_token(mut self, client_token: String) -> Self {
        self.client_token = Some(client_token);
        self
    }

    /// 执行批量更新记录请求
    pub async fn execute(&self, transport: &Transport) -> SDKResult<serde_json::Value> {
        let path = self.api_request.path()
            .replace(":app_token", &self.app_token)
            .replace(":table_id", &self.table_id);

        let mut api_request = self.api_request.clone();
        api_request = api_request.path(&path);

        transport.execute(&api_request).await
    }
}
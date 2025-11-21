#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

use openlark_core::{
    api::ApiRequest,
    core::{BaseResponse, ResponseFormat, api::ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{AppTableService, TableData};

/// 批量新增数据表请求
#[derive(Clone)]
pub struct BatchCreateTablesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的 app_token
    #[serde(skip)]
    app_token: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    #[serde(skip)]
    client_token: Option<String>,
    /// 数据表列表
    pub tables: Vec<TableData>,
}

impl BatchCreateTablesRequest {
    /// 创建批量新增数据表请求
    pub fn new(app_token: String, tables: Vec<TableData>) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(reqwest::Method::POST)
                .path("/open-apis/bitable/v1/apps/:app_token/tables/batch_create"),
            app_token,
            user_id_type: None,
            client_token: None,
            tables,
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

    /// 执行批量新增数据表请求
    pub async fn execute(&self, transport: &Transport) -> SDKResult<serde_json::Value> {
        let path = self.api_request.path()
            .replace(":app_token", &self.app_token);

        let mut api_request = self.api_request.clone();
        api_request = api_request.path(&path);

        transport.execute(&api_request).await
    }
}

impl AppTableService {
    pub fn batch_create_builder(&self, tables: Vec<TableData>) -> BatchCreateTablesRequest {
        BatchCreateTablesRequest::new(self.app_token.clone(), tables)
    }
}
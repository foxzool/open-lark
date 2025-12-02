//! 删除数据表模块

use openlark_core::{
    api::{
        ApiRequest, ApiResponseTrait, BaseResponse, ResponseFormat, HttpMethod,
    },
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use crate::endpoints::BITABLE_V1_TABLE_DELETE;

/// 删除数据表请求
pub struct DeleteTableRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
}

impl Default for DeleteTableRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::delete(BITABLE_V1_TABLE_DELETE.to_string()),
            app_token: String::new(),
            table_id: String::new(),
        }
    }
}

impl DeleteTableRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::delete(
                BITABLE_V1_TABLE_DELETE.to_string()
            ),
            app_token: String::new(),
            table_id: String::new(),
        }
    }

    pub fn builder() -> DeleteTableRequestBuilder {
        DeleteTableRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteTableRequestBuilder {
    request: DeleteTableRequest,
}

impl DeleteTableRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    pub fn build(self) -> DeleteTableRequest {
        self.request
    }
}

/// 删除数据表响应
pub struct DeleteTableResponse {
    /// 删除的数据表ID
    pub deleted: bool,
}

impl ApiResponseTrait for DeleteTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}


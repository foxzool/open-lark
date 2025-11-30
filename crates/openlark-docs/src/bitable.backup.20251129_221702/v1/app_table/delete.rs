//! 删除数据表模块

use openlark_core::{
    core::{
        BaseResponse,
        ResponseFormat,
        api::ApiResponseTrait,
    },
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 删除数据表请求
#[derive(Clone)]
pub struct DeleteTableRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
}

impl DeleteTableRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::DELETE,
                DELETE_TABLE.to_string(),
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
#[derive(Clone)]
pub struct DeleteTableResponse {
    /// 删除的数据表ID
    pub deleted: bool,
}

impl ApiResponseTrait for DeleteTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_table_request() {
        let request = DeleteTableRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
    }
}
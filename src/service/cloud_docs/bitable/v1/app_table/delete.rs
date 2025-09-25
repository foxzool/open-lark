use reqwest::Method;
use serde::Deserialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::AppTableService;

impl AppTableService {
    /// 删除一个数据表
    pub async fn delete(
        &self,
        request: DeleteTableRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteTableResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::DELETE;
        api_req.api_path = BITABLE_V1_TABLE_DELETE
            .replace("{app_token}", &request.app_token)
            .replace("{table_id}", &request.table_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 删除数据表请求
#[derive(Debug, Default)]
pub struct DeleteTableRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
}

impl DeleteTableRequest {
    pub fn builder() -> DeleteTableRequestBuilder {
        DeleteTableRequestBuilder::default()
    }

    /// 创建删除数据表请求
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
        }
    }
}

#[derive(Default)]
pub struct DeleteTableRequestBuilder {
    request: DeleteTableRequest,
}

impl DeleteTableRequestBuilder {
    /// 多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 数据表的 table_id
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    pub fn build(self) -> DeleteTableRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    DeleteTableRequestBuilder,
    AppTableService,
    DeleteTableRequest,
    BaseResponse<DeleteTableResponse>,
    delete
);

#[derive(Deserialize, Debug)]
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
#[allow(unused_variables, unused_unsafe)]
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

    #[test]
    fn test_delete_table_request_new() {
        let request = DeleteTableRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
    }
}

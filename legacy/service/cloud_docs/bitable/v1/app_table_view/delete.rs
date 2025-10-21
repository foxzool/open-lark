use reqwest::Method;
use serde::Deserialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_config,
};

use super::AppTableViewService;

impl AppTableViewService {
    /// 删除视图
    pub async fn delete(
        &self,
        request: DeleteViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteViewResponse>> {
        delete_view(request, &self.config, option).await
    }
}

/// 删除视图
pub async fn delete_view(
    request: DeleteViewRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeleteViewResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::DELETE;
    api_req.api_path = BITABLE_V1_VIEW_DELETE
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{view_id}", &request.view_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

/// 删除视图请求
#[derive(Debug, Default)]
pub struct DeleteViewRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String,
}

impl DeleteViewRequest {
    pub fn builder() -> DeleteViewRequestBuilder {
        DeleteViewRequestBuilder::default()
    }

    /// 创建删除视图请求
    pub fn new(app_token: impl ToString, table_id: impl ToString, view_id: impl ToString) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            view_id: view_id.to_string(),
        }
    }
}

#[derive(Default)]
pub struct DeleteViewRequestBuilder {
    request: DeleteViewRequest,
}

impl DeleteViewRequestBuilder {
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

    /// 视图的 view_id
    pub fn view_id(mut self, view_id: impl ToString) -> Self {
        self.request.view_id = view_id.to_string();
        self
    }

    pub fn build(self) -> DeleteViewRequest {
        self.request
    }
}

impl_executable_builder_config!(
    DeleteViewRequestBuilder,
    DeleteViewRequest,
    BaseResponse<DeleteViewResponse>,
    delete_view
);

#[derive(Deserialize, Debug)]
pub struct DeleteViewResponse {
    /// 删除结果
    pub deleted: bool,
}

impl ApiResponseTrait for DeleteViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_view_request() {
        let request = DeleteViewRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .view_id("vewTpR1urY")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
    }

    #[test]
    fn test_delete_view_request_new() {
        let request =
            DeleteViewRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW", "vewTpR1urY");

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
    }
}

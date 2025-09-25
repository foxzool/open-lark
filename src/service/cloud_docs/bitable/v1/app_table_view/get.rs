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

use super::AppTableViewService;

impl AppTableViewService {
    /// 获取视图
    pub async fn get(
        &self,
        request: GetViewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetViewResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::GET;
        api_req.api_path = BITABLE_V1_VIEW_GET
            .replace("{app_token}", &request.app_token)
            .replace("{table_id}", &request.table_id)
            .replace("{view_id}", &request.view_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 获取视图请求
#[derive(Debug, Default)]
pub struct GetViewRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String,
}

impl GetViewRequest {
    pub fn builder() -> GetViewRequestBuilder {
        GetViewRequestBuilder::default()
    }

    /// 创建获取视图请求
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
pub struct GetViewRequestBuilder {
    request: GetViewRequest,
}

impl GetViewRequestBuilder {
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

    pub fn build(self) -> GetViewRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    GetViewRequestBuilder,
    AppTableViewService,
    GetViewRequest,
    BaseResponse<GetViewResponse>,
    get
);

#[derive(Deserialize, Debug)]
pub struct GetViewResponse {
    /// 视图信息
    pub view: ViewDetailInfo,
}

#[derive(Deserialize, Debug)]
pub struct ViewDetailInfo {
    /// 视图 ID
    pub view_id: String,
    /// 视图名称
    pub view_name: String,
    /// 视图类型
    pub view_type: String,
    /// 视图的自定义属性
    #[serde(default)]
    pub property: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_get_view_request() {
        let request = GetViewRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .view_id("vewTpR1urY")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
    }

    #[test]
    fn test_get_view_request_new() {
        let request = GetViewRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW", "vewTpR1urY");

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
    }
}

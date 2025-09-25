use reqwest::Method;
use serde::{Deserialize, Serialize};

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
    /// 更新数据表
    pub async fn patch(
        &self,
        request: PatchTableRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PatchTableResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::PATCH;
        api_req.api_path = BITABLE_V1_TABLE_PATCH
            .replace("{app_token}", &request.app_token)
            .replace("{table_id}", &request.table_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = serde_json::to_vec(&PatchTableRequestBody { name: request.name })?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 更新数据表请求
#[derive(Debug, Default)]
pub struct PatchTableRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 数据表的新名称
    name: Option<String>,
}

impl PatchTableRequest {
    pub fn builder() -> PatchTableRequestBuilder {
        PatchTableRequestBuilder::default()
    }

    /// 创建更新数据表请求
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            name: None,
        }
    }
}

#[derive(Default)]
pub struct PatchTableRequestBuilder {
    request: PatchTableRequest,
}

impl PatchTableRequestBuilder {
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

    /// 数据表的新名称
    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = Some(name.to_string());
        self
    }

    pub fn build(self) -> PatchTableRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    PatchTableRequestBuilder,
    AppTableService,
    PatchTableRequest,
    BaseResponse<PatchTableResponse>,
    patch
);

#[derive(Serialize)]
struct PatchTableRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PatchTableResponse {
    /// 数据表的名称
    pub name: String,
}

impl ApiResponseTrait for PatchTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_patch_table_request() {
        let request = PatchTableRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .name("更新后的表名")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.name, Some("更新后的表名".to_string()));
    }

    #[test]
    fn test_patch_table_request_new() {
        let request = PatchTableRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.name, None);
    }

    #[test]
    fn test_patch_table_request_body_serialization() {
        let body = PatchTableRequestBody {
            name: Some("新表名".to_string()),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "新表名"
        });

        assert_eq!(serialized, expected);
    }
}

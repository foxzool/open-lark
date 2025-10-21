use reqwest::Method;
use serde::{Deserialize, Serialize};

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

/// 删除字段请求
#[derive(Debug, Serialize, Default)]
pub struct DeleteFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 字段的唯一标识符
    #[serde(skip)]
    field_id: String,
}

impl DeleteFieldRequest {
    pub fn builder() -> DeleteFieldRequestBuilder {
        DeleteFieldRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, table_id: impl ToString, field_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            field_id: field_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct DeleteFieldRequestBuilder {
    request: DeleteFieldRequest,
}

impl DeleteFieldRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 数据表的唯一标识符
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 字段的唯一标识符
    pub fn field_id(mut self, field_id: impl ToString) -> Self {
        self.request.field_id = field_id.to_string();
        self
    }

    pub fn build(self) -> DeleteFieldRequest {
        self.request
    }
}

impl_executable_builder_config!(
    DeleteFieldRequestBuilder,
    DeleteFieldRequest,
    BaseResponse<DeleteFieldResponse>,
    delete_field
);

/// 删除字段响应
#[derive(Debug, Deserialize)]
pub struct DeleteFieldResponse {
    /// 删除的字段 ID
    pub field_id: String,
    /// 是否删除成功
    pub deleted: bool,
}

impl ApiResponseTrait for DeleteFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除字段
pub async fn delete_field(
    request: DeleteFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeleteFieldResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::DELETE;
    api_req.api_path = BITABLE_V1_FIELD_DELETE
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{field_id}", &request.field_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_field_request_builder() {
        let request = DeleteFieldRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .field_id("fldxxxxxx")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.field_id, "fldxxxxxx");
    }
}

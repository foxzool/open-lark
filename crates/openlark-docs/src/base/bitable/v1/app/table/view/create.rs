//! 在数据表中新增一个视图
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/create

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateViewRequest {
    pub view_name: String,
    pub view_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateViewResponse {
    pub view: View,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct View {
    pub view_id: String,
    pub view_name: String,
    pub view_type: String,
}

impl ApiResponseTrait for CreateViewResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct CreateViewBuilder {
    api_req: ApiRequest<CreateViewRequest>,
    app_token: String,
    table_id: String,
}

impl CreateViewBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_view_create".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/views",
            builder.app_token, builder.table_id
        );
        builder.api_req.body = Some(CreateViewRequest::default());
        builder
    }

    pub fn view_name(mut self, view_name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.view_name = view_name.to_string();
        }
        self
    }

    pub fn view_type(mut self, view_type: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.view_type = view_type.to_string();
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}

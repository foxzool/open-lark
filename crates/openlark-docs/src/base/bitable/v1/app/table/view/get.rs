//! 该接口根据 view_id 检索现有视图
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/get

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetViewRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetViewResponse {
    pub view: View,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct View {
    pub view_id: String,
    pub view_name: String,
    pub view_type: String,
    pub property: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetViewResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetViewBuilder {
    api_req: ApiRequest<GetViewRequest>,
    app_token: String,
    table_id: String,
    view_id: String,
}

impl GetViewBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString, view_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_view_get".to_string();
        builder.api_req.method = "GET".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.view_id = view_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
            builder.app_token, builder.table_id, builder.view_id
        );
        builder.api_req.body = None;
        builder
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

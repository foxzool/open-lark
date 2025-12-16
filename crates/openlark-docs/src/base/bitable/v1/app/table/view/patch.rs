//! 该接口用于增量修改视图信息
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/patch

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchViewRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PatchViewResponse {
    pub view: View,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct View {
    pub view_id: String,
    pub view_name: String,
    pub view_type: String,
    pub property: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchViewResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct PatchViewBuilder {
    api_req: ApiRequest<PatchViewRequest>,
    app_token: String,
    table_id: String,
    view_id: String,
}

impl PatchViewBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString, view_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_view_patch".to_string();
        builder.api_req.method = "PATCH".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.view_id = view_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
            builder.app_token, builder.table_id, builder.view_id
        );
        builder.api_req.body = Some(PatchViewRequest::default());
        builder
    }

    pub fn view_name(mut self, view_name: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.view_name = Some(view_name.to_string());
        }
        self
    }

    pub fn property(mut self, property: serde_json::Value) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.property = Some(property);
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

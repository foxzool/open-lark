//! 删除一个数据表
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteTableRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteTableResponse {}

impl ApiResponseTrait for DeleteTableResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct DeleteTableBuilder {
    api_req: ApiRequest<DeleteTableRequest>,
    app_token: String,
    table_id: String,
}

impl DeleteTableBuilder {
    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_table_delete".to_string();
        builder.api_req.method = "DELETE".to_string();
        builder.app_token = app_token.to_string();
        builder.table_id = table_id.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}",
            builder.app_token, builder.table_id
        );
        builder.api_req.body = Some(DeleteTableRequest::default());
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

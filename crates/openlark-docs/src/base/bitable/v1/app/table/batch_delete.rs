//! 删除多个数据表
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/batch_delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteTableRequest {
    pub table_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteTableResponse {}

impl ApiResponseTrait for BatchDeleteTableResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct BatchDeleteTableBuilder {
    api_req: ApiRequest<BatchDeleteTableRequest>,
    app_token: String,
}

impl BatchDeleteTableBuilder {
    pub fn new(app_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "bitable_table_batch_delete".to_string();
        builder.api_req.method = "POST".to_string();
        builder.app_token = app_token.to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/batch_delete",
            builder.app_token
        );
        builder.api_req.body = Some(BatchDeleteTableRequest::default());
        builder
    }

    pub fn table_ids(mut self, table_ids: Vec<String>) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.table_ids = table_ids;
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

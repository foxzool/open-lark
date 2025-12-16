//! 获取词库列表
//!
//! doc: https://open.feishu.cn/document/lingo-v1/repo/list

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListRepoRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListRepoResponse {
    pub items: Vec<Repo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Repo {
    pub id: String,
    pub name: String,
}

impl ApiResponseTrait for ListRepoResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct ListRepoBuilder {
    api_req: ApiRequest<ListRepoRequest>,
}

impl ListRepoBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "lingo_repo_list".to_string();
        builder.api_req.method = "GET".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/lingo/v1/repos".to_string();
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

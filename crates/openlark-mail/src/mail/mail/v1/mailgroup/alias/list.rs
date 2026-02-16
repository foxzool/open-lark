//! 获取邮件组所有别名

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ListMailGroupAliasRequest {
    config: Arc<Config>,
    mailgroup_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailGroupAliasResponse {
    pub data: Option<ListMailGroupAliasData>,
}

impl ApiResponseTrait for ListMailGroupAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailGroupAliasData {
    pub aliases: Vec<MailGroupAlias>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupAlias {
    pub alias_id: String,
    pub alias: String,
}

impl ListMailGroupAliasRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<ListMailGroupAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListMailGroupAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/aliases",
            self.mailgroup_id
        );
        let req: ApiRequest<ListMailGroupAliasResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取邮件组所有别名", "响应数据为空")
        })
    }
}

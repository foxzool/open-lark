#!/bin/bash
# 批量创建 mailgroup API 文件

BASE="crates/openlark-mail/src/mail/mail/v1"

# mailgroup/alias
cat > $BASE/mailgroup/alias/create.rs << 'EOF'
//! 创建邮件组别名

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
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
pub struct CreateMailGroupAliasRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: CreateMailGroupAliasBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMailGroupAliasBody {
    pub alias: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailGroupAliasResponse {
    pub data: Option<MailGroupAliasData>,
}

impl ApiResponseTrait for CreateMailGroupAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupAliasData {
    pub alias_id: String,
}

impl CreateMailGroupAliasRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: CreateMailGroupAliasBody::default(),
        }
    }

    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.body.alias = alias.into();
        self
    }

    pub async fn execute(self) -> SDKResult<CreateMailGroupAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailGroupAliasResponse> {
        let path = format!("/open-apis/mail/v1/mailgroups/{}/aliases", self.mailgroup_id);
        let req: ApiRequest<CreateMailGroupAliasResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "创建邮件组别名")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("创建邮件组别名", "响应数据为空")
        })
    }
}
EOF

cat > $BASE/mailgroup/alias/delete.rs << 'EOF'
//! 删除邮件组别名

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DeleteMailGroupAliasRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    alias_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailGroupAliasResponse {
    pub data: Option<serde_json::Value>,
}

impl DeleteMailGroupAliasRequest {
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        alias_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            alias_id: alias_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteMailGroupAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailGroupAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/aliases/{}",
            self.mailgroup_id, self.alias_id
        );
        let req: ApiRequest<DeleteMailGroupAliasResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(resp)
    }
}
EOF

cat > $BASE/mailgroup/alias/mod.rs << 'EOF'
pub mod create;
pub mod delete;
EOF

# mailgroup/member
cat > $BASE/mailgroup/member/create.rs << 'EOF'
//! 创建邮件组成员

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
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
pub struct CreateMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: CreateMailGroupMemberBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMailGroupMemberBody {
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailGroupMemberResponse {
    pub data: Option<MailGroupMemberData>,
}

impl ApiResponseTrait for CreateMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberData {
    pub member_id: String,
}

impl CreateMailGroupMemberRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: CreateMailGroupMemberBody::default(),
        }
    }

    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.body.member_id = member_id.into();
        self
    }

    pub fn member_type(mut self, member_type: impl Into<String>) -> Self {
        self.body.member_type = Some(member_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<CreateMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailGroupMemberResponse> {
        let path = format!("/open-apis/mail/v1/mailgroups/{}/members", self.mailgroup_id);
        let req: ApiRequest<CreateMailGroupMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "创建邮件组成员")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("创建邮件组成员", "响应数据为空")
        })
    }
}
EOF

cat > $BASE/mailgroup/member/delete.rs << 'EOF'
//! 删除邮件组成员

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DeleteMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    member_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailGroupMemberResponse {
    pub data: Option<serde_json::Value>,
}

impl DeleteMailGroupMemberRequest {
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        member_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            member_id: member_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/{}",
            self.mailgroup_id, self.member_id
        );
        let req: ApiRequest<DeleteMailGroupMemberResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(resp)
    }
}
EOF

cat > $BASE/mailgroup/member/batch_create.rs << 'EOF'
//! 批量创建邮件组成员

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
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
pub struct BatchCreateMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchCreateMailGroupMemberBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateMailGroupMemberBody {
    pub members: Vec<MailGroupMemberItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberItem {
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupMemberResponse {
    pub data: Option<BatchCreateMailGroupMemberData>,
}

impl ApiResponseTrait for BatchCreateMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupMemberData {
    pub results: Vec<MailGroupMemberResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberResult {
    pub member_id: String,
    pub status: String,
}

impl BatchCreateMailGroupMemberRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchCreateMailGroupMemberBody::default(),
        }
    }

    pub fn members(mut self, members: Vec<MailGroupMemberItem>) -> Self {
        self.body.members = members;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/batch_create",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchCreateMailGroupMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "批量创建邮件组成员")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量创建邮件组成员", "响应数据为空")
        })
    }
}
EOF

cat > $BASE/mailgroup/member/mod.rs << 'EOF'
pub mod batch_create;
pub mod create;
pub mod delete;
EOF

echo "✅ mailgroup 子模块已创建"

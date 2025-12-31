//! 更新自定义角色
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/advanced-permission/base-v2/app-role/update
//! doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/advanced-permission/base-v2/app-role/update

use crate::base::base::v2::models::AppRole;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

/// 更新自定义角色
#[derive(Debug)]
pub struct Update {
    config: Config,
    app_token: String,
    role_id: String,
    req: UpdateReq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReq {
    /// 自定义角色的名字
    pub role_name: String,
    /// 数据表权限配置列表（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_roles: Option<Vec<serde_json::Value>>,
    /// Block 权限配置列表（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<serde_json::Value>>,
    /// Base 规则（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_rule: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResp {
    /// 自定义角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AppRole>,
}

impl Update {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
            req: UpdateReq {
                role_name: String::new(),
                table_roles: None,
                block_roles: None,
                base_rule: None,
            },
        }
    }

    /// 应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 角色ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 自定义角色的名字
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.req.role_name = role_name.into();
        self
    }

    /// 数据表权限配置列表（table_roles）
    pub fn table_roles(mut self, table_roles: Vec<serde_json::Value>) -> Self {
        self.req.table_roles = Some(table_roles);
        self
    }

    /// Block 权限配置列表（block_roles）
    pub fn block_roles(mut self, block_roles: Vec<serde_json::Value>) -> Self {
        self.req.block_roles = Some(block_roles);
        self
    }

    /// Base 规则（base_rule）
    pub fn base_rule(mut self, base_rule: serde_json::Value) -> Self {
        self.req.base_rule = Some(base_rule);
        self
    }

    pub async fn send(self) -> SDKResult<UpdateResp> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.role_id, "role_id 不能为空");
        validate_required!(self.req.role_name, "role_name 不能为空");
        if self.req.role_name.chars().count() > 100 {
            return Err(openlark_core::error::validation_error(
                "role_name",
                "role_name 长度不能超过 100 字符",
            ));
        }
        if let Some(table_roles) = &self.req.table_roles {
            if table_roles.len() > 100 {
                return Err(openlark_core::error::validation_error(
                    "table_roles",
                    "table_roles 长度不能超过 100",
                ));
            }
        }

        use crate::common::api_endpoints::BaseApiV2;
        let api_endpoint = BaseApiV2::RoleUpdate(self.app_token, self.role_id);

        let api_request: ApiRequest<UpdateResp> = ApiRequest::put(&api_endpoint.to_url())
            .body(serialize_params(&self.req, "更新自定义角色")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "更新自定义角色")
    }
}

impl ApiResponseTrait for UpdateResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
